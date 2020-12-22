#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::fluid_synth::_fluid_synth_t;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn fluid_convex(val: fluid_real_t) -> fluid_real_t;
    #[no_mangle]
    fn fluid_concave(val: fluid_real_t) -> fluid_real_t;
    #[no_mangle]
    fn fluid_channel_get_cc(chan: *mut fluid_channel_t, num: libc::c_int) -> libc::c_int;
}
pub type fluid_synth_t = _fluid_synth_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_voice_t {
    pub id: libc::c_uint,
    pub status: libc::c_uchar,
    pub chan: libc::c_uchar,
    pub key: libc::c_uchar,
    pub vel: libc::c_uchar,
    pub channel: *mut fluid_channel_t,
    pub gen: [fluid_gen_t; 60],
    pub mod_0: [fluid_mod_t; 64],
    pub mod_count: libc::c_int,
    pub has_looped: libc::c_int,
    pub sample: *mut fluid_sample_t,
    pub check_sample_sanity_flag: libc::c_int,
    pub output_rate: fluid_real_t,
    pub start_time: libc::c_uint,
    pub ticks: libc::c_uint,
    pub noteoff_ticks: libc::c_uint,
    pub amp: fluid_real_t,
    pub phase: fluid_phase_t,
    pub phase_incr: fluid_real_t,
    pub amp_incr: fluid_real_t,
    pub dsp_buf: *mut fluid_real_t,
    pub pitch: fluid_real_t,
    pub attenuation: fluid_real_t,
    pub min_attenuation_cB: fluid_real_t,
    pub root_pitch: fluid_real_t,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub loopstart: libc::c_int,
    pub loopend: libc::c_int,
    pub synth_gain: fluid_real_t,
    pub volenv_data: [fluid_env_data_t; 7],
    pub volenv_count: libc::c_uint,
    pub volenv_section: libc::c_int,
    pub volenv_val: fluid_real_t,
    pub amplitude_that_reaches_noise_floor_nonloop: fluid_real_t,
    pub amplitude_that_reaches_noise_floor_loop: fluid_real_t,
    pub modenv_data: [fluid_env_data_t; 7],
    pub modenv_count: libc::c_uint,
    pub modenv_section: libc::c_int,
    pub modenv_val: fluid_real_t,
    pub modenv_to_fc: fluid_real_t,
    pub modenv_to_pitch: fluid_real_t,
    pub modlfo_val: fluid_real_t,
    pub modlfo_delay: libc::c_uint,
    pub modlfo_incr: fluid_real_t,
    pub modlfo_to_fc: fluid_real_t,
    pub modlfo_to_pitch: fluid_real_t,
    pub modlfo_to_vol: fluid_real_t,
    pub viblfo_val: fluid_real_t,
    pub viblfo_delay: libc::c_uint,
    pub viblfo_incr: fluid_real_t,
    pub viblfo_to_pitch: fluid_real_t,
    pub fres: fluid_real_t,
    pub last_fres: fluid_real_t,
    pub q_lin: fluid_real_t,
    pub filter_gain: fluid_real_t,
    pub hist1: fluid_real_t,
    pub hist2: fluid_real_t,
    pub filter_startup: libc::c_int,
    pub b02: fluid_real_t,
    pub b1: fluid_real_t,
    pub a1: fluid_real_t,
    pub a2: fluid_real_t,
    pub b02_incr: fluid_real_t,
    pub b1_incr: fluid_real_t,
    pub a1_incr: fluid_real_t,
    pub a2_incr: fluid_real_t,
    pub filter_coeff_incr_count: libc::c_int,
    pub pan: fluid_real_t,
    pub amp_left: fluid_real_t,
    pub amp_right: fluid_real_t,
    pub reverb_send: fluid_real_t,
    pub amp_reverb: fluid_real_t,
    pub chorus_send: fluid_real_t,
    pub amp_chorus: fluid_real_t,
    pub interp_method: libc::c_int,
    pub debug: libc::c_int,
}

pub type fluid_real_t = libc::c_float;

pub type fluid_env_data_t = _fluid_env_data_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_env_data_t {
    pub count: libc::c_uint,
    pub coeff: fluid_real_t,
    pub incr: fluid_real_t,
    pub min: fluid_real_t,
    pub max: fluid_real_t,
}

