#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::channel::_fluid_channel_t;
use crate::channel::fluid_channel_get_interp_method;
use crate::channel::fluid_channel_get_num;
use crate::chorus::_fluid_chorus_t;
use crate::conv::fluid_act2hz;
use crate::conv::fluid_atten2amp;
use crate::conv::fluid_cb2amp;
use crate::conv::fluid_ct2hz;
use crate::conv::fluid_ct2hz_real;
use crate::conv::fluid_pan;
use crate::conv::fluid_tc2sec;
use crate::conv::fluid_tc2sec_attack;
use crate::conv::fluid_tc2sec_delay;
use crate::conv::fluid_tc2sec_release;
use crate::fluid_dsp_float::fluid_dsp_float_interpolate_4th_order;
use crate::fluid_dsp_float::fluid_dsp_float_interpolate_7th_order;
use crate::fluid_dsp_float::fluid_dsp_float_interpolate_linear;
use crate::fluid_dsp_float::fluid_dsp_float_interpolate_none;
use crate::fluid_gen::_fluid_gen_t;
use crate::fluid_gen::fluid_gen_init;
use crate::fluid_hash::_fluid_hashtable_t;
use crate::fluid_list::_fluid_list_t;
use crate::fluid_mod::_fluid_mod_t;
use crate::fluid_mod::fluid_mod_clone;
use crate::fluid_mod::fluid_mod_get_dest;
use crate::fluid_mod::fluid_mod_get_value;
use crate::fluid_mod::fluid_mod_test_identity;
use crate::fluid_rev::_fluid_revmodel_t;
use crate::fluid_sfont::_fluid_preset_t;
use crate::fluid_sfont::_fluid_sample_t;
use crate::fluid_sfont::_fluid_sfont_t;
use crate::fluid_synth::_fluid_synth_t;
use crate::fluid_tuning::_fluid_tuning_t;

pub type fluid_settings_t = _fluid_hashtable_t;
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
pub type fluid_mod_t = _fluid_mod_t;
pub type fluid_gen_t = _fluid_gen_t;
pub type fluid_channel_t = _fluid_channel_t;
pub type fluid_synth_t = _fluid_synth_t;
pub type fluid_preset_t = _fluid_preset_t;
pub type fluid_sfont_t = _fluid_sfont_t;
pub type fluid_list_t = _fluid_list_t;
pub type fluid_interp = libc::c_uint;
pub const FLUID_INTERP_HIGHEST: fluid_interp = 7;
pub const FLUID_INTERP_7THORDER: fluid_interp = 7;
pub const FLUID_INTERP_4THORDER: fluid_interp = 4;

pub const FLUID_INTERP_DEFAULT: fluid_interp = 4;

pub const FLUID_INTERP_LINEAR: fluid_interp = 1;

pub const FLUID_INTERP_NONE: fluid_interp = 0;
pub type C2RustUnnamed = libc::c_uint;
pub const FLUID_SAMPLE_DONE: C2RustUnnamed = 2;
pub const FLUID_PRESET_UNSELECTED: C2RustUnnamed = 1;
pub const FLUID_PRESET_SELECTED: C2RustUnnamed = 0;

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

pub type fluid_gen_flags = libc::c_uint;

pub const GEN_ABS_NRPN: fluid_gen_flags = 2;

pub const GEN_SET: fluid_gen_flags = 1;
pub const GEN_UNUSED: fluid_gen_flags = 0;
pub const FLUID_VOICE_ENVRELEASE: fluid_voice_envelope_index_t = 5;
pub const FLUID_VOICE_ENVDECAY: fluid_voice_envelope_index_t = 3;
pub const FLUID_VOICE_ENVHOLD: fluid_voice_envelope_index_t = 2;
pub const FLUID_VOICE_ENVATTACK: fluid_voice_envelope_index_t = 1;
pub const FLUID_VOICE_ENVDELAY: fluid_voice_envelope_index_t = 0;
pub type fluid_voice_add_mod = libc::c_uint;
pub const FLUID_VOICE_DEFAULT: fluid_voice_add_mod = 2;
pub const FLUID_VOICE_ADD: fluid_voice_add_mod = 1;
pub const FLUID_VOICE_OVERWRITE: fluid_voice_add_mod = 0;
pub const FLUID_VOICE_SUSTAINED: fluid_voice_status = 2;
pub const FLUID_VOICE_ON: fluid_voice_status = 1;
pub const FLUID_OK: C2RustUnnamed_0 = 0;
pub type C2RustUnnamed_0 = libc::c_int;
pub const FLUID_FAILED: C2RustUnnamed_0 = -1;
pub type fluid_voice_status = libc::c_uint;
pub const FLUID_VOICE_OFF: fluid_voice_status = 3;
pub const FLUID_VOICE_CLEAN: fluid_voice_status = 0;
pub type fluid_voice_envelope_index_t = libc::c_uint;
pub const FLUID_VOICE_ENVLAST: fluid_voice_envelope_index_t = 7;
pub const FLUID_VOICE_ENVFINISHED: fluid_voice_envelope_index_t = 6;
pub const FLUID_VOICE_ENVSUSTAIN: fluid_voice_envelope_index_t = 4;
pub const FLUID_LOOP_DURING_RELEASE: fluid_loop = 1;
pub const FLUID_LOOP_UNTIL_RELEASE: fluid_loop = 3;
pub const FLUID_UNLOOPED: fluid_loop = 0;
pub const SUSTAIN_SWITCH: fluid_midi_control_change = 64;
pub type fluid_midi_control_change = libc::c_uint;
pub const POLY_ON: fluid_midi_control_change = 127;
pub const POLY_OFF: fluid_midi_control_change = 126;
pub const OMNI_ON: fluid_midi_control_change = 125;
pub const OMNI_OFF: fluid_midi_control_change = 124;
pub const ALL_NOTES_OFF: fluid_midi_control_change = 123;
pub const LOCAL_CONTROL: fluid_midi_control_change = 122;
pub const ALL_CTRL_OFF: fluid_midi_control_change = 121;
pub const ALL_SOUND_OFF: fluid_midi_control_change = 120;
pub const RPN_MSB: fluid_midi_control_change = 101;
pub const RPN_LSB: fluid_midi_control_change = 100;
pub const NRPN_MSB: fluid_midi_control_change = 99;
pub const NRPN_LSB: fluid_midi_control_change = 98;
pub const DATA_ENTRY_DECR: fluid_midi_control_change = 97;
pub const DATA_ENTRY_INCR: fluid_midi_control_change = 96;
pub const EFFECTS_DEPTH5: fluid_midi_control_change = 95;
pub const EFFECTS_DEPTH4: fluid_midi_control_change = 94;
pub const EFFECTS_DEPTH3: fluid_midi_control_change = 93;
pub const EFFECTS_DEPTH2: fluid_midi_control_change = 92;
pub const EFFECTS_DEPTH1: fluid_midi_control_change = 91;
pub const PORTAMENTO_CTRL: fluid_midi_control_change = 84;
pub const GPC8: fluid_midi_control_change = 83;
pub const GPC7: fluid_midi_control_change = 82;
pub const GPC6: fluid_midi_control_change = 81;
pub const GPC5: fluid_midi_control_change = 80;
pub const SOUND_CTRL10: fluid_midi_control_change = 79;
pub const SOUND_CTRL9: fluid_midi_control_change = 78;
pub const SOUND_CTRL8: fluid_midi_control_change = 77;
pub const SOUND_CTRL7: fluid_midi_control_change = 76;
pub const SOUND_CTRL6: fluid_midi_control_change = 75;
pub const SOUND_CTRL5: fluid_midi_control_change = 74;
pub const SOUND_CTRL4: fluid_midi_control_change = 73;
pub const SOUND_CTRL3: fluid_midi_control_change = 72;
pub const SOUND_CTRL2: fluid_midi_control_change = 71;
pub const SOUND_CTRL1: fluid_midi_control_change = 70;
pub const HOLD2_SWITCH: fluid_midi_control_change = 69;
pub const LEGATO_SWITCH: fluid_midi_control_change = 69;
pub const SOFT_PEDAL_SWITCH: fluid_midi_control_change = 67;
pub const SOSTENUTO_SWITCH: fluid_midi_control_change = 66;
pub const PORTAMENTO_SWITCH: fluid_midi_control_change = 65;
pub const GPC4_LSB: fluid_midi_control_change = 51;
pub const GPC3_LSB: fluid_midi_control_change = 50;
pub const GPC2_LSB: fluid_midi_control_change = 49;
pub const GPC1_LSB: fluid_midi_control_change = 48;
pub const EFFECTS2_LSB: fluid_midi_control_change = 45;
pub const EFFECTS1_LSB: fluid_midi_control_change = 44;
pub const EXPRESSION_LSB: fluid_midi_control_change = 43;
pub const PAN_LSB: fluid_midi_control_change = 42;
pub const BALANCE_LSB: fluid_midi_control_change = 40;
pub const VOLUME_LSB: fluid_midi_control_change = 39;
pub const DATA_ENTRY_LSB: fluid_midi_control_change = 38;
pub const PORTAMENTO_TIME_LSB: fluid_midi_control_change = 37;
pub const FOOT_LSB: fluid_midi_control_change = 36;
pub const BREATH_LSB: fluid_midi_control_change = 34;
pub const MODULATION_WHEEL_LSB: fluid_midi_control_change = 33;
pub const BANK_SELECT_LSB: fluid_midi_control_change = 32;
pub const GPC4_MSB: fluid_midi_control_change = 19;
pub const GPC3_MSB: fluid_midi_control_change = 18;
pub const GPC2_MSB: fluid_midi_control_change = 17;
pub const GPC1_MSB: fluid_midi_control_change = 16;
pub const EFFECTS2_MSB: fluid_midi_control_change = 13;
pub const EFFECTS1_MSB: fluid_midi_control_change = 12;
pub const EXPRESSION_MSB: fluid_midi_control_change = 11;
pub const PAN_MSB: fluid_midi_control_change = 10;
pub const BALANCE_MSB: fluid_midi_control_change = 8;
pub const VOLUME_MSB: fluid_midi_control_change = 7;
pub const DATA_ENTRY_MSB: fluid_midi_control_change = 6;
pub const PORTAMENTO_TIME_MSB: fluid_midi_control_change = 5;
pub const FOOT_MSB: fluid_midi_control_change = 4;
pub const BREATH_MSB: fluid_midi_control_change = 2;
pub const MODULATION_MSB: fluid_midi_control_change = 1;
pub const BANK_SELECT_MSB: fluid_midi_control_change = 0;

