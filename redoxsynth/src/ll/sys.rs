pub type LogLevel = u32;
pub const LAST_LOG_LEVEL: LogLevel = 5;
pub type LogFn =
    Option<unsafe fn(_: i32, _: *mut i8, _: *mut libc::c_void) -> ()>;
static mut FLUID_ERRBUF: [u8; 512] = [0; 512];
static mut FLUID_LOG_FUNCTION: [LogFn; 5] = [None; 5];
static mut FLUID_LOG_USER_DATA: [*mut libc::c_void; 5] =
    [0 as *const libc::c_void as *mut libc::c_void; 5];

pub unsafe fn fluid_sys_config() {}

pub unsafe fn fluid_set_log_function(
    level: i32,
    fun: LogFn,
    data: *mut libc::c_void,
) -> LogFn {
    let mut old: LogFn = None;
    if level >= 0 as i32 && level < LAST_LOG_LEVEL as i32 {
        old = FLUID_LOG_FUNCTION[level as usize];
        FLUID_LOG_FUNCTION[level as usize] = fun;
        FLUID_LOG_USER_DATA[level as usize] = data
    }
    return old;
}

pub unsafe fn fluid_error() -> *mut u8 {
    return FLUID_ERRBUF.as_mut_ptr();
}

pub unsafe fn fluid_default_log_function(
    _level: i32,
    _message: *mut i8,
    _data: *mut libc::c_void,
) {
}
