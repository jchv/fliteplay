#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(c_variadic, const_raw_ptr_to_usize_cast, const_transmute)]

pub type fluid_log_level = libc::c_uint;
pub const LAST_LOG_LEVEL: fluid_log_level = 5;
pub const FLUID_DBG: fluid_log_level = 4;
pub const FLUID_INFO: fluid_log_level = 3;
pub const FLUID_WARN: fluid_log_level = 2;
pub const FLUID_ERR: fluid_log_level = 1;
pub const FLUID_PANIC: fluid_log_level = 0;
pub type fluid_log_function_t =
    Option<unsafe extern "C" fn(_: libc::c_int, _: *mut libc::c_char, _: *mut libc::c_void) -> ()>;
pub const FLUID_FAILED: C2RustUnnamed = -1;
pub type C2RustUnnamed = libc::c_int;
pub const FLUID_OK: C2RustUnnamed = 0;

static mut fluid_errbuf: [libc::c_char; 512] = [0; 512];

static mut fluid_log_function: [fluid_log_function_t; 5] = [None; 5];
static mut fluid_log_user_data: [*mut libc::c_void; 5] =
    [0 as *const libc::c_void as *mut libc::c_void; 5];
static mut fluid_log_initialized: libc::c_int = 0 as libc::c_int;
static mut fluid_libname: *mut libc::c_char =
    b"fluidsynth\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
#[no_mangle]
pub unsafe extern "C" fn fluid_sys_config() {}
#[no_mangle]
pub static mut fluid_debug_flags: libc::c_uint = 0 as libc::c_int as libc::c_uint;

#[no_mangle]
pub unsafe extern "C" fn fluid_set_log_function(
    mut level: libc::c_int,
    mut fun: fluid_log_function_t,
    mut data: *mut libc::c_void,
) -> fluid_log_function_t {
    let mut old: fluid_log_function_t = None;
    if level >= 0 as libc::c_int && level < LAST_LOG_LEVEL as libc::c_int {
        old = fluid_log_function[level as usize];
        fluid_log_function[level as usize] = fun;
        fluid_log_user_data[level as usize] = data
    }
    return old;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_log_config() {}

#[no_mangle]
pub unsafe extern "C" fn fluid_strtok(
    mut str: *mut *mut libc::c_char,
    mut delim: *mut libc::c_char,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut d: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut c: libc::c_char = 0;
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
    return fluid_errbuf.as_mut_ptr();
}

#[no_mangle]
pub unsafe extern "C" fn fluid_is_midifile(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut fp: *mut libc::FILE =
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
pub unsafe extern "C" fn fluid_is_soundfont(mut filename: *mut libc::c_char) -> libc::c_int {
    let mut fp: *mut libc::FILE =
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
    mut level: libc::c_int,
    mut message: *mut libc::c_char,
    mut data: *mut libc::c_void,
) {
}
