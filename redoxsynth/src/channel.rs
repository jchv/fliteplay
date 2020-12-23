#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use crate::chorus::_fluid_chorus_t;
use crate::gen::_fluid_gen_t;
use crate::gen::fluid_gen_scale_nrpn;
use crate::hash::_fluid_hashtable_t;
use crate::list::_fluid_list_t;
use crate::fluid_mod::_fluid_mod_t;
use crate::fluid_rev::_fluid_revmodel_t;
use crate::fluid_settings::fluid_settings_str_equal;
use crate::fluid_sfont::_fluid_preset_t;
use crate::fluid_sfont::_fluid_sample_t;
use crate::fluid_sfont::_fluid_sfont_t;
use crate::fluid_synth::_fluid_synth_t;
use crate::fluid_synth::fluid_synth_all_notes_off;
use crate::fluid_synth::fluid_synth_all_sounds_off;
use crate::fluid_synth::fluid_synth_damp_voices;
use crate::fluid_synth::fluid_synth_find_preset;
use crate::fluid_synth::fluid_synth_modulate_voices;
use crate::fluid_synth::fluid_synth_modulate_voices_all;
use crate::fluid_synth::fluid_synth_set_gen;
use crate::fluid_tuning::_fluid_tuning_t;
use crate::fluid_voice::_fluid_env_data_t;
use crate::fluid_voice::_fluid_voice_t;

pub type fluid_settings_t = _fluid_hashtable_t;
pub type fluid_tuning_t = _fluid_tuning_t;
pub type fluid_chorus_t = _fluid_chorus_t;
pub type fluid_revmodel_t = _fluid_revmodel_t;
pub type fluid_real_t = libc::c_float;
pub type fluid_voice_t = _fluid_voice_t;
pub type fluid_env_data_t = _fluid_env_data_t;
pub type fluid_phase_t = libc::c_ulonglong;
pub type fluid_sample_t = _fluid_sample_t;
pub type fluid_mod_t = _fluid_mod_t;
pub type fluid_gen_t = _fluid_gen_t;
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
pub type C2RustUnnamed_0 = libc::c_int;
pub const FLUID_FAILED: C2RustUnnamed_0 = -1;
pub const FLUID_OK: C2RustUnnamed_0 = 0;
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
pub const SUSTAIN_SWITCH: fluid_midi_control_change = 64;
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
pub type midi_rpn_event = libc::c_uint;
pub const RPN_MODULATION_DEPTH_RANGE: midi_rpn_event = 5;
pub const RPN_TUNING_BANK_SELECT: midi_rpn_event = 4;
pub const RPN_TUNING_PROGRAM_CHANGE: midi_rpn_event = 3;
pub const RPN_CHANNEL_COARSE_TUNE: midi_rpn_event = 2;
pub const RPN_CHANNEL_FINE_TUNE: midi_rpn_event = 1;
pub const RPN_PITCH_BEND_RANGE: midi_rpn_event = 0;

