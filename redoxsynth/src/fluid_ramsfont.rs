#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(extern_types)]

use crate::fluid_chorus::_fluid_chorus_t;
use crate::fluid_hash::_fluid_hashtable_t;
use crate::fluid_tuning::_fluid_tuning_t;
use crate::fluid_rev::_fluid_revmodel_t;
use crate::fluid_chan::_fluid_channel_t;

extern "C" {
    #[no_mangle]
    fn fluid_synth_alloc_voice(
        synth: *mut fluid_synth_t,
        sample: *mut fluid_sample_t,
        channum: libc::c_int,
        key: libc::c_int,
        vel: libc::c_int,
    ) -> *mut fluid_voice_t;
    #[no_mangle]
    fn fluid_synth_start_voice(synth: *mut fluid_synth_t, voice: *mut fluid_voice_t);
    #[no_mangle]
    fn fluid_voice_add_mod(voice: *mut fluid_voice_t, mod_0: *mut fluid_mod_t, mode: libc::c_int);
    #[no_mangle]
    fn fluid_mod_test_identity(mod1: *mut fluid_mod_t, mod2: *mut fluid_mod_t) -> libc::c_int;
    #[no_mangle]
    fn fluid_voice_gen_incr(voice: *mut fluid_voice_t, gen: libc::c_int, val: libc::c_float);
    #[no_mangle]
    fn fluid_voice_gen_set(voice: *mut fluid_voice_t, gen: libc::c_int, val: libc::c_float);
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn fluid_list_append(list: *mut fluid_list_t, data: *mut libc::c_void) -> *mut fluid_list_t;
    #[no_mangle]
    fn fluid_voice_get_id(voice: *mut fluid_voice_t) -> libc::c_uint;
    #[no_mangle]
    fn delete_fluid_list(list: *mut fluid_list_t);
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn fluid_list_remove(list: *mut fluid_list_t, data: *mut libc::c_void) -> *mut fluid_list_t;
    #[no_mangle]
    fn fluid_voice_is_playing(voice: *mut fluid_voice_t) -> libc::c_int;
    #[no_mangle]
    fn fluid_voice_update_param(voice: *mut fluid_voice_t, gen: libc::c_int);
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn fluid_preset_zone_inside_range(
        zone: *mut fluid_preset_zone_t,
        key: libc::c_int,
        vel: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn fluid_preset_zone_next(preset: *mut fluid_preset_zone_t) -> *mut fluid_preset_zone_t;
    #[no_mangle]
    fn new_fluid_preset_zone(name: *mut libc::c_char) -> *mut fluid_preset_zone_t;
    #[no_mangle]
    fn delete_fluid_preset_zone(zone: *mut fluid_preset_zone_t) -> libc::c_int;
    #[no_mangle]
    fn fluid_preset_zone_get_inst(zone: *mut fluid_preset_zone_t) -> *mut fluid_inst_t;
    #[no_mangle]
    fn new_fluid_inst() -> *mut fluid_inst_t;
    #[no_mangle]
    fn fluid_inst_add_zone(inst: *mut fluid_inst_t, zone: *mut fluid_inst_zone_t) -> libc::c_int;
    #[no_mangle]
    fn fluid_inst_get_zone(inst: *mut fluid_inst_t) -> *mut fluid_inst_zone_t;
    #[no_mangle]
    fn fluid_inst_get_global_zone(inst: *mut fluid_inst_t) -> *mut fluid_inst_zone_t;
    #[no_mangle]
    fn new_fluid_inst_zone(name: *mut libc::c_char) -> *mut fluid_inst_zone_t;
    #[no_mangle]
    fn delete_fluid_inst_zone(zone: *mut fluid_inst_zone_t) -> libc::c_int;
    #[no_mangle]
    fn fluid_inst_zone_next(zone: *mut fluid_inst_zone_t) -> *mut fluid_inst_zone_t;
    #[no_mangle]
    fn fluid_inst_zone_inside_range(
        zone: *mut fluid_inst_zone_t,
        key: libc::c_int,
        vel: libc::c_int,
    ) -> libc::c_int;
    #[no_mangle]
    fn fluid_inst_zone_get_sample(zone: *mut fluid_inst_zone_t) -> *mut fluid_sample_t;
    #[no_mangle]
    fn fluid_sample_in_rom(sample: *mut fluid_sample_t) -> libc::c_int;
    #[no_mangle]
    fn fluid_voice_off(voice: *mut fluid_voice_t) -> libc::c_int;
}
pub type fluid_settings_t = _fluid_hashtable_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_synth_t {
    pub settings: *mut fluid_settings_t,
    pub polyphony: libc::c_int,
    pub with_reverb: libc::c_char,
    pub with_chorus: libc::c_char,
    pub verbose: libc::c_char,
    pub dump: libc::c_char,
    pub sample_rate: libc::c_double,
    pub midi_channels: libc::c_int,
    pub audio_channels: libc::c_int,
    pub audio_groups: libc::c_int,
    pub effects_channels: libc::c_int,
    pub state: libc::c_uint,
    pub ticks: libc::c_uint,
    pub loaders: *mut fluid_list_t,
    pub sfont: *mut fluid_list_t,
    pub sfont_id: libc::c_uint,
    pub bank_offsets: *mut fluid_list_t,
    pub gain: libc::c_double,
    pub channel: *mut *mut fluid_channel_t,
    pub num_channels: libc::c_int,
    pub nvoice: libc::c_int,
    pub voice: *mut *mut fluid_voice_t,
    pub noteid: libc::c_uint,
    pub storeid: libc::c_uint,
    pub nbuf: libc::c_int,
    pub left_buf: *mut *mut fluid_real_t,
    pub right_buf: *mut *mut fluid_real_t,
    pub fx_left_buf: *mut *mut fluid_real_t,
    pub fx_right_buf: *mut *mut fluid_real_t,
    pub reverb: *mut fluid_revmodel_t,
    pub chorus: *mut fluid_chorus_t,
    pub cur: libc::c_int,
    pub dither_index: libc::c_int,
    pub outbuf: [libc::c_char; 256],
    pub tuning: *mut *mut *mut fluid_tuning_t,
    pub cur_tuning: *mut fluid_tuning_t,
    pub min_note_length_ticks: libc::c_uint,
}
pub type fluid_tuning_t = _fluid_tuning_t;
pub type fluid_chorus_t = _fluid_chorus_t;

pub type fluid_revmodel_t = _fluid_revmodel_t;
pub type fluid_real_t = libc::c_float;
pub type fluid_voice_t = _fluid_voice_t;

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
pub type fluid_list_t = _fluid_list_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_list_t {
    pub data: *mut libc::c_void,
    pub next: *mut fluid_list_t,
}
pub type fluid_synth_t = _fluid_synth_t;
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

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_ramsfont_t {
    pub name: [libc::c_char; 21],
    pub sample: *mut fluid_list_t,
    pub preset: *mut fluid_rampreset_t,
    pub iter_preset: fluid_preset_t,
    pub iter_cur: *mut fluid_rampreset_t,
}
pub type fluid_rampreset_t = _fluid_rampreset_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_rampreset_t {
    pub next: *mut fluid_rampreset_t,
    pub sfont: *mut fluid_ramsfont_t,
    pub name: [libc::c_char; 21],
    pub bank: libc::c_uint,
    pub num: libc::c_uint,
    pub global_zone: *mut fluid_preset_zone_t,
    pub zone: *mut fluid_preset_zone_t,
    pub presetvoices: *mut fluid_list_t,
}
pub type fluid_preset_zone_t = _fluid_preset_zone_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_preset_zone_t {
    pub next: *mut fluid_preset_zone_t,
    pub name: *mut libc::c_char,
    pub inst: *mut fluid_inst_t,
    pub keylo: libc::c_int,
    pub keyhi: libc::c_int,
    pub vello: libc::c_int,
    pub velhi: libc::c_int,
    pub gen: [fluid_gen_t; 60],
    pub mod_0: *mut fluid_mod_t,
}
pub type fluid_inst_t = _fluid_inst_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_inst_t {
    pub name: [libc::c_char; 21],
    pub global_zone: *mut fluid_inst_zone_t,
    pub zone: *mut fluid_inst_zone_t,
}
pub type fluid_inst_zone_t = _fluid_inst_zone_t;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_inst_zone_t {
    pub next: *mut fluid_inst_zone_t,
    pub name: *mut libc::c_char,
    pub sample: *mut fluid_sample_t,
    pub keylo: libc::c_int,
    pub keyhi: libc::c_int,
    pub vello: libc::c_int,
    pub velhi: libc::c_int,
    pub gen: [fluid_gen_t; 60],
    pub mod_0: *mut fluid_mod_t,
}
pub type fluid_ramsfont_t = _fluid_ramsfont_t;
pub const FLUID_OK: C2RustUnnamed = 0;
pub const FLUID_VOICE_ADD: fluid_voice_add_mod = 1;

pub const GEN_OVERRIDEROOTKEY: fluid_gen_type = 58;

pub const GEN_EXCLUSIVECLASS: fluid_gen_type = 57;

pub const GEN_SAMPLEMODE: fluid_gen_type = 54;

pub const GEN_ENDLOOPADDRCOARSEOFS: fluid_gen_type = 50;

pub const GEN_VELOCITY: fluid_gen_type = 47;

pub const GEN_KEYNUM: fluid_gen_type = 46;

pub const GEN_STARTLOOPADDRCOARSEOFS: fluid_gen_type = 45;

pub const GEN_ENDADDRCOARSEOFS: fluid_gen_type = 12;

pub const GEN_STARTADDRCOARSEOFS: fluid_gen_type = 4;

pub const GEN_ENDLOOPADDROFS: fluid_gen_type = 3;

pub const GEN_STARTLOOPADDROFS: fluid_gen_type = 2;

pub const GEN_ENDADDROFS: fluid_gen_type = 1;
pub const GEN_STARTADDROFS: fluid_gen_type = 0;

pub const GEN_LAST: fluid_gen_type = 60;
pub const FLUID_VOICE_OVERWRITE: fluid_voice_add_mod = 0;
pub const FLUID_FAILED: C2RustUnnamed = -1;
pub const FLUID_ERR: fluid_log_level = 1;

pub type fluid_rampreset_voice_t = _fluid_rampreset_voice_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_rampreset_voice_t {
    pub voice: *mut fluid_voice_t,
    pub voiceID: libc::c_uint,
}

pub const GEN_SET: fluid_gen_flags = 1;
pub const FLUID_LOOP_DURING_RELEASE: fluid_loop = 1;
pub const GEN_UNUSED: fluid_gen_flags = 0;
pub const FLUID_UNLOOPED: fluid_loop = 0;
pub type fluid_log_level = libc::c_uint;
pub const LAST_LOG_LEVEL: fluid_log_level = 5;
pub const FLUID_DBG: fluid_log_level = 4;
pub const FLUID_INFO: fluid_log_level = 3;
pub const FLUID_WARN: fluid_log_level = 2;
pub const FLUID_PANIC: fluid_log_level = 0;

pub type fluid_gen_type = libc::c_uint;

pub const GEN_PITCH: fluid_gen_type = 59;

pub const GEN_SCALETUNE: fluid_gen_type = 56;

pub const GEN_RESERVED3: fluid_gen_type = 55;

pub const GEN_SAMPLEID: fluid_gen_type = 53;

pub const GEN_FINETUNE: fluid_gen_type = 52;

pub const GEN_COARSETUNE: fluid_gen_type = 51;

pub const GEN_RESERVED2: fluid_gen_type = 49;

pub const GEN_ATTENUATION: fluid_gen_type = 48;

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

pub const GEN_MODENVTOFILTERFC: fluid_gen_type = 11;

pub const GEN_MODLFOTOFILTERFC: fluid_gen_type = 10;

pub const GEN_FILTERQ: fluid_gen_type = 9;

pub const GEN_FILTERFC: fluid_gen_type = 8;

pub const GEN_MODENVTOPITCH: fluid_gen_type = 7;

pub const GEN_VIBLFOTOPITCH: fluid_gen_type = 6;

pub const GEN_MODLFOTOPITCH: fluid_gen_type = 5;

pub type fluid_gen_flags = libc::c_uint;

pub const GEN_ABS_NRPN: fluid_gen_flags = 2;
pub type fluid_voice_add_mod = libc::c_uint;
pub const FLUID_VOICE_DEFAULT: fluid_voice_add_mod = 2;
pub type C2RustUnnamed = libc::c_int;
pub type fluid_loop = libc::c_uint;
pub const FLUID_LOOP_UNTIL_RELEASE: fluid_loop = 3;
pub const FLUID_NOTUSED: fluid_loop = 2;

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_create_sfont() -> *mut fluid_sfont_t {
    let mut sfont: *mut fluid_sfont_t = 0 as *mut fluid_sfont_t;
    let mut ramsfont: *mut fluid_ramsfont_t = 0 as *mut fluid_ramsfont_t;
    ramsfont = new_fluid_ramsfont();
    if ramsfont.is_null() {
        return 0 as *mut fluid_sfont_t;
    }
    sfont = malloc(::std::mem::size_of::<fluid_sfont_t>() as libc::c_ulong) as *mut fluid_sfont_t;
    if sfont.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return 0 as *mut fluid_sfont_t;
    }
    (*sfont).data = ramsfont as *mut libc::c_void;
    (*sfont).free = Some(
        fluid_ramsfont_sfont_delete as unsafe extern "C" fn(_: *mut fluid_sfont_t) -> libc::c_int,
    );
    (*sfont).get_name = Some(
        fluid_ramsfont_sfont_get_name
            as unsafe extern "C" fn(_: *mut fluid_sfont_t) -> *mut libc::c_char,
    );
    (*sfont).get_preset = Some(
        fluid_ramsfont_sfont_get_preset
            as unsafe extern "C" fn(
                _: *mut fluid_sfont_t,
                _: libc::c_uint,
                _: libc::c_uint,
            ) -> *mut fluid_preset_t,
    );
    (*sfont).iteration_start = Some(
        fluid_ramsfont_sfont_iteration_start as unsafe extern "C" fn(_: *mut fluid_sfont_t) -> (),
    );
    (*sfont).iteration_next = Some(
        fluid_ramsfont_sfont_iteration_next
            as unsafe extern "C" fn(_: *mut fluid_sfont_t, _: *mut fluid_preset_t) -> libc::c_int,
    );
    return sfont;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_sfont_delete(mut sfont: *mut fluid_sfont_t) -> libc::c_int {
    if delete_fluid_ramsfont((*sfont).data as *mut fluid_ramsfont_t) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    free(sfont as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_sfont_get_name(
    mut sfont: *mut fluid_sfont_t,
) -> *mut libc::c_char {
    return fluid_ramsfont_get_name((*sfont).data as *mut fluid_ramsfont_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_sfont_get_preset(
    mut sfont: *mut fluid_sfont_t,
    mut bank: libc::c_uint,
    mut prenum: libc::c_uint,
) -> *mut fluid_preset_t {
    let mut preset: *mut fluid_preset_t = 0 as *mut fluid_preset_t;
    let mut rampreset: *mut fluid_rampreset_t = 0 as *mut fluid_rampreset_t;
    rampreset = fluid_ramsfont_get_preset((*sfont).data as *mut fluid_ramsfont_t, bank, prenum);
    if rampreset.is_null() {
        return 0 as *mut fluid_preset_t;
    }
    preset =
        malloc(::std::mem::size_of::<fluid_preset_t>() as libc::c_ulong) as *mut fluid_preset_t;
    if preset.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return 0 as *mut fluid_preset_t;
    }
    (*preset).sfont = sfont;
    (*preset).data = rampreset as *mut libc::c_void;
    (*preset).free = Some(
        fluid_rampreset_preset_delete
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_name = Some(
        fluid_rampreset_preset_get_name
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> *mut libc::c_char,
    );
    (*preset).get_banknum = Some(
        fluid_rampreset_preset_get_banknum
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_num = Some(
        fluid_rampreset_preset_get_num
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).noteon = Some(
        fluid_rampreset_preset_noteon
            as unsafe extern "C" fn(
                _: *mut fluid_preset_t,
                _: *mut fluid_synth_t,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    (*preset).notify = None;
    return preset;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_sfont_iteration_start(mut sfont: *mut fluid_sfont_t) {
    fluid_ramsfont_iteration_start((*sfont).data as *mut fluid_ramsfont_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_sfont_iteration_next(
    mut sfont: *mut fluid_sfont_t,
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    (*preset).free = Some(
        fluid_rampreset_preset_delete
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_name = Some(
        fluid_rampreset_preset_get_name
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> *mut libc::c_char,
    );
    (*preset).get_banknum = Some(
        fluid_rampreset_preset_get_banknum
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_num = Some(
        fluid_rampreset_preset_get_num
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).noteon = Some(
        fluid_rampreset_preset_noteon
            as unsafe extern "C" fn(
                _: *mut fluid_preset_t,
                _: *mut fluid_synth_t,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    (*preset).notify = None;
    return fluid_ramsfont_iteration_next((*sfont).data as *mut fluid_ramsfont_t, preset);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_preset_delete(
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    free(preset as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_preset_get_name(
    mut preset: *mut fluid_preset_t,
) -> *mut libc::c_char {
    return fluid_rampreset_get_name((*preset).data as *mut fluid_rampreset_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_preset_get_banknum(
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    return fluid_rampreset_get_banknum((*preset).data as *mut fluid_rampreset_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_preset_get_num(
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    return fluid_rampreset_get_num((*preset).data as *mut fluid_rampreset_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_preset_noteon(
    mut preset: *mut fluid_preset_t,
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut key: libc::c_int,
    mut vel: libc::c_int,
) -> libc::c_int {
    return fluid_rampreset_noteon(
        (*preset).data as *mut fluid_rampreset_t,
        synth,
        chan,
        key,
        vel,
    );
}

#[no_mangle]
pub unsafe extern "C" fn new_fluid_ramsfont() -> *mut fluid_ramsfont_t {
    let mut sfont: *mut fluid_ramsfont_t = 0 as *mut fluid_ramsfont_t;
    sfont =
        malloc(::std::mem::size_of::<fluid_ramsfont_t>() as libc::c_ulong) as *mut fluid_ramsfont_t;
    if sfont.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return 0 as *mut fluid_ramsfont_t;
    }
    (*sfont).name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*sfont).sample = 0 as *mut fluid_list_t;
    (*sfont).preset = 0 as *mut fluid_rampreset_t;
    return sfont;
}

#[no_mangle]
pub unsafe extern "C" fn delete_fluid_ramsfont(mut sfont: *mut fluid_ramsfont_t) -> libc::c_int {
    let mut list: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut preset: *mut fluid_rampreset_t = 0 as *mut fluid_rampreset_t;
    list = (*sfont).sample;
    while !list.is_null() {
        let mut sam: *mut fluid_sample_t = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sample_t;
        if (*sam).refcount != 0 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    list = (*sfont).sample;
    while !list.is_null() {
        let mut sam_0: *mut fluid_sample_t = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sample_t;
        delete_fluid_ramsample(sam_0);
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    if !(*sfont).sample.is_null() {
        delete_fluid_list((*sfont).sample);
    }
    preset = (*sfont).preset;
    while !preset.is_null() {
        (*sfont).preset = (*preset).next;
        delete_fluid_rampreset(preset);
        preset = (*sfont).preset
    }
    free(sfont as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_get_name(
    mut sfont: *mut fluid_ramsfont_t,
) -> *mut libc::c_char {
    return (*sfont).name.as_mut_ptr();
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_set_name(
    mut sfont: *mut fluid_ramsfont_t,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    memcpy(
        (*sfont).name.as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_add_preset(
    mut sfont: *mut fluid_ramsfont_t,
    mut preset: *mut fluid_rampreset_t,
) -> libc::c_int {
    let mut cur: *mut fluid_rampreset_t = 0 as *mut fluid_rampreset_t;
    let mut prev: *mut fluid_rampreset_t = 0 as *mut fluid_rampreset_t;
    if (*sfont).preset.is_null() {
        (*preset).next = 0 as *mut fluid_rampreset_t;
        (*sfont).preset = preset
    } else {
        cur = (*sfont).preset;
        prev = 0 as *mut fluid_rampreset_t;
        while !cur.is_null() {
            if (*preset).bank < (*cur).bank
                || (*preset).bank == (*cur).bank && (*preset).num < (*cur).num
            {
                if prev.is_null() {
                    (*preset).next = cur;
                    (*sfont).preset = preset
                } else {
                    (*preset).next = cur;
                    (*prev).next = preset
                }
                return FLUID_OK as libc::c_int;
            }
            prev = cur;
            cur = (*cur).next
        }
        (*preset).next = 0 as *mut fluid_rampreset_t;
        (*prev).next = preset
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_add_izone(
    mut sfont: *mut fluid_ramsfont_t,
    mut bank: libc::c_uint,
    mut num: libc::c_uint,
    mut sample: *mut fluid_sample_t,
    mut lokey: libc::c_int,
    mut hikey: libc::c_int,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut preset: *mut fluid_rampreset_t = fluid_ramsfont_get_preset(sfont, bank, num);
    if preset.is_null() {
        preset = new_fluid_rampreset(sfont);
        if preset.is_null() {
            return FLUID_FAILED as libc::c_int;
        }
        (*preset).bank = bank;
        (*preset).num = num;
        err = fluid_rampreset_add_sample(preset, sample, lokey, hikey);
        if err != FLUID_OK as libc::c_int {
            return FLUID_FAILED as libc::c_int;
        }
        fluid_ramsfont_add_preset(sfont, preset);
    } else {
        err = fluid_rampreset_add_sample(preset, sample, lokey, hikey);
        if err != FLUID_OK as libc::c_int {
            return FLUID_FAILED as libc::c_int;
        }
    }
    (*sfont).sample = fluid_list_append((*sfont).sample, sample as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_remove_izone(
    mut sfont: *mut fluid_ramsfont_t,
    mut bank: libc::c_uint,
    mut num: libc::c_uint,
    mut sample: *mut fluid_sample_t,
) -> libc::c_int {
    let mut err: libc::c_int = 0;
    let mut preset: *mut fluid_rampreset_t = fluid_ramsfont_get_preset(sfont, bank, num);
    if preset.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    err = fluid_rampreset_remove_izone(preset, sample);
    if err != FLUID_OK as libc::c_int {
        return err;
    }
    (*sfont).sample = fluid_list_remove((*sfont).sample, sample as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_izone_set_gen(
    mut sfont: *mut fluid_ramsfont_t,
    mut bank: libc::c_uint,
    mut num: libc::c_uint,
    mut sample: *mut fluid_sample_t,
    mut gen_type: libc::c_int,
    mut value: libc::c_float,
) -> libc::c_int {
    let mut preset: *mut fluid_rampreset_t = fluid_ramsfont_get_preset(sfont, bank, num);
    if preset.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    return fluid_rampreset_izone_set_gen(preset, sample, gen_type, value);
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_izone_set_loop(
    mut sfont: *mut fluid_ramsfont_t,
    mut bank: libc::c_uint,
    mut num: libc::c_uint,
    mut sample: *mut fluid_sample_t,
    mut on: libc::c_int,
    mut loopstart: libc::c_float,
    mut loopend: libc::c_float,
) -> libc::c_int {
    let mut preset: *mut fluid_rampreset_t = fluid_ramsfont_get_preset(sfont, bank, num);
    if preset.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    return fluid_rampreset_izone_set_loop(preset, sample, on, loopstart, loopend);
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_get_preset(
    mut sfont: *mut fluid_ramsfont_t,
    mut bank: libc::c_uint,
    mut num: libc::c_uint,
) -> *mut fluid_rampreset_t {
    let mut preset: *mut fluid_rampreset_t = (*sfont).preset;
    while !preset.is_null() {
        if (*preset).bank == bank && (*preset).num == num {
            return preset;
        }
        preset = (*preset).next
    }
    return 0 as *mut fluid_rampreset_t;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_iteration_start(mut sfont: *mut fluid_ramsfont_t) {
    (*sfont).iter_cur = (*sfont).preset;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_ramsfont_iteration_next(
    mut sfont: *mut fluid_ramsfont_t,
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    if (*sfont).iter_cur.is_null() {
        return 0 as libc::c_int;
    }
    (*preset).data = (*sfont).iter_cur as *mut libc::c_void;
    (*sfont).iter_cur = fluid_rampreset_next((*sfont).iter_cur);
    return 1 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn new_fluid_rampreset(
    mut sfont: *mut fluid_ramsfont_t,
) -> *mut fluid_rampreset_t {
    let mut preset: *mut fluid_rampreset_t =
        malloc(::std::mem::size_of::<fluid_rampreset_t>() as libc::c_ulong)
            as *mut fluid_rampreset_t;
    if preset.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return 0 as *mut fluid_rampreset_t;
    }
    (*preset).next = 0 as *mut fluid_rampreset_t;
    (*preset).sfont = sfont;
    (*preset).name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*preset).bank = 0 as libc::c_int as libc::c_uint;
    (*preset).num = 0 as libc::c_int as libc::c_uint;
    (*preset).global_zone = 0 as *mut fluid_preset_zone_t;
    (*preset).zone = 0 as *mut fluid_preset_zone_t;
    (*preset).presetvoices = 0 as *mut fluid_list_t;
    return preset;
}

#[no_mangle]
pub unsafe extern "C" fn delete_fluid_rampreset(mut preset: *mut fluid_rampreset_t) -> libc::c_int {
    let mut err: libc::c_int = FLUID_OK as libc::c_int;
    let mut zone: *mut fluid_preset_zone_t = 0 as *mut fluid_preset_zone_t;
    let mut data: *mut fluid_rampreset_voice_t = 0 as *mut fluid_rampreset_voice_t;
    if !(*preset).global_zone.is_null() {
        if delete_fluid_preset_zone((*preset).global_zone) != FLUID_OK as libc::c_int {
            err = FLUID_FAILED as libc::c_int
        }
        (*preset).global_zone = 0 as *mut fluid_preset_zone_t
    }
    zone = (*preset).zone;
    while !zone.is_null() {
        (*preset).zone = (*zone).next;
        if delete_fluid_preset_zone(zone) != FLUID_OK as libc::c_int {
            err = FLUID_FAILED as libc::c_int
        }
        zone = (*preset).zone
    }
    if !(*preset).presetvoices.is_null() {
        let mut tmp: *mut fluid_list_t = (*preset).presetvoices;
        let mut next: *mut fluid_list_t = 0 as *mut fluid_list_t;
        while !tmp.is_null() {
            data = (*tmp).data as *mut fluid_rampreset_voice_t;
            free(data as *mut libc::c_void);
            next = (*tmp).next;
            free(tmp as *mut libc::c_void);
            tmp = next
        }
    }
    (*preset).presetvoices = 0 as *mut fluid_list_t;
    free(preset as *mut libc::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_get_banknum(
    mut preset: *mut fluid_rampreset_t,
) -> libc::c_int {
    return (*preset).bank as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_get_num(
    mut preset: *mut fluid_rampreset_t,
) -> libc::c_int {
    return (*preset).num as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_get_name(
    mut preset: *mut fluid_rampreset_t,
) -> *mut libc::c_char {
    return (*preset).name.as_mut_ptr();
}

#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_next(
    mut preset: *mut fluid_rampreset_t,
) -> *mut fluid_rampreset_t {
    return (*preset).next;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_add_zone(
    mut preset: *mut fluid_rampreset_t,
    mut zone: *mut fluid_preset_zone_t,
) -> libc::c_int {
    if (*preset).zone.is_null() {
        (*zone).next = 0 as *mut fluid_preset_zone_t;
        (*preset).zone = zone
    } else {
        (*zone).next = (*preset).zone;
        (*preset).zone = zone
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_add_sample(
    mut preset: *mut fluid_rampreset_t,
    mut sample: *mut fluid_sample_t,
    mut lokey: libc::c_int,
    mut hikey: libc::c_int,
) -> libc::c_int {
    if (*preset).zone.is_null() {
        let mut zone: *mut fluid_preset_zone_t = 0 as *mut fluid_preset_zone_t;
        zone =
            new_fluid_preset_zone(b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
        if zone.is_null() {
            return FLUID_FAILED as libc::c_int;
        }
        (*zone).inst = new_fluid_inst();
        if (*zone).inst.is_null() {
            delete_fluid_preset_zone(zone);
            return FLUID_FAILED as libc::c_int;
        }
        fluid_rampreset_add_zone(preset, zone);
    }

    let mut inst: *mut fluid_inst_t = fluid_preset_zone_get_inst((*preset).zone);
    let mut izone: *mut fluid_inst_zone_t =
        new_fluid_inst_zone(b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char);
    if izone.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if fluid_inst_add_zone(inst, izone) != FLUID_OK as libc::c_int {
        delete_fluid_inst_zone(izone);
        return FLUID_FAILED as libc::c_int;
    }
    (*izone).sample = sample;
    (*izone).keylo = lokey;
    (*izone).keyhi = hikey;
    memcpy(
        (*preset).name.as_mut_ptr() as *mut libc::c_void,
        (*sample).name.as_mut_ptr() as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_izoneforsample(
    mut preset: *mut fluid_rampreset_t,
    mut sample: *mut fluid_sample_t,
) -> *mut fluid_inst_zone_t {
    let mut inst: *mut fluid_inst_t = 0 as *mut fluid_inst_t;
    let mut izone: *mut fluid_inst_zone_t = 0 as *mut fluid_inst_zone_t;
    if (*preset).zone.is_null() {
        return 0 as *mut fluid_inst_zone_t;
    }
    inst = fluid_preset_zone_get_inst((*preset).zone);
    izone = (*inst).zone;
    while !izone.is_null() {
        if (*izone).sample == sample {
            return izone;
        }
        izone = (*izone).next
    }
    return 0 as *mut fluid_inst_zone_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_izone_set_loop(
    mut preset: *mut fluid_rampreset_t,
    mut sample: *mut fluid_sample_t,
    mut on: libc::c_int,
    mut loopstart: libc::c_float,
    mut loopend: libc::c_float,
) -> libc::c_int {
    let mut izone: *mut fluid_inst_zone_t = fluid_rampreset_izoneforsample(preset, sample);
    let mut coarse: libc::c_short = 0;
    let mut fine: libc::c_short = 0;
    if izone.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if on == 0 {
        (*izone).gen[GEN_SAMPLEMODE as libc::c_int as usize].flags =
            GEN_SET as libc::c_int as libc::c_uchar;
        (*izone).gen[GEN_SAMPLEMODE as libc::c_int as usize].val =
            FLUID_UNLOOPED as libc::c_int as libc::c_double;
        fluid_rampreset_updatevoices(
            preset,
            GEN_SAMPLEMODE as libc::c_int,
            FLUID_UNLOOPED as libc::c_int as libc::c_float,
        );
        return FLUID_OK as libc::c_int;
    }

    if loopstart as libc::c_double > 32767.0f64 || (loopstart as libc::c_double) < -32767.0f64 {
        coarse = (loopstart as libc::c_double / 32768.0f64) as libc::c_short;
        fine = (loopstart as libc::c_double
            - coarse as libc::c_float as libc::c_double * 32768.0f64)
            as libc::c_short
    } else {
        coarse = 0 as libc::c_int as libc::c_short;
        fine = loopstart as libc::c_short
    }
    (*izone).gen[GEN_STARTLOOPADDROFS as libc::c_int as usize].flags =
        GEN_SET as libc::c_int as libc::c_uchar;
    (*izone).gen[GEN_STARTLOOPADDROFS as libc::c_int as usize].val = fine as libc::c_double;
    fluid_rampreset_updatevoices(
        preset,
        GEN_STARTLOOPADDROFS as libc::c_int,
        fine as libc::c_float,
    );
    if coarse != 0 {
        (*izone).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize].flags =
            GEN_SET as libc::c_int as libc::c_uchar;
        (*izone).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize].val =
            coarse as libc::c_double
    } else {
        (*izone).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize].flags =
            GEN_UNUSED as libc::c_int as libc::c_uchar
    }
    fluid_rampreset_updatevoices(
        preset,
        GEN_STARTLOOPADDRCOARSEOFS as libc::c_int,
        coarse as libc::c_float,
    );

    if loopend as libc::c_double > 32767.0f64 || (loopend as libc::c_double) < -32767.0f64 {
        coarse = (loopend as libc::c_double / 32768.0f64) as libc::c_short;
        fine = (loopend as libc::c_double - coarse as libc::c_float as libc::c_double * 32768.0f64)
            as libc::c_short
    } else {
        coarse = 0 as libc::c_int as libc::c_short;
        fine = loopend as libc::c_short
    }
    (*izone).gen[GEN_ENDLOOPADDROFS as libc::c_int as usize].flags =
        GEN_SET as libc::c_int as libc::c_uchar;
    (*izone).gen[GEN_ENDLOOPADDROFS as libc::c_int as usize].val = fine as libc::c_double;
    fluid_rampreset_updatevoices(
        preset,
        GEN_ENDLOOPADDROFS as libc::c_int,
        fine as libc::c_float,
    );
    if coarse != 0 {
        (*izone).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize].flags =
            GEN_SET as libc::c_int as libc::c_uchar;
        (*izone).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize].val =
            coarse as libc::c_double
    } else {
        (*izone).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize].flags =
            GEN_UNUSED as libc::c_int as libc::c_uchar
    }
    fluid_rampreset_updatevoices(
        preset,
        GEN_ENDLOOPADDRCOARSEOFS as libc::c_int,
        coarse as libc::c_float,
    );
    (*izone).gen[GEN_SAMPLEMODE as libc::c_int as usize].flags =
        GEN_SET as libc::c_int as libc::c_uchar;
    (*izone).gen[GEN_SAMPLEMODE as libc::c_int as usize].val =
        FLUID_LOOP_DURING_RELEASE as libc::c_int as libc::c_double;
    fluid_rampreset_updatevoices(
        preset,
        GEN_SAMPLEMODE as libc::c_int,
        FLUID_LOOP_DURING_RELEASE as libc::c_int as libc::c_float,
    );
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_izone_set_gen(
    mut preset: *mut fluid_rampreset_t,
    mut sample: *mut fluid_sample_t,
    mut gen_type: libc::c_int,
    mut value: libc::c_float,
) -> libc::c_int {
    let mut izone: *mut fluid_inst_zone_t = fluid_rampreset_izoneforsample(preset, sample);
    if izone.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    (*izone).gen[gen_type as usize].flags = GEN_SET as libc::c_int as libc::c_uchar;
    (*izone).gen[gen_type as usize].val = value as libc::c_double;
    fluid_rampreset_updatevoices(preset, gen_type, value);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_remove_izone(
    mut preset: *mut fluid_rampreset_t,
    mut sample: *mut fluid_sample_t,
) -> libc::c_int {
    let mut inst: *mut fluid_inst_t = 0 as *mut fluid_inst_t;
    let mut izone: *mut fluid_inst_zone_t = 0 as *mut fluid_inst_zone_t;
    let mut prev: *mut fluid_inst_zone_t = 0 as *mut fluid_inst_zone_t;
    let mut found: libc::c_int = 0 as libc::c_int;
    if (*preset).zone.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    inst = fluid_preset_zone_get_inst((*preset).zone);
    izone = (*inst).zone;
    prev = 0 as *mut fluid_inst_zone_t;
    while !izone.is_null() && found == 0 {
        if (*izone).sample == sample {
            if prev.is_null() {
                (*inst).zone = (*izone).next
            } else {
                (*prev).next = (*izone).next
            }
            (*izone).next = 0 as *mut fluid_inst_zone_t;
            delete_fluid_inst_zone(izone);
            found = 1 as libc::c_int
        } else {
            prev = izone;
            izone = (*izone).next
        }
    }
    if found == 0 {
        return FLUID_FAILED as libc::c_int;
    }
    let mut tmp: *mut fluid_list_t = (*preset).presetvoices;
    while !tmp.is_null() {
        let mut presetvoice: *mut fluid_rampreset_voice_t =
            (*tmp).data as *mut fluid_rampreset_voice_t;
        let mut voice: *mut fluid_voice_t = (*presetvoice).voice;
        if fluid_voice_is_playing(voice) != 0 && fluid_voice_get_id(voice) == (*presetvoice).voiceID
        {
            if (*voice).sample == sample {
                fluid_voice_off(voice);
            }
        }
        tmp = (*tmp).next
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_remembervoice(
    mut preset: *mut fluid_rampreset_t,
    mut voice: *mut fluid_voice_t,
) -> libc::c_int {
    let mut presetvoice: *mut fluid_rampreset_voice_t =
        malloc(::std::mem::size_of::<fluid_rampreset_voice_t>() as libc::c_ulong)
            as *mut fluid_rampreset_voice_t;
    if presetvoice.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return FLUID_FAILED as libc::c_int;
    }
    (*presetvoice).voice = voice;
    (*presetvoice).voiceID = fluid_voice_get_id(voice);
    (*preset).presetvoices =
        fluid_list_append((*preset).presetvoices, presetvoice as *mut libc::c_void);
    if (*preset).presetvoices.is_null() {
        free(presetvoice as *mut libc::c_void);
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return FLUID_FAILED as libc::c_int;
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_updatevoices(
    mut preset: *mut fluid_rampreset_t,
    mut gen_type: libc::c_int,
    mut val: libc::c_float,
) {
    let mut tmp: *mut fluid_list_t = (*preset).presetvoices;
    let mut prev: *mut fluid_list_t = 0 as *mut fluid_list_t;
    let mut next: *mut fluid_list_t = 0 as *mut fluid_list_t;
    while !tmp.is_null() {
        let mut presetvoice: *mut fluid_rampreset_voice_t =
            (*tmp).data as *mut fluid_rampreset_voice_t;
        let mut voice: *mut fluid_voice_t = (*presetvoice).voice;
        if fluid_voice_is_playing(voice) == 0 || fluid_voice_get_id(voice) != (*presetvoice).voiceID
        {
            free(presetvoice as *mut libc::c_void);
            next = (*tmp).next;
            free(tmp as *mut libc::c_void);
            if !prev.is_null() {
                (*prev).next = next
            } else {
                (*preset).presetvoices = next
            }
            tmp = next
        } else {
            fluid_voice_gen_set(voice, gen_type, val);
            fluid_voice_update_param(voice, gen_type);
            prev = tmp;
            tmp = (*tmp).next
        }
    }
}

#[no_mangle]
pub unsafe extern "C" fn fluid_rampreset_noteon(
    mut preset: *mut fluid_rampreset_t,
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut key: libc::c_int,
    mut vel: libc::c_int,
) -> libc::c_int {
    let mut preset_zone: *mut fluid_preset_zone_t = 0 as *mut fluid_preset_zone_t;
    let mut inst: *mut fluid_inst_t = 0 as *mut fluid_inst_t;
    let mut inst_zone: *mut fluid_inst_zone_t = 0 as *mut fluid_inst_zone_t;
    let mut global_inst_zone: *mut fluid_inst_zone_t = 0 as *mut fluid_inst_zone_t;
    let mut z: *mut fluid_inst_zone_t = 0 as *mut fluid_inst_zone_t;
    let mut sample: *mut fluid_sample_t = 0 as *mut fluid_sample_t;
    let mut voice: *mut fluid_voice_t = 0 as *mut fluid_voice_t;
    let mut mod_0: *mut fluid_mod_t = 0 as *mut fluid_mod_t;
    let mut mod_list: [*mut fluid_mod_t; 64] = [0 as *mut fluid_mod_t; 64];
    let mut mod_list_count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    preset_zone = (*preset).zone;
    while !preset_zone.is_null() {
        if fluid_preset_zone_inside_range(preset_zone, key, vel) != 0 {
            inst = fluid_preset_zone_get_inst(preset_zone);
            global_inst_zone = fluid_inst_get_global_zone(inst);
            inst_zone = fluid_inst_get_zone(inst);
            while !inst_zone.is_null() {
                sample = fluid_inst_zone_get_sample(inst_zone);
                if fluid_sample_in_rom(sample) != 0 || sample.is_null() {
                    inst_zone = fluid_inst_zone_next(inst_zone)
                } else {
                    if fluid_inst_zone_inside_range(inst_zone, key, vel) != 0 && !sample.is_null() {
                        voice = fluid_synth_alloc_voice(synth, sample, chan, key, vel);
                        if voice.is_null() {
                            return FLUID_FAILED as libc::c_int;
                        }
                        if fluid_rampreset_remembervoice(preset, voice) != FLUID_OK as libc::c_int {
                            return FLUID_FAILED as libc::c_int;
                        }
                        z = inst_zone;
                        i = 0 as libc::c_int;
                        while i < GEN_LAST as libc::c_int {
                            if (*inst_zone).gen[i as usize].flags != 0 {
                                fluid_voice_gen_set(
                                    voice,
                                    i,
                                    (*inst_zone).gen[i as usize].val as libc::c_float,
                                );
                            } else if !global_inst_zone.is_null()
                                && (*global_inst_zone).gen[i as usize].flags as libc::c_int != 0
                            {
                                fluid_voice_gen_set(
                                    voice,
                                    i,
                                    (*global_inst_zone).gen[i as usize].val as libc::c_float,
                                );
                            }
                            i += 1
                        }
                        mod_list_count = 0 as libc::c_int;
                        if !global_inst_zone.is_null() {
                            mod_0 = (*global_inst_zone).mod_0;
                            while !mod_0.is_null() {
                                let fresh0 = mod_list_count;
                                mod_list_count = mod_list_count + 1;
                                mod_list[fresh0 as usize] = mod_0;
                                mod_0 = (*mod_0).next
                            }
                        }
                        mod_0 = (*inst_zone).mod_0;
                        while !mod_0.is_null() {
                            i = 0 as libc::c_int;
                            while i < mod_list_count {
                                if fluid_mod_test_identity(mod_0, mod_list[i as usize]) != 0 {
                                    mod_list[i as usize] = 0 as *mut fluid_mod_t
                                }
                                i += 1
                            }
                            let fresh1 = mod_list_count;
                            mod_list_count = mod_list_count + 1;
                            mod_list[fresh1 as usize] = mod_0;
                            mod_0 = (*mod_0).next
                        }
                        i = 0 as libc::c_int;
                        while i < mod_list_count {
                            mod_0 = mod_list[i as usize];
                            if !mod_0.is_null() {
                                fluid_voice_add_mod(
                                    voice,
                                    mod_0,
                                    FLUID_VOICE_OVERWRITE as libc::c_int,
                                );
                            }
                            i += 1
                        }
                        i = 0 as libc::c_int;
                        while i < GEN_LAST as libc::c_int {
                            if i != GEN_STARTADDROFS as libc::c_int
                                && i != GEN_ENDADDROFS as libc::c_int
                                && i != GEN_STARTLOOPADDROFS as libc::c_int
                                && i != GEN_ENDLOOPADDROFS as libc::c_int
                                && i != GEN_STARTADDRCOARSEOFS as libc::c_int
                                && i != GEN_ENDADDRCOARSEOFS as libc::c_int
                                && i != GEN_STARTLOOPADDRCOARSEOFS as libc::c_int
                                && i != GEN_KEYNUM as libc::c_int
                                && i != GEN_VELOCITY as libc::c_int
                                && i != GEN_ENDLOOPADDRCOARSEOFS as libc::c_int
                                && i != GEN_SAMPLEMODE as libc::c_int
                                && i != GEN_EXCLUSIVECLASS as libc::c_int
                                && i != GEN_OVERRIDEROOTKEY as libc::c_int
                            {
                                if (*preset_zone).gen[i as usize].flags != 0 {
                                    fluid_voice_gen_incr(
                                        voice,
                                        i,
                                        (*preset_zone).gen[i as usize].val as libc::c_float,
                                    );
                                }
                            }
                            i += 1
                        }
                        mod_list_count = 0 as libc::c_int;
                        mod_0 = (*preset_zone).mod_0;
                        while !mod_0.is_null() {
                            i = 0 as libc::c_int;
                            while i < mod_list_count {
                                if fluid_mod_test_identity(mod_0, mod_list[i as usize]) != 0 {
                                    mod_list[i as usize] = 0 as *mut fluid_mod_t
                                }
                                i += 1
                            }
                            let fresh2 = mod_list_count;
                            mod_list_count = mod_list_count + 1;
                            mod_list[fresh2 as usize] = mod_0;
                            mod_0 = (*mod_0).next
                        }
                        i = 0 as libc::c_int;
                        while i < mod_list_count {
                            mod_0 = mod_list[i as usize];
                            if !mod_0.is_null()
                                && (*mod_0).amount != 0 as libc::c_int as libc::c_double
                            {
                                fluid_voice_add_mod(voice, mod_0, FLUID_VOICE_ADD as libc::c_int);
                            }
                            i += 1
                        }
                        fluid_synth_start_voice(synth, voice);
                    }
                    inst_zone = fluid_inst_zone_next(inst_zone)
                }
            }
        }
        preset_zone = fluid_preset_zone_next(preset_zone)
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_sample_set_name(
    mut sample: *mut fluid_sample_t,
    mut name: *mut libc::c_char,
) -> libc::c_int {
    memcpy(
        (*sample).name.as_mut_ptr() as *mut libc::c_void,
        name as *const libc::c_void,
        20 as libc::c_int as libc::c_ulong,
    );
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_sample_set_sound_data(
    mut sample: *mut fluid_sample_t,
    mut data: *mut libc::c_short,
    mut nbframes: libc::c_uint,
    mut copy_data: libc::c_short,
    mut rootkey: libc::c_int,
) -> libc::c_int {
    let mut storedNbFrames: libc::c_uint = 0;
    if !(*sample).data.is_null() {
        free((*sample).data as *mut libc::c_void);
    }
    if copy_data != 0 {
        storedNbFrames = nbframes;
        if storedNbFrames < 48 as libc::c_int as libc::c_uint {
            storedNbFrames = 48 as libc::c_int as libc::c_uint
        }
        (*sample).data = malloc(
            storedNbFrames
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add((4 as libc::c_int * 8 as libc::c_int) as libc::c_uint)
                as libc::c_ulong,
        ) as *mut libc::c_short;
        if (*sample).data.is_null() {
            fluid_log!(
                FLUID_ERR,
                "Out of memory",
            );
            return FLUID_FAILED as libc::c_int;
        }
        memset(
            (*sample).data as *mut libc::c_void,
            0 as libc::c_int,
            storedNbFrames
                .wrapping_mul(2 as libc::c_int as libc::c_uint)
                .wrapping_add((4 as libc::c_int * 8 as libc::c_int) as libc::c_uint)
                as libc::c_ulong,
        );
        memcpy(
            ((*sample).data as *mut libc::c_char)
                .offset((2 as libc::c_int * 8 as libc::c_int) as isize)
                as *mut libc::c_void,
            data as *const libc::c_void,
            nbframes.wrapping_mul(2 as libc::c_int as libc::c_uint) as libc::c_ulong,
        );
        (*sample).start = 8 as libc::c_int as libc::c_uint;
        (*sample).end = (8 as libc::c_int as libc::c_uint).wrapping_add(storedNbFrames)
    } else {
        (*sample).data = data;
        (*sample).start = 0 as libc::c_int as libc::c_uint;
        (*sample).end = nbframes
    }
    (*sample).loopstart = (*sample).start;
    (*sample).loopend = (*sample).end;
    (*sample).samplerate = 44100 as libc::c_int as libc::c_uint;
    (*sample).origpitch = rootkey;
    (*sample).pitchadj = 0 as libc::c_int;
    (*sample).sampletype = 1 as libc::c_int;
    (*sample).valid = 1 as libc::c_int;
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn new_fluid_ramsample() -> *mut fluid_sample_t {
    let mut sample: *mut fluid_sample_t = 0 as *mut fluid_sample_t;
    sample =
        malloc(::std::mem::size_of::<fluid_sample_t>() as libc::c_ulong) as *mut fluid_sample_t;
    if sample.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Out of memory",
        );
        return 0 as *mut fluid_sample_t;
    }
    memset(
        sample as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<fluid_sample_t>() as libc::c_ulong,
    );
    return sample;
}

#[no_mangle]
pub unsafe extern "C" fn delete_fluid_ramsample(mut sample: *mut fluid_sample_t) -> libc::c_int {
    if !(*sample).data.is_null() {
        free((*sample).data as *mut libc::c_void);
    }
    (*sample).data = 0 as *mut libc::c_short;
    free(sample as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
