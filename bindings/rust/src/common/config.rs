#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("arion/common/config.hpp");

        type Config;
        pub fn new_config() -> UniquePtr<Config>;
        pub fn get_log_level(self: &Config) -> i32;
    }
}

pub use ffi::*;