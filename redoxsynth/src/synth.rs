#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_mut
)]
use crate::channel::delete_fluid_channel;
use crate::channel::fluid_channel_cc;
use crate::channel::fluid_channel_get_banknum;
use crate::channel::fluid_channel_get_num;
use crate::channel::fluid_channel_get_preset;
use crate::channel::fluid_channel_get_prognum;
use crate::channel::fluid_channel_get_sfontnum;
use crate::channel::fluid_channel_pitch_bend;
use crate::channel::fluid_channel_pitch_wheel_sens;
use crate::channel::fluid_channel_pressure;
use crate::channel::fluid_channel_reset;
use crate::channel::fluid_channel_set_banknum;
use crate::channel::fluid_channel_set_interp_method;
use crate::channel::fluid_channel_set_preset;
use crate::channel::fluid_channel_set_prognum;
use crate::channel::fluid_channel_set_sfontnum;
use crate::channel::fluid_channel_t;
use crate::channel::new_fluid_channel;
use crate::chorus::delete_fluid_chorus;
use crate::chorus::fluid_chorus_get_depth_ms;
use crate::chorus::fluid_chorus_get_level;
use crate::chorus::fluid_chorus_get_nr;
use crate::chorus::fluid_chorus_get_speed_Hz;
use crate::chorus::fluid_chorus_get_type;
use crate::chorus::fluid_chorus_processmix;
use crate::chorus::fluid_chorus_processreplace;
use crate::chorus::fluid_chorus_reset;
use crate::chorus::fluid_chorus_set_depth_ms;
use crate::chorus::fluid_chorus_set_level;
use crate::chorus::fluid_chorus_set_nr;
use crate::chorus::fluid_chorus_set_speed_Hz;
use crate::chorus::fluid_chorus_set_type;
use crate::chorus::fluid_chorus_t;
use crate::chorus::fluid_chorus_update;
use crate::chorus::new_fluid_chorus;
use crate::conv::fluid_conversion_config;
use crate::defsfont::new_fluid_defsfloader;
use crate::dsp_float::fluid_dsp_float_config;
use crate::gen::fluid_gen_scale;
use crate::hash::fluid_hashtable_t;
use crate::list::delete_fluid_list;
use crate::list::fluid_list_insert_at;
use crate::list::fluid_list_nth;
use crate::list::fluid_list_prepend;
use crate::list::fluid_list_remove;
use crate::list::fluid_list_size;
use crate::list::fluid_list_t;
use crate::modulator::fluid_mod_set_amount;
use crate::modulator::fluid_mod_set_dest;
use crate::modulator::fluid_mod_set_source1;
use crate::modulator::fluid_mod_set_source2;
use crate::modulator::fluid_mod_t;
use crate::reverb::delete_fluid_revmodel;
use crate::reverb::fluid_revmodel_getdamp;
use crate::reverb::fluid_revmodel_getlevel;
use crate::reverb::fluid_revmodel_getroomsize;
use crate::reverb::fluid_revmodel_getwidth;
use crate::reverb::fluid_revmodel_processmix;
use crate::reverb::fluid_revmodel_processreplace;
use crate::reverb::fluid_revmodel_reset;
use crate::reverb::fluid_revmodel_setdamp;
use crate::reverb::fluid_revmodel_setlevel;
use crate::reverb::fluid_revmodel_setroomsize;
use crate::reverb::fluid_revmodel_setwidth;
use crate::reverb::fluid_revmodel_t;
use crate::reverb::new_fluid_revmodel;
use crate::settings::fluid_settings_getint;
use crate::settings::fluid_settings_getnum;
use crate::settings::fluid_settings_getstr;
use crate::settings::fluid_settings_register_int;
use crate::settings::fluid_settings_register_num;
use crate::settings::fluid_settings_register_str;
use crate::settings::fluid_settings_setint;
use crate::settings::fluid_settings_setnum;
use crate::settings::fluid_settings_setstr;
use crate::settings::fluid_settings_str_equal;
use crate::sfont::fluid_preset_t;
use crate::sfont::fluid_sample_t;
use crate::sfont::fluid_sfloader_t;
use crate::sfont::fluid_sfont_t;
use crate::sys::fluid_error;
use crate::sys::fluid_sys_config;
use crate::tuning::fluid_tuning_get_name;
use crate::tuning::fluid_tuning_set_all;
use crate::tuning::fluid_tuning_set_name;
use crate::tuning::fluid_tuning_set_octave;
use crate::tuning::fluid_tuning_set_pitch;
use crate::tuning::fluid_tuning_t;
use crate::tuning::new_fluid_tuning;
use crate::voice::delete_fluid_voice;
use crate::voice::fluid_voice_add_mod;
use crate::voice::fluid_voice_get_channel;
use crate::voice::fluid_voice_get_id;
use crate::voice::fluid_voice_init;
use crate::voice::fluid_voice_is_playing;
use crate::voice::fluid_voice_kill_excl;
use crate::voice::fluid_voice_modulate;
use crate::voice::fluid_voice_modulate_all;
use crate::voice::fluid_voice_noteoff;
use crate::voice::fluid_voice_off;
use crate::voice::fluid_voice_set_gain;
use crate::voice::fluid_voice_set_param;
use crate::voice::fluid_voice_start;
use crate::voice::fluid_voice_t;
use crate::voice::fluid_voice_write;
use crate::voice::new_fluid_voice;
use std::ffi::CStr;
pub type fluid_settings_t = fluid_hashtable_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_synth_t {
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
pub type fluid_real_t = libc::c_float;
pub const FLUID_OK: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_bank_offset_t {
    pub sfont_id: libc::c_int,
    pub offset: libc::c_int,
}
pub const FLUID_SYNTH_STOPPED: fluid_synth_status = 3;
pub const FLUID_FAILED: C2RustUnnamed = -1;
pub const FLUID_SYNTH_PLAYING: fluid_synth_status = 1;
pub type fluid_int_update_t = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int,
>;
pub const FLUID_VOICE_SUSTAINED: fluid_voice_status = 2;
pub const FLUID_VOICE_ON: fluid_voice_status = 1;
pub type fluid_num_update_t = Option<
    unsafe extern "C" fn(
        _: *mut libc::c_void,
        _: *const libc::c_char,
        _: libc::c_double,
    ) -> libc::c_int,
>;
pub const GEN_PITCH: fluid_gen_type = 59;
pub const FLUID_MOD_POSITIVE: fluid_mod_flags = 0;
pub const FLUID_MOD_UNIPOLAR: fluid_mod_flags = 0;
pub const FLUID_MOD_LINEAR: fluid_mod_flags = 0;
pub const FLUID_MOD_GC: fluid_mod_flags = 0;
pub const FLUID_MOD_PITCHWHEELSENS: fluid_mod_src = 16;
pub const FLUID_MOD_BIPOLAR: fluid_mod_flags = 2;
pub const FLUID_MOD_PITCHWHEEL: fluid_mod_src = 14;
pub const GEN_CHORUSSEND: fluid_gen_type = 15;
pub const FLUID_MOD_CC: fluid_mod_flags = 16;
pub const GEN_REVERBSEND: fluid_gen_type = 16;
pub const GEN_ATTENUATION: fluid_gen_type = 48;
pub const FLUID_MOD_NEGATIVE: fluid_mod_flags = 1;
pub const FLUID_MOD_CONCAVE: fluid_mod_flags = 4;
pub const GEN_PAN: fluid_gen_type = 17;
pub const GEN_VIBLFOTOPITCH: fluid_gen_type = 6;
pub const FLUID_MOD_CHANNELPRESSURE: fluid_mod_src = 13;
pub const GEN_FILTERFC: fluid_gen_type = 8;
pub const FLUID_MOD_SWITCH: fluid_mod_flags = 12;
pub const FLUID_MOD_VELOCITY: fluid_mod_src = 2;
pub const FLUID_VOICE_OFF: fluid_voice_status = 3;
pub const FLUID_VOICE_CLEAN: fluid_voice_status = 0;
pub const FLUID_VOICE_ENVRELEASE: fluid_voice_envelope_index_t = 5;
pub const FLUID_MOD_KEYPRESSURE: fluid_mod_src = 10;
pub const MIDI_SYSEX_TUNING_OCTAVE_TUNE_1BYTE: midi_sysex_tuning_msg_id = 8;
pub const MIDI_SYSEX_TUNING_OCTAVE_TUNE_2BYTE: midi_sysex_tuning_msg_id = 9;
pub const MIDI_SYSEX_TUNING_NOTE_TUNE: midi_sysex_tuning_msg_id = 2;
pub type uint8 = libc::c_uchar;
pub const MIDI_SYSEX_TUNING_BULK_DUMP: midi_sysex_tuning_msg_id = 1;
pub const MIDI_SYSEX_UNIV_NON_REALTIME: midi_sysex_manuf = 126;
pub const MIDI_SYSEX_TUNING_BULK_DUMP_REQ: midi_sysex_tuning_msg_id = 0;
pub const MIDI_SYSEX_TUNING_BULK_DUMP_REQ_BANK: midi_sysex_tuning_msg_id = 3;
pub const MIDI_SYSEX_UNIV_REALTIME: midi_sysex_manuf = 127;
pub const GEN_LAST: fluid_gen_type = 60;
pub const FLUID_VOICE_DEFAULT: fluid_voice_add_mod = 2;
pub const FLUID_VOICE_ENVATTACK: fluid_voice_envelope_index_t = 1;
pub const GEN_EXCLUSIVECLASS: fluid_gen_type = 57;
pub type fluid_mod_flags = libc::c_uint;
pub type fluid_mod_src = libc::c_uint;
pub type fluid_gen_type = libc::c_uint;
pub type C2RustUnnamed = libc::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_revmodel_presets_t {
    pub name: *mut libc::c_char,
    pub roomsize: fluid_real_t,
    pub damp: fluid_real_t,
    pub width: fluid_real_t,
    pub level: fluid_real_t,
}
pub type fluid_voice_status = libc::c_uint;
pub type fluid_voice_envelope_index_t = libc::c_uint;
pub type fluid_synth_status = libc::c_uint;
pub type midi_sysex_manuf = libc::c_uint;
pub type midi_sysex_tuning_msg_id = libc::c_uint;
static mut fluid_synth_initialized: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut default_vel2att_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_vel2filter_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_at2viblfo_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_mod2viblfo_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_att_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_pan_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_expr_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_reverb_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_chorus_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
#[no_mangle]
pub static mut default_pitch_bend_mod: fluid_mod_t = fluid_mod_t {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const fluid_mod_t as *mut fluid_mod_t,
};
static mut revmodel_preset: [fluid_revmodel_presets_t; 6] = [
    {
        let mut init = fluid_revmodel_presets_t {
            name: b"Test 1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            roomsize: 0.2f32,
            damp: 0.0f32,
            width: 0.5f32,
            level: 0.9f32,
        };
        init
    },
    {
        let mut init = fluid_revmodel_presets_t {
            name: b"Test 2\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            roomsize: 0.4f32,
            damp: 0.2f32,
            width: 0.5f32,
            level: 0.8f32,
        };
        init
    },
    {
        let mut init = fluid_revmodel_presets_t {
            name: b"Test 3\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            roomsize: 0.6f32,
            damp: 0.4f32,
            width: 0.5f32,
            level: 0.7f32,
        };
        init
    },
    {
        let mut init = fluid_revmodel_presets_t {
            name: b"Test 4\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            roomsize: 0.8f32,
            damp: 0.7f32,
            width: 0.5f32,
            level: 0.6f32,
        };
        init
    },
    {
        let mut init = fluid_revmodel_presets_t {
            name: b"Test 5\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            roomsize: 0.8f32,
            damp: 1.0f32,
            width: 0.5f32,
            level: 0.5f32,
        };
        init
    },
    {
        let mut init = fluid_revmodel_presets_t {
            name: 0 as *const libc::c_char as *mut libc::c_char,
            roomsize: 0.0f32,
            damp: 0.0f32,
            width: 0.0f32,
            level: 0.0f32,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_settings(mut settings: *mut fluid_settings_t) {
    fluid_settings_register_str(
        settings,
        b"synth.verbose\x00" as *const u8 as *const libc::c_char,
        b"no\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.dump\x00" as *const u8 as *const libc::c_char,
        b"no\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.reverb.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.chorus.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.ladspa.active\x00" as *const u8 as *const libc::c_char,
        b"no\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"midi.portname\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.drums-channel.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.polyphony\x00" as *const u8 as *const libc::c_char,
        256 as libc::c_int,
        16 as libc::c_int,
        4096 as libc::c_int,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.midi-channels\x00" as *const u8 as *const libc::c_char,
        16 as libc::c_int,
        16 as libc::c_int,
        256 as libc::c_int,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_num(
        settings,
        b"synth.gain\x00" as *const u8 as *const libc::c_char,
        0.2f32 as libc::c_double,
        0.0f32 as libc::c_double,
        10.0f32 as libc::c_double,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.audio-channels\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        256 as libc::c_int,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.audio-groups\x00" as *const u8 as *const libc::c_char,
        1 as libc::c_int,
        1 as libc::c_int,
        256 as libc::c_int,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.effects-channels\x00" as *const u8 as *const libc::c_char,
        2 as libc::c_int,
        2 as libc::c_int,
        2 as libc::c_int,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_num(
        settings,
        b"synth.sample-rate\x00" as *const u8 as *const libc::c_char,
        44100.0f32 as libc::c_double,
        22050.0f32 as libc::c_double,
        96000.0f32 as libc::c_double,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.min-note-length\x00" as *const u8 as *const libc::c_char,
        10 as libc::c_int,
        0 as libc::c_int,
        65535 as libc::c_int,
        0 as libc::c_int,
        None,
        0 as *mut libc::c_void,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fluid_version(
    mut major: *mut libc::c_int,
    mut minor: *mut libc::c_int,
    mut micro: *mut libc::c_int,
) {
    *major = 1 as libc::c_int;
    *minor = 2 as libc::c_int;
    *micro = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_version_str() -> *mut libc::c_char {
    return b"1.2.1\x00" as *const u8 as *const libc::c_char as *mut libc::c_char;
}
unsafe extern "C" fn fluid_synth_init() {
    fluid_synth_initialized += 1;
    fluid_conversion_config();
    fluid_dsp_float_config();
    fluid_sys_config();
    init_dither();
    fluid_mod_set_source1(
        &mut default_vel2att_mod,
        FLUID_MOD_VELOCITY as libc::c_int,
        FLUID_MOD_GC as libc::c_int
            | FLUID_MOD_CONCAVE as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_NEGATIVE as libc::c_int,
    );
    fluid_mod_set_source2(&mut default_vel2att_mod, 0 as libc::c_int, 0 as libc::c_int);
    fluid_mod_set_dest(&mut default_vel2att_mod, GEN_ATTENUATION as libc::c_int);
    fluid_mod_set_amount(&mut default_vel2att_mod, 960.0f64);
    fluid_mod_set_source1(
        &mut default_vel2filter_mod,
        FLUID_MOD_VELOCITY as libc::c_int,
        FLUID_MOD_GC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_NEGATIVE as libc::c_int,
    );
    fluid_mod_set_source2(
        &mut default_vel2filter_mod,
        FLUID_MOD_VELOCITY as libc::c_int,
        FLUID_MOD_GC as libc::c_int
            | FLUID_MOD_SWITCH as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_dest(&mut default_vel2filter_mod, GEN_FILTERFC as libc::c_int);
    fluid_mod_set_amount(
        &mut default_vel2filter_mod,
        -(2400 as libc::c_int) as libc::c_double,
    );
    fluid_mod_set_source1(
        &mut default_at2viblfo_mod,
        FLUID_MOD_CHANNELPRESSURE as libc::c_int,
        FLUID_MOD_GC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_source2(
        &mut default_at2viblfo_mod,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    fluid_mod_set_dest(&mut default_at2viblfo_mod, GEN_VIBLFOTOPITCH as libc::c_int);
    fluid_mod_set_amount(
        &mut default_at2viblfo_mod,
        50 as libc::c_int as libc::c_double,
    );
    fluid_mod_set_source1(
        &mut default_mod2viblfo_mod,
        1 as libc::c_int,
        FLUID_MOD_CC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_source2(
        &mut default_mod2viblfo_mod,
        0 as libc::c_int,
        0 as libc::c_int,
    );
    fluid_mod_set_dest(
        &mut default_mod2viblfo_mod,
        GEN_VIBLFOTOPITCH as libc::c_int,
    );
    fluid_mod_set_amount(
        &mut default_mod2viblfo_mod,
        50 as libc::c_int as libc::c_double,
    );
    fluid_mod_set_source1(
        &mut default_att_mod,
        7 as libc::c_int,
        FLUID_MOD_CC as libc::c_int
            | FLUID_MOD_CONCAVE as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_NEGATIVE as libc::c_int,
    );
    fluid_mod_set_source2(&mut default_att_mod, 0 as libc::c_int, 0 as libc::c_int);
    fluid_mod_set_dest(&mut default_att_mod, GEN_ATTENUATION as libc::c_int);
    fluid_mod_set_amount(&mut default_att_mod, 960.0f64);
    fluid_mod_set_source1(
        &mut default_pan_mod,
        10 as libc::c_int,
        FLUID_MOD_CC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_BIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_source2(&mut default_pan_mod, 0 as libc::c_int, 0 as libc::c_int);
    fluid_mod_set_dest(&mut default_pan_mod, GEN_PAN as libc::c_int);
    fluid_mod_set_amount(&mut default_pan_mod, 500.0f64);
    fluid_mod_set_source1(
        &mut default_expr_mod,
        11 as libc::c_int,
        FLUID_MOD_CC as libc::c_int
            | FLUID_MOD_CONCAVE as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_NEGATIVE as libc::c_int,
    );
    fluid_mod_set_source2(&mut default_expr_mod, 0 as libc::c_int, 0 as libc::c_int);
    fluid_mod_set_dest(&mut default_expr_mod, GEN_ATTENUATION as libc::c_int);
    fluid_mod_set_amount(&mut default_expr_mod, 960.0f64);
    fluid_mod_set_source1(
        &mut default_reverb_mod,
        91 as libc::c_int,
        FLUID_MOD_CC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_source2(&mut default_reverb_mod, 0 as libc::c_int, 0 as libc::c_int);
    fluid_mod_set_dest(&mut default_reverb_mod, GEN_REVERBSEND as libc::c_int);
    fluid_mod_set_amount(
        &mut default_reverb_mod,
        200 as libc::c_int as libc::c_double,
    );
    fluid_mod_set_source1(
        &mut default_chorus_mod,
        93 as libc::c_int,
        FLUID_MOD_CC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_source2(&mut default_chorus_mod, 0 as libc::c_int, 0 as libc::c_int);
    fluid_mod_set_dest(&mut default_chorus_mod, GEN_CHORUSSEND as libc::c_int);
    fluid_mod_set_amount(
        &mut default_chorus_mod,
        200 as libc::c_int as libc::c_double,
    );
    fluid_mod_set_source1(
        &mut default_pitch_bend_mod,
        FLUID_MOD_PITCHWHEEL as libc::c_int,
        FLUID_MOD_GC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_BIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_source2(
        &mut default_pitch_bend_mod,
        FLUID_MOD_PITCHWHEELSENS as libc::c_int,
        FLUID_MOD_GC as libc::c_int
            | FLUID_MOD_LINEAR as libc::c_int
            | FLUID_MOD_UNIPOLAR as libc::c_int
            | FLUID_MOD_POSITIVE as libc::c_int,
    );
    fluid_mod_set_dest(&mut default_pitch_bend_mod, GEN_PITCH as libc::c_int);
    fluid_mod_set_amount(&mut default_pitch_bend_mod, 12700.0f64);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_verify_settings(
    _settings: *mut fluid_settings_t,
) -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_synth(
    mut settings: *mut fluid_settings_t,
) -> *mut fluid_synth_t {
    let mut current_block: u64;
    let mut i: libc::c_int = 0;
    let mut synth;
    let mut loader;
    if fluid_synth_initialized == 0 as libc::c_int {
        fluid_synth_init();
    }
    fluid_synth_verify_settings(settings);
    synth =
        libc::malloc(::std::mem::size_of::<fluid_synth_t>() as libc::size_t) as *mut fluid_synth_t;
    if synth.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_synth_t;
    }
    libc::memset(
        synth as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<fluid_synth_t>() as libc::size_t,
    );
    (*synth).settings = settings;
    (*synth).with_reverb = fluid_settings_str_equal(
        settings,
        b"synth.reverb.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_char;
    (*synth).with_chorus = fluid_settings_str_equal(
        settings,
        b"synth.chorus.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_char;
    (*synth).verbose = fluid_settings_str_equal(
        settings,
        b"synth.verbose\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_char;
    (*synth).dump = fluid_settings_str_equal(
        settings,
        b"synth.dump\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
    ) as libc::c_char;
    fluid_settings_getint(
        settings,
        b"synth.polyphony\x00" as *const u8 as *const libc::c_char,
        &mut (*synth).polyphony,
    );
    fluid_settings_getnum(
        settings,
        b"synth.sample-rate\x00" as *const u8 as *const libc::c_char,
        &mut (*synth).sample_rate,
    );
    fluid_settings_getint(
        settings,
        b"synth.midi-channels\x00" as *const u8 as *const libc::c_char,
        &mut (*synth).midi_channels,
    );
    fluid_settings_getint(
        settings,
        b"synth.audio-channels\x00" as *const u8 as *const libc::c_char,
        &mut (*synth).audio_channels,
    );
    fluid_settings_getint(
        settings,
        b"synth.audio-groups\x00" as *const u8 as *const libc::c_char,
        &mut (*synth).audio_groups,
    );
    fluid_settings_getint(
        settings,
        b"synth.effects-channels\x00" as *const u8 as *const libc::c_char,
        &mut (*synth).effects_channels,
    );
    fluid_settings_getnum(
        settings,
        b"synth.gain\x00" as *const u8 as *const libc::c_char,
        &mut (*synth).gain,
    );
    fluid_settings_getint(
        settings,
        b"synth.min-note-length\x00" as *const u8 as *const libc::c_char,
        &mut i,
    );
    (*synth).min_note_length_ticks =
        (i as libc::c_double * (*synth).sample_rate / 1000.0f32 as libc::c_double) as libc::c_uint;
    fluid_settings_register_num(
        settings,
        b"synth.gain\x00" as *const u8 as *const libc::c_char,
        0.2f32 as libc::c_double,
        0.0f32 as libc::c_double,
        10.0f32 as libc::c_double,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut fluid_synth_t,
                    _: *mut libc::c_char,
                    _: libc::c_double,
                ) -> libc::c_int,
            >,
            fluid_num_update_t,
        >(Some(
            fluid_synth_update_gain
                as unsafe extern "C" fn(
                    _: *mut fluid_synth_t,
                    _: *mut libc::c_char,
                    _: libc::c_double,
                ) -> libc::c_int,
        )),
        synth as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.polyphony\x00" as *const u8 as *const libc::c_char,
        (*synth).polyphony,
        16 as libc::c_int,
        4096 as libc::c_int,
        0 as libc::c_int,
        ::std::mem::transmute::<
            Option<
                unsafe extern "C" fn(
                    _: *mut fluid_synth_t,
                    _: *mut libc::c_char,
                    _: libc::c_int,
                ) -> libc::c_int,
            >,
            fluid_int_update_t,
        >(Some(
            fluid_synth_update_polyphony
                as unsafe extern "C" fn(
                    _: *mut fluid_synth_t,
                    _: *mut libc::c_char,
                    _: libc::c_int,
                ) -> libc::c_int,
        )),
        synth as *mut libc::c_void,
    );
    if (*synth).midi_channels % 16 as libc::c_int != 0 as libc::c_int {
        let mut n: libc::c_int = (*synth).midi_channels / 16 as libc::c_int;
        (*synth).midi_channels = (n + 1 as libc::c_int) * 16 as libc::c_int;
        fluid_settings_setint(
            settings,
            b"synth.midi-channels\x00" as *const u8 as *const libc::c_char,
            (*synth).midi_channels,
        );
        fluid_log!(FLUID_WARN,
                  "Requested number of MIDI channels is not a multiple of 16. I\'ll increase the number of channels to the next multiple.",
                );
    }
    if (*synth).audio_channels < 1 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "Requested number of audio channels is smaller than 1. Changing this setting to 1.",
        );
        (*synth).audio_channels = 1 as libc::c_int
    } else if (*synth).audio_channels > 128 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "Requested number of audio channels is too big ({}). Limiting this setting to 128.",
            (*synth).audio_channels
        );
        (*synth).audio_channels = 128 as libc::c_int
    }
    if (*synth).audio_groups < 1 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "Requested number of audio groups is smaller than 1. Changing this setting to 1.",
        );
        (*synth).audio_groups = 1 as libc::c_int
    } else if (*synth).audio_groups > 128 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "Requested number of audio groups is too big ({}). Limiting this setting to 128.",
            (*synth).audio_groups
        );
        (*synth).audio_groups = 128 as libc::c_int
    }
    if (*synth).effects_channels != 2 as libc::c_int {
        fluid_log!(
            FLUID_WARN,
            "Invalid number of effects channels ({}).Setting effects channels to 2.",
            (*synth).effects_channels
        );
        (*synth).effects_channels = 2 as libc::c_int
    }
    (*synth).nbuf = (*synth).audio_channels;
    if (*synth).audio_groups > (*synth).nbuf {
        (*synth).nbuf = (*synth).audio_groups
    }
    (*synth).state = FLUID_SYNTH_PLAYING as libc::c_int as libc::c_uint;
    (*synth).sfont = 0 as *mut fluid_list_t;
    (*synth).noteid = 0 as libc::c_int as libc::c_uint;
    (*synth).ticks = 0 as libc::c_int as libc::c_uint;
    (*synth).tuning = 0 as *mut *mut *mut fluid_tuning_t;
    loader = new_fluid_defsfloader();
    if loader.is_null() {
        fluid_log!(FLUID_WARN, "Failed to create the default SoundFont loader",);
    } else {
        fluid_synth_add_sfloader(synth, loader);
    }
    (*synth).channel = libc::malloc(
        ((*synth).midi_channels as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<*mut fluid_channel_t>() as libc::size_t),
    ) as *mut *mut fluid_channel_t;
    if (*synth).channel.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
    } else {
        i = 0 as libc::c_int;
        loop {
            if !(i < (*synth).midi_channels) {
                current_block = 13660591889533726445;
                break;
            }
            let ref mut fresh0 = *(*synth).channel.offset(i as isize);
            *fresh0 = new_fluid_channel(synth, i);
            if (*(*synth).channel.offset(i as isize)).is_null() {
                current_block = 2776114520721993823;
                break;
            }
            i += 1
        }
        match current_block {
            2776114520721993823 => {}
            _ => {
                (*synth).nvoice = (*synth).polyphony;
                (*synth).voice = libc::malloc(
                    ((*synth).nvoice as libc::size_t)
                        .wrapping_mul(::std::mem::size_of::<*mut fluid_voice_t>() as libc::size_t),
                ) as *mut *mut fluid_voice_t;
                if !(*synth).voice.is_null() {
                    i = 0 as libc::c_int;
                    loop {
                        if !(i < (*synth).nvoice) {
                            current_block = 17441561948628420366;
                            break;
                        }
                        let ref mut fresh1 = *(*synth).voice.offset(i as isize);
                        *fresh1 = new_fluid_voice((*synth).sample_rate as fluid_real_t);
                        if (*(*synth).voice.offset(i as isize)).is_null() {
                            current_block = 2776114520721993823;
                            break;
                        }
                        i += 1
                    }
                    match current_block {
                        2776114520721993823 => {}
                        _ => {
                            (*synth).left_buf = 0 as *mut *mut fluid_real_t;
                            (*synth).right_buf = 0 as *mut *mut fluid_real_t;
                            (*synth).fx_left_buf = 0 as *mut *mut fluid_real_t;
                            (*synth).fx_right_buf = 0 as *mut *mut fluid_real_t;
                            (*synth).left_buf =
                                libc::malloc(((*synth).nbuf as libc::size_t).wrapping_mul(
                                    ::std::mem::size_of::<*mut fluid_real_t>() as libc::size_t,
                                )) as *mut *mut fluid_real_t;
                            (*synth).right_buf =
                                libc::malloc(((*synth).nbuf as libc::size_t).wrapping_mul(
                                    ::std::mem::size_of::<*mut fluid_real_t>() as libc::size_t,
                                )) as *mut *mut fluid_real_t;
                            if (*synth).left_buf.is_null() || (*synth).right_buf.is_null() {
                                fluid_log!(FLUID_ERR, "Out of memory",);
                            } else {
                                libc::memset(
                                    (*synth).left_buf as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ((*synth).nbuf as libc::size_t)
                                        .wrapping_mul(::std::mem::size_of::<*mut fluid_real_t>()
                                            as libc::size_t),
                                );
                                libc::memset(
                                    (*synth).right_buf as *mut libc::c_void,
                                    0 as libc::c_int,
                                    ((*synth).nbuf as libc::size_t)
                                        .wrapping_mul(::std::mem::size_of::<*mut fluid_real_t>()
                                            as libc::size_t),
                                );
                                i = 0 as libc::c_int;
                                loop {
                                    if !(i < (*synth).nbuf) {
                                        current_block = 178030534879405462;
                                        break;
                                    }
                                    let ref mut fresh2 = *(*synth).left_buf.offset(i as isize);
                                    *fresh2 = libc::malloc(
                                        (64 as libc::c_int as libc::size_t)
                                            .wrapping_mul(::std::mem::size_of::<fluid_real_t>()
                                                as libc::size_t),
                                    )
                                        as *mut fluid_real_t;
                                    let ref mut fresh3 = *(*synth).right_buf.offset(i as isize);
                                    *fresh3 = libc::malloc(
                                        (64 as libc::c_int as libc::size_t)
                                            .wrapping_mul(::std::mem::size_of::<fluid_real_t>()
                                                as libc::size_t),
                                    )
                                        as *mut fluid_real_t;
                                    if (*(*synth).left_buf.offset(i as isize)).is_null()
                                        || (*(*synth).right_buf.offset(i as isize)).is_null()
                                    {
                                        fluid_log!(FLUID_ERR, "Out of memory",);
                                        current_block = 2776114520721993823;
                                        break;
                                    } else {
                                        i += 1
                                    }
                                }
                                match current_block {
                                    2776114520721993823 => {}
                                    _ => {
                                        (*synth).fx_left_buf = libc::malloc(
                                            ((*synth).effects_channels as libc::size_t)
                                                .wrapping_mul(
                                                    ::std::mem::size_of::<*mut fluid_real_t>()
                                                        as libc::size_t,
                                                ),
                                        )
                                            as *mut *mut fluid_real_t;
                                        (*synth).fx_right_buf = libc::malloc(
                                            ((*synth).effects_channels as libc::size_t)
                                                .wrapping_mul(
                                                    ::std::mem::size_of::<*mut fluid_real_t>()
                                                        as libc::size_t,
                                                ),
                                        )
                                            as *mut *mut fluid_real_t;
                                        if (*synth).fx_left_buf.is_null()
                                            || (*synth).fx_right_buf.is_null()
                                        {
                                            fluid_log!(FLUID_ERR, "Out of memory",);
                                        } else {
                                            libc::memset(
                                                (*synth).fx_left_buf as *mut libc::c_void,
                                                0 as libc::c_int,
                                                (2 as libc::c_int as libc::size_t).wrapping_mul(
                                                    ::std::mem::size_of::<*mut fluid_real_t>()
                                                        as libc::size_t,
                                                ),
                                            );
                                            libc::memset(
                                                (*synth).fx_right_buf as *mut libc::c_void,
                                                0 as libc::c_int,
                                                (2 as libc::c_int as libc::size_t).wrapping_mul(
                                                    ::std::mem::size_of::<*mut fluid_real_t>()
                                                        as libc::size_t,
                                                ),
                                            );
                                            i = 0 as libc::c_int;
                                            loop {
                                                if !(i < (*synth).effects_channels) {
                                                    current_block = 7739940392431776979;
                                                    break;
                                                }
                                                let ref mut fresh4 =
                                                    *(*synth).fx_left_buf.offset(i as isize);
                                                *fresh4 = libc::malloc(
                                                    (64 as libc::c_int as libc::size_t)
                                                        .wrapping_mul(::std::mem::size_of::<
                                                            fluid_real_t,
                                                        >(
                                                        )
                                                            as libc::size_t),
                                                )
                                                    as *mut fluid_real_t;
                                                let ref mut fresh5 =
                                                    *(*synth).fx_right_buf.offset(i as isize);
                                                *fresh5 = libc::malloc(
                                                    (64 as libc::c_int as libc::size_t)
                                                        .wrapping_mul(::std::mem::size_of::<
                                                            fluid_real_t,
                                                        >(
                                                        )
                                                            as libc::size_t),
                                                )
                                                    as *mut fluid_real_t;
                                                if (*(*synth).fx_left_buf.offset(i as isize))
                                                    .is_null()
                                                    || (*(*synth).fx_right_buf.offset(i as isize))
                                                        .is_null()
                                                {
                                                    fluid_log!(FLUID_ERR, "Out of memory",);
                                                    current_block = 2776114520721993823;
                                                    break;
                                                } else {
                                                    i += 1
                                                }
                                            }
                                            match current_block {
                                                2776114520721993823 => {}
                                                _ => {
                                                    (*synth).cur = 64 as libc::c_int;
                                                    (*synth).dither_index = 0 as libc::c_int;
                                                    (*synth).reverb = new_fluid_revmodel();
                                                    if (*synth).reverb.is_null() {
                                                        fluid_log!(FLUID_ERR, "Out of memory",);
                                                    } else {
                                                        fluid_synth_set_reverb(
                                                            synth,
                                                            0.2f32 as libc::c_double,
                                                            0.0f32 as libc::c_double,
                                                            0.5f32 as libc::c_double,
                                                            0.9f32 as libc::c_double,
                                                        );
                                                        (*synth).chorus = new_fluid_chorus(
                                                            (*synth).sample_rate as fluid_real_t,
                                                        );
                                                        if (*synth).chorus.is_null() {
                                                            fluid_log!(FLUID_ERR, "Out of memory",);
                                                        } else {
                                                            if fluid_settings_str_equal(
                                                                settings,
                                                                b"synth.drums-channel.active\x00"
                                                                    as *const u8
                                                                    as *const libc::c_char,
                                                                b"yes\x00" as *const u8
                                                                    as *const libc::c_char
                                                                    as *mut libc::c_char,
                                                            ) != 0
                                                            {
                                                                fluid_synth_bank_select(
                                                                    synth,
                                                                    9 as libc::c_int,
                                                                    128 as libc::c_int
                                                                        as libc::c_uint,
                                                                );
                                                            }
                                                            return synth;
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    delete_fluid_synth(synth);
    return 0 as *mut fluid_synth_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_sample_rate(
    mut synth: *mut fluid_synth_t,
    sample_rate: libc::c_float,
) {
    (*synth).sample_rate = sample_rate as f64;

    let mut i;
    i = 0 as libc::c_int;
    while i < (*synth).nvoice {
        delete_fluid_voice(*(*synth).voice.offset(i as isize));
        let ref mut fresh6 = *(*synth).voice.offset(i as isize);
        *fresh6 = new_fluid_voice((*synth).sample_rate as fluid_real_t);
        i += 1
    }
    delete_fluid_chorus((*synth).chorus);
    (*synth).chorus = new_fluid_chorus((*synth).sample_rate as fluid_real_t);
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_synth(mut synth: *mut fluid_synth_t) -> libc::c_int {
    let mut i;
    let mut k;
    let mut list;
    let mut sfont;
    let mut bank_offset;
    let mut loader;
    if synth.is_null() {
        return FLUID_OK as libc::c_int;
    }
    (*synth).state = FLUID_SYNTH_STOPPED as libc::c_int as libc::c_uint;
    if !(*synth).voice.is_null() {
        i = 0 as libc::c_int;
        while i < (*synth).nvoice {
            if !(*(*synth).voice.offset(i as isize)).is_null()
                && fluid_voice_is_playing(*(*synth).voice.offset(i as isize)) != 0
            {
                fluid_voice_off(*(*synth).voice.offset(i as isize));
            }
            i += 1
        }
    }
    list = (*synth).sfont;
    while !list.is_null() {
        sfont = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sfont_t;
        if !sfont.is_null() && (*sfont).free.is_some() {
            Some((*sfont).free.expect("non-null function pointer"))
                .expect("non-null function pointer")(sfont);
        } else {
        };
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    delete_fluid_list((*synth).sfont);
    list = (*synth).bank_offsets;
    while !list.is_null() {
        bank_offset = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_bank_offset_t;
        libc::free(bank_offset as *mut libc::c_void);
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    delete_fluid_list((*synth).bank_offsets);
    list = (*synth).loaders;
    while !list.is_null() {
        loader = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sfloader_t;
        if !loader.is_null() {
            if !(*loader).fileapi.is_null() && (*(*loader).fileapi).free.is_some() {
                Some(
                    (*(*loader).fileapi)
                        .free
                        .expect("non-null function pointer"),
                )
                .expect("non-null function pointer")((*loader).fileapi);
            }
            if (*loader).free.is_some() {
                Some((*loader).free.expect("non-null function pointer"))
                    .expect("non-null function pointer")(loader);
            }
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    delete_fluid_list((*synth).loaders);
    if !(*synth).channel.is_null() {
        i = 0 as libc::c_int;
        while i < (*synth).midi_channels {
            if !(*(*synth).channel.offset(i as isize)).is_null() {
                delete_fluid_channel(*(*synth).channel.offset(i as isize));
            }
            i += 1
        }
        libc::free((*synth).channel as *mut libc::c_void);
    }
    if !(*synth).voice.is_null() {
        i = 0 as libc::c_int;
        while i < (*synth).nvoice {
            if !(*(*synth).voice.offset(i as isize)).is_null() {
                delete_fluid_voice(*(*synth).voice.offset(i as isize));
            }
            i += 1
        }
        libc::free((*synth).voice as *mut libc::c_void);
    }
    if !(*synth).left_buf.is_null() {
        i = 0 as libc::c_int;
        while i < (*synth).nbuf {
            if !(*(*synth).left_buf.offset(i as isize)).is_null() {
                libc::free(*(*synth).left_buf.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        libc::free((*synth).left_buf as *mut libc::c_void);
    }
    if !(*synth).right_buf.is_null() {
        i = 0 as libc::c_int;
        while i < (*synth).nbuf {
            if !(*(*synth).right_buf.offset(i as isize)).is_null() {
                libc::free(*(*synth).right_buf.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        libc::free((*synth).right_buf as *mut libc::c_void);
    }
    if !(*synth).fx_left_buf.is_null() {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if !(*(*synth).fx_left_buf.offset(i as isize)).is_null() {
                libc::free(*(*synth).fx_left_buf.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        libc::free((*synth).fx_left_buf as *mut libc::c_void);
    }
    if !(*synth).fx_right_buf.is_null() {
        i = 0 as libc::c_int;
        while i < 2 as libc::c_int {
            if !(*(*synth).fx_right_buf.offset(i as isize)).is_null() {
                libc::free(*(*synth).fx_right_buf.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        libc::free((*synth).fx_right_buf as *mut libc::c_void);
    }
    if !(*synth).reverb.is_null() {
        delete_fluid_revmodel((*synth).reverb);
    }
    if !(*synth).chorus.is_null() {
        delete_fluid_chorus((*synth).chorus);
    }
    if !(*synth).tuning.is_null() {
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            if !(*(*synth).tuning.offset(i as isize)).is_null() {
                k = 0 as libc::c_int;
                while k < 128 as libc::c_int {
                    if !(*(*(*synth).tuning.offset(i as isize)).offset(k as isize)).is_null() {
                        libc::free(*(*(*synth).tuning.offset(i as isize)).offset(k as isize)
                            as *mut libc::c_void);
                    }
                    k += 1
                }
                libc::free(*(*synth).tuning.offset(i as isize) as *mut libc::c_void);
            }
            i += 1
        }
        libc::free((*synth).tuning as *mut libc::c_void);
    }
    libc::free(synth as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_error(_synth: *mut fluid_synth_t) -> *mut libc::c_char {
    return fluid_error();
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_noteon(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut key: libc::c_int,
    mut vel: libc::c_int,
) -> libc::c_int {
    let mut channel;
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if vel == 0 as libc::c_int {
        return fluid_synth_noteoff(synth, chan, key);
    }
    channel = *(*synth).channel.offset(chan as isize);
    if (*channel).preset.is_null() {
        if (*synth).verbose != 0 {
            fluid_log!(
                FLUID_INFO,
                "noteon\t{}\t{}\t{}\t{}\t{}\t\t{}\t{}\t{}",
                chan,
                key,
                vel,
                0,
                ((*synth).ticks as libc::c_float / 44100.0f32),
                0.0f32,
                0,
                "channel has no preset"
            );
        }
        return FLUID_FAILED as libc::c_int;
    }
    fluid_synth_release_voice_on_same_note(synth, chan, key);
    let fresh7 = (*synth).noteid;
    (*synth).noteid = (*synth).noteid.wrapping_add(1);
    return fluid_synth_start(
        synth,
        fresh7,
        (*channel).preset,
        0 as libc::c_int,
        chan,
        key,
        vel,
    );
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_noteoff(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut key: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut voice;
    let mut status: libc::c_int = FLUID_FAILED as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as libc::c_int
            && (*voice).chan as libc::c_int == chan
            && (*voice).key as libc::c_int == key
        {
            if (*synth).verbose != 0 {
                let mut used_voices: libc::c_int = 0 as libc::c_int;
                let mut k;
                k = 0 as libc::c_int;
                while k < (*synth).polyphony {
                    if !((**(*synth).voice.offset(k as isize)).status as libc::c_int
                        == FLUID_VOICE_CLEAN as libc::c_int
                        || (**(*synth).voice.offset(k as isize)).status as libc::c_int
                            == FLUID_VOICE_OFF as libc::c_int)
                    {
                        used_voices += 1
                    }
                    k += 1
                }
                fluid_log!(
                    FLUID_INFO,
                    "noteoff\t{}\t{}\t{}\t{}\t{}\t\t{}\t{}",
                    (*voice).chan,
                    (*voice).key,
                    0 as libc::c_int,
                    (*voice).id,
                    ((*voice).start_time.wrapping_add((*voice).ticks) as libc::c_float / 44100.0f32)
                        as libc::c_double,
                    ((*voice).ticks as libc::c_float / 44100.0f32) as libc::c_double,
                    used_voices
                );
            }
            fluid_voice_noteoff(voice);
            status = FLUID_OK as libc::c_int
        }
        i += 1
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_damp_voices(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut voice;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as libc::c_int == chan
            && (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int
        {
            fluid_voice_noteoff(voice);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_cc(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut num: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if num < 0 as libc::c_int || num >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Ctrl out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if val < 0 as libc::c_int || val >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Value out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "cc\t{}\t{}\t{}", chan, num, val);
    }
    fluid_channel_cc(*(*synth).channel.offset(chan as isize), num, val);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_cc(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut num: libc::c_int,
    mut pval: *mut libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if num < 0 as libc::c_int || num >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Ctrl out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    *pval = (**(*synth).channel.offset(chan as isize)).cc[num as usize] as libc::c_int;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_sysex(
    mut synth: *mut fluid_synth_t,
    mut data: *const libc::c_char,
    mut len: libc::c_int,
    mut response: *mut libc::c_char,
    mut response_len: *mut libc::c_int,
    mut handled: *mut libc::c_int,
    mut dryrun: libc::c_int,
) -> libc::c_int {
    let mut avail_response: libc::c_int = 0 as libc::c_int;
    if !handled.is_null() {
        *handled = 0 as libc::c_int
    }
    if !response_len.is_null() {
        avail_response = *response_len;
        *response_len = 0 as libc::c_int
    }
    if synth.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if data.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if !(len > 0 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    if !(response.is_null() || !response_len.is_null()) {
        return FLUID_FAILED as libc::c_int;
    }
    if len < 4 as libc::c_int {
        return FLUID_OK as libc::c_int;
    }
    if (*data.offset(0 as libc::c_int as isize) as libc::c_int
        == MIDI_SYSEX_UNIV_NON_REALTIME as libc::c_int
        || *data.offset(0 as libc::c_int as isize) as libc::c_int
            == MIDI_SYSEX_UNIV_REALTIME as libc::c_int)
        && *data.offset(2 as libc::c_int as isize) as libc::c_int == 0x8 as libc::c_int
    {
        let mut result;
        result = fluid_synth_sysex_midi_tuning(
            synth,
            data,
            len,
            response,
            response_len,
            avail_response,
            handled,
            dryrun,
        );
        return result;
    }
    return FLUID_OK as libc::c_int;
}
unsafe extern "C" fn fluid_synth_sysex_midi_tuning(
    mut synth: *mut fluid_synth_t,
    mut data: *const libc::c_char,
    mut len: libc::c_int,
    mut response: *mut libc::c_char,
    mut response_len: *mut libc::c_int,
    mut avail_response: libc::c_int,
    mut handled: *mut libc::c_int,
    mut dryrun: libc::c_int,
) -> libc::c_int {
    let mut realtime;
    let mut msgid;
    let mut bank: libc::c_int = 0 as libc::c_int;
    let mut prog;
    let mut channels;
    let mut tunedata: [libc::c_double; 128] = [0.; 128];
    let mut keys: [libc::c_int; 128] = [0; 128];
    let mut name: [libc::c_char; 17] = [0; 17];
    let mut note;
    let mut frac;
    let mut frac2;
    let mut chksum;
    let mut i;
    let mut count;
    let mut index;
    let mut dataptr;
    let mut resptr;
    realtime = (*data.offset(0 as libc::c_int as isize) as libc::c_int
        == MIDI_SYSEX_UNIV_REALTIME as libc::c_int) as libc::c_int;
    msgid = *data.offset(3 as libc::c_int as isize) as libc::c_int;
    match msgid {
        0 | 3 => {
            if *data.offset(3 as libc::c_int as isize) as libc::c_int
                == MIDI_SYSEX_TUNING_BULK_DUMP_REQ as libc::c_int
            {
                if len != 5 as libc::c_int
                    || *data.offset(4 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || response.is_null()
                {
                    return FLUID_OK as libc::c_int;
                }
                *response_len = 406 as libc::c_int;
                prog = *data.offset(4 as libc::c_int as isize) as libc::c_int
            } else {
                if len != 6 as libc::c_int
                    || *data.offset(4 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || *data.offset(5 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || response.is_null()
                {
                    return FLUID_OK as libc::c_int;
                }
                *response_len = 407 as libc::c_int;
                bank = *data.offset(4 as libc::c_int as isize) as libc::c_int;
                prog = *data.offset(5 as libc::c_int as isize) as libc::c_int
            }
            if dryrun != 0 {
                if !handled.is_null() {
                    *handled = 1 as libc::c_int
                }
                return FLUID_OK as libc::c_int;
            }
            if avail_response < *response_len {
                return FLUID_FAILED as libc::c_int;
            }
            if fluid_synth_tuning_dump(
                synth,
                bank,
                prog,
                name.as_mut_ptr(),
                17 as libc::c_int,
                tunedata.as_mut_ptr(),
            ) == FLUID_FAILED as libc::c_int
            {
                *response_len = 0 as libc::c_int;
                return FLUID_OK as libc::c_int;
            }
            resptr = response;
            let fresh8 = resptr;
            resptr = resptr.offset(1);
            *fresh8 = MIDI_SYSEX_UNIV_NON_REALTIME as libc::c_int as libc::c_char;
            let fresh9 = resptr;
            resptr = resptr.offset(1);
            *fresh9 = 0 as libc::c_int as libc::c_char;
            let fresh10 = resptr;
            resptr = resptr.offset(1);
            *fresh10 = 0x8 as libc::c_int as libc::c_char;
            let fresh11 = resptr;
            resptr = resptr.offset(1);
            *fresh11 = MIDI_SYSEX_TUNING_BULK_DUMP as libc::c_int as libc::c_char;
            if msgid == MIDI_SYSEX_TUNING_BULK_DUMP_REQ_BANK as libc::c_int {
                let fresh12 = resptr;
                resptr = resptr.offset(1);
                *fresh12 = bank as libc::c_char
            }
            let fresh13 = resptr;
            resptr = resptr.offset(1);
            *fresh13 = prog as libc::c_char;
            libc::strncpy(resptr, name.as_mut_ptr(), 16 as libc::c_int as libc::size_t);
            resptr = resptr.offset(16 as libc::c_int as isize);
            i = 0 as libc::c_int;
            while i < 128 as libc::c_int {
                note = (tunedata[i as usize] / 100.0f64) as libc::c_int;
                note = if note < 0 as libc::c_int {
                    0 as libc::c_int
                } else if note > 127 as libc::c_int {
                    127 as libc::c_int
                } else {
                    note
                };
                frac = (((tunedata[i as usize] - note as libc::c_double * 100.0f64) * 16384.0f64
                    + 50.0f64)
                    / 100.0f64) as libc::c_int;
                frac = if frac < 0 as libc::c_int {
                    0 as libc::c_int
                } else if frac > 16383 as libc::c_int {
                    16383 as libc::c_int
                } else {
                    frac
                };
                let fresh14 = resptr;
                resptr = resptr.offset(1);
                *fresh14 = note as libc::c_char;
                let fresh15 = resptr;
                resptr = resptr.offset(1);
                *fresh15 = (frac >> 7 as libc::c_int) as libc::c_char;
                let fresh16 = resptr;
                resptr = resptr.offset(1);
                *fresh16 = (frac & 0x7f as libc::c_int) as libc::c_char;
                i += 1
            }
            if msgid == MIDI_SYSEX_TUNING_BULK_DUMP_REQ as libc::c_int {
                chksum = (MIDI_SYSEX_UNIV_NON_REALTIME as libc::c_int
                    ^ 0x8 as libc::c_int
                    ^ MIDI_SYSEX_TUNING_BULK_DUMP as libc::c_int
                    ^ prog) as uint8;
                i = 21 as libc::c_int;
                while i < 128 as libc::c_int * 3 as libc::c_int + 21 as libc::c_int {
                    chksum = (chksum as libc::c_int ^ *response.offset(i as isize) as libc::c_int)
                        as uint8;
                    i += 1
                }
            } else {
                i = 1 as libc::c_int;
                chksum = 0 as libc::c_int as uint8;
                while i < 406 as libc::c_int {
                    chksum = (chksum as libc::c_int ^ *response.offset(i as isize) as libc::c_int)
                        as uint8;
                    i += 1
                }
            }
            let fresh17 = resptr;
            // resptr = resptr.offset(1);
            *fresh17 = (chksum as libc::c_int & 0x7f as libc::c_int) as libc::c_char;
            if !handled.is_null() {
                *handled = 1 as libc::c_int
            }
        }
        2 | 7 => {
            dataptr = data.offset(4 as libc::c_int as isize);
            if msgid == MIDI_SYSEX_TUNING_NOTE_TUNE as libc::c_int {
                if len < 10 as libc::c_int
                    || *data.offset(4 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || *data.offset(5 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || len
                        != *data.offset(5 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int
                            + 6 as libc::c_int
                {
                    return FLUID_OK as libc::c_int;
                }
            } else {
                if len < 11 as libc::c_int
                    || *data.offset(4 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || *data.offset(5 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || *data.offset(6 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int
                        != 0
                    || len
                        != *data.offset(5 as libc::c_int as isize) as libc::c_int * 4 as libc::c_int
                            + 7 as libc::c_int
                {
                    return FLUID_OK as libc::c_int;
                }
                let fresh18 = dataptr;
                dataptr = dataptr.offset(1);
                bank = *fresh18 as libc::c_int
            }
            if dryrun != 0 {
                if !handled.is_null() {
                    *handled = 1 as libc::c_int
                }
                return FLUID_OK as libc::c_int;
            }
            let fresh19 = dataptr;
            dataptr = dataptr.offset(1);
            prog = *fresh19 as libc::c_int;
            let fresh20 = dataptr;
            dataptr = dataptr.offset(1);
            count = *fresh20 as libc::c_int;
            i = 0 as libc::c_int;
            index = 0 as libc::c_int;
            while i < count {
                let fresh21 = dataptr;
                dataptr = dataptr.offset(1);
                note = *fresh21 as libc::c_int;
                if note & 0x80 as libc::c_int != 0 {
                    return FLUID_OK as libc::c_int;
                }
                keys[index as usize] = note;
                let fresh22 = dataptr;
                dataptr = dataptr.offset(1);
                note = *fresh22 as libc::c_int;
                let fresh23 = dataptr;
                dataptr = dataptr.offset(1);
                frac = *fresh23 as libc::c_int;
                let fresh24 = dataptr;
                dataptr = dataptr.offset(1);
                frac2 = *fresh24 as libc::c_int;
                if note & 0x80 as libc::c_int != 0
                    || frac & 0x80 as libc::c_int != 0
                    || frac2 & 0x80 as libc::c_int != 0
                {
                    return FLUID_OK as libc::c_int;
                }
                frac = frac << 7 as libc::c_int | frac2;
                if !(note == 0x7f as libc::c_int && frac == 16383 as libc::c_int) {
                    tunedata[index as usize] = note as libc::c_double * 100.0f64
                        + frac as libc::c_double * 100.0f64 / 16384.0f64;
                    index += 1
                }
                i += 1
            }
            if index > 0 as libc::c_int {
                if fluid_synth_tune_notes(
                    synth,
                    bank,
                    prog,
                    index,
                    keys.as_mut_ptr(),
                    tunedata.as_mut_ptr(),
                    realtime,
                ) == FLUID_FAILED as libc::c_int
                {
                    return FLUID_FAILED as libc::c_int;
                }
            }
            if !handled.is_null() {
                *handled = 1 as libc::c_int
            }
        }
        8 | 9 => {
            if msgid == MIDI_SYSEX_TUNING_OCTAVE_TUNE_1BYTE as libc::c_int
                && len != 19 as libc::c_int
                || msgid == MIDI_SYSEX_TUNING_OCTAVE_TUNE_2BYTE as libc::c_int
                    && len != 31 as libc::c_int
            {
                return FLUID_OK as libc::c_int;
            }
            if *data.offset(4 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0
                || *data.offset(5 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0
                || *data.offset(6 as libc::c_int as isize) as libc::c_int & 0x80 as libc::c_int != 0
            {
                return FLUID_OK as libc::c_int;
            }
            if dryrun != 0 {
                if !handled.is_null() {
                    *handled = 1 as libc::c_int
                }
                return FLUID_OK as libc::c_int;
            }
            channels = (*data.offset(4 as libc::c_int as isize) as libc::c_int
                & 0x3 as libc::c_int)
                << 14 as libc::c_int
                | (*data.offset(5 as libc::c_int as isize) as libc::c_int) << 7 as libc::c_int
                | *data.offset(6 as libc::c_int as isize) as libc::c_int;
            if msgid == MIDI_SYSEX_TUNING_OCTAVE_TUNE_1BYTE as libc::c_int {
                i = 0 as libc::c_int;
                while i < 12 as libc::c_int {
                    frac = *data.offset((i + 7 as libc::c_int) as isize) as libc::c_int;
                    if frac & 0x80 as libc::c_int != 0 {
                        return FLUID_OK as libc::c_int;
                    }
                    tunedata[i as usize] = (frac - 64 as libc::c_int) as libc::c_double;
                    i += 1
                }
            } else {
                i = 0 as libc::c_int;
                while i < 12 as libc::c_int {
                    frac = *data.offset((i * 2 as libc::c_int + 7 as libc::c_int) as isize)
                        as libc::c_int;
                    frac2 = *data.offset((i * 2 as libc::c_int + 8 as libc::c_int) as isize)
                        as libc::c_int;
                    if frac & 0x80 as libc::c_int != 0 || frac2 & 0x80 as libc::c_int != 0 {
                        return FLUID_OK as libc::c_int;
                    }
                    tunedata[i as usize] = ((frac << 7 as libc::c_int | frac2)
                        - 8192 as libc::c_int)
                        as libc::c_double
                        * (200.0f64 / 16384.0f64);
                    i += 1
                }
            }
            if fluid_synth_activate_octave_tuning(
                synth,
                0 as libc::c_int,
                0 as libc::c_int,
                b"SYSEX\x00" as *const u8 as *const libc::c_char,
                tunedata.as_mut_ptr(),
                realtime,
            ) == FLUID_FAILED as libc::c_int
            {
                return FLUID_FAILED as libc::c_int;
            }
            if channels != 0 {
                i = 0 as libc::c_int;
                while i < 16 as libc::c_int {
                    if channels & (1 as libc::c_int) << i != 0 {
                        fluid_synth_activate_tuning(
                            synth,
                            i,
                            0 as libc::c_int,
                            0 as libc::c_int,
                            realtime,
                        );
                    }
                    i += 1
                }
            }
            if !handled.is_null() {
                *handled = 1 as libc::c_int
            }
        }
        _ => {}
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_all_notes_off(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut voice;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if ((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
            && (*voice).chan as libc::c_int == chan
        {
            fluid_voice_noteoff(voice);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_all_sounds_off(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut voice;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if ((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
            && (*voice).chan as libc::c_int == chan
        {
            fluid_voice_off(voice);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_system_reset(mut synth: *mut fluid_synth_t) -> libc::c_int {
    let mut i;
    let mut voice;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int
        {
            fluid_voice_off(voice);
        }
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*synth).midi_channels {
        fluid_channel_reset(*(*synth).channel.offset(i as isize));
        i += 1
    }
    fluid_chorus_reset((*synth).chorus);
    fluid_revmodel_reset((*synth).reverb);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_modulate_voices(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut is_cc: libc::c_int,
    mut ctrl: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut voice;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as libc::c_int == chan {
            fluid_voice_modulate(voice, is_cc, ctrl);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_modulate_voices_all(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut voice;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as libc::c_int == chan {
            fluid_voice_modulate_all(voice);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_channel_pressure(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "channelpressure\t{}\t{}", chan, val);
    }
    fluid_channel_pressure(*(*synth).channel.offset(chan as isize), val);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_key_pressure(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut key: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    let mut result: libc::c_int = FLUID_OK as libc::c_int;
    if key < 0 as libc::c_int || key > 127 as libc::c_int {
        return FLUID_FAILED as libc::c_int;
    }
    if val < 0 as libc::c_int || val > 127 as libc::c_int {
        return FLUID_FAILED as libc::c_int;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "keypressure\t{}\t{}\t{}", chan, key, val);
    }
    (**(*synth).channel.offset(chan as isize)).key_pressure[key as usize] = val as libc::c_char;
    let mut voice;
    let mut i;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as libc::c_int == chan && (*voice).key as libc::c_int == key {
            result = fluid_voice_modulate(
                voice,
                0 as libc::c_int,
                FLUID_MOD_KEYPRESSURE as libc::c_int,
            );
            if result != FLUID_OK as libc::c_int {
                break;
            }
        }
        i += 1
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_pitch_bend(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "pitchb\t{}\t{}", chan, val);
    }
    fluid_channel_pitch_bend(*(*synth).channel.offset(chan as isize), val);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_pitch_bend(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut ppitch_bend: *mut libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    *ppitch_bend = (**(*synth).channel.offset(chan as isize)).pitch_bend as libc::c_int;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_pitch_wheel_sens(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut val: libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "pitchsens\t{}\t{}", chan, val);
    }
    fluid_channel_pitch_wheel_sens(*(*synth).channel.offset(chan as isize), val);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_pitch_wheel_sens(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut pval: *mut libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    *pval = (**(*synth).channel.offset(chan as isize)).pitch_wheel_sensitivity as libc::c_int;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_preset(
    mut synth: *mut fluid_synth_t,
    mut sfontnum: libc::c_uint,
    mut banknum: libc::c_uint,
    mut prognum: libc::c_uint,
) -> *mut fluid_preset_t {
    let mut preset;
    let mut sfont;
    let mut offset;
    sfont = fluid_synth_get_sfont_by_id(synth, sfontnum);
    if !sfont.is_null() {
        offset = fluid_synth_get_bank_offset(synth, sfontnum as libc::c_int);
        preset = Some((*sfont).get_preset.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            sfont,
            banknum.wrapping_sub(offset as libc::c_uint),
            prognum,
        );
        if !preset.is_null() {
            return preset;
        }
    }
    return 0 as *mut fluid_preset_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_preset2(
    mut synth: *mut fluid_synth_t,
    mut sfont_name: *mut libc::c_char,
    mut banknum: libc::c_uint,
    mut prognum: libc::c_uint,
) -> *mut fluid_preset_t {
    let mut preset;
    let mut sfont;
    let mut offset;
    sfont = fluid_synth_get_sfont_by_name(synth, sfont_name);
    if !sfont.is_null() {
        offset = fluid_synth_get_bank_offset(synth, (*sfont).id as libc::c_int);
        preset = Some((*sfont).get_preset.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            sfont,
            banknum.wrapping_sub(offset as libc::c_uint),
            prognum,
        );
        if !preset.is_null() {
            return preset;
        }
    }
    return 0 as *mut fluid_preset_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_find_preset(
    mut synth: *mut fluid_synth_t,
    mut banknum: libc::c_uint,
    mut prognum: libc::c_uint,
) -> *mut fluid_preset_t {
    let mut preset;
    let mut sfont;
    let mut list: *mut fluid_list_t = (*synth).sfont;
    let mut offset;
    while !list.is_null() {
        sfont = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sfont_t;
        offset = fluid_synth_get_bank_offset(synth, (*sfont).id as libc::c_int);
        preset = Some((*sfont).get_preset.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            sfont,
            banknum.wrapping_sub(offset as libc::c_uint),
            prognum,
        );
        if !preset.is_null() {
            (*preset).sfont = sfont;
            return preset;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    return 0 as *mut fluid_preset_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_program_change(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut prognum: libc::c_int,
) -> libc::c_int {
    let mut preset;
    let mut channel;
    let mut banknum;
    let mut sfont_id;
    let mut subst_bank;
    let mut subst_prog;
    if prognum < 0 as libc::c_int
        || prognum >= 128 as libc::c_int
        || chan < 0 as libc::c_int
        || chan >= (*synth).midi_channels
    {
        fluid_log!(
            FLUID_ERR,
            "Index out of range (chan={}, prog={})",
            chan,
            prognum
        );
        return FLUID_FAILED as libc::c_int;
    }
    channel = *(*synth).channel.offset(chan as isize);
    banknum = fluid_channel_get_banknum(channel);
    fluid_channel_set_prognum(channel, prognum);
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "prog\t{}\t{}\t{}", chan, banknum, prognum);
    }
    if (*channel).channum == 9 as libc::c_int
        && fluid_settings_str_equal(
            (*synth).settings,
            b"synth.drums-channel.active\x00" as *const u8 as *const libc::c_char,
            b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
    {
        preset = fluid_synth_find_preset(
            synth,
            128 as libc::c_int as libc::c_uint,
            prognum as libc::c_uint,
        )
    } else {
        preset = fluid_synth_find_preset(synth, banknum, prognum as libc::c_uint)
    }
    if preset.is_null() {
        subst_bank = banknum as libc::c_int;
        subst_prog = prognum;
        if banknum != 128 as libc::c_int as libc::c_uint {
            subst_bank = 0 as libc::c_int;
            preset = fluid_synth_find_preset(
                synth,
                0 as libc::c_int as libc::c_uint,
                prognum as libc::c_uint,
            );
            if preset.is_null() && prognum != 0 as libc::c_int {
                preset = fluid_synth_find_preset(
                    synth,
                    0 as libc::c_int as libc::c_uint,
                    0 as libc::c_int as libc::c_uint,
                );
                subst_prog = 0 as libc::c_int
            }
        } else {
            preset = fluid_synth_find_preset(
                synth,
                128 as libc::c_int as libc::c_uint,
                0 as libc::c_int as libc::c_uint,
            );
            subst_prog = 0 as libc::c_int
        }
        if !preset.is_null() {
            fluid_log!(FLUID_WARN,
                      "Instrument not found on channel {} [bank={} prog={}], substituted [bank={} prog={}]",
                      chan, banknum, prognum,
                      subst_bank, subst_prog);
        }
    }
    sfont_id = if !preset.is_null() {
        (*(*preset).sfont).id
    } else {
        0 as libc::c_int as libc::c_uint
    };
    fluid_channel_set_sfontnum(channel, sfont_id);
    fluid_channel_set_preset(channel, preset);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_bank_select(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut bank: libc::c_uint,
) -> libc::c_int {
    if chan >= 0 as libc::c_int && chan < (*synth).midi_channels {
        fluid_channel_set_banknum(*(*synth).channel.offset(chan as isize), bank);
        return FLUID_OK as libc::c_int;
    }
    return FLUID_FAILED as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_sfont_select(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut sfont_id: libc::c_uint,
) -> libc::c_int {
    if chan >= 0 as libc::c_int && chan < (*synth).midi_channels {
        fluid_channel_set_sfontnum(*(*synth).channel.offset(chan as isize), sfont_id);
        return FLUID_OK as libc::c_int;
    }
    return FLUID_FAILED as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_program(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut sfont_id: *mut libc::c_uint,
    mut bank_num: *mut libc::c_uint,
    mut preset_num: *mut libc::c_uint,
) -> libc::c_int {
    let mut channel;
    if chan >= 0 as libc::c_int && chan < (*synth).midi_channels {
        channel = *(*synth).channel.offset(chan as isize);
        *sfont_id = fluid_channel_get_sfontnum(channel);
        *bank_num = fluid_channel_get_banknum(channel);
        *preset_num = fluid_channel_get_prognum(channel) as libc::c_uint;
        return FLUID_OK as libc::c_int;
    }
    return FLUID_FAILED as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_program_select(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut sfont_id: libc::c_uint,
    mut bank_num: libc::c_uint,
    mut preset_num: libc::c_uint,
) -> libc::c_int {
    let mut preset;
    let mut channel;
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_ERR, "Channel number out of range (chan={})", chan);
        return FLUID_FAILED as libc::c_int;
    }
    channel = *(*synth).channel.offset(chan as isize);
    preset = fluid_synth_get_preset(synth, sfont_id, bank_num, preset_num);
    if preset.is_null() {
        fluid_log!(
            FLUID_ERR,
            "There is no preset with bank number {} and preset number {} in SoundFont {}",
            bank_num,
            preset_num,
            sfont_id
        );
        return FLUID_FAILED as libc::c_int;
    }
    fluid_channel_set_sfontnum(channel, sfont_id);
    fluid_channel_set_banknum(channel, bank_num);
    fluid_channel_set_prognum(channel, preset_num as libc::c_int);
    fluid_channel_set_preset(channel, preset);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_program_select2(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut sfont_name: *mut libc::c_char,
    mut bank_num: libc::c_uint,
    mut preset_num: libc::c_uint,
) -> libc::c_int {
    let mut preset;
    let mut channel;
    let mut sfont;
    let mut offset;
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_ERR, "Channel number out of range (chan={})", chan);
        return FLUID_FAILED as libc::c_int;
    }
    channel = *(*synth).channel.offset(chan as isize);
    sfont = fluid_synth_get_sfont_by_name(synth, sfont_name);
    if sfont.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Could not find SoundFont {}",
            CStr::from_ptr(sfont_name).to_str().unwrap()
        );
        return FLUID_FAILED as libc::c_int;
    }
    offset = fluid_synth_get_bank_offset(synth, (*sfont).id as libc::c_int);
    preset = Some((*sfont).get_preset.expect("non-null function pointer"))
        .expect("non-null function pointer")(
        sfont,
        bank_num.wrapping_sub(offset as libc::c_uint),
        preset_num,
    );
    if preset.is_null() {
        fluid_log!(
            FLUID_ERR,
            "There is no preset with bank number {} and preset number {} in SoundFont {}",
            bank_num,
            preset_num,
            CStr::from_ptr(sfont_name).to_str().unwrap()
        );
        return FLUID_FAILED as libc::c_int;
    }
    fluid_channel_set_sfontnum(channel, (*sfont).id);
    fluid_channel_set_banknum(channel, bank_num);
    fluid_channel_set_prognum(channel, preset_num as libc::c_int);
    fluid_channel_set_preset(channel, preset);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_update_presets(mut synth: *mut fluid_synth_t) {
    let mut chan;
    let mut channel;
    chan = 0 as libc::c_int;
    while chan < (*synth).midi_channels {
        channel = *(*synth).channel.offset(chan as isize);
        fluid_channel_set_preset(
            channel,
            fluid_synth_get_preset(
                synth,
                fluid_channel_get_sfontnum(channel),
                fluid_channel_get_banknum(channel),
                fluid_channel_get_prognum(channel) as libc::c_uint,
            ),
        );
        chan += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_update_gain(
    mut synth: *mut fluid_synth_t,
    _name: *mut libc::c_char,
    mut value: libc::c_double,
) -> libc::c_int {
    fluid_synth_set_gain(synth, value as libc::c_float);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_gain(
    mut synth: *mut fluid_synth_t,
    mut gain: libc::c_float,
) {
    let mut i;
    gain = if gain < 0.0f32 {
        0.0f32
    } else if gain > 10.0f32 {
        10.0f32
    } else {
        gain
    };
    (*synth).gain = gain as libc::c_double;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        let mut voice: *mut fluid_voice_t = *(*synth).voice.offset(i as isize);
        if (*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int
        {
            fluid_voice_set_gain(voice, gain);
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_gain(mut synth: *mut fluid_synth_t) -> libc::c_float {
    return (*synth).gain as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_update_polyphony(
    mut synth: *mut fluid_synth_t,
    _name: *mut libc::c_char,
    mut value: libc::c_int,
) -> libc::c_int {
    fluid_synth_set_polyphony(synth, value);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_polyphony(
    mut synth: *mut fluid_synth_t,
    mut polyphony: libc::c_int,
) -> libc::c_int {
    let mut i;
    if polyphony < 1 as libc::c_int || polyphony > (*synth).nvoice {
        return FLUID_FAILED as libc::c_int;
    }
    i = polyphony;
    while i < (*synth).nvoice {
        let mut voice: *mut fluid_voice_t = *(*synth).voice.offset(i as isize);
        if (*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int
        {
            fluid_voice_off(voice);
        }
        i += 1
    }
    (*synth).polyphony = polyphony;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_polyphony(mut synth: *mut fluid_synth_t) -> libc::c_int {
    return (*synth).polyphony;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_internal_bufsize(
    _synth: *mut fluid_synth_t,
) -> libc::c_int {
    return 64 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_program_reset(mut synth: *mut fluid_synth_t) -> libc::c_int {
    let mut i;
    i = 0 as libc::c_int;
    while i < (*synth).midi_channels {
        fluid_synth_program_change(
            synth,
            i,
            fluid_channel_get_prognum(*(*synth).channel.offset(i as isize)),
        );
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_reverb_preset(
    mut synth: *mut fluid_synth_t,
    mut num: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    while !revmodel_preset[i as usize].name.is_null() {
        if i == num {
            fluid_revmodel_setroomsize((*synth).reverb, revmodel_preset[i as usize].roomsize);
            fluid_revmodel_setdamp((*synth).reverb, revmodel_preset[i as usize].damp);
            fluid_revmodel_setwidth((*synth).reverb, revmodel_preset[i as usize].width);
            fluid_revmodel_setlevel((*synth).reverb, revmodel_preset[i as usize].level);
            return FLUID_OK as libc::c_int;
        }
        i += 1
    }
    return FLUID_FAILED as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_reverb(
    mut synth: *mut fluid_synth_t,
    mut roomsize: libc::c_double,
    mut damping: libc::c_double,
    mut width: libc::c_double,
    mut level: libc::c_double,
) {
    fluid_revmodel_setroomsize((*synth).reverb, roomsize as fluid_real_t);
    fluid_revmodel_setdamp((*synth).reverb, damping as fluid_real_t);
    fluid_revmodel_setwidth((*synth).reverb, width as fluid_real_t);
    fluid_revmodel_setlevel((*synth).reverb, level as fluid_real_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_chorus(
    mut synth: *mut fluid_synth_t,
    mut nr: libc::c_int,
    mut level: libc::c_double,
    mut speed: libc::c_double,
    mut depth_ms: libc::c_double,
    mut type_0: libc::c_int,
) {
    fluid_chorus_set_nr((*synth).chorus, nr);
    fluid_chorus_set_level((*synth).chorus, level as fluid_real_t);
    fluid_chorus_set_speed_Hz((*synth).chorus, speed as fluid_real_t);
    fluid_chorus_set_depth_ms((*synth).chorus, depth_ms as fluid_real_t);
    fluid_chorus_set_type((*synth).chorus, type_0);
    fluid_chorus_update((*synth).chorus);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_nwrite_float(
    mut synth: *mut fluid_synth_t,
    mut len: libc::c_int,
    mut left: *mut *mut libc::c_float,
    mut right: *mut *mut libc::c_float,
    _fx_left: *mut *mut libc::c_float,
    _fx_right: *mut *mut libc::c_float,
) -> libc::c_int {
    let mut left_in: *mut *mut fluid_real_t = (*synth).left_buf;
    let mut right_in: *mut *mut fluid_real_t = (*synth).right_buf;
    let mut i;
    let mut num;
    let mut available;
    let mut count;
    let mut bytes;
    if (*synth).state != FLUID_SYNTH_PLAYING as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    count = 0 as libc::c_int;
    num = (*synth).cur;
    if (*synth).cur < 64 as libc::c_int {
        available = 64 as libc::c_int - (*synth).cur;
        num = if available > len { len } else { available };
        bytes = (num as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::size_t)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*synth).audio_channels {
            libc::memcpy(
                *left.offset(i as isize) as *mut libc::c_void,
                (*left_in.offset(i as isize)).offset((*synth).cur as isize) as *const libc::c_void,
                bytes as libc::size_t,
            );
            libc::memcpy(
                *right.offset(i as isize) as *mut libc::c_void,
                (*right_in.offset(i as isize)).offset((*synth).cur as isize) as *const libc::c_void,
                bytes as libc::size_t,
            );
            i += 1
        }
        count += num;
        num += (*synth).cur
    }
    while count < len {
        fluid_synth_one_block(synth, 1 as libc::c_int);
        num = if 64 as libc::c_int > len - count {
            (len) - count
        } else {
            64 as libc::c_int
        };
        bytes = (num as libc::size_t)
            .wrapping_mul(::std::mem::size_of::<libc::c_float>() as libc::size_t)
            as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*synth).audio_channels {
            libc::memcpy(
                (*left.offset(i as isize)).offset(count as isize) as *mut libc::c_void,
                *left_in.offset(i as isize) as *const libc::c_void,
                bytes as libc::size_t,
            );
            libc::memcpy(
                (*right.offset(i as isize)).offset(count as isize) as *mut libc::c_void,
                *right_in.offset(i as isize) as *const libc::c_void,
                bytes as libc::size_t,
            );
            i += 1
        }
        count += num
    }
    (*synth).cur = num;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_process(
    mut synth: *mut fluid_synth_t,
    mut len: libc::c_int,
    _nin: libc::c_int,
    _in_0: *mut *mut libc::c_float,
    mut nout: libc::c_int,
    mut out: *mut *mut libc::c_float,
) -> libc::c_int {
    if nout == 2 as libc::c_int {
        return fluid_synth_write_float(
            synth,
            len,
            *out.offset(0 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
            1 as libc::c_int,
            *out.offset(1 as libc::c_int as isize) as *mut libc::c_void,
            0 as libc::c_int,
            1 as libc::c_int,
        );
    } else {
        let mut left;
        let mut right;
        let mut i;
        left = libc::malloc(
            ((nout / 2 as libc::c_int) as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_float>() as libc::size_t),
        ) as *mut *mut libc::c_float;
        right = libc::malloc(
            ((nout / 2 as libc::c_int) as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut libc::c_float>() as libc::size_t),
        ) as *mut *mut libc::c_float;
        i = 0 as libc::c_int;
        while i < nout / 2 as libc::c_int {
            let ref mut fresh25 = *left.offset(i as isize);
            *fresh25 = *out.offset((2 as libc::c_int * i) as isize);
            let ref mut fresh26 = *right.offset(i as isize);
            *fresh26 = *out.offset((2 as libc::c_int * i + 1 as libc::c_int) as isize);
            i += 1
        }
        fluid_synth_nwrite_float(
            synth,
            len,
            left,
            right,
            0 as *mut *mut libc::c_float,
            0 as *mut *mut libc::c_float,
        );
        libc::free(left as *mut libc::c_void);
        libc::free(right as *mut libc::c_void);
        return 0 as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_write_float(
    mut synth: *mut fluid_synth_t,
    mut len: libc::c_int,
    mut lout: *mut libc::c_void,
    mut loff: libc::c_int,
    mut lincr: libc::c_int,
    mut rout: *mut libc::c_void,
    mut roff: libc::c_int,
    mut rincr: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut j;
    let mut k;
    let mut l;
    let mut left_out: *mut libc::c_float = lout as *mut libc::c_float;
    let mut right_out: *mut libc::c_float = rout as *mut libc::c_float;
    let mut left_in: *mut fluid_real_t = *(*synth).left_buf.offset(0 as libc::c_int as isize);
    let mut right_in: *mut fluid_real_t = *(*synth).right_buf.offset(0 as libc::c_int as isize);
    if (*synth).state != FLUID_SYNTH_PLAYING as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    l = (*synth).cur;
    i = 0 as libc::c_int;
    j = loff;
    k = roff;
    while i < len {
        if l == 64 as libc::c_int {
            fluid_synth_one_block(synth, 0 as libc::c_int);
            l = 0 as libc::c_int
        }
        *left_out.offset(j as isize) = *left_in.offset(l as isize);
        *right_out.offset(k as isize) = *right_in.offset(l as isize);
        i += 1;
        l += 1;
        j += lincr;
        k += rincr
    }
    (*synth).cur = l;
    return 0 as libc::c_int;
}
static mut rand_table: [[libc::c_float; 48000]; 2] = [[0.; 48000]; 2];
unsafe extern "C" fn init_dither() {
    let mut d;
    let mut dp;
    let mut c;
    let mut i;
    c = 0 as libc::c_int;
    while c < 2 as libc::c_int {
        dp = 0 as libc::c_int as libc::c_float;
        i = 0 as libc::c_int;
        while i < 48000 as libc::c_int - 1 as libc::c_int {
            d = libc::rand() as libc::c_float / 2147483647 as libc::c_int as libc::c_float - 0.5f32;
            rand_table[c as usize][i as usize] = d - dp;
            dp = d;
            i += 1
        }
        rand_table[c as usize][(48000 as libc::c_int - 1 as libc::c_int) as usize] =
            0 as libc::c_int as libc::c_float - dp;
        c += 1
    }
}
unsafe extern "C" fn roundi(mut x: libc::c_float) -> libc::c_int {
    if x >= 0.0f32 {
        return (x + 0.5f32) as libc::c_int;
    } else {
        return (x - 0.5f32) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_write_s16(
    mut synth: *mut fluid_synth_t,
    mut len: libc::c_int,
    mut lout: *mut libc::c_void,
    mut loff: libc::c_int,
    mut lincr: libc::c_int,
    mut rout: *mut libc::c_void,
    mut roff: libc::c_int,
    mut rincr: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut j;
    let mut k;
    let mut cur;
    let mut left_out: *mut libc::c_short = lout as *mut libc::c_short;
    let mut right_out: *mut libc::c_short = rout as *mut libc::c_short;
    let mut left_in: *mut fluid_real_t = *(*synth).left_buf.offset(0 as libc::c_int as isize);
    let mut right_in: *mut fluid_real_t = *(*synth).right_buf.offset(0 as libc::c_int as isize);
    let mut left_sample;
    let mut right_sample;
    let mut di: libc::c_int = (*synth).dither_index;
    if (*synth).state != FLUID_SYNTH_PLAYING as libc::c_int as libc::c_uint {
        return 0 as libc::c_int;
    }
    cur = (*synth).cur;
    i = 0 as libc::c_int;
    j = loff;
    k = roff;
    while i < len {
        if cur == 64 as libc::c_int {
            fluid_synth_one_block(synth, 0 as libc::c_int);
            cur = 0 as libc::c_int
        }
        left_sample = roundi(
            *left_in.offset(cur as isize) * 32766.0f32
                + rand_table[0 as libc::c_int as usize][di as usize],
        ) as fluid_real_t;
        right_sample = roundi(
            *right_in.offset(cur as isize) * 32766.0f32
                + rand_table[1 as libc::c_int as usize][di as usize],
        ) as fluid_real_t;
        di += 1;
        if di >= 48000 as libc::c_int {
            di = 0 as libc::c_int
        }
        if left_sample > 32767.0f32 {
            left_sample = 32767.0f32
        }
        if left_sample < -32768.0f32 {
            left_sample = -32768.0f32
        }
        if right_sample > 32767.0f32 {
            right_sample = 32767.0f32
        }
        if right_sample < -32768.0f32 {
            right_sample = -32768.0f32
        }
        *left_out.offset(j as isize) = left_sample as libc::c_short;
        *right_out.offset(k as isize) = right_sample as libc::c_short;
        i += 1;
        cur += 1;
        j += lincr;
        k += rincr
    }
    (*synth).cur = cur;
    (*synth).dither_index = di;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_dither_s16(
    mut dither_index: *mut libc::c_int,
    mut len: libc::c_int,
    mut lin: *mut libc::c_float,
    mut rin: *mut libc::c_float,
    mut lout: *mut libc::c_void,
    mut loff: libc::c_int,
    mut lincr: libc::c_int,
    mut rout: *mut libc::c_void,
    mut roff: libc::c_int,
    mut rincr: libc::c_int,
) {
    let mut i;
    let mut j;
    let mut k;
    let mut left_out: *mut libc::c_short = lout as *mut libc::c_short;
    let mut right_out: *mut libc::c_short = rout as *mut libc::c_short;
    let mut left_sample;
    let mut right_sample;
    let mut di: libc::c_int = *dither_index;
    i = 0 as libc::c_int;
    j = loff;
    k = roff;
    while i < len {
        left_sample = roundi(
            *lin.offset(i as isize) * 32766.0f32
                + rand_table[0 as libc::c_int as usize][di as usize],
        ) as fluid_real_t;
        right_sample = roundi(
            *rin.offset(i as isize) * 32766.0f32
                + rand_table[1 as libc::c_int as usize][di as usize],
        ) as fluid_real_t;
        di += 1;
        if di >= 48000 as libc::c_int {
            di = 0 as libc::c_int
        }
        if left_sample > 32767.0f32 {
            left_sample = 32767.0f32
        }
        if left_sample < -32768.0f32 {
            left_sample = -32768.0f32
        }
        if right_sample > 32767.0f32 {
            right_sample = 32767.0f32
        }
        if right_sample < -32768.0f32 {
            right_sample = -32768.0f32
        }
        *left_out.offset(j as isize) = left_sample as libc::c_short;
        *right_out.offset(k as isize) = right_sample as libc::c_short;
        i += 1;
        j += lincr;
        k += rincr
    }
    *dither_index = di;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_one_block(
    mut synth: *mut fluid_synth_t,
    mut do_not_mix_fx_to_out: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut auchan;
    let mut voice;
    let mut left_buf;
    let mut right_buf;
    let mut reverb_buf;
    let mut chorus_buf;
    let mut byte_size: libc::c_int = (64 as libc::c_int as libc::size_t)
        .wrapping_mul(::std::mem::size_of::<fluid_real_t>() as libc::size_t)
        as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*synth).nbuf {
        libc::memset(
            *(*synth).left_buf.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            byte_size as libc::size_t,
        );
        libc::memset(
            *(*synth).right_buf.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            byte_size as libc::size_t,
        );
        i += 1
    }
    i = 0 as libc::c_int;
    while i < (*synth).effects_channels {
        libc::memset(
            *(*synth).fx_left_buf.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            byte_size as libc::size_t,
        );
        libc::memset(
            *(*synth).fx_right_buf.offset(i as isize) as *mut libc::c_void,
            0 as libc::c_int,
            byte_size as libc::size_t,
        );
        i += 1
    }
    reverb_buf = if (*synth).with_reverb as libc::c_int != 0 {
        *(*synth).fx_left_buf.offset(0 as libc::c_int as isize)
    } else {
        0 as *mut fluid_real_t
    };
    chorus_buf = if (*synth).with_chorus as libc::c_int != 0 {
        *(*synth).fx_left_buf.offset(1 as libc::c_int as isize)
    } else {
        0 as *mut fluid_real_t
    };
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int
        {
            auchan = fluid_channel_get_num(fluid_voice_get_channel(voice));
            auchan %= (*synth).audio_groups;
            left_buf = *(*synth).left_buf.offset(auchan as isize);
            right_buf = *(*synth).right_buf.offset(auchan as isize);
            fluid_voice_write(voice, left_buf, right_buf, reverb_buf, chorus_buf);
        }
        i += 1
    }
    if do_not_mix_fx_to_out != 0 {
        if !reverb_buf.is_null() {
            fluid_revmodel_processreplace(
                (*synth).reverb,
                reverb_buf,
                *(*synth).fx_left_buf.offset(0 as libc::c_int as isize),
                *(*synth).fx_right_buf.offset(0 as libc::c_int as isize),
            );
        }
        if !chorus_buf.is_null() {
            fluid_chorus_processreplace(
                (*synth).chorus,
                chorus_buf,
                *(*synth).fx_left_buf.offset(1 as libc::c_int as isize),
                *(*synth).fx_right_buf.offset(1 as libc::c_int as isize),
            );
        }
    } else {
        if !reverb_buf.is_null() {
            fluid_revmodel_processmix(
                (*synth).reverb,
                reverb_buf,
                *(*synth).left_buf.offset(0 as libc::c_int as isize),
                *(*synth).right_buf.offset(0 as libc::c_int as isize),
            );
        }
        if !chorus_buf.is_null() {
            fluid_chorus_processmix(
                (*synth).chorus,
                chorus_buf,
                *(*synth).left_buf.offset(0 as libc::c_int as isize),
                *(*synth).right_buf.offset(0 as libc::c_int as isize),
            );
        }
    }
    (*synth).ticks = (*synth)
        .ticks
        .wrapping_add(64 as libc::c_int as libc::c_uint);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_free_voice_by_kill(
    mut synth: *mut fluid_synth_t,
) -> *mut fluid_voice_t {
    let mut i;
    let mut best_prio: fluid_real_t = 999999.0f64 as fluid_real_t;
    let mut this_voice_prio;
    let mut voice;
    let mut best_voice_index: libc::c_int = -(1 as libc::c_int);
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as libc::c_int == FLUID_VOICE_CLEAN as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_OFF as libc::c_int
        {
            return voice;
        }
        this_voice_prio = 10000.0f64 as fluid_real_t;
        if (*voice).chan as libc::c_int == 0xff as libc::c_int {
            this_voice_prio = (this_voice_prio as libc::c_double - 2000.0f64) as fluid_real_t
        }
        if (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int {
            this_voice_prio -= 1000 as libc::c_int as libc::c_float
        }
        this_voice_prio -= (*synth).noteid.wrapping_sub(fluid_voice_get_id(voice)) as libc::c_float;
        if (*voice).volenv_section != FLUID_VOICE_ENVATTACK as libc::c_int {
            this_voice_prio = (this_voice_prio as libc::c_double
                + (*voice).volenv_val as libc::c_double * 1000.0f64)
                as fluid_real_t
        }
        if this_voice_prio < best_prio {
            best_voice_index = i;
            best_prio = this_voice_prio
        }
        i += 1
    }
    if best_voice_index < 0 as libc::c_int {
        return 0 as *mut fluid_voice_t;
    }
    voice = *(*synth).voice.offset(best_voice_index as isize);
    fluid_voice_off(voice);
    return voice;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_alloc_voice(
    mut synth: *mut fluid_synth_t,
    mut sample: *mut fluid_sample_t,
    mut chan: libc::c_int,
    mut key: libc::c_int,
    mut vel: libc::c_int,
) -> *mut fluid_voice_t {
    let mut i;
    let mut k;
    let mut voice: *mut fluid_voice_t = 0 as *mut fluid_voice_t;
    let mut channel;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        if (**(*synth).voice.offset(i as isize)).status as libc::c_int
            == FLUID_VOICE_CLEAN as libc::c_int
            || (**(*synth).voice.offset(i as isize)).status as libc::c_int
                == FLUID_VOICE_OFF as libc::c_int
        {
            voice = *(*synth).voice.offset(i as isize);
            break;
        } else {
            i += 1
        }
    }
    if voice.is_null() {
        voice = fluid_synth_free_voice_by_kill(synth)
    }
    if voice.is_null() {
        fluid_log!(
            FLUID_WARN,
            "Failed to allocate a synthesis process. (chan={},key={})",
            chan,
            key
        );
        return 0 as *mut fluid_voice_t;
    }
    if (*synth).verbose != 0 {
        k = 0 as libc::c_int;
        i = 0 as libc::c_int;
        while i < (*synth).polyphony {
            if !((**(*synth).voice.offset(i as isize)).status as libc::c_int
                == FLUID_VOICE_CLEAN as libc::c_int
                || (**(*synth).voice.offset(i as isize)).status as libc::c_int
                    == FLUID_VOICE_OFF as libc::c_int)
            {
                k += 1
            }
            i += 1
        }
        fluid_log!(
            FLUID_INFO,
            "noteon\t{}\t{}\t{}\t{}\t{}\t\t{}\t{}",
            chan,
            key,
            vel,
            (*synth).storeid,
            ((*synth).ticks as libc::c_float / 44100.0f32) as libc::c_double,
            0.0f32 as libc::c_double,
            k
        );
    }
    if chan >= 0 as libc::c_int {
        channel = *(*synth).channel.offset(chan as isize)
    } else {
        fluid_log!(FLUID_WARN, "Channel should be valid",);
        return 0 as *mut fluid_voice_t;
    }
    if fluid_voice_init(
        voice,
        sample,
        channel,
        key,
        vel,
        (*synth).storeid,
        (*synth).ticks,
        (*synth).gain as fluid_real_t,
    ) != FLUID_OK as libc::c_int
    {
        fluid_log!(FLUID_WARN, "Failed to initialize voice",);
        return 0 as *mut fluid_voice_t;
    }
    fluid_voice_add_mod(
        voice,
        &mut default_vel2att_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_vel2filter_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_at2viblfo_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_mod2viblfo_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_att_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_pan_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_expr_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_reverb_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_chorus_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    fluid_voice_add_mod(
        voice,
        &mut default_pitch_bend_mod,
        FLUID_VOICE_DEFAULT as libc::c_int,
    );
    return voice;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_kill_by_exclusive_class(
    mut synth: *mut fluid_synth_t,
    mut new_voice: *mut fluid_voice_t,
) {
    let mut i;
    let mut excl_class: libc::c_int = ((*new_voice).gen[GEN_EXCLUSIVECLASS as libc::c_int as usize]
        .val as fluid_real_t
        + (*new_voice).gen[GEN_EXCLUSIVECLASS as libc::c_int as usize].mod_0 as fluid_real_t
        + (*new_voice).gen[GEN_EXCLUSIVECLASS as libc::c_int as usize].nrpn as fluid_real_t)
        as libc::c_int;
    if excl_class == 0 as libc::c_int {
        return;
    }
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        let mut existing_voice: *mut fluid_voice_t = *(*synth).voice.offset(i as isize);
        if (*existing_voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*existing_voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int
        {
            if !((*existing_voice).chan as libc::c_int != (*new_voice).chan as libc::c_int) {
                if !(((*existing_voice).gen[GEN_EXCLUSIVECLASS as libc::c_int as usize].val
                    as fluid_real_t
                    + (*existing_voice).gen[GEN_EXCLUSIVECLASS as libc::c_int as usize].mod_0
                        as fluid_real_t
                    + (*existing_voice).gen[GEN_EXCLUSIVECLASS as libc::c_int as usize].nrpn
                        as fluid_real_t) as libc::c_int
                    != excl_class)
                {
                    if !(fluid_voice_get_id(existing_voice) == fluid_voice_get_id(new_voice)) {
                        fluid_voice_kill_excl(existing_voice);
                    }
                }
            }
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_start_voice(
    mut synth: *mut fluid_synth_t,
    mut voice: *mut fluid_voice_t,
) {
    fluid_synth_kill_by_exclusive_class(synth, voice);
    fluid_voice_start(voice);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_add_sfloader(
    mut synth: *mut fluid_synth_t,
    mut loader: *mut fluid_sfloader_t,
) {
    (*synth).loaders = fluid_list_prepend((*synth).loaders, loader as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_sfload(
    mut synth: *mut fluid_synth_t,
    mut filename: *const libc::c_char,
    mut reset_presets: libc::c_int,
) -> libc::c_int {
    let mut sfont;
    let mut list;
    let mut loader;
    if filename.is_null() {
        fluid_log!(FLUID_ERR, "Invalid filename",);
        return FLUID_FAILED as libc::c_int;
    }
    list = (*synth).loaders;
    if !list.is_null() {
        loader = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sfloader_t;
        sfont = Some((*loader).load.expect("non-null function pointer"))
            .expect("non-null function pointer")(loader, filename);
        if sfont.is_null() {
            return -(1 as libc::c_int);
        }
        (*synth).sfont_id = (*synth).sfont_id.wrapping_add(1);
        (*sfont).id = (*synth).sfont_id;
        (*synth).sfont = fluid_list_prepend((*synth).sfont, sfont as *mut libc::c_void);
        if reset_presets != 0 {
            fluid_synth_program_reset(synth);
        }
        return (*sfont).id as libc::c_int;
    }
    fluid_log!(
        FLUID_ERR,
        "Failed to load SoundFont \"{}\"",
        CStr::from_ptr(filename).to_str().unwrap()
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_sfunload(
    mut synth: *mut fluid_synth_t,
    mut id: libc::c_uint,
    mut reset_presets: libc::c_int,
) -> libc::c_int {
    let mut sfont: *mut fluid_sfont_t = fluid_synth_get_sfont_by_id(synth, id);
    if sfont.is_null() {
        fluid_log!(FLUID_ERR, "No SoundFont with id = {}", id);
        return FLUID_FAILED as libc::c_int;
    }
    (*synth).sfont = fluid_list_remove((*synth).sfont, sfont as *mut libc::c_void);
    if reset_presets != 0 {
        fluid_synth_program_reset(synth);
    } else {
        fluid_synth_update_presets(synth);
    }
    if (if !sfont.is_null() && (*sfont).free.is_some() {
        Some((*sfont).free.expect("non-null function pointer")).expect("non-null function pointer")(
            sfont,
        )
    } else {
        0 as libc::c_int
    }) != 0 as libc::c_int
    {
        let mut r: libc::c_int = if !sfont.is_null() && (*sfont).free.is_some() {
            Some((*sfont).free.expect("non-null function pointer"))
                .expect("non-null function pointer")(sfont)
        } else {
            0 as libc::c_int
        };
        if r == 0 as libc::c_int {
            fluid_log!(FLUID_DBG as libc::c_int, "Unloaded SoundFont",);
        }
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_sfreload(
    mut synth: *mut fluid_synth_t,
    mut id: libc::c_uint,
) -> libc::c_int {
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    let mut sfont;
    let mut index: libc::c_int = 0 as libc::c_int;
    let mut list;
    let mut loader;
    sfont = fluid_synth_get_sfont_by_id(synth, id);
    if sfont.is_null() {
        fluid_log!(FLUID_ERR, "No SoundFont with id = {}", id);
        return FLUID_FAILED as libc::c_int;
    }
    list = (*synth).sfont;
    while !list.is_null() {
        if sfont
            == (if !list.is_null() {
                (*list).data
            } else {
                0 as *mut libc::c_void
            }) as *mut fluid_sfont_t
        {
            break;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        };
        index += 1
    }
    libc::strcpy(
        filename.as_mut_ptr(),
        Some((*sfont).get_name.expect("non-null function pointer"))
            .expect("non-null function pointer")(sfont),
    );
    if fluid_synth_sfunload(synth, id, 0 as libc::c_int) != FLUID_OK as libc::c_int {
        return FLUID_FAILED as libc::c_int;
    }
    list = (*synth).loaders;
    while !list.is_null() {
        loader = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sfloader_t;
        sfont = Some((*loader).load.expect("non-null function pointer"))
            .expect("non-null function pointer")(loader, filename.as_mut_ptr());
        if !sfont.is_null() {
            (*sfont).id = id;
            (*synth).sfont =
                fluid_list_insert_at((*synth).sfont, index, sfont as *mut libc::c_void);
            fluid_synth_update_presets(synth);
            return (*sfont).id as libc::c_int;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    fluid_log!(
        FLUID_ERR,
        "Failed to load SoundFont \"{}\"",
        CStr::from_ptr(filename.as_ptr()).to_str().unwrap()
    );
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_add_sfont(
    mut synth: *mut fluid_synth_t,
    mut sfont: *mut fluid_sfont_t,
) -> libc::c_int {
    (*synth).sfont_id = (*synth).sfont_id.wrapping_add(1);
    (*sfont).id = (*synth).sfont_id;
    (*synth).sfont = fluid_list_prepend((*synth).sfont, sfont as *mut libc::c_void);
    fluid_synth_program_reset(synth);
    return (*sfont).id as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_remove_sfont(
    mut synth: *mut fluid_synth_t,
    mut sfont: *mut fluid_sfont_t,
) {
    let mut sfont_id: libc::c_int = (*sfont).id as libc::c_int;
    (*synth).sfont = fluid_list_remove((*synth).sfont, sfont as *mut libc::c_void);
    fluid_synth_remove_bank_offset(synth, sfont_id);
    fluid_synth_program_reset(synth);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_sfcount(mut synth: *mut fluid_synth_t) -> libc::c_int {
    return fluid_list_size((*synth).sfont);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_sfont(
    mut synth: *mut fluid_synth_t,
    mut num: libc::c_uint,
) -> *mut fluid_sfont_t {
    return if !fluid_list_nth((*synth).sfont, num as libc::c_int).is_null() {
        (*fluid_list_nth((*synth).sfont, num as libc::c_int)).data
    } else {
        0 as *mut libc::c_void
    } as *mut fluid_sfont_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_sfont_by_id(
    mut synth: *mut fluid_synth_t,
    mut id: libc::c_uint,
) -> *mut fluid_sfont_t {
    let mut list: *mut fluid_list_t = (*synth).sfont;
    let mut sfont;
    while !list.is_null() {
        sfont = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sfont_t;
        if (*sfont).id == id {
            return sfont;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    return 0 as *mut fluid_sfont_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_sfont_by_name(
    mut synth: *mut fluid_synth_t,
    mut name: *mut libc::c_char,
) -> *mut fluid_sfont_t {
    let mut list: *mut fluid_list_t = (*synth).sfont;
    let mut sfont;
    while !list.is_null() {
        sfont = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sfont_t;
        if libc::strcmp(
            Some((*sfont).get_name.expect("non-null function pointer"))
                .expect("non-null function pointer")(sfont),
            name,
        ) == 0 as libc::c_int
        {
            return sfont;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    return 0 as *mut fluid_sfont_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_channel_preset(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
) -> *mut fluid_preset_t {
    if chan >= 0 as libc::c_int && chan < (*synth).midi_channels {
        return fluid_channel_get_preset(*(*synth).channel.offset(chan as isize));
    }
    return 0 as *mut fluid_preset_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_voicelist(
    mut synth: *mut fluid_synth_t,
    mut buf: *mut *mut fluid_voice_t,
    mut bufsize: libc::c_int,
    mut ID: libc::c_int,
) {
    let mut i;
    let mut count: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        let mut voice: *mut fluid_voice_t = *(*synth).voice.offset(i as isize);
        if count >= bufsize {
            return;
        }
        if ((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
            && ((*voice).id as libc::c_int == ID || ID < 0 as libc::c_int)
        {
            let fresh27 = count;
            count = count + 1;
            let ref mut fresh28 = *buf.offset(fresh27 as isize);
            *fresh28 = voice
        }
        i += 1
    }
    if count >= bufsize {
        return;
    }
    let fresh29 = count;
    let ref mut fresh30 = *buf.offset(fresh29 as isize);
    *fresh30 = 0 as *mut fluid_voice_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_reverb_on(
    mut synth: *mut fluid_synth_t,
    mut on: libc::c_int,
) {
    (*synth).with_reverb = on as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_chorus_on(
    mut synth: *mut fluid_synth_t,
    mut on: libc::c_int,
) {
    (*synth).with_chorus = on as libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_chorus_nr(mut synth: *mut fluid_synth_t) -> libc::c_int {
    return fluid_chorus_get_nr((*synth).chorus);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_chorus_level(
    mut synth: *mut fluid_synth_t,
) -> libc::c_double {
    return fluid_chorus_get_level((*synth).chorus) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_chorus_speed_Hz(
    mut synth: *mut fluid_synth_t,
) -> libc::c_double {
    return fluid_chorus_get_speed_Hz((*synth).chorus) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_chorus_depth_ms(
    mut synth: *mut fluid_synth_t,
) -> libc::c_double {
    return fluid_chorus_get_depth_ms((*synth).chorus) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_chorus_type(mut synth: *mut fluid_synth_t) -> libc::c_int {
    return fluid_chorus_get_type((*synth).chorus);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_reverb_roomsize(
    mut synth: *mut fluid_synth_t,
) -> libc::c_double {
    return fluid_revmodel_getroomsize((*synth).reverb) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_reverb_damp(
    mut synth: *mut fluid_synth_t,
) -> libc::c_double {
    return fluid_revmodel_getdamp((*synth).reverb) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_reverb_level(
    mut synth: *mut fluid_synth_t,
) -> libc::c_double {
    return fluid_revmodel_getlevel((*synth).reverb) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_reverb_width(
    mut synth: *mut fluid_synth_t,
) -> libc::c_double {
    return fluid_revmodel_getwidth((*synth).reverb) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_release_voice_on_same_note(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut key: libc::c_int,
) {
    let mut i;
    let mut voice;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if ((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
            && (*voice).chan as libc::c_int == chan
            && (*voice).key as libc::c_int == key
            && fluid_voice_get_id(voice) != (*synth).noteid
        {
            fluid_voice_noteoff(voice);
        }
        i += 1
    }
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_interp_method(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut interp_method: libc::c_int,
) -> libc::c_int {
    let mut i;
    i = 0 as libc::c_int;
    while i < (*synth).midi_channels {
        if (*(*synth).channel.offset(i as isize)).is_null() {
            fluid_log!(FLUID_ERR, "Channels don't exist (yet)!",);
            return FLUID_FAILED as libc::c_int;
        }
        if chan < 0 as libc::c_int
            || fluid_channel_get_num(*(*synth).channel.offset(i as isize)) == chan
        {
            fluid_channel_set_interp_method(*(*synth).channel.offset(i as isize), interp_method);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_count_midi_channels(
    mut synth: *mut fluid_synth_t,
) -> libc::c_int {
    return (*synth).midi_channels;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_count_audio_channels(
    mut synth: *mut fluid_synth_t,
) -> libc::c_int {
    return (*synth).audio_channels;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_count_audio_groups(
    mut synth: *mut fluid_synth_t,
) -> libc::c_int {
    return (*synth).audio_groups;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_count_effects_channels(
    mut synth: *mut fluid_synth_t,
) -> libc::c_int {
    return (*synth).effects_channels;
}
unsafe extern "C" fn fluid_synth_get_tuning(
    mut synth: *mut fluid_synth_t,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
) -> *mut fluid_tuning_t {
    if bank < 0 as libc::c_int || bank >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Bank number out of range",);
        return 0 as *mut fluid_tuning_t;
    }
    if prog < 0 as libc::c_int || prog >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Program number out of range",);
        return 0 as *mut fluid_tuning_t;
    }
    if (*synth).tuning.is_null()
        || (*(*synth).tuning.offset(bank as isize)).is_null()
        || (*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).is_null()
    {
        fluid_log!(FLUID_WARN, "No tuning at bank {}, prog {}", bank, prog);
        return 0 as *mut fluid_tuning_t;
    }
    return *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
}
unsafe extern "C" fn fluid_synth_create_tuning(
    mut synth: *mut fluid_synth_t,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
    mut name: *const libc::c_char,
) -> *mut fluid_tuning_t {
    if bank < 0 as libc::c_int || bank >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Bank number out of range",);
        return 0 as *mut fluid_tuning_t;
    }
    if prog < 0 as libc::c_int || prog >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Program number out of range",);
        return 0 as *mut fluid_tuning_t;
    }
    if (*synth).tuning.is_null() {
        (*synth).tuning = libc::malloc(
            (128 as libc::c_int as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut *mut fluid_tuning_t>() as libc::size_t),
        ) as *mut *mut *mut fluid_tuning_t;
        if (*synth).tuning.is_null() {
            fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
            return 0 as *mut fluid_tuning_t;
        }
        libc::memset(
            (*synth).tuning as *mut libc::c_void,
            0 as libc::c_int,
            (128 as libc::c_int as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut *mut fluid_tuning_t>() as libc::size_t),
        );
    }
    if (*(*synth).tuning.offset(bank as isize)).is_null() {
        let ref mut fresh31 = *(*synth).tuning.offset(bank as isize);
        *fresh31 = libc::malloc(
            (128 as libc::c_int as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut fluid_tuning_t>() as libc::size_t),
        ) as *mut *mut fluid_tuning_t;
        if (*(*synth).tuning.offset(bank as isize)).is_null() {
            fluid_log!(FLUID_PANIC as libc::c_int, "Out of memory",);
            return 0 as *mut fluid_tuning_t;
        }
        libc::memset(
            *(*synth).tuning.offset(bank as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (128 as libc::c_int as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut fluid_tuning_t>() as libc::size_t),
        );
    }
    if (*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).is_null() {
        let ref mut fresh32 = *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
        *fresh32 = new_fluid_tuning(name, bank, prog);
        if (*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).is_null() {
            return 0 as *mut fluid_tuning_t;
        }
    }
    if fluid_tuning_get_name(*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize))
        .is_null()
        || libc::strcmp(
            fluid_tuning_get_name(*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)),
            name,
        ) != 0 as libc::c_int
    {
        fluid_tuning_set_name(
            *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize),
            name,
        );
    }
    return *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_create_key_tuning(
    mut synth: *mut fluid_synth_t,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
    mut name: *const libc::c_char,
    mut pitch: *mut libc::c_double,
) -> libc::c_int {
    let mut tuning: *mut fluid_tuning_t = fluid_synth_create_tuning(synth, bank, prog, name);
    if tuning.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if !pitch.is_null() {
        fluid_tuning_set_all(tuning, pitch);
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_create_octave_tuning(
    mut synth: *mut fluid_synth_t,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
    mut name: *const libc::c_char,
    mut pitch: *const libc::c_double,
) -> libc::c_int {
    let mut tuning;
    if synth.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if !(bank >= 0 as libc::c_int && bank < 128 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    if !(prog >= 0 as libc::c_int && prog < 128 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    if name.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if pitch.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    tuning = fluid_synth_create_tuning(synth, bank, prog, name);
    if tuning.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    fluid_tuning_set_octave(tuning, pitch);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_activate_octave_tuning(
    mut synth: *mut fluid_synth_t,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
    mut name: *const libc::c_char,
    mut pitch: *const libc::c_double,
    _apply: libc::c_int,
) -> libc::c_int {
    return fluid_synth_create_octave_tuning(synth, bank, prog, name, pitch);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_tune_notes(
    mut synth: *mut fluid_synth_t,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
    mut len: libc::c_int,
    mut key: *mut libc::c_int,
    mut pitch: *mut libc::c_double,
    _apply: libc::c_int,
) -> libc::c_int {
    let mut tuning;
    let mut i;
    if synth.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if !(bank >= 0 as libc::c_int && bank < 128 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    if !(prog >= 0 as libc::c_int && prog < 128 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    if !(len > 0 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    if key.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if pitch.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    tuning = fluid_synth_get_tuning(synth, bank, prog);
    if tuning.is_null() {
        tuning = new_fluid_tuning(
            b"Unnamed\x00" as *const u8 as *const libc::c_char,
            bank,
            prog,
        )
    }
    if tuning.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < len {
        fluid_tuning_set_pitch(tuning, *key.offset(i as isize), *pitch.offset(i as isize));
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_select_tuning(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
) -> libc::c_int {
    let mut tuning;
    if synth.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if !(bank >= 0 as libc::c_int && bank < 128 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    if !(prog >= 0 as libc::c_int && prog < 128 as libc::c_int) {
        return FLUID_FAILED as libc::c_int;
    }
    tuning = fluid_synth_get_tuning(synth, bank, prog);
    if tuning.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    let ref mut fresh33 = (**(*synth).channel.offset(chan as isize)).tuning;
    *fresh33 = *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_activate_tuning(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
    _apply: libc::c_int,
) -> libc::c_int {
    return fluid_synth_select_tuning(synth, chan, bank, prog);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_reset_tuning(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
) -> libc::c_int {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    let ref mut fresh34 = (**(*synth).channel.offset(chan as isize)).tuning;
    *fresh34 = 0 as *mut fluid_tuning_t;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_tuning_iteration_start(mut synth: *mut fluid_synth_t) {
    (*synth).cur_tuning = 0 as *mut fluid_tuning_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_tuning_iteration_next(
    mut synth: *mut fluid_synth_t,
    mut bank: *mut libc::c_int,
    mut prog: *mut libc::c_int,
) -> libc::c_int {
    let mut b: libc::c_int = 0 as libc::c_int;
    let mut p: libc::c_int = 0 as libc::c_int;
    if (*synth).tuning.is_null() {
        return 0 as libc::c_int;
    }
    if !(*synth).cur_tuning.is_null() {
        b = (*(*synth).cur_tuning).bank;
        p = 1 as libc::c_int + (*(*synth).cur_tuning).prog;
        if p >= 128 as libc::c_int {
            p = 0 as libc::c_int;
            b += 1
        }
    }
    while b < 128 as libc::c_int {
        if !(*(*synth).tuning.offset(b as isize)).is_null() {
            while p < 128 as libc::c_int {
                if !(*(*(*synth).tuning.offset(b as isize)).offset(p as isize)).is_null() {
                    (*synth).cur_tuning = *(*(*synth).tuning.offset(b as isize)).offset(p as isize);
                    *bank = b;
                    *prog = p;
                    return 1 as libc::c_int;
                }
                p += 1
            }
        }
        p = 0 as libc::c_int;
        b += 1
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_tuning_dump(
    mut synth: *mut fluid_synth_t,
    mut bank: libc::c_int,
    mut prog: libc::c_int,
    mut name: *mut libc::c_char,
    mut len: libc::c_int,
    mut pitch: *mut libc::c_double,
) -> libc::c_int {
    let mut tuning: *mut fluid_tuning_t = fluid_synth_get_tuning(synth, bank, prog);
    if tuning.is_null() {
        return FLUID_FAILED as libc::c_int;
    }
    if !name.is_null() {
        libc::strncpy(
            name,
            fluid_tuning_get_name(tuning),
            (len - 1 as libc::c_int) as libc::size_t,
        );
        *name.offset((len - 1 as libc::c_int) as isize) = 0 as libc::c_int as libc::c_char
    }
    if !pitch.is_null() {
        libc::memcpy(
            pitch as *mut libc::c_void,
            &mut *(*tuning)
                .pitch
                .as_mut_ptr()
                .offset(0 as libc::c_int as isize) as *mut libc::c_double
                as *const libc::c_void,
            (128 as libc::c_int as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<libc::c_double>() as libc::size_t),
        );
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_settings(
    mut synth: *mut fluid_synth_t,
) -> *mut fluid_settings_t {
    return (*synth).settings;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_setstr(
    mut synth: *mut fluid_synth_t,
    mut name: *mut libc::c_char,
    mut str: *mut libc::c_char,
) -> libc::c_int {
    return fluid_settings_setstr((*synth).settings, name, str);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_getstr(
    mut synth: *mut fluid_synth_t,
    mut name: *mut libc::c_char,
    mut str: *mut *mut libc::c_char,
) -> libc::c_int {
    return fluid_settings_getstr((*synth).settings, name, str);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_setnum(
    mut synth: *mut fluid_synth_t,
    mut name: *mut libc::c_char,
    mut val: libc::c_double,
) -> libc::c_int {
    return fluid_settings_setnum((*synth).settings, name, val);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_getnum(
    mut synth: *mut fluid_synth_t,
    mut name: *mut libc::c_char,
    mut val: *mut libc::c_double,
) -> libc::c_int {
    return fluid_settings_getnum((*synth).settings, name, val);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_setint(
    mut synth: *mut fluid_synth_t,
    mut name: *mut libc::c_char,
    mut val: libc::c_int,
) -> libc::c_int {
    return fluid_settings_setint((*synth).settings, name, val);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_getint(
    mut synth: *mut fluid_synth_t,
    mut name: *mut libc::c_char,
    mut val: *mut libc::c_int,
) -> libc::c_int {
    return fluid_settings_getint((*synth).settings, name, val);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_gen(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut param: libc::c_int,
    mut value: libc::c_float,
) -> libc::c_int {
    let mut i;
    let mut voice;
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if param < 0 as libc::c_int || param >= GEN_LAST as libc::c_int {
        fluid_log!(FLUID_WARN, "Parameter number out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    (**(*synth).channel.offset(chan as isize)).gen[param as usize] = value;
    (**(*synth).channel.offset(chan as isize)).gen_abs[param as usize] =
        0 as libc::c_int as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as libc::c_int == chan {
            fluid_voice_set_param(voice, param, value, 0 as libc::c_int);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_gen2(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut param: libc::c_int,
    mut value: libc::c_float,
    mut absolute: libc::c_int,
    mut normalized: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut voice;
    let mut v;
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if param < 0 as libc::c_int || param >= GEN_LAST as libc::c_int {
        fluid_log!(FLUID_WARN, "Parameter number out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    v = if normalized != 0 {
        fluid_gen_scale(param, value)
    } else {
        value
    };
    (**(*synth).channel.offset(chan as isize)).gen[param as usize] = v;
    (**(*synth).channel.offset(chan as isize)).gen_abs[param as usize] = absolute as libc::c_char;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as libc::c_int == chan {
            fluid_voice_set_param(voice, param, v, absolute);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_gen(
    mut synth: *mut fluid_synth_t,
    mut chan: libc::c_int,
    mut param: libc::c_int,
) -> libc::c_float {
    if chan < 0 as libc::c_int || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return 0.0f64 as libc::c_float;
    }
    if param < 0 as libc::c_int || param >= GEN_LAST as libc::c_int {
        fluid_log!(FLUID_WARN, "Parameter number out of range",);
        return 0.0f64 as libc::c_float;
    }
    return (**(*synth).channel.offset(chan as isize)).gen[param as usize];
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_start(
    mut synth: *mut fluid_synth_t,
    mut id: libc::c_uint,
    mut preset: *mut fluid_preset_t,
    _audio_chan: libc::c_int,
    mut midi_chan: libc::c_int,
    mut key: libc::c_int,
    mut vel: libc::c_int,
) -> libc::c_int {
    let mut r;
    if midi_chan < 0 as libc::c_int || midi_chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if key < 0 as libc::c_int || key >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Key out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    if vel <= 0 as libc::c_int || vel >= 128 as libc::c_int {
        fluid_log!(FLUID_WARN, "Velocity out of range",);
        return FLUID_FAILED as libc::c_int;
    }
    (*synth).storeid = id;
    r = Some((*preset).noteon.expect("non-null function pointer"))
        .expect("non-null function pointer")(preset, synth, midi_chan, key, vel);
    return r;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_stop(
    mut synth: *mut fluid_synth_t,
    mut id: libc::c_uint,
) -> libc::c_int {
    let mut i;
    let mut voice;
    let mut status: libc::c_int = FLUID_FAILED as libc::c_int;
    i = 0 as libc::c_int;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
            && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as libc::c_int
            && fluid_voice_get_id(voice) == id
        {
            fluid_voice_noteoff(voice);
            status = FLUID_OK as libc::c_int
        }
        i += 1
    }
    return status;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_bank_offset0(
    mut synth: *mut fluid_synth_t,
    mut sfont_id: libc::c_int,
) -> *mut fluid_bank_offset_t {
    let mut list: *mut fluid_list_t = (*synth).bank_offsets;
    let mut offset;
    while !list.is_null() {
        offset = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_bank_offset_t;
        if (*offset).sfont_id == sfont_id {
            return offset;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut fluid_list_t
        }
    }
    return 0 as *mut fluid_bank_offset_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_bank_offset(
    mut synth: *mut fluid_synth_t,
    mut sfont_id: libc::c_int,
    mut offset: libc::c_int,
) -> libc::c_int {
    let mut bank_offset;
    bank_offset = fluid_synth_get_bank_offset0(synth, sfont_id);
    if bank_offset.is_null() {
        bank_offset = libc::malloc(::std::mem::size_of::<fluid_bank_offset_t>() as libc::size_t)
            as *mut fluid_bank_offset_t;
        if bank_offset.is_null() {
            return -(1 as libc::c_int);
        }
        (*bank_offset).sfont_id = sfont_id;
        (*bank_offset).offset = offset;
        (*synth).bank_offsets =
            fluid_list_prepend((*synth).bank_offsets, bank_offset as *mut libc::c_void)
    } else {
        (*bank_offset).offset = offset
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_get_bank_offset(
    mut synth: *mut fluid_synth_t,
    mut sfont_id: libc::c_int,
) -> libc::c_int {
    let mut bank_offset;
    bank_offset = fluid_synth_get_bank_offset0(synth, sfont_id);
    return if bank_offset.is_null() {
        0 as libc::c_int
    } else {
        (*bank_offset).offset
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_remove_bank_offset(
    mut synth: *mut fluid_synth_t,
    mut sfont_id: libc::c_int,
) {
    let mut bank_offset;
    bank_offset = fluid_synth_get_bank_offset0(synth, sfont_id);
    if !bank_offset.is_null() {
        (*synth).bank_offsets =
            fluid_list_remove((*synth).bank_offsets, bank_offset as *mut libc::c_void)
    };
}
