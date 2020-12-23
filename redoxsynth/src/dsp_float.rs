#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::voice::fluid_voice_t;
pub type fluid_real_t = libc::c_float;
pub type fluid_phase_t = libc::c_ulonglong;
pub type fluid_gen_type = libc::c_uint;
pub const GEN_LAST: fluid_gen_type = 60;
pub const GEN_PITCH: fluid_gen_type = 59;
pub const GEN_OVERRIDEROOTKEY: fluid_gen_type = 58;
pub const GEN_EXCLUSIVECLASS: fluid_gen_type = 57;
pub const GEN_SCALETUNE: fluid_gen_type = 56;
pub const GEN_RESERVED3: fluid_gen_type = 55;
pub const GEN_SAMPLEMODE: fluid_gen_type = 54;
pub const GEN_SAMPLEID: fluid_gen_type = 53;
pub const GEN_FINETUNE: fluid_gen_type = 52;
pub const GEN_COARSETUNE: fluid_gen_type = 51;
pub const GEN_ENDLOOPADDRCOARSEOFS: fluid_gen_type = 50;
pub const GEN_RESERVED2: fluid_gen_type = 49;
pub const GEN_ATTENUATION: fluid_gen_type = 48;
pub const GEN_VELOCITY: fluid_gen_type = 47;
pub const GEN_KEYNUM: fluid_gen_type = 46;
pub const GEN_STARTLOOPADDRCOARSEOFS: fluid_gen_type = 45;
pub const GEN_VELRANGE: fluid_gen_type = 44;
pub const GEN_KEYRANGE: fluid_gen_type = 43;
pub const GEN_RESERVED1: fluid_gen_type = 42;
pub const GEN_INSTRUMENT: fluid_gen_type = 41;
pub const GEN_KEYTOVOLENVDECAY: fluid_gen_type = 40;
pub const GEN_KEYTOVOLENVHOLD: fluid_gen_type = 39;
pub const GEN_VOLENVRELEASE: fluid_gen_type = 38;
pub const GEN_VOLENVSUSTAIN: fluid_gen_type = 37;
pub const GEN_VOLENVDECAY: fluid_gen_type = 36;
pub const GEN_VOLENVHOLD: fluid_gen_type = 35;
pub const GEN_VOLENVATTACK: fluid_gen_type = 34;
pub const GEN_VOLENVDELAY: fluid_gen_type = 33;
pub const GEN_KEYTOMODENVDECAY: fluid_gen_type = 32;
pub const GEN_KEYTOMODENVHOLD: fluid_gen_type = 31;
pub const GEN_MODENVRELEASE: fluid_gen_type = 30;
pub const GEN_MODENVSUSTAIN: fluid_gen_type = 29;
pub const GEN_MODENVDECAY: fluid_gen_type = 28;
pub const GEN_MODENVHOLD: fluid_gen_type = 27;
pub const GEN_MODENVATTACK: fluid_gen_type = 26;
pub const GEN_MODENVDELAY: fluid_gen_type = 25;
pub const GEN_VIBLFOFREQ: fluid_gen_type = 24;
pub const GEN_VIBLFODELAY: fluid_gen_type = 23;
pub const GEN_MODLFOFREQ: fluid_gen_type = 22;
pub const GEN_MODLFODELAY: fluid_gen_type = 21;
pub const GEN_UNUSED4: fluid_gen_type = 20;
pub const GEN_UNUSED3: fluid_gen_type = 19;
pub const GEN_UNUSED2: fluid_gen_type = 18;
pub const GEN_PAN: fluid_gen_type = 17;
pub const GEN_REVERBSEND: fluid_gen_type = 16;
pub const GEN_CHORUSSEND: fluid_gen_type = 15;
pub const GEN_UNUSED1: fluid_gen_type = 14;
pub const GEN_MODLFOTOVOL: fluid_gen_type = 13;
pub const GEN_ENDADDRCOARSEOFS: fluid_gen_type = 12;
pub const GEN_MODENVTOFILTERFC: fluid_gen_type = 11;
pub const GEN_MODLFOTOFILTERFC: fluid_gen_type = 10;
pub const GEN_FILTERQ: fluid_gen_type = 9;
pub const GEN_FILTERFC: fluid_gen_type = 8;
pub const GEN_MODENVTOPITCH: fluid_gen_type = 7;
pub const GEN_VIBLFOTOPITCH: fluid_gen_type = 6;
pub const GEN_MODLFOTOPITCH: fluid_gen_type = 5;
pub const GEN_STARTADDRCOARSEOFS: fluid_gen_type = 4;
pub const GEN_ENDLOOPADDROFS: fluid_gen_type = 3;
pub const GEN_STARTLOOPADDROFS: fluid_gen_type = 2;
pub const GEN_ENDADDROFS: fluid_gen_type = 1;
pub const GEN_STARTADDROFS: fluid_gen_type = 0;
pub type uint32 = libc::c_uint;
pub type fluid_voice_envelope_index_t = libc::c_uint;
pub const FLUID_VOICE_ENVLAST: fluid_voice_envelope_index_t = 7;
pub const FLUID_VOICE_ENVFINISHED: fluid_voice_envelope_index_t = 6;
pub const FLUID_VOICE_ENVRELEASE: fluid_voice_envelope_index_t = 5;
pub const FLUID_VOICE_ENVSUSTAIN: fluid_voice_envelope_index_t = 4;
pub const FLUID_VOICE_ENVDECAY: fluid_voice_envelope_index_t = 3;
pub const FLUID_VOICE_ENVHOLD: fluid_voice_envelope_index_t = 2;
pub const FLUID_VOICE_ENVATTACK: fluid_voice_envelope_index_t = 1;
pub const FLUID_VOICE_ENVDELAY: fluid_voice_envelope_index_t = 0;
pub const FLUID_LOOP_UNTIL_RELEASE: fluid_loop = 3;
pub const FLUID_LOOP_DURING_RELEASE: fluid_loop = 1;
pub type fluid_loop = libc::c_uint;
pub const FLUID_NOTUSED: fluid_loop = 2;
pub const FLUID_UNLOOPED: fluid_loop = 0;
static mut interp_coeff_linear: [[fluid_real_t; 2]; 256] = [[0.; 2]; 256];
static mut interp_coeff: [[fluid_real_t; 4]; 256] = [[0.; 4]; 256];
static mut sinc_table7: [[fluid_real_t; 7]; 256] = [[0.; 7]; 256];
#[no_mangle]
pub unsafe extern "C" fn fluid_dsp_float_config() {
    let mut i: libc::c_int = 0;
    let mut i2: libc::c_int = 0;
    let mut x: libc::c_double = 0.;
    let mut v: libc::c_double = 0.;
    let mut i_shifted: libc::c_double = 0.;
    i = 0 as libc::c_int;
    while i < 256 as libc::c_int {
        x = i as libc::c_double / 256 as libc::c_int as libc::c_double;
        interp_coeff[i as usize][0 as libc::c_int as usize] =
            (x * (-0.5f64 + x * (1 as libc::c_int as libc::c_double - 0.5f64 * x))) as fluid_real_t;
        interp_coeff[i as usize][1 as libc::c_int as usize] =
            (1.0f64 + x * x * (1.5f64 * x - 2.5f64)) as fluid_real_t;
        interp_coeff[i as usize][2 as libc::c_int as usize] =
            (x * (0.5f64 + x * (2.0f64 - 1.5f64 * x))) as fluid_real_t;
        interp_coeff[i as usize][3 as libc::c_int as usize] =
            (0.5f64 * x * x * (x - 1.0f64)) as fluid_real_t;
        interp_coeff_linear[i as usize][0 as libc::c_int as usize] = (1.0f64 - x) as fluid_real_t;
        interp_coeff_linear[i as usize][1 as libc::c_int as usize] = x as fluid_real_t;
        i += 1
    }
    i = 0 as libc::c_int;
    while i < 7 as libc::c_int {
        i2 = 0 as libc::c_int;
        while i2 < 256 as libc::c_int {
            i_shifted = i as libc::c_double - 7 as libc::c_int as libc::c_double / 2.0f64
                + i2 as libc::c_double / 256 as libc::c_int as libc::c_double;
            if f64::abs(i_shifted) > 0.000001f64 {
                v = f64::sin(i_shifted * std::f64::consts::PI) as fluid_real_t as libc::c_double
                    / (std::f64::consts::PI * i_shifted);
                v *= 0.5f64 as fluid_real_t as libc::c_double
                    * (1.0f64
                        + f64::cos(
                            2.0f64 * std::f64::consts::PI * i_shifted
                                / 7 as libc::c_int as fluid_real_t as libc::c_double,
                        ))
            } else {
                v = 1.0f64
            }
            sinc_table7[(256 as libc::c_int - i2 - 1 as libc::c_int) as usize][i as usize] =
                v as fluid_real_t;
            i2 += 1
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_dsp_float_interpolate_none(
    mut voice: *mut fluid_voice_t,
) -> libc::c_int {
    let mut dsp_phase: fluid_phase_t = (*voice).phase;
    let mut dsp_phase_incr: fluid_phase_t = 0;
    let mut dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let mut dsp_buf: *mut fluid_real_t = (*voice).dsp_buf;
    let mut dsp_amp: fluid_real_t = (*voice).amp;
    let mut dsp_amp_incr: fluid_real_t = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint = 0;
    let mut end_index: libc::c_uint = 0;
    let mut looping: libc::c_int = 0;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as libc::c_double
            - (*voice).phase_incr as libc::c_int as libc::c_double)
            * 4294967296.0f64) as uint32 as libc::c_ulonglong;
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
                as fluid_phase_t as fluid_phase_t;
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
            ) as fluid_phase_t as fluid_phase_t;
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
    let mut dsp_phase: fluid_phase_t = (*voice).phase;
    let mut dsp_phase_incr: fluid_phase_t = 0;
    let mut dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let mut dsp_buf: *mut fluid_real_t = (*voice).dsp_buf;
    let mut dsp_amp: fluid_real_t = (*voice).amp;
    let mut dsp_amp_incr: fluid_real_t = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint = 0;
    let mut end_index: libc::c_uint = 0;
    let mut point: libc::c_short = 0;
    let mut coeffs: *mut fluid_real_t = 0 as *mut fluid_real_t;
    let mut looping: libc::c_int = 0;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as libc::c_double
            - (*voice).phase_incr as libc::c_int as libc::c_double)
            * 4294967296.0f64) as uint32 as libc::c_ulonglong;
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
            coeffs = interp_coeff_linear[(((dsp_phase
                & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
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
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = interp_coeff_linear[(((dsp_phase
                & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(dsp_phase_index as isize) as libc::c_int as libc::c_float
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * point as libc::c_int as libc::c_float);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
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
            ) as fluid_phase_t as fluid_phase_t;
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
    let mut dsp_phase: fluid_phase_t = (*voice).phase;
    let mut dsp_phase_incr: fluid_phase_t = 0;
    let mut dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let mut dsp_buf: *mut fluid_real_t = (*voice).dsp_buf;
    let mut dsp_amp: fluid_real_t = (*voice).amp;
    let mut dsp_amp_incr: fluid_real_t = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint = 0;
    let mut start_index: libc::c_uint = 0;
    let mut end_index: libc::c_uint = 0;
    let mut start_point: libc::c_short = 0;
    let mut end_point1: libc::c_short = 0;
    let mut end_point2: libc::c_short = 0;
    let mut coeffs: *mut fluid_real_t = 0 as *mut fluid_real_t;
    let mut looping: libc::c_int = 0;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as libc::c_double
            - (*voice).phase_incr as libc::c_int as libc::c_double)
            * 4294967296.0f64) as uint32 as libc::c_ulonglong;
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
            coeffs = interp_coeff[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
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
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        while dsp_i < 64 as libc::c_int as libc::c_uint && dsp_phase_index <= end_index {
            coeffs = interp_coeff[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
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
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = interp_coeff[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
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
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = interp_coeff[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
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
                as fluid_phase_t as fluid_phase_t;
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
            ) as fluid_phase_t as fluid_phase_t;
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
    let mut dsp_phase: fluid_phase_t = (*voice).phase;
    let mut dsp_phase_incr: fluid_phase_t = 0;
    let mut dsp_data: *mut libc::c_short = (*(*voice).sample).data;
    let mut dsp_buf: *mut fluid_real_t = (*voice).dsp_buf;
    let mut dsp_amp: fluid_real_t = (*voice).amp;
    let mut dsp_amp_incr: fluid_real_t = (*voice).amp_incr;
    let mut dsp_i: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut dsp_phase_index: libc::c_uint = 0;
    let mut start_index: libc::c_uint = 0;
    let mut end_index: libc::c_uint = 0;
    let mut start_points: [libc::c_short; 3] = [0; 3];
    let mut end_points: [libc::c_short; 3] = [0; 3];
    let mut coeffs: *mut fluid_real_t = 0 as *mut fluid_real_t;
    let mut looping: libc::c_int = 0;
    dsp_phase_incr = ((*voice).phase_incr as libc::c_ulonglong) << 32 as libc::c_int
        | (((*voice).phase_incr as libc::c_double
            - (*voice).phase_incr as libc::c_int as libc::c_double)
            * 4294967296.0f64) as uint32 as libc::c_ulonglong;
    dsp_phase = (dsp_phase as libc::c_ulonglong)
        .wrapping_add(0x80000000 as libc::c_uint as fluid_phase_t) as fluid_phase_t
        as fluid_phase_t;
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
            coeffs = sinc_table7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * start_points[2 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * start_points[1 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * start_points[0 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as fluid_real_t
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        start_index = start_index.wrapping_add(1);
        while dsp_phase_index == start_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = sinc_table7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * start_points[1 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * start_points[0 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as fluid_real_t
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        start_index = start_index.wrapping_add(1);
        while dsp_phase_index == start_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = sinc_table7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * start_points[0 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as fluid_real_t
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        start_index = start_index.wrapping_sub(2 as libc::c_int as libc::c_uint);
        while dsp_i < 64 as libc::c_int as libc::c_uint && dsp_phase_index <= end_index {
            coeffs = sinc_table7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as fluid_real_t
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as fluid_real_t
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(3 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        if dsp_i >= 64 as libc::c_int as libc::c_uint {
            break;
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = sinc_table7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as fluid_real_t
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as fluid_real_t
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * end_points[0 as libc::c_int as usize] as fluid_real_t);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = sinc_table7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as fluid_real_t
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as fluid_real_t
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_add(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * end_points[0 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * end_points[1 as libc::c_int as usize] as fluid_real_t);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
            dsp_phase_index = (dsp_phase >> 32 as libc::c_int) as libc::c_uint;
            dsp_amp += dsp_amp_incr;
            dsp_i = dsp_i.wrapping_add(1)
        }
        end_index = end_index.wrapping_add(1);
        while dsp_phase_index <= end_index && dsp_i < 64 as libc::c_int as libc::c_uint {
            coeffs = sinc_table7[(((dsp_phase & 0xffffffff as libc::c_uint as libc::c_ulonglong)
                as uint32
                & 0xff000000 as libc::c_uint)
                >> 24 as libc::c_int) as usize]
                .as_mut_ptr();
            *dsp_buf.offset(dsp_i as isize) = dsp_amp
                * (*coeffs.offset(0 as libc::c_int as isize)
                    * *dsp_data.offset(
                        dsp_phase_index.wrapping_sub(3 as libc::c_int as libc::c_uint) as isize,
                    ) as fluid_real_t
                    + *coeffs.offset(1 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(2 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(2 as libc::c_int as isize)
                        * *dsp_data.offset(
                            dsp_phase_index.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                        ) as fluid_real_t
                    + *coeffs.offset(3 as libc::c_int as isize)
                        * *dsp_data.offset(dsp_phase_index as isize) as fluid_real_t
                    + *coeffs.offset(4 as libc::c_int as isize)
                        * end_points[0 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(5 as libc::c_int as isize)
                        * end_points[1 as libc::c_int as usize] as fluid_real_t
                    + *coeffs.offset(6 as libc::c_int as isize)
                        * end_points[2 as libc::c_int as usize] as fluid_real_t);
            dsp_phase = (dsp_phase as libc::c_ulonglong).wrapping_add(dsp_phase_incr)
                as fluid_phase_t as fluid_phase_t;
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
            ) as fluid_phase_t as fluid_phase_t;
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
        .wrapping_sub(0x80000000 as libc::c_uint as fluid_phase_t) as fluid_phase_t
        as fluid_phase_t;
    (*voice).phase = dsp_phase;
    (*voice).amp = dsp_amp;
    return dsp_i as libc::c_int;
}
