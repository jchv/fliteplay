pub type ChorusMod = u32;
pub const CHORUS_MOD_TRIANGLE: ChorusMod = 1;
pub const CHORUS_MOD_SINE: ChorusMod = 0;
pub const FLUID_OK: i32 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Chorus {
    pub type_0: i32,
    pub new_type: i32,
    pub depth_ms: f32,
    pub new_depth_ms: f32,
    pub level: f32,
    pub new_level: f32,
    pub speed_hz: f32,
    pub new_speed_hz: f32,
    pub number_blocks: i32,
    pub new_number_blocks: i32,
    pub chorusbuf: *mut f32,
    pub counter: i32,
    pub phase: [isize; 99],
    pub modulation_period_samples: isize,
    pub lookup_tab: *mut i32,
    pub sample_rate: f32,
    pub sinc_table: [[f32; 128]; 5],
}

pub fn new_fluid_chorus(sample_rate: f32) -> *mut Chorus {
    unsafe {
        let mut i;
        let mut ii;
        let mut chorus;
        chorus = libc::malloc(::std::mem::size_of::<Chorus>() as libc::size_t) as *mut Chorus;
        if chorus.is_null() {
            fluid_log!(FLUID_PANIC as i32, "chorus: Out of memory",);
            return 0 as *mut Chorus;
        }
        libc::memset(
            chorus as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<Chorus>() as libc::size_t,
        );
        (*chorus).sample_rate = sample_rate;
        i = 0 as i32;
        while i < 5 as i32 {
            ii = 0 as i32;
            while ii < (1 as i32) << 8 as i32 - 1 as i32 {
                let i_shifted: f64 = i as f64 - 5 as i32 as f64 / 2.0f64
                    + ii as f64 / ((1 as i32) << 8 as i32 - 1 as i32) as f64;
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
                                            / 5 as i32 as f32 as f64,
                                    )))) as f32
                }
                ii += 1
            }
            i += 1
        }
        (*chorus).lookup_tab = libc::malloc(
            (((*chorus).sample_rate as f64 / 0.29f64) as i32 as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<i32>() as libc::size_t),
        ) as *mut i32;
        if (*chorus).lookup_tab.is_null() {
            fluid_log!(FLUID_PANIC as i32, "chorus: Out of memory",);
        } else {
            (*chorus).chorusbuf = libc::malloc(
                (((1 as i32) << 12 as i32 - 1 as i32) as libc::size_t)
                    .wrapping_mul(::std::mem::size_of::<f32>() as libc::size_t),
            ) as *mut f32;
            if (*chorus).chorusbuf.is_null() {
                fluid_log!(FLUID_PANIC as i32, "chorus: Out of memory",);
            } else if !(fluid_chorus_init(chorus.as_mut().unwrap()) != FLUID_OK as i32) {
                return chorus;
            }
        }
        delete_fluid_chorus(chorus.as_mut().unwrap());
        return 0 as *mut Chorus;
    }
}

pub fn fluid_chorus_init(chorus: &mut Chorus) -> i32 {
    let mut i: i32 = 0;
    unsafe {
        while i < (1 as i32) << 12 as i32 - 1 as i32 {
            *chorus.chorusbuf.offset(i as isize) = 0.0f64 as f32;
            i += 1
        }
    }
    fluid_chorus_set_nr(chorus, 3 as i32);
    fluid_chorus_set_level(chorus, 2.0f32);
    fluid_chorus_set_speed_hz(chorus, 0.3f32);
    fluid_chorus_set_depth_ms(chorus, 8.0f32);
    fluid_chorus_set_type(chorus, CHORUS_MOD_SINE as i32);
    return fluid_chorus_update(chorus);
}

pub fn fluid_chorus_set_nr(chorus: &mut Chorus, nr: i32) {
    chorus.new_number_blocks = nr;
}

pub fn fluid_chorus_get_nr(chorus: &Chorus) -> i32 {
    return chorus.number_blocks;
}

pub fn fluid_chorus_set_level(chorus: &mut Chorus, level: f32) {
    chorus.new_level = level;
}

pub fn fluid_chorus_get_level(chorus: &Chorus) -> f32 {
    return chorus.level;
}

pub fn fluid_chorus_set_speed_hz(chorus: &mut Chorus, speed_hz: f32) {
    chorus.new_speed_hz = speed_hz;
}

pub fn fluid_chorus_get_speed_hz(chorus: &Chorus) -> f32 {
    return chorus.speed_hz;
}

pub fn fluid_chorus_set_depth_ms(chorus: &mut Chorus, depth_ms: f32) {
    chorus.new_depth_ms = depth_ms;
}

pub fn fluid_chorus_get_depth_ms(chorus: &Chorus) -> f32 {
    return chorus.depth_ms;
}

pub fn fluid_chorus_set_type(chorus: &mut Chorus, type_0: i32) {
    chorus.new_type = type_0;
}

pub fn fluid_chorus_get_type(chorus: &Chorus) -> i32 {
    return chorus.type_0;
}

