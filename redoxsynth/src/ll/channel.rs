use super::gen::fluid_gen_scale_nrpn;
use super::settings::fluid_settings_str_equal;
use super::sfont::Preset;
use super::synth::fluid_synth_all_notes_off;
use super::synth::fluid_synth_all_sounds_off;
use super::synth::fluid_synth_damp_voices;
use super::synth::fluid_synth_find_preset;
use super::synth::fluid_synth_modulate_voices;
use super::synth::fluid_synth_modulate_voices_all;
use super::synth::fluid_synth_set_gen;
use super::synth::Synth;
use super::tuning::Tuning;
#[derive(Clone)]
pub struct Channel {
    pub(crate) channum: i32,
    sfontnum: u32,
    banknum: u32,
    prognum: u32,
    pub(crate) preset: *mut Preset,
    pub(crate) key_pressure: [i8; 128],
    pub(crate) channel_pressure: i16,
    pub(crate) pitch_bend: i16,
    pub(crate) pitch_wheel_sensitivity: i16,
    pub(crate) cc: [i16; 128],
    bank_msb: u8,
    interp_method: i32,
    pub(crate) tuning: *mut Tuning,
    nrpn_select: i16,
    nrpn_active: i16,
    pub(crate) gen: [f32; 60],
    pub(crate) gen_abs: [i8; 60],
}
pub type InterpolationType = u32;
pub const INTERPOLATION_DEFAULT: InterpolationType = 4;
pub type C2RustUnnamed = u32;
pub const FLUID_PRESET_UNSELECTED: C2RustUnnamed = 1;
pub const FLUID_PRESET_SELECTED: C2RustUnnamed = 0;
pub type ModSrc = u32;
pub const FLUID_MOD_PITCHWHEELSENS: ModSrc = 16;
pub const FLUID_MOD_PITCHWHEEL: ModSrc = 14;
pub const FLUID_MOD_CHANNELPRESSURE: ModSrc = 13;
pub type GenType = u32;
pub const GEN_LAST: GenType = 60;
pub const GEN_FINETUNE: GenType = 52;
pub const GEN_COARSETUNE: GenType = 51;
pub const FLUID_OK: i32 = 0;
pub type MidiControlChange = u32;
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

pub fn new_fluid_channel(synth: &Synth, num: i32) -> Channel {
    let mut chan = Channel {
        channum: num,
        sfontnum: 0 as _,
        banknum: 0 as _,
        prognum: 0 as _,
        preset: 0 as _,
        key_pressure: [0; 128],
        channel_pressure: 0 as _,
        pitch_bend: 0 as _,
        pitch_wheel_sensitivity: 0 as _,
        cc: [0; 128],
        bank_msb: 0 as _,
        interp_method: 0 as _,
        tuning: 0 as _,
        nrpn_select: 0 as _,
        nrpn_active: 0 as _,
        gen: [0f32; 60],
        gen_abs: [0; 60],
    };
    fluid_channel_init(&mut chan, synth);
    fluid_channel_init_ctrl(&mut chan, 0 as i32);
    return chan;
}

pub fn fluid_channel_init(chan: &mut Channel, synth: &Synth) {
    chan.prognum = 0 as i32 as u32;
    chan.banknum = 0 as i32 as u32;
    chan.sfontnum = 0 as i32 as u32;
    match unsafe { chan.preset.as_ref() } {
        Some(preset) => match preset.free {
            Some(free) => { unsafe { free(chan.preset); } },
            _ => {},
        },
        _ => {},
    }
    chan.preset = unsafe { fluid_synth_find_preset(synth, chan.banknum, chan.prognum) };
    chan.interp_method = INTERPOLATION_DEFAULT as i32;
    chan.tuning = 0 as *mut Tuning;
    chan.nrpn_select = 0 as i32 as i16;
    chan.nrpn_active = 0 as i32 as i16;
}

