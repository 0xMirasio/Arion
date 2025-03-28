#[cxx::bridge]
pub mod ffi {
    unsafe extern "C++" {
        include!("arion/common/config.hpp");

        type Config;
        pub fn new_config() -> UniquePtr<Config>;
        
        fn get_enable_sleep_syscalls(self: &Config) -> bool;
    }
}

impl Config {
    pub fn is_enable_sleep_syscalls(&self) -> bool {
        self.get_enable_sleep_syscalls()
    }
}

pub use ffi::*;