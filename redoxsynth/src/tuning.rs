#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
pub type fluid_log_level = libc::c_uint;
pub const LAST_LOG_LEVEL: fluid_log_level = 5;
pub const FLUID_DBG: fluid_log_level = 4;
pub const FLUID_INFO: fluid_log_level = 3;
pub const FLUID_WARN: fluid_log_level = 2;
pub const FLUID_ERR: fluid_log_level = 1;
pub const FLUID_PANIC: fluid_log_level = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_tuning_t {
    pub name: *mut libc::c_char,
    pub bank: libc::c_int,
    pub prog: libc::c_int,
    pub pitch: [libc::c_double; 128],
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_tuning(
    mut name: *const libc::c_char,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
) -> *mut fluid_tuning_t {
    let mut tuning: *mut fluid_tuning_t = 0 as *mut fluid_tuning_t;
    let mut i: libc::c_int = 0;
    tuning = libc::malloc(::std::mem::size_of::<fluid_tuning_t>() as libc::size_t)
        as *mut fluid_tuning_t;
    if tuning.is_null() {
        fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
        return 0 as *mut fluid_tuning_t;
    }
    (*tuning).name = 0 as *mut libc::c_char;
    if !name.is_null() {
        (*tuning).name = libc::strcpy(
            libc::malloc(libc::strlen(name) + 1) as *mut libc::c_char,
            name,
        )
    }
    (*tuning).bank = bank;
    (*tuning).prog = prog;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        (*tuning).pitch[i as usize] = i as libc::c_double * 100.0f64;
        i += 1
    }
    return tuning;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_duplicate(
    mut tuning: *mut fluid_tuning_t,
) -> *mut fluid_tuning_t {
    let mut new_tuning: *mut fluid_tuning_t = 0 as *mut fluid_tuning_t;
    let mut i: libc::c_int = 0;
    new_tuning = libc::malloc(::std::mem::size_of::<fluid_tuning_t>() as libc::size_t)
        as *mut fluid_tuning_t;
    if new_tuning.is_null() {
        fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
        return 0 as *mut fluid_tuning_t;
    }
    if !(*tuning).name.is_null() {
        (*new_tuning).name = libc::strcpy(
            libc::malloc(libc::strlen((*tuning).name) + 1) as *mut libc::c_char,
            (*tuning).name,
        );
        if (*new_tuning).name.is_null() {
            libc::free(new_tuning as *mut libc::c_void);
            fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
            return 0 as *mut fluid_tuning_t;
        }
    } else {
        (*new_tuning).name = 0 as *mut libc::c_char
    }
    (*new_tuning).bank = (*tuning).bank;
    (*new_tuning).prog = (*tuning).prog;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        (*new_tuning).pitch[i as usize] = (*tuning).pitch[i as usize];
        i += 1
    }
    return new_tuning;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_tuning(mut tuning: *mut fluid_tuning_t) {
    if tuning.is_null() {
        return;
    }
    if !(*tuning).name.is_null() {
        libc::free((*tuning).name as *mut libc::c_void);
    }
    libc::free(tuning as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_name(
    mut tuning: *mut fluid_tuning_t,
    mut name: *const libc::c_char,
) {
    if !(*tuning).name.is_null() {
        libc::free((*tuning).name as *mut libc::c_void);
        (*tuning).name = 0 as *mut libc::c_char
    }
    if !name.is_null() {
        (*tuning).name = libc::strcpy(
            libc::malloc(libc::strlen(name) + 1) as *mut libc::c_char,
            name,
        )
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_get_name(
    mut tuning: *mut fluid_tuning_t,
) -> *mut libc::c_char {
    return (*tuning).name;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_key(
    mut tuning: *mut fluid_tuning_t,
    mut key: libc::c_int,
    mut pitch: libc::c_double,
) {
    (*tuning).pitch[key as usize] = pitch;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_octave(
    mut tuning: *mut fluid_tuning_t,
    mut pitch_deriv: *const libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        (*tuning).pitch[i as usize] =
            i as libc::c_double * 100.0f64 + *pitch_deriv.offset((i % 12 as libc::c_int) as isize);
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_all(
    mut tuning: *mut fluid_tuning_t,
    mut pitch: *mut libc::c_double,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        (*tuning).pitch[i as usize] = *pitch.offset(i as isize);
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_pitch(
    mut tuning: *mut fluid_tuning_t,
    mut key: libc::c_int,
    mut pitch: libc::c_double,
) {
    if key >= 0 as libc::c_int && key < 128 as libc::c_int {
        (*tuning).pitch[key as usize] = pitch
    };
}