pub fn fluid_channel_init_ctrl(chan: &mut Channel, is_all_ctrl_off: i32) {
    chan.channel_pressure = 0 as i32 as i16;
    chan.pitch_bend = 0x2000 as i32 as i16;
    let mut i = 0 as i32;
    while i < GEN_LAST as i32 {
        chan.gen[i as usize] = 0.0f32;
        chan.gen_abs[i as usize] = 0 as i32 as i8;
        i += 1
    }
    if is_all_ctrl_off != 0 {
        i = 0 as i32;
        while i < ALL_SOUND_OFF as i32 {
            if !(i >= EFFECTS_DEPTH1 as i32 && i <= EFFECTS_DEPTH5 as i32) {
                if !(i >= SOUND_CTRL1 as i32 && i <= SOUND_CTRL10 as i32) {
                    if !(i == BANK_SELECT_MSB as i32
                        || i == BANK_SELECT_LSB as i32
                        || i == VOLUME_MSB as i32
                        || i == VOLUME_LSB as i32
                        || i == PAN_MSB as i32
                        || i == PAN_LSB as i32)
                    {
                        chan.cc[i as usize] = 0 as i32 as i16
                    }
                }
            }
            i += 1
        }
    } else {
        i = 0 as i32;
        while i < 128 as i32 {
            chan.cc[i as usize] = 0 as i32 as i16;
            i += 1
        }
    }
    i = 0 as i32;
    while i < 128 as i32 {
        chan.key_pressure[i as usize] = 0 as i32 as i8;
        i += 1
    }
    chan.cc[RPN_LSB as i32 as usize] = 127 as i32 as i16;
    chan.cc[RPN_MSB as i32 as usize] = 127 as i32 as i16;
    chan.cc[NRPN_LSB as i32 as usize] = 127 as i32 as i16;
    chan.cc[NRPN_MSB as i32 as usize] = 127 as i32 as i16;
    chan.cc[EXPRESSION_MSB as i32 as usize] = 127 as i32 as i16;
    chan.cc[EXPRESSION_LSB as i32 as usize] = 127 as i32 as i16;
    if is_all_ctrl_off == 0 {
        chan.pitch_wheel_sensitivity = 2 as i32 as i16;
        i = SOUND_CTRL1 as i32;
        while i <= SOUND_CTRL10 as i32 {
            chan.cc[i as usize] = 64 as i32 as i16;
            i += 1
        }
        chan.cc[VOLUME_MSB as i32 as usize] = 100 as i32 as i16;
        chan.cc[VOLUME_LSB as i32 as usize] = 0 as i32 as i16;
        chan.cc[PAN_MSB as i32 as usize] = 64 as i32 as i16;
        chan.cc[PAN_LSB as i32 as usize] = 0 as i32 as i16
    };
}

pub fn fluid_channel_reset(chan: &mut Channel, synth: &Synth) {
    fluid_channel_init(chan, synth);
    fluid_channel_init_ctrl(chan, 0 as i32);
}

impl Drop for Channel {
    fn drop(&mut self) {
        match unsafe { self.preset.as_ref() } {
            Some(preset) => match preset.free {
                Some(free) => { unsafe { free(self.preset); } },
                _ => {},
            },
            _ => {},
        }
    }
}

pub fn fluid_channel_set_preset(chan: &mut Channel, preset: *mut Preset) -> i32 {
    unsafe {
        if !chan.preset.is_null() && (*chan.preset).notify.is_some() {
            Some((*chan.preset).notify.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                chan.preset,
                FLUID_PRESET_UNSELECTED as i32,
                chan.channum,
            );
        }
        if !preset.is_null() && (*preset).notify.is_some() {
            Some((*preset).notify.expect("non-null function pointer"))
                .expect("non-null function pointer")(
                preset,
                FLUID_PRESET_SELECTED as i32,
                chan.channum,
            );
        }
        if !chan.preset.is_null() {
            if !chan.preset.is_null() && (*chan.preset).free.is_some() {
                Some((*chan.preset).free.expect("non-null function pointer"))
                    .expect("non-null function pointer")(chan.preset);
            }
        }
    }
    chan.preset = preset;
    return FLUID_OK as i32;
}

pub fn fluid_channel_get_preset(chan: &Channel) -> *mut Preset {
    return chan.preset;
}

pub fn fluid_channel_get_banknum(chan: &Channel) -> u32 {
    return chan.banknum;
}

pub fn fluid_channel_set_prognum(
    chan: &mut Channel,
    prognum: i32,
) -> i32 {
    chan.prognum = prognum as u32;
    return FLUID_OK as i32;
}

pub fn fluid_channel_get_prognum(chan: &Channel) -> i32 {
    return chan.prognum as i32;
}

pub fn fluid_channel_set_banknum(
    chan: &mut Channel,
    banknum: u32,
) -> i32 {
    chan.banknum = banknum;
    return FLUID_OK as i32;
}

