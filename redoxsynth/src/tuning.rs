#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_tuning_t {
    pub name: *mut libc::c_char,
    pub bank: libc::c_int,
    pub prog: libc::c_int,
    pub pitch: [f64; 128],
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_tuning(
    mut name: *const libc::c_char,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
) -> *mut fluid_tuning_t {
    let mut tuning;
    let mut i;
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
        (*tuning).pitch[i as usize] = i as f64 * 100.0f64;
        i += 1
    }
    return tuning;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_duplicate(
    mut tuning: *mut fluid_tuning_t,
) -> *mut fluid_tuning_t {
    let mut new_tuning;
    let mut i;
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
    mut pitch: f64,
) {
    (*tuning).pitch[key as usize] = pitch;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_octave(
    mut tuning: *mut fluid_tuning_t,
    mut pitch_deriv: *const f64,
) {
    let mut i;
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        (*tuning).pitch[i as usize] =
            i as f64 * 100.0f64 + *pitch_deriv.offset((i % 12 as libc::c_int) as isize);
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_all(
    mut tuning: *mut fluid_tuning_t,
    mut pitch: *mut f64,
) {
    let mut i;
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
    mut pitch: f64,
) {
    if key >= 0 as libc::c_int && key < 128 as libc::c_int {
        (*tuning).pitch[key as usize] = pitch
    };
}