pub type fluid_loop = libc::c_uint;
pub const FLUID_NOTUSED: fluid_loop = 2;

#[no_mangle]
pub unsafe extern "C" fn new_fluid_voice(mut output_rate: fluid_real_t) -> *mut fluid_voice_t {
    let mut voice: *mut fluid_voice_t = 0 as *mut fluid_voice_t;
    voice =
        libc::malloc(::std::mem::size_of::<fluid_voice_t>() as libc::size_t) as *mut fluid_voice_t;
    if voice.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_voice_t;
    }
    (*voice).status = FLUID_VOICE_CLEAN as libc::c_int as libc::c_uchar;
    (*voice).chan = 0xff as libc::c_int as libc::c_uchar;
    (*voice).key = 0 as libc::c_int as libc::c_uchar;
    (*voice).vel = 0 as libc::c_int as libc::c_uchar;
    (*voice).channel = 0 as *mut fluid_channel_t;
    (*voice).sample = 0 as *mut fluid_sample_t;
    (*voice).output_rate = output_rate;

    (*voice).volenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].count =
        0xffffffff as libc::c_uint;
    (*voice).volenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].coeff = 1.0f32;
    (*voice).volenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].incr = 0.0f32;
    (*voice).volenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].min = -1.0f32;
    (*voice).volenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].max = 2.0f32;
    (*voice).volenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].count =
        0xffffffff as libc::c_uint;
    (*voice).volenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].coeff = 0.0f32;
    (*voice).volenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].incr = 0.0f32;
    (*voice).volenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].min = -1.0f32;
    (*voice).volenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].max = 1.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].count =
        0xffffffff as libc::c_uint;
    (*voice).modenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].coeff = 1.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].incr = 0.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].min = -1.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVSUSTAIN as libc::c_int as usize].max = 2.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].count =
        0xffffffff as libc::c_uint;
    (*voice).modenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].coeff = 0.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].incr = 0.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].min = -1.0f32;
    (*voice).modenv_data[FLUID_VOICE_ENVFINISHED as libc::c_int as usize].max = 1.0f32;
    return voice;
}

