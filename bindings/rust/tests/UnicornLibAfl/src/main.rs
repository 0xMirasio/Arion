use unicorn_engine::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};
use unicorn_engine::{RegisterX86, Unicorn};
use libafl::{
    corpus::{Corpus, InMemoryCorpus, Testcase,OnDiskCorpus},
    events::SimpleEventManager,
    executors::{ExitKind, InProcessExecutor},
    feedbacks::{TimeFeedback,MaxMapFeedback,CrashFeedback,TimeoutFeedback},
    fuzzer::StdFuzzer,
    mutators::{havoc_mutations, StdScheduledMutator},
    observers::{StdMapObserver,HitcountsMapObserver,CanTrack,TimeObserver},
    schedulers::QueueScheduler,
    stages::StdMutationalStage,
    state::{HasCorpus, StdState},
    Fuzzer,
    inputs::{BytesInput, HasMutatorBytes},
    monitors::SimpleMonitor,
    {feedback_or, feedback_or_fast},
};



use libafl_bolts::rands::StdRand;
use libafl_bolts::tuples::tuple_list;
use libafl_targets::{EDGES_MAP, MAX_EDGES_FOUND};
use std::path::PathBuf;

const PAGE_SIZE : usize = 0x4000;
const TEXT_START: u64 = 0x1000;
const TEXT_END: u64 = TEXT_START + CODE.len() as u64 - 1;
const STACK_ADDR: u64 = 0x7000_0000;
const STACK_SIZE: usize = 0x10000; // 64 KB=
const DATA_ADDR: u64 = 0x8000;
const CODE: [u8; 62] = [0xf3,0xf,0x1e,0xfa,0x55,0x48,0x89,0xe5,0x89,0x7d,0xec,0x48,0x89,0x75,0xe0,0xb8,0xef,0xbe,0xad,0xde,0x48,0x89,0x45,0xf8,0xb8,0x0,0x80,0x0,0x0,0xf,0xb6,0x10,0x80,0xfa,0x29,0x75,0x12,0x48,0x83,0xc0,0x1,0xf,0xb6,0x0,0x3c,0x2a,0x75,0x7,0x48,0x8b,0x45,0xf8,0xc6,0x0,0x0,0xb8,0x0,0x0,0x0,0x0,0x5d,0xc3];
/*
unsigned char * data_b1 = (unsigned char*)0x8000;
unsigned char * data_b2 = (unsigned char*)0x8001;

int main(int argc, char ** argv) {

    char * will_crash=(char*)0xdeadbeef;

    if ((unsigned char)*data_b1 == 41){
        if ((unsigned char)*data_b2 == 42){
            *will_crash = 0; // CRASH
        }

    }
    return 0;
}
*/


fn harness(input: &BytesInput) -> ExitKind {
    let mut uc = Unicorn::new(Arch::X86, Mode::MODE_64).expect("Failed to initialize Unicorn instance");
    let raw_input = input.mutator_bytes();

    uc.mem_map(TEXT_START, PAGE_SIZE, Permission::ALL).expect("Failed to map text");
    uc.mem_write(TEXT_START, &CODE).expect("Failed to write instructions");
    uc.mem_map(DATA_ADDR, PAGE_SIZE, Permission::ALL).expect("Failed to map rodata");
    uc.mem_map(STACK_ADDR, STACK_SIZE, Permission::READ | Permission::WRITE).expect("Failed to map stack");
    let stack_top = STACK_ADDR + STACK_SIZE as u64 - 0x10; // 16-byte aligned
    uc.reg_write(RegisterX86::RSP, stack_top).expect("Failed to set RSP");

    uc.mem_write(DATA_ADDR, &raw_input).expect("Failed to write input data");
    uc.add_code_hook(TEXT_START, TEXT_START + PAGE_SIZE as u64, move |uc, address, _size| {
        if address >= TEXT_END {
            uc.emu_stop().unwrap();
        }
    }).unwrap();

    uc.emu_start(TEXT_START, TEXT_START + CODE.len() as u64, 10 * SECOND_SCALE, 1000).unwrap();
    ExitKind::Ok
}

fn main() {
    // Setup observer and feedback
    #[allow(static_mut_refs)] // only a problem on nightly
    let edges_observer = unsafe {
        HitcountsMapObserver::new(StdMapObserver::from_mut_ptr(
            "edges",
            EDGES_MAP.as_mut_ptr(),
            MAX_EDGES_FOUND,
        ))
        .track_indices()
    };

    // Create an observation channel to keep track of the execution time
    let time_observer = TimeObserver::new("time");
    let map_feedback = MaxMapFeedback::new(&edges_observer);

    let mut feedback = feedback_or!(
        map_feedback,
        TimeFeedback::new(&time_observer)
    );

    let mut objective = feedback_or_fast!(CrashFeedback::new(), TimeoutFeedback::new());

    let mon = SimpleMonitor::new(|s| println!("{s}"));

    let objective_dir = PathBuf::from("crashs");
    if objective_dir.exists() {
        std::fs::remove_dir_all(&objective_dir).unwrap();
    }
    std::fs::create_dir_all(&objective_dir).unwrap();
    

    // Setup corpus and state
    let mut state = StdState::new(
        StdRand::with_seed(0),
        InMemoryCorpus::new(),
        OnDiskCorpus::new(objective_dir).unwrap(),
        &mut feedback,
        &mut objective,
    )
    .unwrap();

    // Setup event manager
    let mut mgr = SimpleEventManager::new(mon);

    // Setup fuzzer
    let scheduler = QueueScheduler::new();
    let mut fuzzer = StdFuzzer::new(scheduler, feedback, objective);

    // Create the executor for an in-process function with just one observer
    let mut binding = harness;
    let mut executor = InProcessExecutor::new(
        &mut binding,
        tuple_list!(edges_observer, time_observer),
        &mut fuzzer,
        &mut state,
        &mut mgr,
    )
    .expect("Failed to create the Executor");

    // Add initial test input
    state
        .corpus_mut()
        .add(Testcase::new(b"AAAA".to_vec().into()))
        .expect("Failed to generate the initial corpus");

    let mutator = StdScheduledMutator::new(havoc_mutations());
    let mut stages = tuple_list!(StdMutationalStage::new(mutator));

    fuzzer
        .fuzz_loop(&mut stages, &mut executor, &mut state, &mut mgr)
        .expect("Error in the fuzzing loop");
}