#[no_mangle]
pub unsafe extern "C" fn new_fluid_channel(
    mut synth: *mut fluid_synth_t,
    mut num: libc::c_int,
) -> *mut fluid_channel_t {
    let mut chan: *mut fluid_channel_t = 0 as *mut fluid_channel_t;
    chan = libc::malloc(::std::mem::size_of::<fluid_channel_t>() as libc::size_t)
        as *mut fluid_channel_t;
    if chan.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_channel_t;
    }
    (*chan).synth = synth;
    (*chan).channum = num;
    (*chan).preset = 0 as *mut fluid_preset_t;
    fluid_channel_init(chan);
    fluid_channel_init_ctrl(chan, 0 as libc::c_int);
    return chan;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_init(mut chan: *mut fluid_channel_t) {
    (*chan).prognum = 0 as libc::c_int as libc::c_uint;
    (*chan).banknum = 0 as libc::c_int as libc::c_uint;
    (*chan).sfontnum = 0 as libc::c_int as libc::c_uint;
    if !(*chan).preset.is_null() {
        if !(*chan).preset.is_null() && (*(*chan).preset).free.is_some() {
            Some((*(*chan).preset).free.expect("non-null function pointer"))
                .expect("non-null function pointer")((*chan).preset);
        }
    }
    (*chan).preset = fluid_synth_find_preset((*chan).synth, (*chan).banknum, (*chan).prognum);
    (*chan).interp_method = FLUID_INTERP_DEFAULT as libc::c_int;
    (*chan).tuning = 0 as *mut fluid_tuning_t;
    (*chan).nrpn_select = 0 as libc::c_int as libc::c_short;
    (*chan).nrpn_active = 0 as libc::c_int as libc::c_short;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_init_ctrl(
    mut chan: *mut fluid_channel_t,
    mut is_all_ctrl_off: libc::c_int,
) {
    let mut i: libc::c_int = 0;
    (*chan).channel_pressure = 0 as libc::c_int as libc::c_short;
    (*chan).pitch_bend = 0x2000 as libc::c_int as libc::c_short;
    i = 0 as libc::c_int;
    while i < GEN_LAST as libc::c_int {
        (*chan).gen[i as usize] = 0.0f32;
        (*chan).gen_abs[i as usize] = 0 as libc::c_int as libc::c_char;
        i += 1
    }
    if is_all_ctrl_off != 0 {
        i = 0 as libc::c_int;
        while i < ALL_SOUND_OFF as libc::c_int {
            if !(i >= EFFECTS_DEPTH1 as libc::c_int && i <= EFFECTS_DEPTH5 as libc::c_int) {
                if !(i >= SOUND_CTRL1 as libc::c_int && i <= SOUND_CTRL10 as libc::c_int) {
                    if !(i == BANK_SELECT_MSB as libc::c_int
                        || i == BANK_SELECT_LSB as libc::c_int
                        || i == VOLUME_MSB as libc::c_int
                        || i == VOLUME_LSB as libc::c_int
                        || i == PAN_MSB as libc::c_int
                        || i == PAN_LSB as libc::c_int)
                    {
                        (*chan).cc[i as usize] = 0 as libc::c_int as libc::c_short
                    }
                }
            }
            i += 1
        }
    } else {
        i = 0 as libc::c_int;
        while i < 128 as libc::c_int {
            (*chan).cc[i as usize] = 0 as libc::c_int as libc::c_short;
            i += 1
        }
    }
    i = 0 as libc::c_int;
    while i < 128 as libc::c_int {
        (*chan).key_pressure[i as usize] = 0 as libc::c_int as libc::c_char;
        i += 1
    }

    (*chan).cc[RPN_LSB as libc::c_int as usize] = 127 as libc::c_int as libc::c_short;
    (*chan).cc[RPN_MSB as libc::c_int as usize] = 127 as libc::c_int as libc::c_short;

    (*chan).cc[NRPN_LSB as libc::c_int as usize] = 127 as libc::c_int as libc::c_short;
    (*chan).cc[NRPN_MSB as libc::c_int as usize] = 127 as libc::c_int as libc::c_short;

    (*chan).cc[EXPRESSION_MSB as libc::c_int as usize] = 127 as libc::c_int as libc::c_short;
    (*chan).cc[EXPRESSION_LSB as libc::c_int as usize] = 127 as libc::c_int as libc::c_short;
    if is_all_ctrl_off == 0 {
        (*chan).pitch_wheel_sensitivity = 2 as libc::c_int as libc::c_short;
        i = SOUND_CTRL1 as libc::c_int;
        while i <= SOUND_CTRL10 as libc::c_int {
            (*chan).cc[i as usize] = 64 as libc::c_int as libc::c_short;
            i += 1
        }
        (*chan).cc[VOLUME_MSB as libc::c_int as usize] = 100 as libc::c_int as libc::c_short;
        (*chan).cc[VOLUME_LSB as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
        (*chan).cc[PAN_MSB as libc::c_int as usize] = 64 as libc::c_int as libc::c_short;
        (*chan).cc[PAN_LSB as libc::c_int as usize] = 0 as libc::c_int as libc::c_short
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_reset(mut chan: *mut fluid_channel_t) {
    fluid_channel_init(chan);
    fluid_channel_init_ctrl(chan, 0 as libc::c_int);
}

#[no_mangle]
pub unsafe extern "C" fn delete_fluid_channel(mut chan: *mut fluid_channel_t) -> libc::c_int {
    if !(*chan).preset.is_null() {
        if !(*chan).preset.is_null() && (*(*chan).preset).free.is_some() {
            Some((*(*chan).preset).free.expect("non-null function pointer"))
                .expect("non-null function pointer")((*chan).preset);
        }
    }
    libc::free(chan as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_preset(
    mut chan: *mut fluid_channel_t,
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    if !(*chan).preset.is_null() && (*(*chan).preset).notify.is_some() {
        Some((*(*chan).preset).notify.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            (*chan).preset,
            FLUID_PRESET_UNSELECTED as libc::c_int,
            (*chan).channum,
        );
    }
    if !preset.is_null() && (*preset).notify.is_some() {
        Some((*preset).notify.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            preset,
            FLUID_PRESET_SELECTED as libc::c_int,
            (*chan).channum,
        );
    }
    if !(*chan).preset.is_null() {
        if !(*chan).preset.is_null() && (*(*chan).preset).free.is_some() {
            Some((*(*chan).preset).free.expect("non-null function pointer"))
                .expect("non-null function pointer")((*chan).preset);
        }
    }
    (*chan).preset = preset;
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_preset(
    mut chan: *mut fluid_channel_t,
) -> *mut fluid_preset_t {
    return (*chan).preset;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_banknum(mut chan: *mut fluid_channel_t) -> libc::c_uint {
    return (*chan).banknum;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_prognum(
    mut chan: *mut fluid_channel_t,
    mut prognum: libc::c_int,
) -> libc::c_int {
    (*chan).prognum = prognum as libc::c_uint;
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_prognum(mut chan: *mut fluid_channel_t) -> libc::c_int {
    return (*chan).prognum as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_banknum(
    mut chan: *mut fluid_channel_t,
    mut banknum: libc::c_uint,
) -> libc::c_int {
    (*chan).banknum = banknum;
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_cc(
    mut chan: *mut fluid_channel_t,
    mut num: libc::c_int,
    mut value: libc::c_int,
) -> libc::c_int {
    (*chan).cc[num as usize] = value as libc::c_short;
    match num {
        64 => {
            if value < 64 as libc::c_int {
                fluid_synth_damp_voices((*chan).synth, (*chan).channum);
            }
        }
        0 => {
            if (*chan).channum == 9 as libc::c_int
                && fluid_settings_str_equal(
                    (*(*chan).synth).settings,
                    b"synth.drums-channel.active\x00" as *const u8 as *const libc::c_char,
                    b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
            {
                return FLUID_OK as libc::c_int;
            }
            (*chan).bank_msb = (value & 0x7f as libc::c_int) as libc::c_uchar;

            fluid_channel_set_banknum(chan, (value & 0x7f as libc::c_int) as libc::c_uint);
        }
        32 => {
            if (*chan).channum == 9 as libc::c_int
                && fluid_settings_str_equal(
                    (*(*chan).synth).settings,
                    b"synth.drums-channel.active\x00" as *const u8 as *const libc::c_char,
                    b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
                ) != 0
            {
                return FLUID_OK as libc::c_int;
            }

            fluid_channel_set_banknum(
                chan,
                (value as libc::c_uint & 0x7f as libc::c_int as libc::c_uint)
                    .wrapping_add(((*chan).bank_msb as libc::c_uint) << 7 as libc::c_int),
            );
        }
        123 => {
            fluid_synth_all_notes_off((*chan).synth, (*chan).channum);
        }
        120 => {
            fluid_synth_all_sounds_off((*chan).synth, (*chan).channum);
        }
        121 => {
            fluid_channel_init_ctrl(chan, 1 as libc::c_int);
            fluid_synth_modulate_voices_all((*chan).synth, (*chan).channum);
        }
        6 => {
            let mut data: libc::c_int = (value << 7 as libc::c_int)
                + (*chan).cc[DATA_ENTRY_LSB as libc::c_int as usize] as libc::c_int;
            if (*chan).nrpn_active != 0 {
                if (*chan).cc[NRPN_MSB as libc::c_int as usize] as libc::c_int == 120 as libc::c_int
                    && ((*chan).cc[NRPN_LSB as libc::c_int as usize] as libc::c_int)
                        < 100 as libc::c_int
                {
                    if ((*chan).nrpn_select as libc::c_int) < GEN_LAST as libc::c_int {
                        let mut val: libc::c_float =
                            fluid_gen_scale_nrpn((*chan).nrpn_select as libc::c_int, data);
                        fluid_synth_set_gen(
                            (*chan).synth,
                            (*chan).channum,
                            (*chan).nrpn_select as libc::c_int,
                            val,
                        );
                    }
                    (*chan).nrpn_select = 0 as libc::c_int as libc::c_short
                }
            } else if (*chan).cc[RPN_MSB as libc::c_int as usize] as libc::c_int == 0 as libc::c_int
            {
                match (*chan).cc[RPN_LSB as libc::c_int as usize] as libc::c_int {
                    0 => {
                        fluid_channel_pitch_wheel_sens(chan, value);
                    }
                    1 => {
                        fluid_synth_set_gen(
                            (*chan).synth,
                            (*chan).channum,
                            GEN_FINETUNE as libc::c_int,
                            ((data - 8192 as libc::c_int) as libc::c_double / 8192.0f64 * 100.0f64)
                                as libc::c_float,
                        );
                    }
                    2 => {
                        fluid_synth_set_gen(
                            (*chan).synth,
                            (*chan).channum,
                            GEN_COARSETUNE as libc::c_int,
                            (value - 64 as libc::c_int) as libc::c_float,
                        );
                    }
                    3 | 4 | 5 | _ => {}
                }
            }
        }
        99 => {
            (*chan).cc[NRPN_LSB as libc::c_int as usize] = 0 as libc::c_int as libc::c_short;
            (*chan).nrpn_select = 0 as libc::c_int as libc::c_short;
            (*chan).nrpn_active = 1 as libc::c_int as libc::c_short
        }
        98 => {
            if (*chan).cc[NRPN_MSB as libc::c_int as usize] as libc::c_int == 120 as libc::c_int {
                if value == 100 as libc::c_int {
                    (*chan).nrpn_select =
                        ((*chan).nrpn_select as libc::c_int + 100 as libc::c_int) as libc::c_short
                } else if value == 101 as libc::c_int {
                    (*chan).nrpn_select =
                        ((*chan).nrpn_select as libc::c_int + 1000 as libc::c_int) as libc::c_short
                } else if value == 102 as libc::c_int {
                    (*chan).nrpn_select =
                        ((*chan).nrpn_select as libc::c_int + 10000 as libc::c_int) as libc::c_short
                } else if value < 100 as libc::c_int {
                    (*chan).nrpn_select =
                        ((*chan).nrpn_select as libc::c_int + value) as libc::c_short
                }
            }
            (*chan).nrpn_active = 1 as libc::c_int as libc::c_short
        }
        101 | 100 => (*chan).nrpn_active = 0 as libc::c_int as libc::c_short,
        _ => {
            fluid_synth_modulate_voices((*chan).synth, (*chan).channum, 1 as libc::c_int, num);
        }
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_cc(
    mut chan: *mut fluid_channel_t,
    mut num: libc::c_int,
) -> libc::c_int {
    return if num >= 0 as libc::c_int && num < 128 as libc::c_int {
        (*chan).cc[num as usize] as libc::c_int
    } else {
        0 as libc::c_int
    };
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_pressure(
    mut chan: *mut fluid_channel_t,
    mut val: libc::c_int,
) -> libc::c_int {
    (*chan).channel_pressure = val as libc::c_short;
    fluid_synth_modulate_voices(
        (*chan).synth,
        (*chan).channum,
        0 as libc::c_int,
        FLUID_MOD_CHANNELPRESSURE as libc::c_int,
    );
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_pitch_bend(
    mut chan: *mut fluid_channel_t,
    mut val: libc::c_int,
) -> libc::c_int {
    (*chan).pitch_bend = val as libc::c_short;
    fluid_synth_modulate_voices(
        (*chan).synth,
        (*chan).channum,
        0 as libc::c_int,
        FLUID_MOD_PITCHWHEEL as libc::c_int,
    );
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_pitch_wheel_sens(
    mut chan: *mut fluid_channel_t,
    mut val: libc::c_int,
) -> libc::c_int {
    (*chan).pitch_wheel_sensitivity = val as libc::c_short;
    fluid_synth_modulate_voices(
        (*chan).synth,
        (*chan).channum,
        0 as libc::c_int,
        FLUID_MOD_PITCHWHEELSENS as libc::c_int,
    );
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_num(mut chan: *mut fluid_channel_t) -> libc::c_int {
    return (*chan).channum;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_interp_method(
    mut chan: *mut fluid_channel_t,
    mut new_method: libc::c_int,
) {
    (*chan).interp_method = new_method;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_interp_method(
    mut chan: *mut fluid_channel_t,
) -> libc::c_int {
    return (*chan).interp_method;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_sfontnum(
    mut chan: *mut fluid_channel_t,
) -> libc::c_uint {
    return (*chan).sfontnum;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_sfontnum(
    mut chan: *mut fluid_channel_t,
    mut sfontnum: libc::c_uint,
) -> libc::c_int {
    (*chan).sfontnum = sfontnum;
    return FLUID_OK as libc::c_int;
}
