#[derive(Copy, Clone)]
#[repr(C)]
pub struct Tuning {
    pub name: *mut libc::c_char,
    pub bank: libc::c_int,
    pub prog: libc::c_int,
    pub pitch: [f64; 128],
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_tuning(
    name: *const libc::c_char,
    bank: libc::c_int,
    prog: libc::c_int,
) -> *mut Tuning {
    let mut tuning;
    let mut i;
    tuning = libc::malloc(::std::mem::size_of::<Tuning>() as libc::size_t)
        as *mut Tuning;
    if tuning.is_null() {
        fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
        return 0 as *mut Tuning;
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
    tuning: *mut Tuning,
) -> *mut Tuning {
    let mut new_tuning;
    let mut i;
    new_tuning = libc::malloc(::std::mem::size_of::<Tuning>() as libc::size_t)
        as *mut Tuning;
    if new_tuning.is_null() {
        fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
        return 0 as *mut Tuning;
    }
    if !(*tuning).name.is_null() {
        (*new_tuning).name = libc::strcpy(
            libc::malloc(libc::strlen((*tuning).name) + 1) as *mut libc::c_char,
            (*tuning).name,
        );
        if (*new_tuning).name.is_null() {
            libc::free(new_tuning as *mut libc::c_void);
            fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
            return 0 as *mut Tuning;
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
pub unsafe extern "C" fn delete_fluid_tuning(tuning: *mut Tuning) {
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
    mut tuning: *mut Tuning,
    name: *const libc::c_char,
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
    tuning: *mut Tuning,
) -> *mut libc::c_char {
    return (*tuning).name;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_key(
    mut tuning: *mut Tuning,
    key: libc::c_int,
    pitch: f64,
) {
    (*tuning).pitch[key as usize] = pitch;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_tuning_set_octave(
    mut tuning: *mut Tuning,
    pitch_deriv: *const f64,
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
    mut tuning: *mut Tuning,
    pitch: *mut f64,
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
    mut tuning: *mut Tuning,
    key: libc::c_int,
    pitch: f64,
) {
    if key >= 0 as libc::c_int && key < 128 as libc::c_int {
        (*tuning).pitch[key as usize] = pitch
    };
}