pub fn fluid_channel_cc(
    chan: &mut Channel,
    synth: &mut Synth,
    num: i32,
    value: i32,
) -> i32 {
    unsafe {
        chan.cc[num as usize] = value as i16;
        match num {
            64 => {
                if value < 64 as i32 {
                    fluid_synth_damp_voices(synth, chan.channum);
                }
            }
            0 => {
                if chan.channum == 9 as i32
                    && fluid_settings_str_equal(
                        &synth.settings,
                        "synth.drums-channel.active",
                        "yes",
                    ) != false
                {
                    return FLUID_OK as i32;
                }
                chan.bank_msb = (value & 0x7f as i32) as u8;
                fluid_channel_set_banknum(chan, (value & 0x7f as i32) as u32);
            }
            32 => {
                if chan.channum == 9 as i32
                    && fluid_settings_str_equal(
                        &synth.settings,
                        "synth.drums-channel.active",
                        "yes",
                    ) != false
                {
                    return FLUID_OK as i32;
                }
                fluid_channel_set_banknum(
                    chan,
                    (value as u32 & 0x7f as i32 as u32)
                        .wrapping_add((chan.bank_msb as u32) << 7 as i32),
                );
            }
            123 => {
                fluid_synth_all_notes_off(synth, chan.channum);
            }
            120 => {
                fluid_synth_all_sounds_off(synth, chan.channum);
            }
            121 => {
                fluid_channel_init_ctrl(chan, 1 as i32);
                fluid_synth_modulate_voices_all(synth, chan.channum);
            }
            6 => {
                let data: i32 = (value << 7 as i32)
                    + chan.cc[DATA_ENTRY_LSB as i32 as usize] as i32;
                if chan.nrpn_active != 0 {
                    if chan.cc[NRPN_MSB as i32 as usize] as i32 == 120 as i32
                        && (chan.cc[NRPN_LSB as i32 as usize] as i32)
                            < 100 as i32
                    {
                        if (chan.nrpn_select as i32) < GEN_LAST as i32 {
                            let val: f32 =
                                fluid_gen_scale_nrpn(chan.nrpn_select as i32, data);
                            fluid_synth_set_gen(
                                synth,
                                chan.channum,
                                chan.nrpn_select as i32,
                                val,
                            );
                        }
                        chan.nrpn_select = 0 as i32 as i16
                    }
                } else if chan.cc[RPN_MSB as i32 as usize] as i32 == 0 as i32
                {
                    match chan.cc[RPN_LSB as i32 as usize] as i32 {
                        0 => {
                            fluid_channel_pitch_wheel_sens(chan, synth, value);
                        }
                        1 => {
                            fluid_synth_set_gen(
                                synth,
                                chan.channum,
                                GEN_FINETUNE as i32,
                                ((data - 8192 as i32) as f64 / 8192.0f64 * 100.0f64)
                                    as f32,
                            );
                        }
                        2 => {
                            fluid_synth_set_gen(
                                synth,
                                chan.channum,
                                GEN_COARSETUNE as i32,
                                (value - 64 as i32) as f32,
                            );
                        }
                        3 | 4 | 5 | _ => {}
                    }
                }
            }
            99 => {
                chan.cc[NRPN_LSB as i32 as usize] = 0 as i32 as i16;
                chan.nrpn_select = 0 as i32 as i16;
                chan.nrpn_active = 1 as i32 as i16
            }
            98 => {
                if chan.cc[NRPN_MSB as i32 as usize] as i32 == 120 as i32 {
                    if value == 100 as i32 {
                        chan.nrpn_select =
                            (chan.nrpn_select as i32 + 100 as i32) as i16
                    } else if value == 101 as i32 {
                        chan.nrpn_select =
                            (chan.nrpn_select as i32 + 1000 as i32) as i16
                    } else if value == 102 as i32 {
                        chan.nrpn_select =
                            (chan.nrpn_select as i32 + 10000 as i32) as i16
                    } else if value < 100 as i32 {
                        chan.nrpn_select =
                            (chan.nrpn_select as i32 + value) as i16
                    }
                }
                chan.nrpn_active = 1 as i32 as i16
            }
            101 | 100 => chan.nrpn_active = 0 as i32 as i16,
            _ => {
                fluid_synth_modulate_voices(synth, chan.channum, 1 as i32, num);
            }
        }
    }
    return FLUID_OK as i32;
}

pub fn fluid_channel_get_cc(chan: &Channel, num: i32) -> i32 {
    return if num >= 0 as i32 && num < 128 as i32 {
        chan.cc[num as usize] as i32
    } else {
        0 as i32
    };
}

pub fn fluid_channel_pressure(chan: &mut Channel, synth: &mut Synth, val: i32) -> i32 {
    chan.channel_pressure = val as i16;
    unsafe {
        fluid_synth_modulate_voices(
            synth,
            chan.channum,
            0 as i32,
            FLUID_MOD_CHANNELPRESSURE as i32,
        );
    }
    return FLUID_OK as i32;
}

pub fn fluid_channel_pitch_bend(chan: &mut Channel, synth: &mut Synth, val: i32) -> i32 {
    chan.pitch_bend = val as i16;
    unsafe {
        fluid_synth_modulate_voices(
            synth,
            chan.channum,
            0 as i32,
            FLUID_MOD_PITCHWHEEL as i32,
        );
    }
    return FLUID_OK as i32;
}

pub fn fluid_channel_pitch_wheel_sens(
    chan: &mut Channel,
    synth: &mut Synth,
    val: i32,
) -> i32 {
    chan.pitch_wheel_sensitivity = val as i16;
    unsafe {
        fluid_synth_modulate_voices(
            synth,
            chan.channum,
            0 as i32,
            FLUID_MOD_PITCHWHEELSENS as i32,
        );
    }
    return FLUID_OK as i32;
}

pub fn fluid_channel_get_num(chan: &Channel) -> i32 {
    return chan.channum;
}

pub fn fluid_channel_set_interp_method(chan: &mut Channel, new_method: i32) {
    chan.interp_method = new_method;
}

pub fn fluid_channel_get_interp_method(chan: &Channel) -> i32 {
    return chan.interp_method;
}

pub fn fluid_channel_get_sfontnum(chan: &Channel) -> u32 {
    return chan.sfontnum;
}

pub fn fluid_channel_set_sfontnum(
    chan: &mut Channel,
    sfontnum: u32,
) -> i32 {
    chan.sfontnum = sfontnum;
    return FLUID_OK as i32;
}