pub fn delete_fluid_chorus(chorus: &mut Chorus) {
    if !chorus.chorusbuf.is_null() {
        unsafe { libc::free(chorus.chorusbuf as *mut libc::c_void); }
    }
    if !chorus.lookup_tab.is_null() {
        unsafe { libc::free(chorus.lookup_tab as *mut libc::c_void); }
    }
    unsafe { libc::free(chorus as *mut Chorus as *mut libc::c_void); }
}

pub fn fluid_chorus_update(chorus: &mut Chorus) -> i32 {
    let mut i: i32;
    let mut modulation_depth_samples: i32;
    if chorus.new_number_blocks < 0 as i32 {
        fluid_log!(
            FLUID_WARN,
            "chorus: number blocks must be >=0! Setting value to 0.",
        );
        chorus.new_number_blocks = 0 as i32
    } else if chorus.new_number_blocks > 99 as i32 {
        fluid_log!(
            FLUID_WARN,
            "chorus: number blocks larger than max. allowed! Setting value to {}.",
            99
        );
        chorus.new_number_blocks = 99 as i32
    }
    if (chorus.new_speed_hz as f64) < 0.29f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: speed is too low (min {})! Setting value to min.",
            0.29f64
        );
        chorus.new_speed_hz = 0.29f64 as f32
    } else if chorus.new_speed_hz > 5 as i32 as libc::c_float {
        fluid_log!(
            FLUID_WARN,
            "chorus: speed must be below {} Hz! Setting value to max.",
            5
        );
        chorus.new_speed_hz = 5 as i32 as f32
    }
    if (chorus.new_depth_ms as f64) < 0.0f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: depth must be positive! Setting value to 0.",
        );
        chorus.new_depth_ms = 0.0f64 as f32
    }
    if (chorus.new_level as f64) < 0.0f64 {
        fluid_log!(
            FLUID_WARN,
            "chorus: level must be positive! Setting value to 0.",
        );
        chorus.new_level = 0.0f64 as f32
    } else if chorus.new_level > 10 as i32 as libc::c_float {
        fluid_log!(
            FLUID_WARN,
            "chorus: level must be < 10. A reasonable level is << 1! Setting it to 0.1.",
        );
        chorus.new_level = 0.1f64 as f32
    }
    chorus.modulation_period_samples =
        (chorus.sample_rate / chorus.new_speed_hz) as isize;
    modulation_depth_samples =
        (chorus.new_depth_ms as f64 / 1000.0f64 * chorus.sample_rate as f64) as i32;
    if modulation_depth_samples > (1 as i32) << 12 as i32 - 1 as i32 {
        fluid_log!(
            FLUID_WARN,
            "chorus: Too high depth. Setting it to max ({}).",
            (1) << 12 - 1
        );
        modulation_depth_samples = (1 as i32) << 12 as i32 - 1 as i32
    }
    if chorus.type_0 == CHORUS_MOD_SINE as i32 {
        fluid_chorus_sine(
            chorus.lookup_tab,
            chorus.modulation_period_samples as i32,
            modulation_depth_samples,
        );
    } else if chorus.type_0 == CHORUS_MOD_TRIANGLE as i32 {
        fluid_chorus_triangle(
            chorus.lookup_tab,
            chorus.modulation_period_samples as i32,
            modulation_depth_samples,
        );
    } else {
        fluid_log!(
            FLUID_WARN,
            "chorus: Unknown modulation type. Using sinewave.",
        );
        chorus.type_0 = CHORUS_MOD_SINE as i32;
        fluid_chorus_sine(
            chorus.lookup_tab,
            chorus.modulation_period_samples as i32,
            modulation_depth_samples,
        );
    }
    i = 0 as i32;
    while i < chorus.number_blocks {
        chorus.phase[i as usize] = (chorus.modulation_period_samples as f64 * i as f64
            / chorus.number_blocks as f64) as i32
            as isize;
        i += 1
    }
    chorus.counter = 0 as i32;
    chorus.type_0 = chorus.new_type;
    chorus.depth_ms = chorus.new_depth_ms;
    chorus.level = chorus.new_level;
    chorus.speed_hz = chorus.new_speed_hz;
    chorus.number_blocks = chorus.new_number_blocks;
    return FLUID_OK as i32;
}

