#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::gen::fluid_gen_scale_nrpn;
use crate::settings::fluid_settings_str_equal;
use crate::sfont::fluid_preset_t;
use crate::synth::fluid_synth_all_notes_off;
use crate::synth::fluid_synth_all_sounds_off;
use crate::synth::fluid_synth_damp_voices;
use crate::synth::fluid_synth_find_preset;
use crate::synth::fluid_synth_modulate_voices;
use crate::synth::fluid_synth_modulate_voices_all;
use crate::synth::fluid_synth_set_gen;
use crate::synth::fluid_synth_t;
use crate::tuning::fluid_tuning_t;
pub type fluid_real_t = libc::c_float;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_channel_t {
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
pub type fluid_interp = libc::c_uint;
pub const FLUID_INTERP_DEFAULT: fluid_interp = 4;
pub type C2RustUnnamed = libc::c_uint;
pub const FLUID_PRESET_UNSELECTED: C2RustUnnamed = 1;
pub const FLUID_PRESET_SELECTED: C2RustUnnamed = 0;
pub type fluid_mod_src = libc::c_uint;
pub const FLUID_MOD_PITCHWHEELSENS: fluid_mod_src = 16;
pub const FLUID_MOD_PITCHWHEEL: fluid_mod_src = 14;
pub const FLUID_MOD_CHANNELPRESSURE: fluid_mod_src = 13;
pub type fluid_gen_type = libc::c_uint;
pub const GEN_LAST: fluid_gen_type = 60;
pub const GEN_FINETUNE: fluid_gen_type = 52;
pub const GEN_COARSETUNE: fluid_gen_type = 51;
pub type C2RustUnnamed_0 = libc::c_int;
pub const FLUID_OK: C2RustUnnamed_0 = 0;
pub type fluid_midi_control_change = libc::c_uint;
pub const ALL_SOUND_OFF: fluid_midi_control_change = 120;
pub const RPN_MSB: fluid_midi_control_change = 101;
pub const RPN_LSB: fluid_midi_control_change = 100;
pub const NRPN_MSB: fluid_midi_control_change = 99;
pub const NRPN_LSB: fluid_midi_control_change = 98;
pub const EFFECTS_DEPTH5: fluid_midi_control_change = 95;
pub const EFFECTS_DEPTH1: fluid_midi_control_change = 91;
pub const SOUND_CTRL10: fluid_midi_control_change = 79;
pub const SOUND_CTRL1: fluid_midi_control_change = 70;
pub const EXPRESSION_LSB: fluid_midi_control_change = 43;
pub const PAN_LSB: fluid_midi_control_change = 42;
pub const VOLUME_LSB: fluid_midi_control_change = 39;
pub const DATA_ENTRY_LSB: fluid_midi_control_change = 38;
pub const BANK_SELECT_LSB: fluid_midi_control_change = 32;
pub const EXPRESSION_MSB: fluid_midi_control_change = 11;
pub const PAN_MSB: fluid_midi_control_change = 10;
pub const VOLUME_MSB: fluid_midi_control_change = 7;
pub const BANK_SELECT_MSB: fluid_midi_control_change = 0;
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
