use arion_engine::common::global_def::ARION_LOG_LEVEL;
use arion_engine::common::config;

fn main() {
    println!("Arion Debug level -> {:?}", ARION_LOG_LEVEL::DEBUG as i32);
    let config = config::new_config();

    //let is_sleep_enabled = config.is_enable_sleep_syscalls();
    //println!("config -> {}", is_sleep_enabled);


}
