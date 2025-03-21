use unicorn_engine::unicorn_const::{Arch, Mode, Permission, SECOND_SCALE};
use unicorn_engine::{RegisterX86, Unicorn};
use capstone::prelude::*;

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
const PAGE_SIZE : usize = 0x4000;
const TEXT_START: u64 = 0x1000;
const TEXT_END: u64 = TEXT_START + CODE.len() as u64 - 1;
const STACK_ADDR: u64 = 0x7000_0000;
const STACK_SIZE: usize = 0x10000; // 64 KB=
const DATA_ADDR: u64 = 0x8000;

fn main() {

    let mut uc = Unicorn::new(Arch::X86, Mode::MODE_64).expect("Failed to initialize Unicorn instance");
    println!("Unicorn Engine Inited");

    uc.mem_map(TEXT_START, PAGE_SIZE, Permission::ALL).expect("Failed to map text");
    uc.mem_write(TEXT_START, &CODE).expect("Failed to write instructions");
    println!("Map addr: 0x{:x}, size: 0x{:x} RWX TEXT", TEXT_START, PAGE_SIZE);

    uc.mem_map(DATA_ADDR, PAGE_SIZE, Permission::ALL).expect("Failed to map rodata");
    println!("Map addr: 0x{:x}, size: 0x{:x} RWX RODATA", DATA_ADDR, PAGE_SIZE);

    uc.mem_map(STACK_ADDR, STACK_SIZE, Permission::READ | Permission::WRITE).expect("Failed to map stack");
    let stack_top = STACK_ADDR + STACK_SIZE as u64 - 0x10; // 16-byte aligned
    uc.reg_write(RegisterX86::RSP, stack_top).expect("Failed to set RSP");
    println!("Map addr: 0x{:x}, size: 0x{:x} RW STACK", STACK_ADDR, STACK_SIZE);

    let cs = Capstone::new()
        .x86()
        .mode(capstone::arch::x86::ArchMode::Mode64)
        .build()
        .expect("Failed to create Capstone");

    let cs = std::sync::Arc::new(cs);
    let cs_clone = cs.clone();

    uc.add_code_hook(TEXT_START, TEXT_START + PAGE_SIZE as u64, move |uc, address, size| {
        let mut buf = vec![0u8; size as usize];
        if let Ok(_) = uc.mem_read(address, &mut buf) {
            if let Ok(insns) = cs_clone.disasm_all(&buf, address) {
                for insn in insns.iter() {
                    let rsp = uc.reg_read(RegisterX86::RSP).unwrap();
                    let rax = uc.reg_read(RegisterX86::RAX).unwrap();
                    let rip = uc.reg_read(RegisterX86::RIP).unwrap();
                    let rdx = uc.reg_read(RegisterX86::RDX).unwrap();
                    println!("0x{:x}:\t{}\t{}", rip, insn.mnemonic().unwrap_or(""), insn.op_str().unwrap_or(""));
                    println!("RSP=0x{:x} | RAX=0x{:x} | RDX=0x{:x}", rsp, rax, rdx);
                }
            }
        }
        if address >= TEXT_END {
            println!("END_ADDRESS!");
            uc.emu_stop().unwrap();
        }

    }).unwrap();

    uc.emu_start(TEXT_START, PAGE_SIZE as u64, 10 * SECOND_SCALE, 1000).expect("Failed to start emulation");
    println!("Emu Ended");
}