pub type fluid_phase_t = libc::c_ulonglong;
pub type fluid_sample_t = _fluid_sample_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_sample_t {
    pub name: [libc::c_char; 21],
    pub start: libc::c_uint,
    pub end: libc::c_uint,
    pub loopstart: libc::c_uint,
    pub loopend: libc::c_uint,
    pub samplerate: libc::c_uint,
    pub origpitch: libc::c_int,
    pub pitchadj: libc::c_int,
    pub sampletype: libc::c_int,
    pub valid: libc::c_int,
    pub data: *mut libc::c_short,
    pub amplitude_that_reaches_noise_floor_is_valid: libc::c_int,
    pub amplitude_that_reaches_noise_floor: libc::c_double,
    pub refcount: libc::c_uint,
    pub notify: Option<unsafe extern "C" fn(_: *mut fluid_sample_t, _: libc::c_int) -> libc::c_int>,
    pub userdata: *mut libc::c_void,
}
pub type fluid_mod_t = _fluid_mod_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_mod_t {
    pub dest: libc::c_uchar,
    pub src1: libc::c_uchar,
    pub flags1: libc::c_uchar,
    pub src2: libc::c_uchar,
    pub flags2: libc::c_uchar,
    pub amount: libc::c_double,
    pub next: *mut fluid_mod_t,
}

