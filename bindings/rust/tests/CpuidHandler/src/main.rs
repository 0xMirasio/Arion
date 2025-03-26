use arion_engine::common::global_def::ARION_LOG_LEVEL;
use arion_engine::common::config;

fn main() {
    println!("Arion Debug level -> {:?}", ARION_LOG_LEVEL::DEBUG as i32);
    let config = config::new_config();
    println!("config -> {:p}", config.get_field("log_lvl"));

}