#[no_mangle]
pub unsafe extern "C" fn delete_fluid_voice(mut voice: *mut fluid_voice_t) -> libc::c_int {
    if voice.is_null() {
        return FLUID_OK as libc::c_int;
    }
    libc::free(voice as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_init(
    mut voice: *mut fluid_voice_t,
    mut sample: *mut fluid_sample_t,
    mut channel: *mut fluid_channel_t,
    mut key: libc::c_int,
    mut vel: libc::c_int,
    mut id: libc::c_uint,
    mut start_time: libc::c_uint,
    mut gain: fluid_real_t,
) -> libc::c_int {
    (*voice).id = id;
    (*voice).chan = fluid_channel_get_num(channel) as libc::c_uchar;
    (*voice).key = key as libc::c_uchar;
    (*voice).vel = vel as libc::c_uchar;
    (*voice).channel = channel;
    (*voice).mod_count = 0 as libc::c_int;
    (*voice).sample = sample;
    (*voice).start_time = start_time;
    (*voice).ticks = 0 as libc::c_int as libc::c_uint;
    (*voice).noteoff_ticks = 0 as libc::c_int as libc::c_uint;
    (*voice).debug = 0 as libc::c_int;
    (*voice).has_looped = 0 as libc::c_int;
    (*voice).last_fres = -(1 as libc::c_int) as fluid_real_t;
    (*voice).filter_startup = 1 as libc::c_int;
    (*voice).interp_method = fluid_channel_get_interp_method((*voice).channel);

    (*voice).volenv_count = 0 as libc::c_int as libc::c_uint;
    (*voice).volenv_section = 0 as libc::c_int;
    (*voice).volenv_val = 0.0f32;
    (*voice).amp = 0.0f32;
    (*voice).modenv_count = 0 as libc::c_int as libc::c_uint;
    (*voice).modenv_section = 0 as libc::c_int;
    (*voice).modenv_val = 0.0f32;

    (*voice).modlfo_val = 0.0f64 as fluid_real_t;

    (*voice).viblfo_val = 0.0f32;
    (*voice).hist1 = 0 as libc::c_int as fluid_real_t;
    (*voice).hist2 = 0 as libc::c_int as fluid_real_t;

    fluid_gen_init(
        &mut *(*voice).gen.as_mut_ptr().offset(0 as libc::c_int as isize),
        channel,
    );
    (*voice).synth_gain = gain;
    if ((*voice).synth_gain as libc::c_double) < 0.0000001f64 {
        (*voice).synth_gain = 0.0000001f64 as fluid_real_t
    }
    (*voice).amplitude_that_reaches_noise_floor_nonloop =
        (0.00003f64 / (*voice).synth_gain as libc::c_double) as fluid_real_t;
    (*voice).amplitude_that_reaches_noise_floor_loop =
        (0.00003f64 / (*voice).synth_gain as libc::c_double) as fluid_real_t;
    (*(*voice).sample).refcount = (*(*voice).sample).refcount.wrapping_add(1);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_set(
    mut voice: *mut fluid_voice_t,
    mut i: libc::c_int,
    mut val: libc::c_float,
) {
    (*voice).gen[i as usize].val = val as libc::c_double;
    (*voice).gen[i as usize].flags = GEN_SET as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_incr(
    mut voice: *mut fluid_voice_t,
    mut i: libc::c_int,
    mut val: libc::c_float,
) {
    (*voice).gen[i as usize].val += val as libc::c_double;
    (*voice).gen[i as usize].flags = GEN_SET as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_get(
    mut voice: *mut fluid_voice_t,
    mut gen: libc::c_int,
) -> libc::c_float {
    return (*voice).gen[gen as usize].val as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_value(
    mut voice: *mut fluid_voice_t,
    mut num: libc::c_int,
) -> fluid_real_t {
    if (*voice).gen[num as usize].flags as libc::c_int == GEN_ABS_NRPN as libc::c_int {
        return (*voice).gen[num as usize].nrpn as fluid_real_t;
    } else {
        return ((*voice).gen[num as usize].val
            + (*voice).gen[num as usize].mod_0
            + (*voice).gen[num as usize].nrpn) as fluid_real_t;
    };
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_write(
    mut voice: *mut fluid_voice_t,
    mut dsp_left_buf: *mut fluid_real_t,
    mut dsp_right_buf: *mut fluid_real_t,
    mut dsp_reverb_buf: *mut fluid_real_t,
    mut dsp_chorus_buf: *mut fluid_real_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut fres: fluid_real_t = 0.;
    let mut target_amp: fluid_real_t = 0.;
    let mut count: libc::c_int = 0;
    let mut dsp_interp_method: libc::c_int = (*voice).interp_method;
    let mut dsp_buf: [fluid_real_t; 64] = [0.; 64];
    let mut env_data: *mut fluid_env_data_t = 0 as *mut fluid_env_data_t;
    let mut x: fluid_real_t = 0.;

    if !((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
        || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
    {
        return FLUID_OK as libc::c_int;
    }
    if (*voice).sample.is_null() {
        fluid_voice_off(voice);
        return FLUID_OK as libc::c_int;
    }
    if (*voice).noteoff_ticks != 0 as libc::c_int as libc::c_uint
        && (*voice).ticks >= (*voice).noteoff_ticks
    {
        fluid_voice_noteoff(voice);
    }
    fluid_voice_check_sample_sanity(voice);

    env_data = &mut *(*voice)
        .volenv_data
        .as_mut_ptr()
        .offset((*voice).volenv_section as isize) as *mut fluid_env_data_t;
    while (*voice).volenv_count >= (*env_data).count {
        // If we're switching envelope stages from decay to sustain, force the value to be the end value of the previous stage
        if !env_data.is_null() && (*voice).volenv_section == FLUID_VOICE_ENVDECAY as libc::c_int {
            (*voice).volenv_val = (*env_data).min * (*env_data).coeff
        }
        (*voice).volenv_section += 1;
        env_data = &mut *(*voice)
            .volenv_data
            .as_mut_ptr()
            .offset((*voice).volenv_section as isize) as *mut fluid_env_data_t;
        (*voice).volenv_count = 0 as libc::c_int as libc::c_uint
    }
    x = (*env_data).coeff * (*voice).volenv_val + (*env_data).incr;
    if x < (*env_data).min {
        x = (*env_data).min;
        (*voice).volenv_section += 1;
        (*voice).volenv_count = 0 as libc::c_int as libc::c_uint
    } else if x > (*env_data).max {
        x = (*env_data).max;
        (*voice).volenv_section += 1;
        (*voice).volenv_count = 0 as libc::c_int as libc::c_uint
    }
    (*voice).volenv_val = x;
    (*voice).volenv_count = (*voice).volenv_count.wrapping_add(1);
    if (*voice).volenv_section == FLUID_VOICE_ENVFINISHED as libc::c_int {
        fluid_voice_off(voice);
        return FLUID_OK as libc::c_int;
    }

    env_data = &mut *(*voice)
        .modenv_data
        .as_mut_ptr()
        .offset((*voice).modenv_section as isize) as *mut fluid_env_data_t;
    while (*voice).modenv_count >= (*env_data).count {
        (*voice).modenv_section += 1;
        env_data = &mut *(*voice)
            .modenv_data
            .as_mut_ptr()
            .offset((*voice).modenv_section as isize) as *mut fluid_env_data_t;
        (*voice).modenv_count = 0 as libc::c_int as libc::c_uint
    }
    x = (*env_data).coeff * (*voice).modenv_val + (*env_data).incr;
    if x < (*env_data).min {
        x = (*env_data).min;
        (*voice).modenv_section += 1;
        (*voice).modenv_count = 0 as libc::c_int as libc::c_uint
    } else if x > (*env_data).max {
        x = (*env_data).max;
        (*voice).modenv_section += 1;
        (*voice).modenv_count = 0 as libc::c_int as libc::c_uint
    }
    (*voice).modenv_val = x;
    (*voice).modenv_count = (*voice).modenv_count.wrapping_add(1);
    if (*voice).ticks >= (*voice).modlfo_delay {
        (*voice).modlfo_val += (*voice).modlfo_incr;
        if (*voice).modlfo_val as libc::c_double > 1.0f64 {
            (*voice).modlfo_incr = -(*voice).modlfo_incr;
            (*voice).modlfo_val = 2.0f64 as fluid_real_t - (*voice).modlfo_val
        } else if ((*voice).modlfo_val as libc::c_double) < -1.0f64 {
            (*voice).modlfo_incr = -(*voice).modlfo_incr;
            (*voice).modlfo_val = -2.0f64 as fluid_real_t - (*voice).modlfo_val
        }
    }
    if (*voice).ticks >= (*voice).viblfo_delay {
        (*voice).viblfo_val += (*voice).viblfo_incr;
        if (*voice).viblfo_val > 1.0f64 as fluid_real_t {
            (*voice).viblfo_incr = -(*voice).viblfo_incr;
            (*voice).viblfo_val = 2.0f64 as fluid_real_t - (*voice).viblfo_val
        } else if ((*voice).viblfo_val as libc::c_double) < -1.0f64 {
            (*voice).viblfo_incr = -(*voice).viblfo_incr;
            (*voice).viblfo_val = -2.0f64 as fluid_real_t - (*voice).viblfo_val
        }
    }
    if !((*voice).volenv_section == FLUID_VOICE_ENVDELAY as libc::c_int) {
        if (*voice).volenv_section == FLUID_VOICE_ENVATTACK as libc::c_int {
            target_amp = fluid_atten2amp((*voice).attenuation)
                * fluid_cb2amp((*voice).modlfo_val * -(*voice).modlfo_to_vol)
                * (*voice).volenv_val;
            current_block = 576355610076403033;
        } else {
            let mut amplitude_that_reaches_noise_floor: fluid_real_t = 0.;
            let mut amp_max: fluid_real_t = 0.;
            target_amp = fluid_atten2amp((*voice).attenuation)
                * fluid_cb2amp(
                    960.0f32 * (1.0f32 - (*voice).volenv_val)
                        + (*voice).modlfo_val * -(*voice).modlfo_to_vol,
                );
            if (*voice).has_looped != 0 {
                amplitude_that_reaches_noise_floor =
                    (*voice).amplitude_that_reaches_noise_floor_loop
            } else {
                amplitude_that_reaches_noise_floor =
                    (*voice).amplitude_that_reaches_noise_floor_nonloop
            }

            amp_max = fluid_atten2amp((*voice).min_attenuation_cB) * (*voice).volenv_val;
            if amp_max < amplitude_that_reaches_noise_floor {
                fluid_voice_off(voice);
                current_block = 3632332525568699835;
            } else {
                current_block = 576355610076403033;
            }
        }
        match current_block {
            3632332525568699835 => {}
            _ => {
                (*voice).amp_incr =
                    (target_amp - (*voice).amp) / 64 as libc::c_int as libc::c_float;
                if !((*voice).amp == 0.0f32 && (*voice).amp_incr == 0.0f32) {
                    (*voice).phase_incr = fluid_ct2hz_real(
                        (*voice).pitch
                            + (*voice).modlfo_val * (*voice).modlfo_to_pitch
                            + (*voice).viblfo_val * (*voice).viblfo_to_pitch
                            + (*voice).modenv_val * (*voice).modenv_to_pitch,
                    ) / (*voice).root_pitch;

                    if (*voice).phase_incr == 0 as libc::c_int as libc::c_float {
                        (*voice).phase_incr = 1 as libc::c_int as fluid_real_t
                    }

                    fres = fluid_ct2hz(
                        (*voice).fres
                            + (*voice).modlfo_val * (*voice).modlfo_to_fc
                            + (*voice).modenv_val * (*voice).modenv_to_fc,
                    );
                    if fres > 0.45f32 * (*voice).output_rate {
                        fres = 0.45f32 * (*voice).output_rate
                    } else if fres < 5 as libc::c_int as libc::c_float {
                        fres = 5 as libc::c_int as fluid_real_t
                    }

                    if f64::abs((fres - (*voice).last_fres) as libc::c_double) > 0.01f64 {
                        let mut omega: fluid_real_t = (2.0f64
                            * std::f64::consts::PI
                            * (fres / (*voice).output_rate) as libc::c_double)
                            as fluid_real_t;
                        let mut sin_coeff: fluid_real_t = f64::sin(omega.into()) as fluid_real_t;
                        let mut cos_coeff: fluid_real_t = f64::cos(omega.into()) as fluid_real_t;
                        let mut alpha_coeff: fluid_real_t = sin_coeff / (2.0f32 * (*voice).q_lin);
                        let mut a0_inv: fluid_real_t = 1.0f32 / (1.0f32 + alpha_coeff);

                        let mut a1_temp: fluid_real_t = -2.0f32 * cos_coeff * a0_inv;
                        let mut a2_temp: fluid_real_t = (1.0f32 - alpha_coeff) * a0_inv;
                        let mut b1_temp: fluid_real_t =
                            (1.0f32 - cos_coeff) * a0_inv * (*voice).filter_gain;
                        let mut b02_temp: fluid_real_t = b1_temp * 0.5f32;
                        if (*voice).filter_startup != 0 {
                            (*voice).a1 = a1_temp;
                            (*voice).a2 = a2_temp;
                            (*voice).b02 = b02_temp;
                            (*voice).b1 = b1_temp;
                            (*voice).filter_coeff_incr_count = 0 as libc::c_int;
                            (*voice).filter_startup = 0 as libc::c_int
                        //       printf("Setting initial filter coefficients.\n");
                        } else {
                            (*voice).a1_incr =
                                (a1_temp - (*voice).a1) / 64 as libc::c_int as libc::c_float;
                            (*voice).a2_incr =
                                (a2_temp - (*voice).a2) / 64 as libc::c_int as libc::c_float;
                            (*voice).b02_incr =
                                (b02_temp - (*voice).b02) / 64 as libc::c_int as libc::c_float;
                            (*voice).b1_incr =
                                (b1_temp - (*voice).b1) / 64 as libc::c_int as libc::c_float;

                            (*voice).filter_coeff_incr_count = 64 as libc::c_int
                        }
                        (*voice).last_fres = fres
                    }
                    (*voice).dsp_buf = dsp_buf.as_mut_ptr();
                    match (*voice).interp_method {
                        0 => count = fluid_dsp_float_interpolate_none(voice),
                        1 => count = fluid_dsp_float_interpolate_linear(voice),
                        7 => count = fluid_dsp_float_interpolate_7th_order(voice),
                        4 | _ => count = fluid_dsp_float_interpolate_4th_order(voice),
                    }
                    if count > 0 as libc::c_int {
                        fluid_voice_effects(
                            voice,
                            count,
                            dsp_left_buf,
                            dsp_right_buf,
                            dsp_reverb_buf,
                            dsp_chorus_buf,
                        );
                    }

                    if count < 64 as libc::c_int {
                        fluid_voice_off(voice);
                    }
                }
            }
        }
    }
    (*voice).ticks = (*voice)
        .ticks
        .wrapping_add(64 as libc::c_int as libc::c_uint);
    return FLUID_OK as libc::c_int;
}

//removed inline

#[inline]
unsafe extern "C" fn fluid_voice_effects(
    mut voice: *mut fluid_voice_t,
    mut count: libc::c_int,
    mut dsp_left_buf: *mut fluid_real_t,
    mut dsp_right_buf: *mut fluid_real_t,
    mut dsp_reverb_buf: *mut fluid_real_t,
    mut dsp_chorus_buf: *mut fluid_real_t,
) {
    let mut dsp_hist1: fluid_real_t = (*voice).hist1;
    let mut dsp_hist2: fluid_real_t = (*voice).hist2;
    let mut dsp_a1: fluid_real_t = (*voice).a1;
    let mut dsp_a2: fluid_real_t = (*voice).a2;
    let mut dsp_b02: fluid_real_t = (*voice).b02;
    let mut dsp_b1: fluid_real_t = (*voice).b1;
    let mut dsp_a1_incr: fluid_real_t = (*voice).a1_incr;
    let mut dsp_a2_incr: fluid_real_t = (*voice).a2_incr;
    let mut dsp_b02_incr: fluid_real_t = (*voice).b02_incr;
    let mut dsp_b1_incr: fluid_real_t = (*voice).b1_incr;
    let mut dsp_filter_coeff_incr_count: libc::c_int = (*voice).filter_coeff_incr_count;
    let mut dsp_buf: *mut fluid_real_t = (*voice).dsp_buf;
    let mut dsp_centernode: fluid_real_t = 0.;
    let mut dsp_i: libc::c_int = 0;
    let mut v: libc::c_float = 0.;
    if f64::abs(dsp_hist1 as libc::c_double) < 1e-20f64 {
        dsp_hist1 = 0.0f32
    }
    if dsp_filter_coeff_incr_count > 0 as libc::c_int {
        dsp_i = 0 as libc::c_int;
        while dsp_i < count {
            dsp_centernode =
                *dsp_buf.offset(dsp_i as isize) - dsp_a1 * dsp_hist1 - dsp_a2 * dsp_hist2;
            *dsp_buf.offset(dsp_i as isize) =
                dsp_b02 * (dsp_centernode + dsp_hist2) + dsp_b1 * dsp_hist1;
            dsp_hist2 = dsp_hist1;
            dsp_hist1 = dsp_centernode;
            let fresh0 = dsp_filter_coeff_incr_count;
            dsp_filter_coeff_incr_count = dsp_filter_coeff_incr_count - 1;
            if fresh0 > 0 as libc::c_int {
                dsp_a1 += dsp_a1_incr;
                dsp_a2 += dsp_a2_incr;
                dsp_b02 += dsp_b02_incr;
                dsp_b1 += dsp_b1_incr
            }
            dsp_i += 1
        }
    } else {
        dsp_i = 0 as libc::c_int;
        while dsp_i < count {
            dsp_centernode =
                *dsp_buf.offset(dsp_i as isize) - dsp_a1 * dsp_hist1 - dsp_a2 * dsp_hist2;
            *dsp_buf.offset(dsp_i as isize) =
                dsp_b02 * (dsp_centernode + dsp_hist2) + dsp_b1 * dsp_hist1;
            dsp_hist2 = dsp_hist1;
            dsp_hist1 = dsp_centernode;
            dsp_i += 1
        }
    }

    if -0.5f64 < (*voice).pan as libc::c_double && ((*voice).pan as libc::c_double) < 0.5f64 {
        dsp_i = 0 as libc::c_int;
        while dsp_i < count {
            v = (*voice).amp_left * *dsp_buf.offset(dsp_i as isize);
            let ref mut fresh1 = *dsp_left_buf.offset(dsp_i as isize);
            *fresh1 += v;
            let ref mut fresh2 = *dsp_right_buf.offset(dsp_i as isize);
            *fresh2 += v;
            dsp_i += 1
        }
    } else {
        if (*voice).amp_left as libc::c_double != 0.0f64 {
            dsp_i = 0 as libc::c_int;
            while dsp_i < count {
                let ref mut fresh3 = *dsp_left_buf.offset(dsp_i as isize);
                *fresh3 += (*voice).amp_left * *dsp_buf.offset(dsp_i as isize);
                dsp_i += 1
            }
        }
        if (*voice).amp_right as libc::c_double != 0.0f64 {
            dsp_i = 0 as libc::c_int;
            while dsp_i < count {
                let ref mut fresh4 = *dsp_right_buf.offset(dsp_i as isize);
                *fresh4 += (*voice).amp_right * *dsp_buf.offset(dsp_i as isize);
                dsp_i += 1
            }
        }
    }

    if !dsp_reverb_buf.is_null() && (*voice).amp_reverb as libc::c_double != 0.0f64 {
        dsp_i = 0 as libc::c_int;
        while dsp_i < count {
            let ref mut fresh5 = *dsp_reverb_buf.offset(dsp_i as isize);
            *fresh5 += (*voice).amp_reverb * *dsp_buf.offset(dsp_i as isize);
            dsp_i += 1
        }
    }

    if !dsp_chorus_buf.is_null() && (*voice).amp_chorus != 0 as libc::c_int as libc::c_float {
        dsp_i = 0 as libc::c_int;
        while dsp_i < count {
            let ref mut fresh6 = *dsp_chorus_buf.offset(dsp_i as isize);
            *fresh6 += (*voice).amp_chorus * *dsp_buf.offset(dsp_i as isize);
            dsp_i += 1
        }
    }
    (*voice).hist1 = dsp_hist1;
    (*voice).hist2 = dsp_hist2;
    (*voice).a1 = dsp_a1;
    (*voice).a2 = dsp_a2;
    (*voice).b02 = dsp_b02;
    (*voice).b1 = dsp_b1;
    (*voice).filter_coeff_incr_count = dsp_filter_coeff_incr_count;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_get_channel(
    mut voice: *mut fluid_voice_t,
) -> *mut fluid_channel_t {
    return (*voice).channel;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_start(mut voice: *mut fluid_voice_t) {
    fluid_voice_calculate_runtime_synthesis_parameters(voice);

    (*voice).check_sample_sanity_flag = (1 as libc::c_int) << 1 as libc::c_int;
    (*voice).status = FLUID_VOICE_ON as libc::c_int as libc::c_uchar;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_calculate_runtime_synthesis_parameters(
    mut voice: *mut fluid_voice_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut list_of_generators_to_initialize: [libc::c_int; 35] = [
        GEN_STARTADDROFS as libc::c_int,
        GEN_ENDADDROFS as libc::c_int,
        GEN_STARTLOOPADDROFS as libc::c_int,
        GEN_ENDLOOPADDROFS as libc::c_int,
        GEN_MODLFOTOPITCH as libc::c_int,
        GEN_VIBLFOTOPITCH as libc::c_int,
        GEN_MODENVTOPITCH as libc::c_int,
        GEN_FILTERFC as libc::c_int,
        GEN_FILTERQ as libc::c_int,
        GEN_MODLFOTOFILTERFC as libc::c_int,
        GEN_MODENVTOFILTERFC as libc::c_int,
        GEN_MODLFOTOVOL as libc::c_int,
        GEN_CHORUSSEND as libc::c_int,
        GEN_REVERBSEND as libc::c_int,
        GEN_PAN as libc::c_int,
        GEN_MODLFODELAY as libc::c_int,
        GEN_MODLFOFREQ as libc::c_int,
        GEN_VIBLFODELAY as libc::c_int,
        GEN_VIBLFOFREQ as libc::c_int,
        GEN_MODENVDELAY as libc::c_int,
        GEN_MODENVATTACK as libc::c_int,
        GEN_MODENVHOLD as libc::c_int,
        GEN_MODENVDECAY as libc::c_int,
        GEN_MODENVRELEASE as libc::c_int,
        GEN_VOLENVDELAY as libc::c_int,
        GEN_VOLENVATTACK as libc::c_int,
        GEN_VOLENVHOLD as libc::c_int,
        GEN_VOLENVDECAY as libc::c_int,
        GEN_VOLENVRELEASE as libc::c_int,
        GEN_KEYNUM as libc::c_int,
        GEN_VELOCITY as libc::c_int,
        GEN_ATTENUATION as libc::c_int,
        GEN_OVERRIDEROOTKEY as libc::c_int,
        GEN_PITCH as libc::c_int,
        -(1 as libc::c_int),
    ];
    i = 0 as libc::c_int;
    while i < (*voice).mod_count {
        let mut mod_0: *mut fluid_mod_t =
            &mut *(*voice).mod_0.as_mut_ptr().offset(i as isize) as *mut fluid_mod_t;
        let mut modval: fluid_real_t = fluid_mod_get_value(mod_0, (*voice).channel, voice);
        let mut dest_gen_index: libc::c_int = (*mod_0).dest as libc::c_int;
        let mut dest_gen: *mut fluid_gen_t =
            &mut *(*voice).gen.as_mut_ptr().offset(dest_gen_index as isize) as *mut fluid_gen_t;
        (*dest_gen).mod_0 += modval as libc::c_double;
        i += 1
    }
    if !(*(*voice).channel).tuning.is_null() {
        let mut tuning: *mut fluid_tuning_t = (*(*voice).channel).tuning;
        (*voice).gen[GEN_PITCH as libc::c_int as usize].val = (*tuning).pitch
            [60 as libc::c_int as usize]
            + (*voice).gen[GEN_SCALETUNE as libc::c_int as usize].val / 100.0f32 as libc::c_double
                * ((*tuning).pitch[(*voice).key as usize]
                    - (*tuning).pitch[60 as libc::c_int as usize])
    } else {
        (*voice).gen[GEN_PITCH as libc::c_int as usize].val =
            (*voice).gen[GEN_SCALETUNE as libc::c_int as usize].val
                * ((*voice).key as libc::c_int as libc::c_float - 60.0f32) as libc::c_double
                + (100.0f32 * 60.0f32) as libc::c_double
    }
    i = 0 as libc::c_int;
    while list_of_generators_to_initialize[i as usize] != -(1 as libc::c_int) {
        fluid_voice_update_param(voice, list_of_generators_to_initialize[i as usize]);
        i += 1
    }

    (*voice).min_attenuation_cB = fluid_voice_get_lower_boundary_for_attenuation(voice);
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn calculate_hold_decay_buffers(
    mut voice: *mut fluid_voice_t,
    mut gen_base: libc::c_int,
    mut gen_key2base: libc::c_int,
    mut is_decay: libc::c_int,
) -> libc::c_int {
    let mut timecents: fluid_real_t = 0.;
    let mut seconds: fluid_real_t = 0.;
    let mut buffers: libc::c_int = 0;

    timecents = (((*voice).gen[gen_base as usize].val as fluid_real_t
        + (*voice).gen[gen_base as usize].mod_0 as fluid_real_t
        + (*voice).gen[gen_base as usize].nrpn as fluid_real_t) as libc::c_double
        + ((*voice).gen[gen_key2base as usize].val as fluid_real_t
            + (*voice).gen[gen_key2base as usize].mod_0 as fluid_real_t
            + (*voice).gen[gen_key2base as usize].nrpn as fluid_real_t) as libc::c_double
            * (60.0f64 - (*voice).key as libc::c_int as libc::c_double))
        as fluid_real_t;
    if is_decay != 0 {
        if timecents as libc::c_double > 8000.0f64 {
            timecents = 8000.0f64 as fluid_real_t
        }
    } else {
        if timecents > 5000 as libc::c_int as libc::c_float {
            timecents = 5000.0f64 as fluid_real_t
        }
        if timecents as libc::c_double <= -32768.0f64 {
            return 0 as libc::c_int;
        }
    }
    if (timecents as libc::c_double) < -12000.0f64 {
        timecents = -12000.0f64 as fluid_real_t
    }
    seconds = fluid_tc2sec(timecents);

    buffers = (((*voice).output_rate * seconds / 64 as libc::c_int as fluid_real_t)
        as libc::c_double
        + 0.5f64) as libc::c_int;
    return buffers;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_update_param(
    mut voice: *mut fluid_voice_t,
    mut gen: libc::c_int,
) {
    let mut q_dB: libc::c_double = 0.;
    let mut x: fluid_real_t = 0.;
    let mut y: fluid_real_t = 0.;
    let mut count: libc::c_uint = 0;
    // Alternate attenuation scale used by EMU10K1 cards when setting the attenuation at the preset or instrument level within the SoundFont bank.
    static mut ALT_ATTENUATION_SCALE: libc::c_float = 0.4f64 as libc::c_float;
    let mut current_block_195: u64;
    match gen {
        17 => {
            (*voice).pan = (*voice).gen[GEN_PAN as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_PAN as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_PAN as libc::c_int as usize].nrpn as fluid_real_t;
            (*voice).amp_left =
                fluid_pan((*voice).pan, 1 as libc::c_int) * (*voice).synth_gain / 32768.0f32;
            (*voice).amp_right =
                fluid_pan((*voice).pan, 0 as libc::c_int) * (*voice).synth_gain / 32768.0f32;
            current_block_195 = 5267916556966421873;
        }
        48 => {
            (*voice).attenuation = (*voice).gen[GEN_ATTENUATION as libc::c_int as usize].val
                as fluid_real_t
                * ALT_ATTENUATION_SCALE
                + (*voice).gen[GEN_ATTENUATION as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_ATTENUATION as libc::c_int as usize].nrpn as fluid_real_t;

            (*voice).attenuation = if ((*voice).attenuation as libc::c_double) < 0.0f64 {
                0.0f64
            } else if (*voice).attenuation as libc::c_double > 1440.0f64 {
                1440.0f64
            } else {
                (*voice).attenuation as libc::c_double
            } as fluid_real_t;
            current_block_195 = 5267916556966421873;
        }
        59 | 51 | 52 => {
            (*voice).pitch = (*voice).gen[GEN_PITCH as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_PITCH as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_PITCH as libc::c_int as usize].nrpn as fluid_real_t
                + 100.0f32
                    * ((*voice).gen[GEN_COARSETUNE as libc::c_int as usize].val as fluid_real_t
                        + (*voice).gen[GEN_COARSETUNE as libc::c_int as usize].mod_0
                            as fluid_real_t
                        + (*voice).gen[GEN_COARSETUNE as libc::c_int as usize].nrpn
                            as fluid_real_t)
                + ((*voice).gen[GEN_FINETUNE as libc::c_int as usize].val as fluid_real_t
                    + (*voice).gen[GEN_FINETUNE as libc::c_int as usize].mod_0 as fluid_real_t
                    + (*voice).gen[GEN_FINETUNE as libc::c_int as usize].nrpn as fluid_real_t);
            current_block_195 = 5267916556966421873;
        }
        16 => {
            (*voice).reverb_send = ((*voice).gen[GEN_REVERBSEND as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_REVERBSEND as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_REVERBSEND as libc::c_int as usize].nrpn as fluid_real_t)
                / 1000.0f32;
            (*voice).reverb_send = if ((*voice).reverb_send as libc::c_double) < 0.0f64 {
                0.0f64
            } else if (*voice).reverb_send as libc::c_double > 1.0f64 {
                1.0f64
            } else {
                (*voice).reverb_send as libc::c_double
            } as fluid_real_t;
            (*voice).amp_reverb = (*voice).reverb_send * (*voice).synth_gain / 32768.0f32;
            current_block_195 = 5267916556966421873;
        }
        15 => {
            (*voice).chorus_send = ((*voice).gen[GEN_CHORUSSEND as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_CHORUSSEND as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_CHORUSSEND as libc::c_int as usize].nrpn as fluid_real_t)
                / 1000.0f32;
            (*voice).chorus_send = if ((*voice).chorus_send as libc::c_double) < 0.0f64 {
                0.0f64
            } else if (*voice).chorus_send as libc::c_double > 1.0f64 {
                1.0f64
            } else {
                (*voice).chorus_send as libc::c_double
            } as fluid_real_t;
            (*voice).amp_chorus = (*voice).chorus_send * (*voice).synth_gain / 32768.0f32;
            current_block_195 = 5267916556966421873;
        }
        58 => {
            if (*voice).gen[GEN_OVERRIDEROOTKEY as libc::c_int as usize].val
                > -(1 as libc::c_int) as libc::c_double
            {
                //FIXME: use flag instead of -1
                (*voice).root_pitch = ((*voice).gen[GEN_OVERRIDEROOTKEY as libc::c_int as usize]
                    .val
                    * 100.0f32 as libc::c_double
                    - (*(*voice).sample).pitchadj as libc::c_double)
                    as fluid_real_t
            } else {
                (*voice).root_pitch = (*(*voice).sample).origpitch as libc::c_float * 100.0f32
                    - (*(*voice).sample).pitchadj as libc::c_float
            }
            (*voice).root_pitch = fluid_ct2hz((*voice).root_pitch);
            if !(*voice).sample.is_null() {
                (*voice).root_pitch *=
                    (*voice).output_rate / (*(*voice).sample).samplerate as libc::c_float
            }
            current_block_195 = 5267916556966421873;
        }
        8 => {
            (*voice).fres = (*voice).gen[GEN_FILTERFC as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_FILTERFC as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_FILTERFC as libc::c_int as usize].nrpn as fluid_real_t;
            (*voice).last_fres = -1.0f32;
            current_block_195 = 5267916556966421873;
        }
        9 => {
            q_dB = (((*voice).gen[GEN_FILTERQ as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_FILTERQ as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_FILTERQ as libc::c_int as usize].nrpn as fluid_real_t)
                / 10.0f32) as libc::c_double;

            q_dB = if q_dB < 0.0f32 as libc::c_double {
                0.0f32 as libc::c_double
            } else if q_dB > 96.0f32 as libc::c_double {
                96.0f32 as libc::c_double
            } else {
                q_dB
            };
            q_dB -= 3.01f32 as libc::c_double;
            (*voice).q_lin = f64::powf(10.0f32 as libc::c_double, q_dB / 20.0f32 as libc::c_double)
                as fluid_real_t;
            (*voice).filter_gain =
                (1.0f64 / f64::sqrt((*voice).q_lin as libc::c_double)) as fluid_real_t;
            (*voice).last_fres = -1.0f64 as fluid_real_t;
            current_block_195 = 5267916556966421873;
        }
        5 => {
            (*voice).modlfo_to_pitch = (*voice).gen[GEN_MODLFOTOPITCH as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_MODLFOTOPITCH as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODLFOTOPITCH as libc::c_int as usize].nrpn as fluid_real_t;
            (*voice).modlfo_to_pitch = if ((*voice).modlfo_to_pitch as libc::c_double) < -12000.0f64
            {
                -12000.0f64
            } else if (*voice).modlfo_to_pitch as libc::c_double > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).modlfo_to_pitch as libc::c_double
            } as fluid_real_t;
            current_block_195 = 5267916556966421873;
        }
        13 => {
            (*voice).modlfo_to_vol = (*voice).gen[GEN_MODLFOTOVOL as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_MODLFOTOVOL as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODLFOTOVOL as libc::c_int as usize].nrpn as fluid_real_t;
            (*voice).modlfo_to_vol = if ((*voice).modlfo_to_vol as libc::c_double) < -960.0f64 {
                -960.0f64
            } else if (*voice).modlfo_to_vol as libc::c_double > 960.0f64 {
                960.0f64
            } else {
                (*voice).modlfo_to_vol as libc::c_double
            } as fluid_real_t;
            current_block_195 = 5267916556966421873;
        }
        10 => {
            (*voice).modlfo_to_fc = (*voice).gen[GEN_MODLFOTOFILTERFC as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_MODLFOTOFILTERFC as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODLFOTOFILTERFC as libc::c_int as usize].nrpn as fluid_real_t;
            (*voice).modlfo_to_fc =
                if (*voice).modlfo_to_fc < -(12000 as libc::c_int) as libc::c_float {
                    -(12000 as libc::c_int) as libc::c_float
                } else if (*voice).modlfo_to_fc > 12000 as libc::c_int as libc::c_float {
                    12000 as libc::c_int as libc::c_float
                } else {
                    (*voice).modlfo_to_fc
                };
            current_block_195 = 5267916556966421873;
        }
        21 => {
            x = (*voice).gen[GEN_MODLFODELAY as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_MODLFODELAY as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODLFODELAY as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -12000.0f32 {
                -12000.0f32
            } else if x > 5000.0f32 {
                5000.0f32
            } else {
                x
            };
            (*voice).modlfo_delay = ((*voice).output_rate * fluid_tc2sec_delay(x)) as libc::c_uint;
            current_block_195 = 5267916556966421873;
        }
        22 => {
            x = (*voice).gen[GEN_MODLFOFREQ as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_MODLFOFREQ as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODLFOFREQ as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -16000.0f32 {
                -16000.0f32
            } else if x > 4500.0f32 {
                4500.0f32
            } else {
                x
            };
            (*voice).modlfo_incr = 4.0f32 * 64 as libc::c_int as libc::c_float * fluid_act2hz(x)
                / (*voice).output_rate;
            current_block_195 = 5267916556966421873;
        }
        24 => {
            x = (*voice).gen[GEN_VIBLFOFREQ as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_VIBLFOFREQ as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_VIBLFOFREQ as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -16000.0f32 {
                -16000.0f32
            } else if x > 4500.0f32 {
                4500.0f32
            } else {
                x
            };
            (*voice).viblfo_incr = 4.0f32 * 64 as libc::c_int as libc::c_float * fluid_act2hz(x)
                / (*voice).output_rate;
            current_block_195 = 5267916556966421873;
        }
        23 => {
            x = (*voice).gen[GEN_VIBLFODELAY as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_VIBLFODELAY as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_VIBLFODELAY as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -12000.0f32 {
                -12000.0f32
            } else if x > 5000.0f32 {
                5000.0f32
            } else {
                x
            };
            (*voice).viblfo_delay = ((*voice).output_rate * fluid_tc2sec_delay(x)) as libc::c_uint;
            current_block_195 = 5267916556966421873;
        }
        6 => {
            (*voice).viblfo_to_pitch = (*voice).gen[GEN_VIBLFOTOPITCH as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_VIBLFOTOPITCH as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_VIBLFOTOPITCH as libc::c_int as usize].nrpn as fluid_real_t;
            (*voice).viblfo_to_pitch = if ((*voice).viblfo_to_pitch as libc::c_double) < -12000.0f64
            {
                -12000.0f64
            } else if (*voice).viblfo_to_pitch as libc::c_double > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).viblfo_to_pitch as libc::c_double
            } as fluid_real_t;
            current_block_195 = 5267916556966421873;
        }
        46 => {
            x = (*voice).gen[GEN_KEYNUM as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_KEYNUM as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_KEYNUM as libc::c_int as usize].nrpn as fluid_real_t;
            if x >= 0 as libc::c_int as libc::c_float {
                (*voice).key = x as libc::c_uchar
            }
            current_block_195 = 5267916556966421873;
        }
        47 => {
            x = (*voice).gen[GEN_VELOCITY as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_VELOCITY as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_VELOCITY as libc::c_int as usize].nrpn as fluid_real_t;
            if x > 0 as libc::c_int as libc::c_float {
                (*voice).vel = x as libc::c_uchar
            }
            current_block_195 = 5267916556966421873;
        }
        7 => {
            (*voice).modenv_to_pitch = (*voice).gen[GEN_MODENVTOPITCH as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_MODENVTOPITCH as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODENVTOPITCH as libc::c_int as usize].nrpn as fluid_real_t;
            (*voice).modenv_to_pitch = if ((*voice).modenv_to_pitch as libc::c_double) < -12000.0f64
            {
                -12000.0f64
            } else if (*voice).modenv_to_pitch as libc::c_double > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).modenv_to_pitch as libc::c_double
            } as fluid_real_t;
            current_block_195 = 5267916556966421873;
        }
        11 => {
            (*voice).modenv_to_fc = (*voice).gen[GEN_MODENVTOFILTERFC as libc::c_int as usize].val
                as fluid_real_t
                + (*voice).gen[GEN_MODENVTOFILTERFC as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODENVTOFILTERFC as libc::c_int as usize].nrpn as fluid_real_t;

            (*voice).modenv_to_fc = if ((*voice).modenv_to_fc as libc::c_double) < -12000.0f64 {
                -12000.0f64
            } else if (*voice).modenv_to_fc as libc::c_double > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).modenv_to_fc as libc::c_double
            } as fluid_real_t;
            current_block_195 = 5267916556966421873;
        }
        0 | 4 => {
            if !(*voice).sample.is_null() {
                (*voice).start = (*(*voice).sample)
                    .start
                    .wrapping_add(
                        ((*voice).gen[GEN_STARTADDROFS as libc::c_int as usize].val as fluid_real_t
                            + (*voice).gen[GEN_STARTADDROFS as libc::c_int as usize].mod_0
                                as fluid_real_t
                            + (*voice).gen[GEN_STARTADDROFS as libc::c_int as usize].nrpn
                                as fluid_real_t) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_STARTADDRCOARSEOFS as libc::c_int as usize].val
                                as fluid_real_t
                                + (*voice).gen[GEN_STARTADDRCOARSEOFS as libc::c_int as usize].mod_0
                                    as fluid_real_t
                                + (*voice).gen[GEN_STARTADDRCOARSEOFS as libc::c_int as usize].nrpn
                                    as fluid_real_t) as libc::c_int)
                            as libc::c_uint,
                    ) as libc::c_int;
                (*voice).check_sample_sanity_flag = (1 as libc::c_int) << 0 as libc::c_int
            }
            current_block_195 = 5267916556966421873;
        }
        1 | 12 => {
            if !(*voice).sample.is_null() {
                (*voice).end = (*(*voice).sample)
                    .end
                    .wrapping_add(
                        ((*voice).gen[GEN_ENDADDROFS as libc::c_int as usize].val as fluid_real_t
                            + (*voice).gen[GEN_ENDADDROFS as libc::c_int as usize].mod_0
                                as fluid_real_t
                            + (*voice).gen[GEN_ENDADDROFS as libc::c_int as usize].nrpn
                                as fluid_real_t) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_ENDADDRCOARSEOFS as libc::c_int as usize].val
                                as fluid_real_t
                                + (*voice).gen[GEN_ENDADDRCOARSEOFS as libc::c_int as usize].mod_0
                                    as fluid_real_t
                                + (*voice).gen[GEN_ENDADDRCOARSEOFS as libc::c_int as usize].nrpn
                                    as fluid_real_t) as libc::c_int)
                            as libc::c_uint,
                    ) as libc::c_int;
                (*voice).check_sample_sanity_flag = (1 as libc::c_int) << 0 as libc::c_int
            }
            current_block_195 = 5267916556966421873;
        }
        2 | 45 => {
            if !(*voice).sample.is_null() {
                (*voice).loopstart = (*(*voice).sample)
                    .loopstart
                    .wrapping_add(
                        ((*voice).gen[GEN_STARTLOOPADDROFS as libc::c_int as usize].val
                            as fluid_real_t
                            + (*voice).gen[GEN_STARTLOOPADDROFS as libc::c_int as usize].mod_0
                                as fluid_real_t
                            + (*voice).gen[GEN_STARTLOOPADDROFS as libc::c_int as usize].nrpn
                                as fluid_real_t) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize].val
                                as fluid_real_t
                                + (*voice).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .mod_0 as fluid_real_t
                                + (*voice).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .nrpn as fluid_real_t)
                                as libc::c_int) as libc::c_uint,
                    ) as libc::c_int;
                (*voice).check_sample_sanity_flag = (1 as libc::c_int) << 0 as libc::c_int
            }
            current_block_195 = 5267916556966421873;
        }
        3 | 50 => {
            if !(*voice).sample.is_null() {
                (*voice).loopend = (*(*voice).sample)
                    .loopend
                    .wrapping_add(
                        ((*voice).gen[GEN_ENDLOOPADDROFS as libc::c_int as usize].val
                            as fluid_real_t
                            + (*voice).gen[GEN_ENDLOOPADDROFS as libc::c_int as usize].mod_0
                                as fluid_real_t
                            + (*voice).gen[GEN_ENDLOOPADDROFS as libc::c_int as usize].nrpn
                                as fluid_real_t) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize].val
                                as fluid_real_t
                                + (*voice).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .mod_0 as fluid_real_t
                                + (*voice).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .nrpn as fluid_real_t)
                                as libc::c_int) as libc::c_uint,
                    ) as libc::c_int;
                (*voice).check_sample_sanity_flag = (1 as libc::c_int) << 0 as libc::c_int
            }
            current_block_195 = 5267916556966421873;
        }
        33 => {
            x = (*voice).gen[GEN_VOLENVDELAY as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_VOLENVDELAY as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_VOLENVDELAY as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -12000.0f32 {
                -12000.0f32
            } else if x > 5000.0f32 {
                5000.0f32
            } else {
                x
            };
            count = ((*voice).output_rate * fluid_tc2sec_delay(x)
                / 64 as libc::c_int as libc::c_float) as libc::c_uint;
            (*voice).volenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].count = count;
            (*voice).volenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].coeff = 0.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].incr = 0.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].min = -1.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].max = 1.0f32;
            current_block_195 = 5267916556966421873;
        }
        34 => {
            x = (*voice).gen[GEN_VOLENVATTACK as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_VOLENVATTACK as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_VOLENVATTACK as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -12000.0f32 {
                -12000.0f32
            } else if x > 8000.0f32 {
                8000.0f32
            } else {
                x
            };
            count = (1 as libc::c_int as libc::c_uint).wrapping_add(
                ((*voice).output_rate * fluid_tc2sec_attack(x) / 64 as libc::c_int as libc::c_float)
                    as libc::c_uint,
            );
            (*voice).volenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].count = count;
            (*voice).volenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].coeff = 1.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].incr = if count != 0
            {
                (1.0f32) / count as libc::c_float
            } else {
                0.0f32
            };
            (*voice).volenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].min = -1.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].max = 1.0f32;
            current_block_195 = 5267916556966421873;
        }
        35 | 39 => {
            count = calculate_hold_decay_buffers(
                voice,
                GEN_VOLENVHOLD as libc::c_int,
                GEN_KEYTOVOLENVHOLD as libc::c_int,
                0 as libc::c_int,
            ) as libc::c_uint;
            (*voice).volenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].count = count;
            (*voice).volenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].coeff = 1.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].incr = 0.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].min = -1.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].max = 2.0f32;
            current_block_195 = 5267916556966421873;
        }
        36 => {
            current_block_195 = 16592787104725195690;
        }
        37 | 40 => {
            current_block_195 = 16592787104725195690;
        }
        38 => {
            x = (*voice).gen[GEN_VOLENVRELEASE as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_VOLENVRELEASE as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_VOLENVRELEASE as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -7200.0f32 {
                -7200.0f32
            } else if x > 8000.0f32 {
                8000.0f32
            } else {
                x
            };
            count = (1 as libc::c_int as libc::c_uint).wrapping_add(
                ((*voice).output_rate * fluid_tc2sec_release(x)
                    / 64 as libc::c_int as libc::c_float) as libc::c_uint,
            );
            (*voice).volenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].count = count;
            (*voice).volenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].coeff = 1.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].incr =
                if count != 0 {
                    (-1.0f32) / count as libc::c_float
                } else {
                    0.0f32
                };
            (*voice).volenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].min = 0.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].max = 1.0f32;
            current_block_195 = 5267916556966421873;
        }
        25 => {
            x = (*voice).gen[GEN_MODENVDELAY as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_MODENVDELAY as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODENVDELAY as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -12000.0f32 {
                -12000.0f32
            } else if x > 5000.0f32 {
                5000.0f32
            } else {
                x
            };
            (*voice).modenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].count =
                ((*voice).output_rate * fluid_tc2sec_delay(x) / 64 as libc::c_int as libc::c_float)
                    as libc::c_uint;
            (*voice).modenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].coeff = 0.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].incr = 0.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].min = -1.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVDELAY as libc::c_int as usize].max = 1.0f32;
            current_block_195 = 5267916556966421873;
        }
        26 => {
            x = (*voice).gen[GEN_MODENVATTACK as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_MODENVATTACK as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODENVATTACK as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -12000.0f32 {
                -12000.0f32
            } else if x > 8000.0f32 {
                8000.0f32
            } else {
                x
            };
            count = (1 as libc::c_int as libc::c_uint).wrapping_add(
                ((*voice).output_rate * fluid_tc2sec_attack(x) / 64 as libc::c_int as libc::c_float)
                    as libc::c_uint,
            );
            (*voice).modenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].count = count;
            (*voice).modenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].coeff = 1.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].incr = if count != 0
            {
                (1.0f32) / count as libc::c_float
            } else {
                0.0f32
            };
            (*voice).modenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].min = -1.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVATTACK as libc::c_int as usize].max = 1.0f32;
            current_block_195 = 5267916556966421873;
        }
        27 | 31 => {
            count = calculate_hold_decay_buffers(
                voice,
                GEN_MODENVHOLD as libc::c_int,
                GEN_KEYTOMODENVHOLD as libc::c_int,
                0 as libc::c_int,
            ) as libc::c_uint;
            (*voice).modenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].count = count;
            (*voice).modenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].coeff = 1.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].incr = 0.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].min = -1.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVHOLD as libc::c_int as usize].max = 2.0f32;
            current_block_195 = 5267916556966421873;
        }
        28 => {
            current_block_195 = 9635119298622998056;
        }
        29 | 32 => {
            current_block_195 = 9635119298622998056;
        }
        30 => {
            x = (*voice).gen[GEN_MODENVRELEASE as libc::c_int as usize].val as fluid_real_t
                + (*voice).gen[GEN_MODENVRELEASE as libc::c_int as usize].mod_0 as fluid_real_t
                + (*voice).gen[GEN_MODENVRELEASE as libc::c_int as usize].nrpn as fluid_real_t;
            x = if x < -12000.0f32 {
                -12000.0f32
            } else if x > 8000.0f32 {
                8000.0f32
            } else {
                x
            };
            count = (1 as libc::c_int as libc::c_uint).wrapping_add(
                ((*voice).output_rate * fluid_tc2sec_release(x)
                    / 64 as libc::c_int as libc::c_float) as libc::c_uint,
            );
            (*voice).modenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].count = count;
            (*voice).modenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].coeff = 1.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].incr =
                if count != 0 {
                    (-1.0f32 / count as libc::c_float) as libc::c_double
                } else {
                    0.0f64
                } as fluid_real_t;
            (*voice).modenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].min = 0.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVRELEASE as libc::c_int as usize].max = 2.0f32;
            current_block_195 = 5267916556966421873;
        }
        _ => {
            current_block_195 = 5267916556966421873;
        }
    }
    match current_block_195 {
        9635119298622998056 => {
            count = calculate_hold_decay_buffers(
                voice,
                GEN_MODENVDECAY as libc::c_int,
                GEN_KEYTOMODENVDECAY as libc::c_int,
                1 as libc::c_int,
            ) as libc::c_uint;
            y = 1.0f32
                - 0.001f32
                    * ((*voice).gen[GEN_MODENVSUSTAIN as libc::c_int as usize].val as fluid_real_t
                        + (*voice).gen[GEN_MODENVSUSTAIN as libc::c_int as usize].mod_0
                            as fluid_real_t
                        + (*voice).gen[GEN_MODENVSUSTAIN as libc::c_int as usize].nrpn
                            as fluid_real_t);
            y = if y < 0.0f32 {
                0.0f32
            } else if y > 1.0f32 {
                1.0f32
            } else {
                y
            };
            (*voice).modenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].count = count;
            (*voice).modenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].coeff = 1.0f32;
            (*voice).modenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].incr = if count != 0
            {
                (-1.0f32) / count as libc::c_float
            } else {
                0.0f32
            };
            (*voice).modenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].min = y;
            (*voice).modenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].max = 2.0f32
        }
        16592787104725195690 => {
            y = 1.0f32
                - 0.001f32
                    * ((*voice).gen[GEN_VOLENVSUSTAIN as libc::c_int as usize].val as fluid_real_t
                        + (*voice).gen[GEN_VOLENVSUSTAIN as libc::c_int as usize].mod_0
                            as fluid_real_t
                        + (*voice).gen[GEN_VOLENVSUSTAIN as libc::c_int as usize].nrpn
                            as fluid_real_t);
            y = if y < 0.0f32 {
                0.0f32
            } else if y > 1.0f32 {
                1.0f32
            } else {
                y
            };
            count = calculate_hold_decay_buffers(
                voice,
                GEN_VOLENVDECAY as libc::c_int,
                GEN_KEYTOVOLENVDECAY as libc::c_int,
                1 as libc::c_int,
            ) as libc::c_uint;
            (*voice).volenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].count = count;
            (*voice).volenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].coeff = 1.0f32;
            (*voice).volenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].incr = if count != 0
            {
                (-1.0f32) / count as libc::c_float
            } else {
                0.0f32
            };
            (*voice).volenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].min = y;
            (*voice).volenv_data[FLUID_VOICE_ENVDECAY as libc::c_int as usize].max = 2.0f32
        }
        _ => {}
    };
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_modulate(
    mut voice: *mut fluid_voice_t,
    mut cc: libc::c_int,
    mut ctrl: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut mod_0: *mut fluid_mod_t = 0 as *mut fluid_mod_t;
    let mut gen: libc::c_int = 0;
    let mut modval: fluid_real_t = 0.;
    i = 0 as libc::c_int;
    while i < (*voice).mod_count {
        mod_0 = &mut *(*voice).mod_0.as_mut_ptr().offset(i as isize) as *mut fluid_mod_t;

        if (*mod_0).src1 as libc::c_int == ctrl
            && (*mod_0).flags1 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0 as libc::c_int
            && cc != 0 as libc::c_int
            || (*mod_0).src1 as libc::c_int == ctrl
                && (*mod_0).flags1 as libc::c_int & FLUID_MOD_CC as libc::c_int == 0 as libc::c_int
                && cc == 0 as libc::c_int
            || ((*mod_0).src2 as libc::c_int == ctrl
                && (*mod_0).flags2 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0 as libc::c_int
                && cc != 0 as libc::c_int
                || (*mod_0).src2 as libc::c_int == ctrl
                    && (*mod_0).flags2 as libc::c_int & FLUID_MOD_CC as libc::c_int
                        == 0 as libc::c_int
                    && cc == 0 as libc::c_int)
        {
            gen = fluid_mod_get_dest(mod_0);
            modval = 0.0f64 as fluid_real_t;
            k = 0 as libc::c_int;
            while k < (*voice).mod_count {
                if (*voice).mod_0[k as usize].dest as libc::c_int == gen {
                    modval += fluid_mod_get_value(
                        &mut *(*voice).mod_0.as_mut_ptr().offset(k as isize),
                        (*voice).channel,
                        voice,
                    )
                }
                k += 1
            }
            (*voice).gen[gen as usize].mod_0 = modval as libc::c_double;
            fluid_voice_update_param(voice, gen);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_modulate_all(mut voice: *mut fluid_voice_t) -> libc::c_int {
    let mut mod_0: *mut fluid_mod_t = 0 as *mut fluid_mod_t;
    let mut i: libc::c_int = 0;
    let mut k: libc::c_int = 0;
    let mut gen: libc::c_int = 0;
    let mut modval: fluid_real_t = 0.;
    i = 0 as libc::c_int;
    while i < (*voice).mod_count {
        mod_0 = &mut *(*voice).mod_0.as_mut_ptr().offset(i as isize) as *mut fluid_mod_t;
        gen = fluid_mod_get_dest(mod_0);
        modval = 0.0f64 as fluid_real_t;
        k = 0 as libc::c_int;
        while k < (*voice).mod_count {
            if (*voice).mod_0[k as usize].dest as libc::c_int == gen {
                modval += fluid_mod_get_value(
                    &mut *(*voice).mod_0.as_mut_ptr().offset(k as isize),
                    (*voice).channel,
                    voice,
                )
            }
            k += 1
        }
        (*voice).gen[gen as usize].mod_0 = modval as libc::c_double;
        fluid_voice_update_param(voice, gen);
        i += 1
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_noteoff(mut voice: *mut fluid_voice_t) -> libc::c_int {
    let mut at_tick: libc::c_uint = 0;
    at_tick = (*(*(*voice).channel).synth).min_note_length_ticks;
    if at_tick > (*voice).ticks {
        (*voice).noteoff_ticks = at_tick;
        return FLUID_OK as libc::c_int;
    }
    if !(*voice).channel.is_null()
        && (*(*voice).channel).cc[SUSTAIN_SWITCH as libc::c_int as usize] as libc::c_int
            >= 64 as libc::c_int
    {
        (*voice).status = FLUID_VOICE_SUSTAINED as libc::c_int as libc::c_uchar
    } else {
        if (*voice).volenv_section == FLUID_VOICE_ENVATTACK as libc::c_int {
            if (*voice).volenv_val > 0 as libc::c_int as libc::c_float {
                let mut lfo: fluid_real_t = (*voice).modlfo_val * -(*voice).modlfo_to_vol;
                let mut amp: fluid_real_t = ((*voice).volenv_val as libc::c_double
                    * f64::powf(
                        10.0f64,
                        (lfo / -(200 as libc::c_int) as libc::c_float) as libc::c_double,
                    )) as fluid_real_t;
                let mut env_value: fluid_real_t =
                    -((-(200 as libc::c_int) as libc::c_double * f64::ln(amp as libc::c_double)
                        / f64::ln(10.0f64)
                        - lfo as libc::c_double)
                        / 960.0f64
                        - 1 as libc::c_int as libc::c_double) as fluid_real_t;
                env_value = if (env_value as libc::c_double) < 0.0f64 {
                    0.0f64
                } else if env_value as libc::c_double > 1.0f64 {
                    1.0f64
                } else {
                    env_value as libc::c_double
                } as fluid_real_t;
                (*voice).volenv_val = env_value
            }
        }
        (*voice).volenv_section = FLUID_VOICE_ENVRELEASE as libc::c_int;
        (*voice).volenv_count = 0 as libc::c_int as libc::c_uint;
        (*voice).modenv_section = FLUID_VOICE_ENVRELEASE as libc::c_int;
        (*voice).modenv_count = 0 as libc::c_int as libc::c_uint
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_kill_excl(mut voice: *mut fluid_voice_t) -> libc::c_int {
    if !((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
        || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
    {
        return FLUID_OK as libc::c_int;
    }

    fluid_voice_gen_set(
        voice,
        GEN_EXCLUSIVECLASS as libc::c_int,
        0 as libc::c_int as libc::c_float,
    );
    if (*voice).volenv_section != FLUID_VOICE_ENVRELEASE as libc::c_int {
        (*voice).volenv_section = FLUID_VOICE_ENVRELEASE as libc::c_int;
        (*voice).volenv_count = 0 as libc::c_int as libc::c_uint;
        (*voice).modenv_section = FLUID_VOICE_ENVRELEASE as libc::c_int;
        (*voice).modenv_count = 0 as libc::c_int as libc::c_uint
    }

    fluid_voice_gen_set(
        voice,
        GEN_VOLENVRELEASE as libc::c_int,
        -(200 as libc::c_int) as libc::c_float,
    );
    fluid_voice_update_param(voice, GEN_VOLENVRELEASE as libc::c_int);

    fluid_voice_gen_set(
        voice,
        GEN_MODENVRELEASE as libc::c_int,
        -(200 as libc::c_int) as libc::c_float,
    );
    fluid_voice_update_param(voice, GEN_MODENVRELEASE as libc::c_int);
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_off(mut voice: *mut fluid_voice_t) -> libc::c_int {
    (*voice).chan = 0xff as libc::c_int as libc::c_uchar;
    (*voice).volenv_section = FLUID_VOICE_ENVFINISHED as libc::c_int;
    (*voice).volenv_count = 0 as libc::c_int as libc::c_uint;
    (*voice).modenv_section = FLUID_VOICE_ENVFINISHED as libc::c_int;
    (*voice).modenv_count = 0 as libc::c_int as libc::c_uint;
    (*voice).status = FLUID_VOICE_OFF as libc::c_int as libc::c_uchar;
    if !(*voice).sample.is_null() {
        (*(*voice).sample).refcount = (*(*voice).sample).refcount.wrapping_sub(1);
        if (*(*voice).sample).refcount == 0 as libc::c_int as libc::c_uint
            && (*(*voice).sample).notify.is_some()
        {
            Some(
                (*(*voice).sample)
                    .notify
                    .expect("non-null function pointer"),
            )
            .expect("non-null function pointer")(
                (*voice).sample, FLUID_SAMPLE_DONE as libc::c_int
            );
        }
        (*voice).sample = 0 as *mut fluid_sample_t
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_add_mod(
    mut voice: *mut fluid_voice_t,
    mut mod_0: *mut fluid_mod_t,
    mut mode: libc::c_int,
) {
    let mut i: libc::c_int = 0;

    if (*mod_0).flags1 as libc::c_int & FLUID_MOD_CC as libc::c_int == 0 as libc::c_int
        && ((*mod_0).src1 as libc::c_int != 0 as libc::c_int
            && (*mod_0).src1 as libc::c_int != 2 as libc::c_int
            && (*mod_0).src1 as libc::c_int != 3 as libc::c_int
            && (*mod_0).src1 as libc::c_int != 10 as libc::c_int
            && (*mod_0).src1 as libc::c_int != 13 as libc::c_int
            && (*mod_0).src1 as libc::c_int != 14 as libc::c_int
            && (*mod_0).src1 as libc::c_int != 16 as libc::c_int)
    {
        fluid_log!(
            FLUID_WARN,
            "Ignoring invalid controller, using non-CC source {}.",
            (*mod_0).src1 as libc::c_int
        );
        return;
    }
    if mode == FLUID_VOICE_ADD as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*voice).mod_count {
            if fluid_mod_test_identity(&mut *(*voice).mod_0.as_mut_ptr().offset(i as isize), mod_0)
                != 0
            {
                //		printf("Adding modulator...\n");
                (*voice).mod_0[i as usize].amount += (*mod_0).amount;
                return;
            }
            i += 1
        }
    } else if mode == FLUID_VOICE_OVERWRITE as libc::c_int {
        i = 0 as libc::c_int;
        while i < (*voice).mod_count {
            if fluid_mod_test_identity(&mut *(*voice).mod_0.as_mut_ptr().offset(i as isize), mod_0)
                != 0
            {
                //		printf("Replacing modulator...amount is %f\n",mod->amount);
                (*voice).mod_0[i as usize].amount = (*mod_0).amount;
                return;
            }
            i += 1
        }
    }
    if (*voice).mod_count < 64 as libc::c_int {
        let fresh7 = (*voice).mod_count;
        (*voice).mod_count = (*voice).mod_count + 1;
        fluid_mod_clone(
            &mut *(*voice).mod_0.as_mut_ptr().offset(fresh7 as isize),
            mod_0,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_get_id(mut voice: *mut fluid_voice_t) -> libc::c_uint {
    return (*voice).id;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_is_playing(mut voice: *mut fluid_voice_t) -> libc::c_int {
    return ((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
        || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
        as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_get_lower_boundary_for_attenuation(
    mut voice: *mut fluid_voice_t,
) -> fluid_real_t {
    let mut i: libc::c_int = 0;
    let mut mod_0: *mut fluid_mod_t = 0 as *mut fluid_mod_t;
    let mut possible_att_reduction_cB: fluid_real_t = 0 as libc::c_int as fluid_real_t;
    let mut lower_bound: fluid_real_t = 0.;
    i = 0 as libc::c_int;
    while i < (*voice).mod_count {
        mod_0 = &mut *(*voice).mod_0.as_mut_ptr().offset(i as isize) as *mut fluid_mod_t;

        if (*mod_0).dest as libc::c_int == GEN_ATTENUATION as libc::c_int
            && ((*mod_0).flags1 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0
                || (*mod_0).flags2 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0)
        {
            let mut current_val: fluid_real_t = fluid_mod_get_value(mod_0, (*voice).channel, voice);
            let mut v: fluid_real_t = f64::abs((*mod_0).amount) as fluid_real_t;
            if (*mod_0).src1 as libc::c_int == FLUID_MOD_PITCHWHEEL as libc::c_int
                || (*mod_0).flags1 as libc::c_int & FLUID_MOD_BIPOLAR as libc::c_int != 0
                || (*mod_0).flags2 as libc::c_int & FLUID_MOD_BIPOLAR as libc::c_int != 0
                || (*mod_0).amount < 0 as libc::c_int as libc::c_double
            {
                v = (v as libc::c_double * -1.0f64) as fluid_real_t
            } else {
                v = 0 as libc::c_int as fluid_real_t
            }
            if current_val > v {
                possible_att_reduction_cB += current_val - v
            }
        }
        i += 1
    }
    lower_bound = (*voice).attenuation - possible_att_reduction_cB;
    if lower_bound < 0 as libc::c_int as libc::c_float {
        lower_bound = 0 as libc::c_int as fluid_real_t
    }
    return lower_bound;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_check_sample_sanity(mut voice: *mut fluid_voice_t) {
    let mut min_index_nonloop: libc::c_int = (*(*voice).sample).start as libc::c_int;
    let mut max_index_nonloop: libc::c_int = (*(*voice).sample).end as libc::c_int;
    let mut min_index_loop: libc::c_int =
        (*(*voice).sample).start as libc::c_int + 0 as libc::c_int;
    let mut max_index_loop: libc::c_int =
        (*(*voice).sample).end as libc::c_int - 0 as libc::c_int + 1 as libc::c_int;
    if (*voice).check_sample_sanity_flag == 0 {
        return;
    }
    if (*voice).start < min_index_nonloop {
        (*voice).start = min_index_nonloop
    } else if (*voice).start > max_index_nonloop {
        (*voice).start = max_index_nonloop
    }
    if (*voice).end < min_index_nonloop {
        (*voice).end = min_index_nonloop
    } else if (*voice).end > max_index_nonloop {
        (*voice).end = max_index_nonloop
    }
    if (*voice).start > (*voice).end {
        let mut temp: libc::c_int = (*voice).start;
        (*voice).start = (*voice).end;
        (*voice).end = temp
    }
    if (*voice).start == (*voice).end {
        fluid_voice_off(voice);
        return;
    }
    if (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
        == FLUID_LOOP_UNTIL_RELEASE as libc::c_int
        || (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
            == FLUID_LOOP_DURING_RELEASE as libc::c_int
    {
        if (*voice).loopstart < min_index_loop {
            (*voice).loopstart = min_index_loop
        } else if (*voice).loopstart > max_index_loop {
            (*voice).loopstart = max_index_loop
        }
        if (*voice).loopend < min_index_loop {
            (*voice).loopend = min_index_loop
        } else if (*voice).loopend > max_index_loop {
            (*voice).loopend = max_index_loop
        }
        if (*voice).loopstart > (*voice).loopend {
            let mut temp_0: libc::c_int = (*voice).loopstart;
            (*voice).loopstart = (*voice).loopend;
            (*voice).loopend = temp_0
        }
        if (*voice).loopend < (*voice).loopstart + 2 as libc::c_int {
            (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val =
                FLUID_UNLOOPED as libc::c_int as libc::c_double
        }
        if (*voice).loopstart >= (*(*voice).sample).loopstart as libc::c_int
            && (*voice).loopend <= (*(*voice).sample).loopend as libc::c_int
        {
            if (*(*voice).sample).amplitude_that_reaches_noise_floor_is_valid != 0 {
                (*voice).amplitude_that_reaches_noise_floor_loop =
                    ((*(*voice).sample).amplitude_that_reaches_noise_floor
                        / (*voice).synth_gain as libc::c_double) as fluid_real_t
            } else {
                (*voice).amplitude_that_reaches_noise_floor_loop =
                    (*voice).amplitude_that_reaches_noise_floor_nonloop
            }
        }
    }

    if (*voice).check_sample_sanity_flag & (1 as libc::c_int) << 1 as libc::c_int != 0 {
        if max_index_loop - min_index_loop < 2 as libc::c_int {
            if (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
                == FLUID_LOOP_UNTIL_RELEASE as libc::c_int
                || (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
                    == FLUID_LOOP_DURING_RELEASE as libc::c_int
            {
                (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val =
                    FLUID_UNLOOPED as libc::c_int as libc::c_double
            }
        }

        (*voice).phase = ((*voice).start as libc::c_ulonglong) << 32 as libc::c_int
    }
    if (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
        == FLUID_LOOP_UNTIL_RELEASE as libc::c_int
        && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as libc::c_int
        || (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val as libc::c_int
            == FLUID_LOOP_DURING_RELEASE as libc::c_int
    {
        let mut index_in_sample: libc::c_int =
            ((*voice).phase >> 32 as libc::c_int) as libc::c_uint as libc::c_int;
        if index_in_sample >= (*voice).loopend {
            (*voice).phase = ((*voice).loopstart as libc::c_ulonglong) << 32 as libc::c_int
        }
    }
    (*voice).check_sample_sanity_flag = 0 as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_set_param(
    mut voice: *mut fluid_voice_t,
    mut gen: libc::c_int,
    mut nrpn_value: fluid_real_t,
    mut abs: libc::c_int,
) -> libc::c_int {
    (*voice).gen[gen as usize].nrpn = nrpn_value as libc::c_double;
    (*voice).gen[gen as usize].flags = if abs != 0 {
        GEN_ABS_NRPN as libc::c_int
    } else {
        GEN_SET as libc::c_int
    } as libc::c_uchar;
    fluid_voice_update_param(voice, gen);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_set_gain(
    mut voice: *mut fluid_voice_t,
    mut gain: fluid_real_t,
) -> libc::c_int {
    if (gain as libc::c_double) < 0.0000001f64 {
        gain = 0.0000001f64 as fluid_real_t
    }
    (*voice).synth_gain = gain;
    (*voice).amp_left = fluid_pan((*voice).pan, 1 as libc::c_int) * gain / 32768.0f32;
    (*voice).amp_right = fluid_pan((*voice).pan, 0 as libc::c_int) * gain / 32768.0f32;
    (*voice).amp_reverb = (*voice).reverb_send * gain / 32768.0f32;
    (*voice).amp_chorus = (*voice).chorus_send * gain / 32768.0f32;
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_voice_optimize_sample(mut s: *mut fluid_sample_t) -> libc::c_int {
    let mut peak_max: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut peak_min: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut peak: libc::c_short = 0;
    let mut normalized_amplitude_during_loop: fluid_real_t = 0.;
    let mut result: libc::c_double = 0.;
    let mut i: libc::c_int = 0;
    if (*s).valid == 0 || (*s).sampletype & 0x10 as libc::c_int != 0 {
        return FLUID_OK as libc::c_int;
    }
    if (*s).amplitude_that_reaches_noise_floor_is_valid == 0 {
        i = (*s).loopstart as libc::c_int;
        while i < (*s).loopend as libc::c_int {
            let mut val: libc::c_short = *(*s).data.offset(i as isize);
            if val as libc::c_int > peak_max as libc::c_int {
                peak_max = val
            } else if (val as libc::c_int) < peak_min as libc::c_int {
                peak_min = val
            }
            i += 1
        }
        if peak_max as libc::c_int > -(peak_min as libc::c_int) {
            peak = peak_max
        } else {
            peak = -(peak_min as libc::c_int) as libc::c_short
        }
        if peak as libc::c_int == 0 as libc::c_int {
            peak = 1 as libc::c_int as libc::c_short
        }
        normalized_amplitude_during_loop =
            (peak as fluid_real_t as libc::c_double / 32768.0f64) as fluid_real_t;
        result = 0.00003f64 / normalized_amplitude_during_loop as libc::c_double;
        (*s).amplitude_that_reaches_noise_floor = result;
        (*s).amplitude_that_reaches_noise_floor_is_valid = 1 as libc::c_int
    }
    return FLUID_OK as libc::c_int;
}
