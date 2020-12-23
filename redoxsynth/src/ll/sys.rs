pub type LogLevel = libc::c_uint;
pub const LAST_LOG_LEVEL: LogLevel = 5;
pub type LogFn =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char, _: *mut libc::c_void) -> ()>;
static mut FLUID_ERRBUF: [libc::c_char; 512] = [0; 512];
static mut FLUID_LOG_FUNCTION: [LogFn; 5] = [None; 5];
static mut FLUID_LOG_USER_DATA: [*mut libc::c_void; 5] =
    [0 as *const libc::c_void as *mut libc::c_void; 5];
#[no_mangle]
pub unsafe extern "C" fn fluid_sys_config() {}
#[no_mangle]
pub static mut fluid_debug_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn fluid_set_log_function(
    level: libc::c_int,
    fun: LogFn,
    data: *mut libc::c_void,
) -> LogFn {
    let mut old: LogFn = None;
    if level >= 0 as libc::c_int && level < LAST_LOG_LEVEL as libc::c_int {
        old = FLUID_LOG_FUNCTION[level as usize];
        FLUID_LOG_FUNCTION[level as usize] = fun;
        FLUID_LOG_USER_DATA[level as usize] = data
    }
    return old;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_log_config() {}
#[no_mangle]
pub unsafe extern "C" fn fluid_strtok(
    str: *mut *mut libc::c_char,
    delim: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s;
    let mut d;
    let token;
    let mut c;
    if str.is_null() || delim.is_null() || *delim == 0 {
        fluid_log!(FLUID_ERR, "Null pointer",);
        return 0 as *mut libc::c_char;
    }
    s = *str;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    loop {
        c = *s;
        if c == 0 {
            *str = 0 as *mut libc::c_char;
            return 0 as *mut libc::c_char;
        }
        d = delim;
        while *d != 0 {
            if c as libc::c_int == *d as libc::c_int {
                s = s.offset(1);
                break;
            } else {
                d = d.offset(1)
            }
        }
        if !(*d != 0) {
            break;
        }
    }
    token = s;
    s = s.offset(1 as libc::c_int as isize);
    while *s != 0 {
        c = *s;
        d = delim;
        while *d != 0 {
            if c as libc::c_int == *d as libc::c_int {
                *s = '\u{0}' as i32 as libc::c_char;
                *str = s.offset(1 as libc::c_int as isize);
                return token;
            }
            d = d.offset(1)
        }
        s = s.offset(1)
    }
    *str = 0 as *mut libc::c_char;
    return token;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_error() -> *mut libc::c_char {
    return FLUID_ERRBUF.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn fluid_is_midifile(filename: *mut libc::c_char) -> libc::c_int {
    let fp: *mut libc::FILE =
        libc::fopen(filename, b"rb\x00" as *const u8 as *const libc::c_char);
    let mut id: [libc::c_char; 4] = [0; 4];
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    if libc::fread(
        id.as_mut_ptr() as *mut libc::c_void,
        1 as libc::size_t,
        4 as libc::size_t,
        fp,
    ) != 4 as libc::size_t
    {
        libc::fclose(fp);
        return 0 as libc::c_int;
    }
    libc::fclose(fp);
    return (libc::strncmp(
        id.as_mut_ptr(),
        b"MThd\x00" as *const u8 as *const libc::c_char,
        4 as libc::size_t,
    ) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_is_soundfont(filename: *mut libc::c_char) -> libc::c_int {
    let fp: *mut libc::FILE =
        libc::fopen(filename, b"rb\x00" as *const u8 as *const libc::c_char);
    let mut id: [libc::c_char; 4] = [0; 4];
    if fp.is_null() {
        return 0 as libc::c_int;
    }
    if libc::fread(
        id.as_mut_ptr() as *mut libc::c_void,
        1 as libc::size_t,
        4 as libc::size_t,
        fp,
    ) != 4 as libc::size_t
    {
        libc::fclose(fp);
        return 0 as libc::c_int;
    }
    libc::fclose(fp);
    return (libc::strncmp(
        id.as_mut_ptr(),
        b"RIFF\x00" as *const u8 as *const libc::c_char,
        4 as libc::size_t,
    ) == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_default_log_function(
    _level: libc::c_int,
    _message: *mut libc::c_char,
    _data: *mut libc::c_void,
) {
}
