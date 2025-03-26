#![allow(non_camel_case_types, non_snake_case)]
use std::os::raw::c_int;

pub const ARION_BUF_SZ: usize = 0x1000;
pub const ARION_UUID_SZ: usize = 0x24;
pub const ARION_SYSTEM_PAGE_SZ: usize = 0x1000;
pub const ARION_FD_SZ: usize = 0x4;
pub const ARION_UNIX_PATH_MAX: usize = 0x6C;
pub const ARION_MAX_U32: u32 = 0xFFFFFFFF;
pub const ARION_MAX_U64: u64 = 0xFFFFFFFFFFFFFFFF;
pub const ARION_PROCESS_PID: u32 = 0x1;
pub const ARION_CYCLES_PER_THREAD: u32 = 0x1000;

pub const EMPTY: usize = 0;

pub type ADDR = u64;
pub type PROT_FLAGS = u8;
pub type BYTE = u8;
pub type REG = u64;

pub type RVAL8 = u8;
pub type RVAL16 = u16;
pub type RVAL32 = u32;
pub type RVAL64 = u64;
pub type RVAL128 = [BYTE; 16];
pub type RVAL256 = [BYTE; 32];
pub type RVAL512 = [BYTE; 64];

#[repr(C)]
pub union RVAL {
    pub r8: RVAL8,
    pub r16: RVAL16,
    pub r32: RVAL32,
    pub r64: RVAL64,
    pub r128: RVAL128,
    pub r256: RVAL256,
    pub r512: RVAL512,
}

#[repr(C)]
pub struct SEGMENT {
    pub virt_addr: ADDR,
    pub file_addr: ADDR,
    pub align: usize,
    pub virt_sz: usize,
    pub phy_sz: usize,
    pub flags: PROT_FLAGS,
}

#[repr(C)]
pub struct SIGNAL {
    pub source_pid: libc::pid_t,
    pub signo: c_int,
}

impl SIGNAL {
    pub fn new(source_pid: libc::pid_t, signo: c_int) -> Self {
        Self { source_pid, signo }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum LINKAGE_TYPE {
    UNKNOWN_LINKAGE = 0,
    DYNAMIC_LINKAGE = 1,
    STATIC_LINKAGE = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum CPU_ARCH {
    UNKNOWN_ARCH = 0,
    X86_ARCH = 1,
    X8664_ARCH = 2,
    ARM_ARCH = 3,
    ARM64_ARCH = 4,
}

#[derive(Debug, Clone, Copy)]
pub enum ARION_LOG_LEVEL {
    TRACE = 0,
    DEBUG = 1,
    INFO = 2,
    WARN = 3,
    ERROR = 4,
    CRITICAL = 5,
    OFF = 6,
}
