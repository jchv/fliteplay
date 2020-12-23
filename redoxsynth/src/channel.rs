use crate::gen::fluid_gen_scale_nrpn;
use crate::settings::fluid_settings_str_equal;
use crate::sfont::Preset;
use crate::synth::fluid_synth_all_notes_off;
use crate::synth::fluid_synth_all_sounds_off;
use crate::synth::fluid_synth_damp_voices;
use crate::synth::fluid_synth_find_preset;
use crate::synth::fluid_synth_modulate_voices;
use crate::synth::fluid_synth_modulate_voices_all;
use crate::synth::fluid_synth_set_gen;
use crate::synth::fluid_synth_t;
use crate::tuning::Tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Channel {
    pub channum: libc::c_int,
    pub sfontnum: libc::c_uint,
    pub banknum: libc::c_uint,
    pub prognum: libc::c_uint,
    pub preset: *mut Preset,
    pub synth: *mut fluid_synth_t,
    pub key_pressure: [libc::c_char; 128],
    pub channel_pressure: libc::c_short,
    pub pitch_bend: libc::c_short,
    pub pitch_wheel_sensitivity: libc::c_short,
    pub cc: [libc::c_short; 128],
    pub bank_msb: libc::c_uchar,
    pub interp_method: libc::c_int,
    pub tuning: *mut Tuning,
    pub nrpn_select: libc::c_short,
    pub nrpn_active: libc::c_short,
    pub gen: [f32; 60],
    pub gen_abs: [libc::c_char; 60],
}
pub type InterpolationType = libc::c_uint;
pub const INTERPOLATION_DEFAULT: InterpolationType = 4;
pub type C2RustUnnamed = libc::c_uint;
pub const FLUID_PRESET_UNSELECTED: C2RustUnnamed = 1;
pub const FLUID_PRESET_SELECTED: C2RustUnnamed = 0;
pub type ModSrc = libc::c_uint;
pub const FLUID_MOD_PITCHWHEELSENS: ModSrc = 16;
pub const FLUID_MOD_PITCHWHEEL: ModSrc = 14;
pub const FLUID_MOD_CHANNELPRESSURE: ModSrc = 13;
pub type GenType = libc::c_uint;
pub const GEN_LAST: GenType = 60;
pub const GEN_FINETUNE: GenType = 52;
pub const GEN_COARSETUNE: GenType = 51;
pub const FLUID_OK: libc::c_int = 0;
pub type MidiControlChange = libc::c_uint;
pub const ALL_SOUND_OFF: MidiControlChange = 120;
pub const RPN_MSB: MidiControlChange = 101;
pub const RPN_LSB: MidiControlChange = 100;
pub const NRPN_MSB: MidiControlChange = 99;
pub const NRPN_LSB: MidiControlChange = 98;
pub const EFFECTS_DEPTH5: MidiControlChange = 95;
pub const EFFECTS_DEPTH1: MidiControlChange = 91;
pub const SOUND_CTRL10: MidiControlChange = 79;
pub const SOUND_CTRL1: MidiControlChange = 70;
pub const EXPRESSION_LSB: MidiControlChange = 43;
pub const PAN_LSB: MidiControlChange = 42;
pub const VOLUME_LSB: MidiControlChange = 39;
pub const DATA_ENTRY_LSB: MidiControlChange = 38;
pub const BANK_SELECT_LSB: MidiControlChange = 32;
pub const EXPRESSION_MSB: MidiControlChange = 11;
pub const PAN_MSB: MidiControlChange = 10;
pub const VOLUME_MSB: MidiControlChange = 7;
pub const BANK_SELECT_MSB: MidiControlChange = 0;
#[no_mangle]
pub unsafe extern "C" fn new_fluid_channel(
    synth: *mut fluid_synth_t,
    num: libc::c_int,
) -> *mut Channel {
    let mut chan = libc::malloc(::std::mem::size_of::<Channel>() as libc::size_t)
        as *mut Channel;
    if chan.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut Channel;
    }
    (*chan).synth = synth;
    (*chan).channum = num;
    (*chan).preset = 0 as *mut Preset;
    fluid_channel_init(chan);
    fluid_channel_init_ctrl(chan, 0 as libc::c_int);
    return chan;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_init(mut chan: *mut Channel) {
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
    (*chan).interp_method = INTERPOLATION_DEFAULT as libc::c_int;
    (*chan).tuning = 0 as *mut Tuning;
    (*chan).nrpn_select = 0 as libc::c_int as libc::c_short;
    (*chan).nrpn_active = 0 as libc::c_int as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_init_ctrl(
    mut chan: *mut Channel,
    is_all_ctrl_off: libc::c_int,
) {
    (*chan).channel_pressure = 0 as libc::c_int as libc::c_short;
    (*chan).pitch_bend = 0x2000 as libc::c_int as libc::c_short;
    let mut i = 0 as libc::c_int;
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
pub unsafe extern "C" fn fluid_channel_reset(chan: *mut Channel) {
    fluid_channel_init(chan);
    fluid_channel_init_ctrl(chan, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_channel(chan: *mut Channel) -> libc::c_int {
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
    mut chan: *mut Channel,
    preset: *mut Preset,
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
    chan: *mut Channel,
) -> *mut Preset {
    return (*chan).preset;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_banknum(chan: *mut Channel) -> libc::c_uint {
    return (*chan).banknum;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_prognum(
    mut chan: *mut Channel,
    prognum: libc::c_int,
) -> libc::c_int {
    (*chan).prognum = prognum as libc::c_uint;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_prognum(chan: *mut Channel) -> libc::c_int {
    return (*chan).prognum as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_banknum(
    mut chan: *mut Channel,
    banknum: libc::c_uint,
) -> libc::c_int {
    (*chan).banknum = banknum;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_cc(
    mut chan: *mut Channel,
    num: libc::c_int,
    value: libc::c_int,
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
            let data: libc::c_int = (value << 7 as libc::c_int)
                + (*chan).cc[DATA_ENTRY_LSB as libc::c_int as usize] as libc::c_int;
            if (*chan).nrpn_active != 0 {
                if (*chan).cc[NRPN_MSB as libc::c_int as usize] as libc::c_int == 120 as libc::c_int
                    && ((*chan).cc[NRPN_LSB as libc::c_int as usize] as libc::c_int)
                        < 100 as libc::c_int
                {
                    if ((*chan).nrpn_select as libc::c_int) < GEN_LAST as libc::c_int {
                        let val: libc::c_float =
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
                            ((data - 8192 as libc::c_int) as f64 / 8192.0f64 * 100.0f64)
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
    chan: *mut Channel,
    num: libc::c_int,
) -> libc::c_int {
    return if num >= 0 as libc::c_int && num < 128 as libc::c_int {
        (*chan).cc[num as usize] as libc::c_int
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_pressure(
    chan: *mut Channel,
    val: libc::c_int,
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
    mut chan: *mut Channel,
    val: libc::c_int,
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
    mut chan: *mut Channel,
    val: libc::c_int,
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
pub unsafe extern "C" fn fluid_channel_get_num(chan: *mut Channel) -> libc::c_int {
    return (*chan).channum;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_interp_method(
    mut chan: *mut Channel,
    new_method: libc::c_int,
) {
    (*chan).interp_method = new_method;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_interp_method(
    chan: *mut Channel,
) -> libc::c_int {
    return (*chan).interp_method;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_get_sfontnum(
    chan: *mut Channel,
) -> libc::c_uint {
    return (*chan).sfontnum;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_channel_set_sfontnum(
    mut chan: *mut Channel,
    sfontnum: libc::c_uint,
) -> libc::c_int {
    (*chan).sfontnum = sfontnum;
    return FLUID_OK as libc::c_int;
}