pub type fluid_gen_t = _fluid_gen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_gen_t {
    pub flags: libc::c_uchar,
    pub val: libc::c_double,
    pub mod_0: libc::c_double,
    pub nrpn: libc::c_double,
}
pub type fluid_channel_t = _fluid_channel_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_channel_t {
    pub channum: libc::c_int,
    pub sfontnum: libc::c_uint,
    pub banknum: libc::c_uint,
    pub prognum: libc::c_uint,
    pub preset: *mut fluid_preset_t,
    pub synth: *mut fluid_synth_t,
    pub key_pressure: [libc::c_char; 128],
    pub channel_pressure: libc::c_short,
    pub pitch_bend: libc::c_short,
    pub pitch_wheel_sensitivity: libc::c_short,
    pub cc: [libc::c_short; 128],
    pub bank_msb: libc::c_uchar,
    pub interp_method: libc::c_int,
    pub tuning: *mut fluid_tuning_t,
    pub nrpn_select: libc::c_short,
    pub nrpn_active: libc::c_short,
    pub gen: [fluid_real_t; 60],
    pub gen_abs: [libc::c_char; 60],
}
pub type fluid_tuning_t = _fluid_tuning_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_tuning_t {
    pub name: *mut libc::c_char,
    pub bank: libc::c_int,
    pub prog: libc::c_int,
    pub pitch: [libc::c_double; 128],
}
pub type fluid_preset_t = _fluid_preset_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_preset_t {
    pub data: *mut libc::c_void,
    pub sfont: *mut fluid_sfont_t,
    pub free: Option<unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int>,
    pub get_name: Option<unsafe extern "C" fn(_: *mut fluid_preset_t) -> *mut libc::c_char>,
    pub get_banknum: Option<unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int>,
    pub get_num: Option<unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int>,
    pub noteon: Option<
        unsafe extern "C" fn(
            _: *mut fluid_preset_t,
            _: *mut fluid_synth_t,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub notify: Option<
        unsafe extern "C" fn(_: *mut fluid_preset_t, _: libc::c_int, _: libc::c_int) -> libc::c_int,
    >,
}
pub type fluid_sfont_t = _fluid_sfont_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_sfont_t {
    pub data: *mut libc::c_void,
    pub id: libc::c_uint,
    pub free: Option<unsafe extern "C" fn(_: *mut fluid_sfont_t) -> libc::c_int>,
    pub get_name: Option<unsafe extern "C" fn(_: *mut fluid_sfont_t) -> *mut libc::c_char>,
    pub get_preset: Option<
        unsafe extern "C" fn(
            _: *mut fluid_sfont_t,
            _: libc::c_uint,
            _: libc::c_uint,
        ) -> *mut fluid_preset_t,
    >,
    pub iteration_start: Option<unsafe extern "C" fn(_: *mut fluid_sfont_t) -> ()>,
    pub iteration_next:
        Option<unsafe extern "C" fn(_: *mut fluid_sfont_t, _: *mut fluid_preset_t) -> libc::c_int>,
}
pub type fluid_voice_t = _fluid_voice_t;

pub type fluid_log_level = libc::c_uint;

pub const LAST_LOG_LEVEL: fluid_log_level = 5;

pub const FLUID_DBG: fluid_log_level = 4;

pub const FLUID_INFO: fluid_log_level = 3;

pub const FLUID_WARN: fluid_log_level = 2;

pub const FLUID_ERR: fluid_log_level = 1;
pub const FLUID_PANIC: fluid_log_level = 0;
pub type fluid_mod_flags = libc::c_uint;
pub const FLUID_MOD_CC: fluid_mod_flags = 16;
pub const FLUID_MOD_GC: fluid_mod_flags = 0;
pub const FLUID_MOD_SWITCH: fluid_mod_flags = 12;
pub const FLUID_MOD_CONVEX: fluid_mod_flags = 8;
pub const FLUID_MOD_CONCAVE: fluid_mod_flags = 4;
pub const FLUID_MOD_LINEAR: fluid_mod_flags = 0;
pub const FLUID_MOD_BIPOLAR: fluid_mod_flags = 2;
pub const FLUID_MOD_UNIPOLAR: fluid_mod_flags = 0;
pub const FLUID_MOD_NEGATIVE: fluid_mod_flags = 1;
pub const FLUID_MOD_POSITIVE: fluid_mod_flags = 0;
pub type fluid_mod_src = libc::c_uint;
pub const FLUID_MOD_PITCHWHEELSENS: fluid_mod_src = 16;
pub const FLUID_MOD_PITCHWHEEL: fluid_mod_src = 14;
pub const FLUID_MOD_CHANNELPRESSURE: fluid_mod_src = 13;
pub const FLUID_MOD_KEYPRESSURE: fluid_mod_src = 10;
pub const FLUID_MOD_KEY: fluid_mod_src = 3;
pub const FLUID_MOD_VELOCITY: fluid_mod_src = 2;
pub const FLUID_MOD_NONE: fluid_mod_src = 0;
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

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_clone(mut mod_0: *mut fluid_mod_t, mut src: *mut fluid_mod_t) {
    (*mod_0).dest = (*src).dest;
    (*mod_0).src1 = (*src).src1;
    (*mod_0).flags1 = (*src).flags1;
    (*mod_0).src2 = (*src).src2;
    (*mod_0).flags2 = (*src).flags2;
    (*mod_0).amount = (*src).amount;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_source1(
    mut mod_0: *mut fluid_mod_t,
    mut src: libc::c_int,
    mut flags: libc::c_int,
) {
    (*mod_0).src1 = src as libc::c_uchar;
    (*mod_0).flags1 = flags as libc::c_uchar;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_source2(
    mut mod_0: *mut fluid_mod_t,
    mut src: libc::c_int,
    mut flags: libc::c_int,
) {
    (*mod_0).src2 = src as libc::c_uchar;
    (*mod_0).flags2 = flags as libc::c_uchar;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_dest(mut mod_0: *mut fluid_mod_t, mut dest: libc::c_int) {
    (*mod_0).dest = dest as libc::c_uchar;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_set_amount(
    mut mod_0: *mut fluid_mod_t,
    mut amount: libc::c_double,
) {
    (*mod_0).amount = amount;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_source1(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).src1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_flags1(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).flags1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_source2(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).src2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_flags2(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).flags2 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_dest(mut mod_0: *mut fluid_mod_t) -> libc::c_int {
    return (*mod_0).dest as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_amount(mut mod_0: *mut fluid_mod_t) -> libc::c_double {
    return (*mod_0).amount as fluid_real_t as libc::c_double;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_get_value(
    mut mod_0: *mut fluid_mod_t,
    mut chan: *mut fluid_channel_t,
    mut voice: *mut fluid_voice_t,
) -> fluid_real_t {
    let mut v1: fluid_real_t = 0.0f64 as fluid_real_t;
    let mut v2: fluid_real_t = 1.0f64 as fluid_real_t;
    let mut range1: fluid_real_t = 127.0f64 as fluid_real_t;
    let mut range2: fluid_real_t = 127.0f64 as fluid_real_t;
    if chan.is_null() {
        return 0.0f32;
    }

    if (*mod_0).src2 as libc::c_int == FLUID_MOD_VELOCITY as libc::c_int
        && (*mod_0).src1 as libc::c_int == FLUID_MOD_VELOCITY as libc::c_int
        && (*mod_0).flags1 as libc::c_int
            == FLUID_MOD_GC as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int
                | FLUID_MOD_NEGATIVE as libc::c_int
                | FLUID_MOD_LINEAR as libc::c_int
        && (*mod_0).flags2 as libc::c_int
            == FLUID_MOD_GC as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int
                | FLUID_MOD_POSITIVE as libc::c_int
                | FLUID_MOD_SWITCH as libc::c_int
        && (*mod_0).dest as libc::c_int == GEN_FILTERFC as libc::c_int
    {
        return 0 as libc::c_int as fluid_real_t;
    }
    if (*mod_0).src1 as libc::c_int > 0 as libc::c_int {
        if (*mod_0).flags1 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0 {
            v1 = fluid_channel_get_cc(chan, (*mod_0).src1 as libc::c_int) as fluid_real_t
        } else {
            match (*mod_0).src1 as libc::c_int {
                0 => v1 = range1,
                2 => v1 = (*voice).vel as fluid_real_t,
                3 => v1 = (*voice).key as fluid_real_t,
                10 => v1 = (*chan).key_pressure[(*voice).key as usize] as fluid_real_t,
                13 => v1 = (*chan).channel_pressure as fluid_real_t,
                14 => {
                    v1 = (*chan).pitch_bend as fluid_real_t;
                    range1 = 0x4000 as libc::c_int as fluid_real_t
                }
                16 => v1 = (*chan).pitch_wheel_sensitivity as fluid_real_t,
                _ => v1 = 0.0f64 as fluid_real_t,
            }
        }
        match (*mod_0).flags1 as libc::c_int & 0xf as libc::c_int {
            0 => v1 /= range1,
            1 => v1 = 1.0f32 - v1 / range1,
            2 => v1 = -1.0f32 + 2.0f32 * v1 / range1,
            3 => v1 = 1.0f32 - 2.0f32 * v1 / range1,
            4 => v1 = fluid_concave(v1),
            5 => v1 = fluid_concave(127 as libc::c_int as libc::c_float - v1),
            6 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            7 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            8 => v1 = fluid_convex(v1),
            9 => v1 = fluid_convex(127 as libc::c_int as libc::c_float - v1),
            10 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            11 => {
                v1 = if v1 > 64 as libc::c_int as libc::c_float {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v1 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v1),
                    )
                }
            }
            12 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    0.0f32
                }
            }
            13 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    0.0f32
                } else {
                    1.0f32
                }
            }
            14 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    -1.0f32
                }
            }
            15 => {
                v1 = if v1 >= 64 as libc::c_int as libc::c_float {
                    -1.0f32
                } else {
                    1.0f32
                }
            }
            _ => {}
        }
    } else {
        return 0.0f64 as fluid_real_t;
    }

    if v1 == 0.0f32 {
        return 0.0f32;
    }
    if (*mod_0).src2 as libc::c_int > 0 as libc::c_int {
        if (*mod_0).flags2 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0 {
            v2 = fluid_channel_get_cc(chan, (*mod_0).src2 as libc::c_int) as fluid_real_t
        } else {
            match (*mod_0).src2 as libc::c_int {
                0 => v2 = range2,
                2 => v2 = (*voice).vel as fluid_real_t,
                3 => v2 = (*voice).key as fluid_real_t,
                10 => v2 = (*chan).key_pressure[(*voice).key as usize] as fluid_real_t,
                13 => v2 = (*chan).channel_pressure as fluid_real_t,
                14 => v2 = (*chan).pitch_bend as fluid_real_t,
                16 => v2 = (*chan).pitch_wheel_sensitivity as fluid_real_t,
                _ => v1 = 0.0f32,
            }
        }
        match (*mod_0).flags2 as libc::c_int & 0xf as libc::c_int {
            0 => v2 /= range2,
            1 => v2 = 1.0f32 - v2 / range2,
            2 => v2 = -1.0f32 + 2.0f32 * v2 / range2,
            3 => v2 = -1.0f32 + 2.0f32 * v2 / range2,
            4 => v2 = fluid_concave(v2),
            5 => v2 = fluid_concave(127 as libc::c_int as libc::c_float - v2),
            6 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            7 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    -fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_concave(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            8 => v2 = fluid_convex(v2),
            9 => v2 = 1.0f32 - fluid_convex(v2),
            10 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            11 => {
                v2 = if v2 > 64 as libc::c_int as libc::c_float {
                    -fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (v2 - 64 as libc::c_int as libc::c_float),
                    )
                } else {
                    fluid_convex(
                        2 as libc::c_int as libc::c_float
                            * (64 as libc::c_int as libc::c_float - v2),
                    )
                }
            }
            12 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    0.0f32
                }
            }
            13 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
                    0.0f32
                } else {
                    1.0f32
                }
            }
            14 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
                    1.0f32
                } else {
                    -1.0f32
                }
            }
            15 => {
                v2 = if v2 >= 64 as libc::c_int as libc::c_float {
                    -1.0f32
                } else {
                    1.0f32
                }
            }
            _ => {}
        }
    } else {
        v2 = 1.0f32
    }
    return (*mod_0).amount as fluid_real_t * v1 * v2;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_new() -> *mut fluid_mod_t {
    let mut mod_0: *mut fluid_mod_t =
        malloc(::std::mem::size_of::<fluid_mod_t>() as libc::c_ulong) as *mut fluid_mod_t;
    if mod_0.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return 0 as *mut fluid_mod_t;
    }
    return mod_0;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_delete(mut mod_0: *mut fluid_mod_t) {
    free(mod_0 as *mut libc::c_void);
}

