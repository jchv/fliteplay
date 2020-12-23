use crate::voice::fluid_voice_t;
pub type Phase = libc::c_ulonglong;
pub type GenType = libc::c_uint;
pub const GEN_SAMPLEMODE: GenType = 54;
pub type VoiceEnvelopeIndex = libc::c_uint;
pub const FLUID_VOICE_ENVRELEASE: VoiceEnvelopeIndex = 5;
pub const FLUID_LOOP_UNTIL_RELEASE: LoopMode = 3;
pub const FLUID_LOOP_DURING_RELEASE: LoopMode = 1;
pub type LoopMode = libc::c_uint;
static mut INTERP_COEFF_LINEAR: [[f32; 2]; 256] = [[0.; 2]; 256];
static mut INTERP_COEFF: [[f32; 4]; 256] = [[0.; 4]; 256];
static mut SINC_TABLE7: [[f32; 7]; 256] = [[0.; 7]; 256];
#[no_mangle]
pub unsafe extern "C" fn fluid_dsp_float_config() {
    let mut i: libc::c_int;
    let mut i2: libc::c_int;
    let mut x: f64;
    let mut v: f64;
    let mut i_shifted: f64;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        x = i as f64 / 256 as libc::c_int as f64;
        INTERP_COEFF[i as usize][0 as libc::c_int as usize] =
            (x * (-0.5f64 + x * (1 as libc::c_int as f64 - 0.5f64 * x))) as f32;
        INTERP_COEFF[i as usize][1 as libc::c_int as usize] =
            (1.0f64 + x * x * (1.5f64 * x - 2.5f64)) as f32;
        INTERP_COEFF[i as usize][2 as libc::c_int as usize] =
            (x * (0.5f64 + x * (2.0f64 - 1.5f64 * x))) as f32;
        INTERP_COEFF[i as usize][3 as libc::c_int as usize] =
            (0.5f64 * x * x * (x - 1.0f64)) as f32;
        INTERP_COEFF_LINEAR[i as usize][0 as libc::c_int as usize] = (1.0f64 - x) as f32;
        INTERP_COEFF_LINEAR[i as usize][1 as libc::c_int as usize] = x as f32;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        i2 = 0 as libc::c_int;
        while i2 < 256 as libc::c_int {
            i_shifted = i as f64 - 7 as libc::c_int as f64 / 2.0f64
                + i2 as f64 / 256 as libc::c_int as f64;
            if f64::abs(i_shifted) > 0.000001f64 {
                v = f64::sin(i_shifted * std::f64::consts::PI) as f32 as f64
                    / (std::f64::consts::PI * i_shifted);
                v *= 0.5f64 as f32 as f64
                    * (1.0f64
                        + f64::cos(
                            2.0f64 * std::f64::consts::PI * i_shifted
                                / 7 as libc::c_int as f32 as f64,
                        ))
            } else {
                v = 1.0f64
            }
            SINC_TABLE7[(256 as libc::c_int - i2 - 1 as libc::c_int) as usize][i as usize] =
                v as f32;
            i2 += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_dsp_float_interpolate_none(
    mut voice: *mut fluid_voice_t,
) -> libc::c_int {
    let mut dsp_phase: Phase = (*voice).phase;
    let dsp_phase_incr: Phase;
    let dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let dsp_buf: *mut f32 = (*voice).dsp_buf;
    let mut dsp_amp: f32 = (*voice).amp;
    let dsp_amp_incr: f32 = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint;
    let end_index: libc::c_uint;
    let looping: libc::c_int;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as f64
            - (*voice).phase_incr as libc::c_int as f64)
            * 4294967296.0f64) as u32 as libc::c_ulonglong;
    looping = ((*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
        == FLUID_LOOP_DURING_RELEASE as libc::c_int
        || (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
            == FLUID_LOOP_UNTIL_RELEASE as libc::c_int
            && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as libc::c_int)
        as libc::c_int;
    end_index = if looping != 0 {
        ((*voice).loopend) - 1 as libc::c_int
    } else {
        (*voice).end
    } as libc::c_uint;
    loop {
        dsp_phase_index = (dsp_phase.wrapping_add(0x80000000 as libc::c_uint as libc::c_ulonglong)
            >> 32 as libc::c_int) as libc::c_uint;
        while dsp_i < 64 as libc::c_int as libc::c_uint && dsp_phase_index <= end_index {
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int as libc::c_float;
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase
                .wrapping_add(0x80000000 as libc::c_uint as libc::c_ulonglong)
                >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if looping == 0 {
            break;
        }
        if dsp_phase_index > end_index {
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_sub(
                (((*voice).loopend - (*voice).loopstart) as libc::c_ulonglong) << 32 as libc::c_int,
            ) as Phase as Phase;
            (*voice).has_looped = 1 as libc::c_int
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
    }
    (*voice).phase = dsp_phase;
    (*voice).amp = dsp_amp;
    return dsp_i as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_dsp_float_interpolate_linear(
    mut voice: *mut fluid_voice_t,
) -> libc::c_int {
    let mut dsp_phase: Phase = (*voice).phase;
    let dsp_phase_incr: Phase;
    let dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let dsp_buf: *mut f32 = (*voice).dsp_buf;
    let mut dsp_amp: f32 = (*voice).amp;
    let dsp_amp_incr: f32 = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint;
    let mut end_index: libc::c_uint;
    let point: libc::c_short;
    let mut coeffs: *mut f32;
    let looping: libc::c_int;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as f64
            - (*voice).phase_incr as libc::c_int as f64)
            * 4294967296.0f64) as u32 as libc::c_ulonglong;
    looping = ((*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
        == FLUID_LOOP_DURING_RELEASE as libc::c_int
        || (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
            == FLUID_LOOP_UNTIL_RELEASE as libc::c_int
            && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as libc::c_int)
        as libc::c_int;
    end_index = ((if looping != 0 {
        ((*voice).loopend) - 1 as libc::c_int
    } else {
        (*voice).end
    }) - 1 as libc::c_int) as libc::c_uint;
    if looping != 0 {
        point = *dsp_data.offset((*voice).loopstart as isize)
    } else {
        point = *dsp_data.offset((*voice).end as isize)
    }
    loop {
        dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
        while dsp_i < 64 as libc::c_int as libc::c_uint && dsp_phase_index <= end_index {
            coeffs = INTERP_COEFF_LINEAR[(((dsp_phase
                & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int as libc::c_float
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int as libc::c_float);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = INTERP_COEFF_LINEAR[(((dsp_phase
                & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int as libc::c_float
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * point as libc::c_int as libc::c_float);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if looping == 0 {
            break;
        }
        if dsp_phase_index > end_index {
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_sub(
                (((*voice).loopend - (*voice).loopstart) as libc::c_ulonglong) << 32 as libc::c_int,
            ) as Phase as Phase;
            (*voice).has_looped = 1 as libc::c_int
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_sub(1)
    }
    (*voice).phase = dsp_phase;
    (*voice).amp = dsp_amp;
    return dsp_i as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_dsp_float_interpolate_4th_order(
    mut voice: *mut fluid_voice_t,
) -> libc::c_int {
    let mut dsp_phase: Phase = (*voice).phase;
    let dsp_phase_incr: Phase;
    let dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let dsp_buf: *mut f32 = (*voice).dsp_buf;
    let mut dsp_amp: f32 = (*voice).amp;
    let dsp_amp_incr: f32 = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint;
    let mut start_index: libc::c_uint;
    let mut end_index: libc::c_uint;
    let mut start_point: libc::c_short;
    let end_point1: libc::c_short;
    let end_point2: libc::c_short;
    let mut coeffs: *mut f32;
    let looping: libc::c_int;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as f64
            - (*voice).phase_incr as libc::c_int as f64)
            * 4294967296.0f64) as u32 as libc::c_ulonglong;
    looping = ((*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
        == FLUID_LOOP_DURING_RELEASE as libc::c_int
        || (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
            == FLUID_LOOP_UNTIL_RELEASE as libc::c_int
            && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as libc::c_int)
        as libc::c_int;
    end_index = ((if looping != 0 {
        ((*voice).loopend) - 1 as libc::c_int
    } else {
        (*voice).end
    }) - 2 as libc::c_int) as libc::c_uint;
    if (*voice).has_looped != 0 {
        start_index = (*voice).loopstart as libc::c_uint;
        start_point = *dsp_data.offset(((*voice).loopend - 1 as libc::c_int) as isize)
    } else {
        start_index = (*voice).start as libc::c_uint;
        start_point = *dsp_data.offset((*voice).start as isize)
    }
    if looping != 0 {
        end_point1 = *dsp_data.offset((*voice).loopstart as isize);
        end_point2 = *dsp_data.offset(((*voice).loopstart + 1 as libc::c_int) as isize)
    } else {
        end_point1 = *dsp_data.offset((*voice).end as isize);
        end_point2 = end_point1
    }
    loop {
        dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
        while dsp_phase_index == start_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = INTERP_COEFF[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * start_point as libc::c_int as libc::c_float
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int
                            as libc::c_float
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int as libc::c_float
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int as libc::c_float);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        while dsp_i < 64 as libc::c_int as libc::c_uint && dsp_phase_index <= end_index {
            coeffs = INTERP_COEFF[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ) as libc::c_int as libc::c_float
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int
                            as libc::c_float
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int as libc::c_float
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int as libc::c_float);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = INTERP_COEFF[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ) as libc::c_int as libc::c_float
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int
                            as libc::c_float
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as libc::c_int as libc::c_float
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * end_point1 as libc::c_int as libc::c_float);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = INTERP_COEFF[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                    ) as libc::c_int as libc::c_float
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int
                            as libc::c_float
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * end_point1 as libc::c_int as libc::c_float
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * end_point2 as libc::c_int as libc::c_float);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if looping == 0 {
            break;
        }
        if dsp_phase_index > end_index {
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_sub(
                (((*voice).loopend - (*voice).loopstart) as libc::c_ulonglong) << 32 as libc::c_int,
            ) as Phase as Phase;
            if (*voice).has_looped == 0 {
                (*voice).has_looped = 1 as libc::c_int;
                start_index = (*voice).loopstart as libc::c_uint;
                start_point = *dsp_data.offset(((*voice).loopend - 1 as libc::c_int) as isize)
            }
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_sub(2 as libc::c_int as libc::c_uint)
    }
    (*voice).phase = dsp_phase;
    (*voice).amp = dsp_amp;
    return dsp_i as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_dsp_float_interpolate_7th_order(
    mut voice: *mut fluid_voice_t,
) -> libc::c_int {
    let mut dsp_phase: Phase = (*voice).phase;
    let dsp_phase_incr: Phase;
    let dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let dsp_buf: *mut f32 = (*voice).dsp_buf;
    let mut dsp_amp: f32 = (*voice).amp;
    let dsp_amp_incr: f32 = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint;
    let mut start_index: libc::c_uint;
    let mut end_index: libc::c_uint;
    let mut start_points: [libc::c_short; 3] = [0; 3];
    let mut end_points: [libc::c_short; 3] = [0; 3];
    let mut coeffs: *mut f32;
    let looping: libc::c_int;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as f64
            - (*voice).phase_incr as libc::c_int as f64)
            * 4294967296.0f64) as u32 as libc::c_ulonglong;
    dsp_phase = (dsp_phase as libc::c_ulonglong)
        .wrapping_add(0x80000000 as libc::c_uint as Phase) as Phase
        as Phase;
    looping = ((*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
        == FLUID_LOOP_DURING_RELEASE as libc::c_int
        || (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
            == FLUID_LOOP_UNTIL_RELEASE as libc::c_int
            && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as libc::c_int)
        as libc::c_int;
    end_index = ((if looping != 0 {
        ((*voice).loopend) - 1 as libc::c_int
    } else {
        (*voice).end
    }) - 3 as libc::c_int) as libc::c_uint;
    if (*voice).has_looped != 0 {
        start_index = (*voice).loopstart as libc::c_uint;
        start_points[0 as libc::c_int as usize] =
            *dsp_data.offset(((*voice).loopend - 1 as libc::c_int) as isize);
        start_points[1 as libc::c_int as usize] =
            *dsp_data.offset(((*voice).loopend - 2 as libc::c_int) as isize);
        start_points[2 as libc::c_int as usize] =
            *dsp_data.offset(((*voice).loopend - 3 as libc::c_int) as isize)
    } else {
        start_index = (*voice).start as libc::c_uint;
        start_points[0 as libc::c_int as usize] = *dsp_data.offset((*voice).start as isize);
        start_points[1 as libc::c_int as usize] = start_points[0 as libc::c_int as usize];
        start_points[2 as libc::c_int as usize] = start_points[0 as libc::c_int as usize]
    }
    if looping != 0 {
        end_points[0 as libc::c_int as usize] = *dsp_data.offset((*voice).loopstart as isize);
        end_points[1 as libc::c_int as usize] =
            *dsp_data.offset(((*voice).loopstart + 1 as libc::c_int) as isize);
        end_points[2 as libc::c_int as usize] =
            *dsp_data.offset(((*voice).loopstart + 2 as libc::c_int) as isize)
    } else {
        end_points[0 as libc::c_int as usize] = *dsp_data.offset((*voice).end as isize);
        end_points[1 as libc::c_int as usize] = end_points[0 as libc::c_int as usize];
        end_points[2 as libc::c_int as usize] = end_points[0 as libc::c_int as usize]
    }
    loop {
        dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
        while dsp_phase_index == start_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = SINC_TABLE7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * start_points[2 as libc::c_int as usize] as f32
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * start_points[1 as libc::c_int as usize] as f32
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * start_points[0 as libc::c_int as usize] as f32
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as f32
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as f32);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        start_index = start_index.wrapping_add(1);
        while dsp_phase_index == start_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = SINC_TABLE7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * start_points[1 as libc::c_int as usize] as f32
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * start_points[0 as libc::c_int as usize] as f32
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as f32
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as f32);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        start_index = start_index.wrapping_add(1);
        while dsp_phase_index == start_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = SINC_TABLE7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * start_points[0 as libc::c_int as usize] as f32
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as f32
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as f32);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        start_index = start_index.wrapping_sub(2 as libc::c_int as libc::c_uint);
        while dsp_i < 64 as libc::c_int as libc::c_uint && dsp_phase_index <= end_index {
            coeffs = SINC_TABLE7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as f32
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as f32
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as f32);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = SINC_TABLE7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as f32
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as f32
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * end_points[0 as libc::c_int as usize] as f32);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = SINC_TABLE7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as f32
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as f32
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * end_points[0 as libc::c_int as usize] as f32
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * end_points[1 as libc::c_int as usize] as f32);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = SINC_TABLE7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as u32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as f32
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as f32
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as f32
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * end_points[0 as libc::c_int as usize] as f32
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * end_points[1 as libc::c_int as usize] as f32
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * end_points[2 as libc::c_int as usize] as f32);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as Phase as Phase;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if looping == 0 {
            break;
        }
        if dsp_phase_index > end_index {
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_sub(
                (((*voice).loopend - (*voice).loopstart) as libc::c_ulonglong) << 32 as libc::c_int,
            ) as Phase as Phase;
            if (*voice).has_looped == 0 {
                (*voice).has_looped = 1 as libc::c_int;
                start_index = (*voice).loopstart as libc::c_uint;
                start_points[0 as libc::c_int as usize] =
                    *dsp_data.offset(((*voice).loopend - 1 as libc::c_int) as isize);
                start_points[1 as libc::c_int as usize] =
                    *dsp_data.offset(((*voice).loopend - 2 as libc::c_int) as isize);
                start_points[2 as libc::c_int as usize] =
                    *dsp_data.offset(((*voice).loopend - 3 as libc::c_int) as isize)
            }
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_sub(3 as libc::c_int as libc::c_uint)
    }
    dsp_phase = (dsp_phase as libc::c_ulonglong)
        .wrapping_sub(0x80000000 as libc::c_uint as Phase) as Phase
        as Phase;
    (*voice).phase = dsp_phase;
    (*voice).amp = dsp_amp;
    return dsp_i as libc::c_int;
}