pub fn fluid_chorus_processmix(
    mut chorus: &mut Chorus,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    unsafe {
        let mut sample_index: i32;
        let mut i: i32;
        let mut d_in: f32;
        let mut d_out: f32;
        sample_index = 0 as i32;
        while sample_index < 64 as i32 {
            d_in = *in_0.offset(sample_index as isize);
            d_out = 0.0f32;
            *chorus.chorusbuf.offset(chorus.counter as isize) = d_in;
            i = 0 as i32;
            while i < chorus.number_blocks {
                let mut ii: i32;
                let mut pos_subsamples: i32 =
                    ((1 as i32) << 8 as i32 - 1 as i32) * chorus.counter
                        - *chorus
                            .lookup_tab
                            .offset(chorus.phase[i as usize] as isize);
                let mut pos_samples: i32 =
                    pos_subsamples / ((1 as i32) << 8 as i32 - 1 as i32);
                pos_subsamples &=
                    ((1 as i32) << 8 as i32 - 1 as i32) - 1 as i32;
                ii = 0 as i32;
                while ii < 5 as i32 {
                    d_out += *chorus.chorusbuf.offset(
                        (pos_samples
                            & ((1 as i32) << 12 as i32 - 1 as i32)
                                - 1 as i32) as isize,
                    ) * chorus.sinc_table[ii as usize][pos_subsamples as usize];
                    pos_samples -= 1;
                    ii += 1
                }
                chorus.phase[i as usize] += 1;
                chorus.phase[i as usize] %= chorus.modulation_period_samples;
                i += 1
            }
            d_out *= chorus.level;
            let ref mut fresh0 = *left_out.offset(sample_index as isize);
            *fresh0 += d_out;
            let ref mut fresh1 = *right_out.offset(sample_index as isize);
            *fresh1 += d_out;
            chorus.counter += 1;
            chorus.counter %= (1 as i32) << 12 as i32 - 1 as i32;
            sample_index += 1
        }
    }
}

pub fn fluid_chorus_processreplace(
    mut chorus: &mut Chorus,
    in_0: *mut f32,
    left_out: *mut f32,
    right_out: *mut f32,
) {
    unsafe {
        let mut sample_index: i32;
        let mut i: i32;
        let mut d_in: f32;
        let mut d_out: f32;
        sample_index = 0 as i32;
        while sample_index < 64 as i32 {
            d_in = *in_0.offset(sample_index as isize);
            d_out = 0.0f32;
            *chorus.chorusbuf.offset(chorus.counter as isize) = d_in;
            i = 0 as i32;
            while i < chorus.number_blocks {
                let mut ii: i32;
                let mut pos_subsamples: i32 =
                    ((1 as i32) << 8 as i32 - 1 as i32) * chorus.counter
                        - *chorus
                            .lookup_tab
                            .offset(chorus.phase[i as usize] as isize);
                let mut pos_samples: i32 =
                    pos_subsamples / ((1 as i32) << 8 as i32 - 1 as i32);
                pos_subsamples &=
                    ((1 as i32) << 8 as i32 - 1 as i32) - 1 as i32;
                ii = 0 as i32;
                while ii < 5 as i32 {
                    d_out += *chorus.chorusbuf.offset(
                        (pos_samples
                            & ((1 as i32) << 12 as i32 - 1 as i32)
                                - 1 as i32) as isize,
                    ) * chorus.sinc_table[ii as usize][pos_subsamples as usize];
                    pos_samples -= 1;
                    ii += 1
                }
                chorus.phase[i as usize] += 1;
                chorus.phase[i as usize] %= chorus.modulation_period_samples;
                i += 1
            }
            d_out *= chorus.level;
            *left_out.offset(sample_index as isize) = d_out;
            *right_out.offset(sample_index as isize) = d_out;
            chorus.counter += 1;
            chorus.counter %= (1 as i32) << 12 as i32 - 1 as i32;
            sample_index += 1
        }
    }
}

pub fn fluid_chorus_sine(buf: *mut i32, len: i32, depth: i32) {
    unsafe {
        let mut i: i32;
        let mut val: f64;
        i = 0 as i32;
        while i < len {
            val = f64::sin(i as f64 / len as f64 * 2.0f64 * std::f64::consts::PI);
            *buf.offset(i as isize) = ((1.0f64 + val) * depth as f64 / 2.0f64
                * ((1 as i32) << 8 as i32 - 1 as i32) as f64)
                as i32;
            *buf.offset(i as isize) -= 3 as i32
                * ((1 as i32) << 12 as i32 - 1 as i32)
                * ((1 as i32) << 8 as i32 - 1 as i32);
            i += 1
        }
    }
}

pub fn fluid_chorus_triangle(buf: *mut i32, len: i32, depth: i32) {
    unsafe {
        let mut i: i32 = 0 as i32;
        let mut ii: i32 = len - 1 as i32;
        let mut val: f64;
        let mut val2: f64;
        while i <= ii {
            val = i as f64 * 2.0f64 / len as f64
                * depth as f64
                * ((1 as i32) << 8 as i32 - 1 as i32) as f64;
            val2 = ((val + 0.5f64) as i32
                - 3 as i32
                    * ((1 as i32) << 12 as i32 - 1 as i32)
                    * ((1 as i32) << 8 as i32 - 1 as i32))
                as f64;
            let fresh2 = i;
            i = i + 1;
            *buf.offset(fresh2 as isize) = val2 as i32;
            let fresh3 = ii;
            ii = ii - 1;
            *buf.offset(fresh3 as isize) = val2 as i32
        }
    }
}

pub fn fluid_chorus_reset(chorus: &mut Chorus) {
    fluid_chorus_init(chorus);
}
