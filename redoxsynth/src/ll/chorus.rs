pub type ChorusMod = libc::c_uint;
pub const CHORUS_MOD_TRIANGLE: ChorusMod = 1;
pub const CHORUS_MOD_SINE: ChorusMod = 0;
pub const FLUID_OK: libc::c_int = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Chorus {
    pub type_0: libc::c_int,
    pub new_type: libc::c_int,
    pub depth_ms: f32,
    pub new_depth_ms: f32,
    pub level: f32,
    pub new_level: f32,
    pub speed_hz: f32,
    pub new_speed_hz: f32,
    pub number_blocks: libc::c_int,
    pub new_number_blocks: libc::c_int,
    pub chorusbuf: *mut f32,
    pub counter: libc::c_int,
    pub phase: [libc::c_long; 99],
    pub modulation_period_samples: libc::c_long,
    pub lookup_tab: *mut libc::c_int,
    pub sample_rate: f32,
    pub sinc_table: [[f32; 128]; 5],
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_chorus(sample_rate: f32) -> *mut Chorus {
    let mut i;
    let mut ii;
    let mut chorus;
    chorus = libc::malloc(::std::mem::size_of::<Chorus>() as libc::size_t)
        as *mut Chorus;
    if chorus.is_null() {
        fluid_log!(FLUID_PANIC as libc::c_int, "chorus: Out of memory",);
        return 0 as *mut Chorus;
    }
    libc::memset(
        chorus as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<Chorus>() as libc::size_t,
    );
    (*chorus).sample_rate = sample_rate;
    i = 0 as libc::c_int;
    while i < 5 as libc::c_int {
        ii = 0 as libc::c_int;
        while ii < (1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int {
            let i_shifted: f64 = i as f64
                - 5 as libc::c_int as f64 / 2.0f64
                + ii as f64
                    / ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) as f64;
            if f64::abs(i_shifted) < 0.000001f64 {
                (*chorus).sinc_table[i as usize][ii as usize] = 1.0f64 as f32
            } else {
                (*chorus).sinc_table[i as usize][ii as usize] =
                    (f64::sin(i_shifted * std::f64::consts::PI) as f32 as f64
                        / (std::f64::consts::PI * i_shifted)) as f32;
                (*chorus).sinc_table[i as usize][ii as usize] =
                    ((*chorus).sinc_table[i as usize][ii as usize] as f64
                        * (0.5f64 as f32 as f64
                            * (1.0f64
                                + f64::cos(
                                    2.0f64 * std::f64::consts::PI * i_shifted
                                        / 5 as libc::c_int as f32 as f64,
                                )))) as f32
            }
            ii += 1
        }
        i += 1
    }
    (*chorus).lookup_tab = libc::malloc(
        (((*chorus).sample_rate as f64 / 0.29f64) as libc::c_int as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<libc::c_int>() as libc::size_t),
    ) as *mut libc::c_int;
    if (*chorus).lookup_tab.is_null() {
        fluid_log!(FLUID_PANIC as libc::c_int, "chorus: Out of memory",);
    } else {
        (*chorus).chorusbuf = libc::malloc(
            (((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int) as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<f32>() as libc::size_t),
        ) as *mut f32;
        if (*chorus).chorusbuf.is_null() {
            fluid_log!(FLUID_PANIC as libc::c_int, "chorus: Out of memory",);
        } else if !(fluid_chorus_init(chorus) != FLUID_OK as libc::c_int) {
            return chorus;
        }
    }
    delete_fluid_chorus(chorus);
    return 0 as *mut Chorus;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_init(chorus: *mut Chorus) -> libc::c_int {
    let mut i: libc::c_int = 0;
    while i < (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int {
        *(*chorus).chorusbuf.offset(i as isize) = 0.0f64 as f32;
        i += 1
    }
    fluid_chorus_set_nr(chorus, 3 as libc::c_int);
    fluid_chorus_set_level(chorus, 2.0f32);
    fluid_chorus_set_speed_Hz(chorus, 0.3f32);
    fluid_chorus_set_depth_ms(chorus, 8.0f32);
    fluid_chorus_set_type(chorus, CHORUS_MOD_SINE as libc::c_int);
    return fluid_chorus_update(chorus);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_nr(chorus: *mut Chorus, nr: libc::c_int) {
    (*chorus).new_number_blocks = nr;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_nr(chorus: *const Chorus) -> libc::c_int {
    return (*chorus).number_blocks;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_level(
    chorus: *mut Chorus,
    level: f32,
) {
    (*chorus).new_level = level;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_level(chorus: *const Chorus) -> f32 {
    return (*chorus).level;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_speed_Hz(
    mut chorus: *mut Chorus,
    speed_hz: f32,
) {
    (*chorus).new_speed_hz = speed_hz;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_speed_Hz(
    chorus: *const Chorus,
) -> f32 {
    return (*chorus).speed_hz;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_depth_ms(
    chorus: *mut Chorus,
    depth_ms: f32,
) {
    (*chorus).new_depth_ms = depth_ms;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_depth_ms(
    chorus: *const Chorus,
) -> f32 {
    return (*chorus).depth_ms;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_set_type(
    chorus: *mut Chorus,
    type_0: libc::c_int,
) {
    (*chorus).new_type = type_0;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_get_type(chorus: *const Chorus) -> libc::c_int {
    return (*chorus).type_0;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_chorus(chorus: *mut Chorus) {
    if chorus.is_null() {
        return;
    }
    if !(*chorus).chorusbuf.is_null() {
        libc::free((*chorus).chorusbuf as *mut libc::c_void);
    }
    if !(*chorus).lookup_tab.is_null() {
        libc::free((*chorus).lookup_tab as *mut libc::c_void);
    }
    libc::free(chorus as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_update(chorus: *mut Chorus) -> libc::c_int {
    let mut i: libc::c_int;
    let mut modulation_depth_samples: libc::c_int;
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
    if ((*chorus).new_speed_hz as f64) < 0.29f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: speed is too low (min {})! Setting value to min.",
            0.29f64
        );
        (*chorus).new_speed_hz = 0.29f64 as f32
    } else if (*chorus).new_speed_hz > 5 as libc::c_int as libc::c_float {
        fluid_log!(
            FLUID_WARN,
            "chorus: speed must be below {} Hz! Setting value to max.",
            5
        );
        (*chorus).new_speed_hz = 5 as libc::c_int as f32
    }
    if ((*chorus).new_depth_ms as f64) < 0.0f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: depth must be positive! Setting value to 0.",
        );
        (*chorus).new_depth_ms = 0.0f64 as f32
    }
    if ((*chorus).new_level as f64) < 0.0f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: level must be positive! Setting value to 0.",
        );
        (*chorus).new_level = 0.0f64 as f32
    } else if (*chorus).new_level > 10 as libc::c_int as libc::c_float {
        fluid_log!(
            FLUID_WARN,
            "chorus: level must be < 10. A reasonable level is << 1! Setting it to 0.1.",
        );
        (*chorus).new_level = 0.1f64 as f32
    }
    (*chorus).modulation_period_samples =
        ((*chorus).sample_rate / (*chorus).new_speed_hz) as libc::c_long;
    modulation_depth_samples = ((*chorus).new_depth_ms as f64 / 1000.0f64
        * (*chorus).sample_rate as f64) as libc::c_int;
    if modulation_depth_samples > (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "chorus: Too high depth. Setting it to max ({}).",
            (1) << 12 - 1
        );
        modulation_depth_samples = (1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int
    }
    if (*chorus).type_0 == CHORUS_MOD_SINE as libc::c_int {
        fluid_chorus_sine(
            (*chorus).lookup_tab,
            (*chorus).modulation_period_samples as libc::c_int,
            modulation_depth_samples,
        );
    } else if (*chorus).type_0 == CHORUS_MOD_TRIANGLE as libc::c_int {
        Chorusriangle(
            (*chorus).lookup_tab,
            (*chorus).modulation_period_samples as libc::c_int,
            modulation_depth_samples,
        );
    } else {
        fluid_log!(
            FLUID_WARN,
            "chorus: Unknown modulation type. Using sinewave.",
        );
        (*chorus).type_0 = CHORUS_MOD_SINE as libc::c_int;
        fluid_chorus_sine(
            (*chorus).lookup_tab,
            (*chorus).modulation_period_samples as libc::c_int,
            modulation_depth_samples,
        );
    }
    i = 0 as libc::c_int;
    while i < (*chorus).number_blocks {
        (*chorus).phase[i as usize] = ((*chorus).modulation_period_samples as f64
            * i as f64
            / (*chorus).number_blocks as f64)
            as libc::c_int as libc::c_long;
        i += 1
    }
    (*chorus).counter = 0 as libc::c_int;
    (*chorus).type_0 = (*chorus).new_type;
    (*chorus).depth_ms = (*chorus).new_depth_ms;
    (*chorus).level = (*chorus).new_level;
    (*chorus).speed_hz = (*chorus).new_speed_hz;
    (*chorus).number_blocks = (*chorus).new_number_blocks;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_processmix(
    mut chorus: *mut Chorus,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    let mut sample_index: libc::c_int;
    let mut i: libc::c_int;
    let mut d_in: f32;
    let mut d_out: f32;
    sample_index = 0 as libc::c_int;
    while sample_index < 64 as libc::c_int {
        d_in = *in_0.offset(sample_index as isize);
        d_out = 0.0f32;
        *(*chorus).chorusbuf.offset((*chorus).counter as isize) = d_in;
        i = 0 as libc::c_int;
        while i < (*chorus).number_blocks {
            let mut ii: libc::c_int;
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
    mut chorus: *mut Chorus,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    let mut sample_index: libc::c_int;
    let mut i: libc::c_int;
    let mut d_in: f32;
    let mut d_out: f32;
    sample_index = 0 as libc::c_int;
    while sample_index < 64 as libc::c_int {
        d_in = *in_0.offset(sample_index as isize);
        d_out = 0.0f32;
        *(*chorus).chorusbuf.offset((*chorus).counter as isize) = d_in;
        i = 0 as libc::c_int;
        while i < (*chorus).number_blocks {
            let mut ii: libc::c_int;
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
    buf: *mut libc::c_int,
    len: libc::c_int,
    depth: libc::c_int,
) {
    let mut i: libc::c_int;
    let mut val: f64;
    i = 0 as libc::c_int;
    while i < len {
        val = f64::sin(i as f64 / len as f64 * 2.0f64 * std::f64::consts::PI);
        *buf.offset(i as isize) = ((1.0f64 + val) * depth as f64 / 2.0f64
            * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) as f64)
            as libc::c_int;
        *buf.offset(i as isize) -= 3 as libc::c_int
            * ((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int)
            * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int);
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn Chorusriangle(
    buf: *mut libc::c_int,
    len: libc::c_int,
    depth: libc::c_int,
) {
    let mut i: libc::c_int = 0 as libc::c_int;
    let mut ii: libc::c_int = len - 1 as libc::c_int;
    let mut val: f64;
    let mut val2: f64;
    while i <= ii {
        val = i as f64 * 2.0f64 / len as f64
            * depth as f64
            * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int) as f64;
        val2 = ((val + 0.5f64) as libc::c_int
            - 3 as libc::c_int
                * ((1 as libc::c_int) << 12 as libc::c_int - 1 as libc::c_int)
                * ((1 as libc::c_int) << 8 as libc::c_int - 1 as libc::c_int))
            as f64;
        let fresh2 = i;
        i = i + 1;
        *buf.offset(fresh2 as isize) = val2 as libc::c_int;
        let fresh3 = ii;
        ii = ii - 1;
        *buf.offset(fresh3 as isize) = val2 as libc::c_int
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_chorus_reset(chorus: *mut Chorus) {
    fluid_chorus_init(chorus);
}