#[no_mangle]
pub unsafe extern "C" fn fluid_mod_test_identity(
    mut mod1: *mut fluid_mod_t,
    mut mod2: *mut fluid_mod_t,
) -> libc::c_int {
    if (*mod1).dest as libc::c_int != (*mod2).dest as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).src1 as libc::c_int != (*mod2).src1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).src2 as libc::c_int != (*mod2).src2 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).flags1 as libc::c_int != (*mod2).flags1 as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*mod1).flags2 as libc::c_int != (*mod2).flags2 as libc::c_int {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_dump_modulator(mut mod_0: *mut fluid_mod_t) {
    let mut src1: libc::c_int = (*mod_0).src1 as libc::c_int;
    let mut dest: libc::c_int = (*mod_0).dest as libc::c_int;
    let mut src2: libc::c_int = (*mod_0).src2 as libc::c_int;
    let mut flags1: libc::c_int = (*mod_0).flags1 as libc::c_int;
    let mut flags2: libc::c_int = (*mod_0).flags2 as libc::c_int;
    let mut amount: fluid_real_t = (*mod_0).amount as fluid_real_t;
    // printf(b"Src: \x00" as *const u8 as *const libc::c_char);
    if flags1 & FLUID_MOD_CC as libc::c_int != 0 {
        // printf(b"MIDI CC=%i\x00" as *const u8 as *const libc::c_char, src1);
    } else {
        match src1 {
            0 => {
                // printf(b"None\x00" as *const u8 as *const libc::c_char);
            }
            2 => {
                // printf(b"note-on velocity\x00" as *const u8 as *const libc::c_char);
            }
            3 => {
                // printf(b"Key nr\x00" as *const u8 as *const libc::c_char);
            }
            10 => {
                // printf(b"Poly pressure\x00" as *const u8 as *const libc::c_char);
            }
            13 => {
                // printf(b"Chan pressure\x00" as *const u8 as *const libc::c_char);
            }
            14 => {
                // printf(b"Pitch Wheel\x00" as *const u8 as *const libc::c_char);
            }
            16 => {
                // printf(b"Pitch Wheel sens\x00" as *const u8 as *const libc::c_char);
            }
            _ => {
                // printf(
                //    b"(unknown: %i)\x00" as *const u8 as *const libc::c_char,
                //    src1,
                //);
            }
        }
    }
    if flags1 & FLUID_MOD_NEGATIVE as libc::c_int != 0 {
        // printf(b"- \x00" as *const u8 as *const libc::c_char);
    } else {
        // printf(b"+ \x00" as *const u8 as *const libc::c_char);
    }
    if flags1 & FLUID_MOD_BIPOLAR as libc::c_int != 0 {
        // printf(b"bip \x00" as *const u8 as *const libc::c_char);
    } else {
        // printf(b"unip \x00" as *const u8 as *const libc::c_char);
    }
    // printf(b"-> \x00" as *const u8 as *const libc::c_char);
    match dest {
        9 => {
            // printf(b"Q\x00" as *const u8 as *const libc::c_char);
        }
        8 => {
            // printf(b"fc\x00" as *const u8 as *const libc::c_char);
        }
        6 => {
            // printf(b"VibLFO-to-pitch\x00" as *const u8 as *const libc::c_char);
        }
        7 => {
            // printf(b"ModEnv-to-pitch\x00" as *const u8 as *const libc::c_char);
        }
        5 => {
            // printf(b"ModLFO-to-pitch\x00" as *const u8 as *const libc::c_char);
        }
        15 => {
            // printf(b"Chorus send\x00" as *const u8 as *const libc::c_char);
        }
        16 => {
            // printf(b"Reverb send\x00" as *const u8 as *const libc::c_char);
        }
        17 => {
            // printf(b"pan\x00" as *const u8 as *const libc::c_char);
        }
        48 => {
            // printf(b"att\x00" as *const u8 as *const libc::c_char);
        }
        _ => {
            // printf(b"dest %i\x00" as *const u8 as *const libc::c_char, dest);
        }
    }

    // printf(
    //    b", amount %f flags %i src2 %i flags2 %i\n\x00" as *const u8 as *const libc::c_char,
    //    amount as libc::c_double,
    //    flags1,
    //    src2,
    //    flags2,
    //);
}
