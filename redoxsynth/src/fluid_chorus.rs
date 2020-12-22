#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
extern "C" {
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn cos(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn sin(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn fabs(_: libc::c_double) -> libc::c_double;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
}

pub type fluid_chorus_mod = libc::c_uint;
pub const FLUID_CHORUS_MOD_TRIANGLE: fluid_chorus_mod = 1;
pub const FLUID_CHORUS_MOD_SINE: fluid_chorus_mod = 0;

pub type fluid_log_level = libc::c_uint;

pub const LAST_LOG_LEVEL: fluid_log_level = 5;

pub const FLUID_DBG: fluid_log_level = 4;

pub const FLUID_INFO: fluid_log_level = 3;

pub const FLUID_WARN: fluid_log_level = 2;

pub const FLUID_ERR: fluid_log_level = 1;
pub const FLUID_PANIC: fluid_log_level = 0;

pub type fluid_real_t = libc::c_float;
pub type C2RustUnnamed = libc::c_int;
pub const FLUID_FAILED: C2RustUnnamed = -1;
pub const FLUID_OK: C2RustUnnamed = 0;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_chorus_t {
    pub type_0: libc::c_int,
    pub new_type: libc::c_int,
    pub depth_ms: fluid_real_t,
    pub new_depth_ms: fluid_real_t,
    pub level: fluid_real_t,
    pub new_level: fluid_real_t,
    pub speed_Hz: fluid_real_t,
    pub new_speed_Hz: fluid_real_t,
    pub number_blocks: libc::c_int,
    pub new_number_blocks: libc::c_int,
    pub chorusbuf: *mut fluid_real_t,
    pub counter: libc::c_int,
    pub phase: [libc::c_long; 99],
    pub modulation_period_samples: libc::c_long,
    pub lookup_tab: *mut libc::c_int,
    pub sample_rate: fluid_real_t,
    pub sinc_table: [[fluid_real_t; 128]; 5],
}
pub type fluid_chorus_t = _fluid_chorus_t;
#[no_mangle]
pub unsafe extern "C" fn new_fluid_chorus(mut sample_rate: fluid_real_t) -> *mut fluid_chorus_t {
    let mut i: libc::c_int = 0;
    let mut ii: libc::c_int = 0;
    let mut chorus: *mut fluid_chorus_t = 0 as *mut fluid_chorus_t;
    chorus =
        malloc(::std::mem::size_of::<fluid_chorus_t>() as libc::c_ulong) as *mut fluid_chorus_t;
    if chorus.is_null() {
        fluid_log!(
            FLUID_PANIC as libc::c_int,
            "chorus: Out of memory",
        );
        return 0 as *mut fluid_chorus_t;
    }
    memset(
        chorus as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<fluid_chorus_t>() as libc::c_ulong,
    );
    (*chorus).sample_rate = sample_rate;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        ii = 0 as libc::c_int;
        while ii < (1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int {
            let mut i_shifted: libc::c_double = i as libc::c_double
                - 5 as libc::c_int as libc::c_double / 2.0f64
                + ii as libc::c_double
                    / ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) as libc::c_double;
            if fabs(i_shifted) < 0.000001f64 {
                (*chorus).sinc_table[i as usize][ii as usize] = 1.0f64 as fluid_real_t
            } else {
                (*chorus).sinc_table[i as usize][ii as usize] =
                    (sin(i_shifted * 3.14159265358979323846f64) as fluid_real_t as libc::c_double
                        / (3.14159265358979323846f64 * i_shifted))
                        as fluid_real_t;
                (*chorus).sinc_table[i as usize][ii as usize] =
                    ((*chorus).sinc_table[i as usize][ii as usize] as libc::c_double
                        * (0.5f64 as fluid_real_t as libc::c_double
                            * (1.0f64
                                + cos(2.0f64 * 3.14159265358979323846f64 * i_shifted
                                    / 5 as libc::c_int as fluid_real_t as libc::c_double))))
                        as fluid_real_t
            }
            ii += 1
        }
        i += 1
    }

    (*chorus).lookup_tab = malloc(
        (((*chorus).sample_rate as libc::c_double / 0.29f64) as libc::c_int as libc::c_ulong)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::c_ulong),
    ) as *mut libc::c_int;
    if (*chorus).lookup_tab.is_null() {
        fluid_log!(
            FLUID_PANIC as libc::c_int,
            "chorus: Out of memory",
        );
    } else {
        (*chorus).chorusbuf = malloc(
            (((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int) as libc::c_ulong)
                .wrapping_mul(::std::mem::size_of::<fluid_real_t>() as libc::c_ulong),
        ) as *mut fluid_real_t;
        if (*chorus).chorusbuf.is_null() {
            fluid_log!(
                FLUID_PANIC as libc::c_int,
                "chorus: Out of memory",
            );
        } else if !(fluid_chorus_init(chorus) != FLUID_OK as libc::c_int) {
            return chorus;
        }
    }
    delete_fluid_chorus(chorus);
    return 0 as *mut fluid_chorus_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_init(mut chorus: *mut fluid_chorus_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int {
        *(*chorus).chorusbuf.offset(i as isize) = 0.0f64 as fluid_real_t;
        i += 1
    }
    fluid_chorus_set_nr(chorus, 3 as libc::c_int);
    fluid_chorus_set_level(chorus, 2.0f32);
    fluid_chorus_set_speed_Hz(chorus, 0.3f32);
    fluid_chorus_set_depth_ms(chorus, 8.0f32);
    fluid_chorus_set_type(chorus, FLUID_CHORUS_MOD_SINE as libc::c_int);
    return fluid_chorus_update(chorus);
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_nr(mut chorus: *mut fluid_chorus_t, mut nr: libc::c_int) {
    (*chorus).new_number_blocks = nr;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_nr(mut chorus: *mut fluid_chorus_t) -> libc::c_int {
    return (*chorus).number_blocks;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_level(
    mut chorus: *mut fluid_chorus_t,
    mut level: fluid_real_t,
) {
    (*chorus).new_level = level;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_level(mut chorus: *mut fluid_chorus_t) -> fluid_real_t {
    return (*chorus).level;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_speed_Hz(
    mut chorus: *mut fluid_chorus_t,
    mut speed_Hz: fluid_real_t,
) {
    (*chorus).new_speed_Hz = speed_Hz;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_speed_Hz(
    mut chorus: *mut fluid_chorus_t,
) -> fluid_real_t {
    return (*chorus).speed_Hz;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_depth_ms(
    mut chorus: *mut fluid_chorus_t,
    mut depth_ms: fluid_real_t,
) {
    (*chorus).new_depth_ms = depth_ms;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_depth_ms(
    mut chorus: *mut fluid_chorus_t,
) -> fluid_real_t {
    return (*chorus).depth_ms;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_type(
    mut chorus: *mut fluid_chorus_t,
    mut type_0: libc::c_int,
) {
    (*chorus).new_type = type_0;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_type(mut chorus: *mut fluid_chorus_t) -> libc::c_int {
    return (*chorus).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_chorus(mut chorus: *mut fluid_chorus_t) {
    if chorus.is_null() {
        return;
    }
    if !(*chorus).chorusbuf.is_null() {
        free((*chorus).chorusbuf as *mut libc::c_void);
    }
    if !(*chorus).lookup_tab.is_null() {
        free((*chorus).lookup_tab as *mut libc::c_void);
    }
    free(chorus as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_update(mut chorus: *mut fluid_chorus_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut modulation_depth_samples: libc::c_int = 0;
    if (*chorus).new_number_blocks < 0 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "chorus: number blocks must be >=0! Setting value to 0.",
        );
        (*chorus).new_number_blocks = 0 as libc::c_int
    } else if (*chorus).new_number_blocks > 99 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "chorus: number blocks larger than max. allowed! Setting value to {}.",
            99
        );
        (*chorus).new_number_blocks = 99 as libc::c_int
    }
    if ((*chorus).new_speed_Hz as libc::c_double) < 0.29f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: speed is too low (min {})! Setting value to min.",
            0.29f64
        );
        (*chorus).new_speed_Hz = 0.29f64 as fluid_real_t
    } else if (*chorus).new_speed_Hz > 5 as libc::c_int as libc::c_float {
        fluid_log!(
            FLUID_WARN,
            "chorus: speed must be below {} Hz! Setting value to max.",
            5
        );
        (*chorus).new_speed_Hz = 5 as libc::c_int as fluid_real_t
    }
    if ((*chorus).new_depth_ms as libc::c_double) < 0.0f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: depth must be positive! Setting value to 0.",
        );
        (*chorus).new_depth_ms = 0.0f64 as fluid_real_t
    }
    if ((*chorus).new_level as libc::c_double) < 0.0f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: level must be positive! Setting value to 0.",
        );
        (*chorus).new_level = 0.0f64 as fluid_real_t
    } else if (*chorus).new_level > 10 as libc::c_int as libc::c_float {
        fluid_log!(
            FLUID_WARN,
            "chorus: level must be < 10. A reasonable level is << 1! Setting it to 0.1.",
        );
        (*chorus).new_level = 0.1f64 as fluid_real_t
    }
    (*chorus).modulation_period_samples =
        ((*chorus).sample_rate / (*chorus).new_speed_Hz) as libc::c_long;

    modulation_depth_samples = ((*chorus).new_depth_ms as libc::c_double / 1000.0f64
        * (*chorus).sample_rate as libc::c_double) as libc::c_int;
    if modulation_depth_samples > (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "chorus: Too high depth. Setting it to max ({}).",
            (1) << 12 - 1
        );
        modulation_depth_samples = (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int
    }
    if (*chorus).type_0 == FLUID_CHORUS_MOD_SINE as libc::c_int {
        fluid_chorus_sine(
            (*chorus).lookup_tab,
            (*chorus).modulation_period_samples as libc::c_int,
            modulation_depth_samples,
        );
    } else if (*chorus).type_0 == FLUID_CHORUS_MOD_TRIANGLE as libc::c_int {
        fluid_chorus_triangle(
            (*chorus).lookup_tab,
            (*chorus).modulation_period_samples as libc::c_int,
            modulation_depth_samples,
        );
    } else {
        fluid_log!(
            FLUID_WARN,
            "chorus: Unknown modulation type. Using sinewave.",
        );
        (*chorus).type_0 = FLUID_CHORUS_MOD_SINE as libc::c_int;
        fluid_chorus_sine(
            (*chorus).lookup_tab,
            (*chorus).modulation_period_samples as libc::c_int,
            modulation_depth_samples,
        );
    }
    i = 0 as libc::c_int;
    while i < (*chorus).number_blocks {
        (*chorus).phase[i as usize] = ((*chorus).modulation_period_samples as libc::c_double
            * i as libc::c_double
            / (*chorus).number_blocks as libc::c_double)
            as libc::c_int as libc::c_long;
        i += 1
    }
    (*chorus).counter = 0 as libc::c_int;
    (*chorus).type_0 = (*chorus).new_type;
    (*chorus).depth_ms = (*chorus).new_depth_ms;
    (*chorus).level = (*chorus).new_level;
    (*chorus).speed_Hz = (*chorus).new_speed_Hz;
    (*chorus).number_blocks = (*chorus).new_number_blocks;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_processmix(
    mut chorus: *mut fluid_chorus_t,
    mut in_0: *mut fluid_real_t,
    mut left_out: *mut fluid_real_t,
    mut right_out: *mut fluid_real_t,
) {
    let mut sample_index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut d_in: fluid_real_t = 0.;
    let mut d_out: fluid_real_t = 0.;
    sample_index = 0 as libc::c_int;
    while sample_index < 64 as libc::c_int {
        d_in = *in_0.offset(sample_index as isize);
        d_out = 0.0f32;

        *(*chorus).chorusbuf.offset((*chorus).counter as isize) = d_in;
        i = 0 as libc::c_int;
        while i < (*chorus).number_blocks {
            let mut ii: libc::c_int = 0;
            let mut pos_subsamples: libc::c_int =
                ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) * (*chorus).counter
                    - *(*chorus)
                        .lookup_tab
                        .offset((*chorus).phase[i as usize] as isize);
            let mut pos_samples: libc::c_int =
                pos_subsamples / ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int);
            pos_subsamples &=
                ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) - 1 as libc::c_int;
            ii = 0 as libc::c_int;
            while ii < 5 as libc::c_int {
                d_out += *(*chorus).chorusbuf.offset(
                    (pos_samples
                        & ((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int)
                            - 1 as libc::c_int) as isize,
                ) * (*chorus).sinc_table[ii as usize][pos_subsamples as usize];
                pos_samples -= 1;
                ii += 1
            }
            (*chorus).phase[i as usize] += 1;
            (*chorus).phase[i as usize] %= (*chorus).modulation_period_samples;
            i += 1
        }
        d_out *= (*chorus).level;
        let ref mut fresh0 = *left_out.offset(sample_index as isize);
        *fresh0 += d_out;
        let ref mut fresh1 = *right_out.offset(sample_index as isize);
        *fresh1 += d_out;
        (*chorus).counter += 1;
        (*chorus).counter %= (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int;
        sample_index += 1
    }
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_processreplace(
    mut chorus: *mut fluid_chorus_t,
    mut in_0: *mut fluid_real_t,
    mut left_out: *mut fluid_real_t,
    mut right_out: *mut fluid_real_t,
) {
    let mut sample_index: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut d_in: fluid_real_t = 0.;
    let mut d_out: fluid_real_t = 0.;
    sample_index = 0 as libc::c_int;
    while sample_index < 64 as libc::c_int {
        d_in = *in_0.offset(sample_index as isize);
        d_out = 0.0f32;

        *(*chorus).chorusbuf.offset((*chorus).counter as isize) = d_in;
        i = 0 as libc::c_int;
        while i < (*chorus).number_blocks {
            let mut ii: libc::c_int = 0;
            let mut pos_subsamples: libc::c_int =
                ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) * (*chorus).counter
                    - *(*chorus)
                        .lookup_tab
                        .offset((*chorus).phase[i as usize] as isize);
            let mut pos_samples: libc::c_int =
                pos_subsamples / ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int);
            pos_subsamples &=
                ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) - 1 as libc::c_int;
            ii = 0 as libc::c_int;
            while ii < 5 as libc::c_int {
                d_out += *(*chorus).chorusbuf.offset(
                    (pos_samples
                        & ((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int)
                            - 1 as libc::c_int) as isize,
                ) * (*chorus).sinc_table[ii as usize][pos_subsamples as usize];
                pos_samples -= 1;
                ii += 1
            }
            (*chorus).phase[i as usize] += 1;
            (*chorus).phase[i as usize] %= (*chorus).modulation_period_samples;
            i += 1
        }
        d_out *= (*chorus).level;
        *left_out.offset(sample_index as isize) = d_out;
        *right_out.offset(sample_index as isize) = d_out;
        (*chorus).counter += 1;
        (*chorus).counter %= (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int;
        sample_index += 1
    }
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_sine(
    mut buf: *mut libc::c_int,
    mut len: libc::c_int,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    let mut val: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < len {
        val = sin(i as libc::c_double / len as libc::c_double * 2.0f64 * 3.14159265358979323846f64);
        *buf.offset(i as isize) = ((1.0f64 + val) * depth as libc::c_double / 2.0f64
            * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) as libc::c_double)
            as libc::c_int;
        *buf.offset(i as isize) -= 3 as libc::c_int
            * ((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int)
            * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int);
        i += 1
    }
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_triangle(
    mut buf: *mut libc::c_int,
    mut len: libc::c_int,
    mut depth: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ii: libc::c_int = len - 1 as libc::c_int;
    let mut val: libc::c_double = 0.;
    let mut val2: libc::c_double = 0.;
    while i <= ii {
        val = i as libc::c_double * 2.0f64 / len as libc::c_double
            * depth as libc::c_double
            * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) as libc::c_double;
        val2 = ((val + 0.5f64) as libc::c_int
            - 3 as libc::c_int
                * ((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int)
                * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int))
            as libc::c_double;
        let fresh2 = i;
        i = i + 1;
        *buf.offset(fresh2 as isize) = val2 as libc::c_int;
        let fresh3 = ii;
        ii = ii - 1;
        *buf.offset(fresh3 as isize) = val2 as libc::c_int
    }
}

#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_reset(mut chorus: *mut fluid_chorus_t) {
    fluid_chorus_init(chorus);
}
