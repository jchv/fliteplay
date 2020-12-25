use super::channel::fluid_channel_cc;
use super::channel::fluid_channel_get_banknum;
use super::channel::fluid_channel_get_num;
use super::channel::fluid_channel_get_preset;
use super::channel::fluid_channel_get_prognum;
use super::channel::fluid_channel_get_sfontnum;
use super::channel::fluid_channel_pitch_bend;
use super::channel::fluid_channel_pitch_wheel_sens;
use super::channel::fluid_channel_pressure;
use super::channel::fluid_channel_reset;
use super::channel::fluid_channel_set_banknum;
use super::channel::fluid_channel_set_interp_method;
use super::channel::fluid_channel_set_preset;
use super::channel::fluid_channel_set_prognum;
use super::channel::fluid_channel_set_sfontnum;
use super::channel::new_fluid_channel;
use super::channel::Channel;
use super::chorus::Chorus;
use super::defsfont::new_fluid_defsfloader;
use super::dsp_float::fluid_dsp_float_config;
use super::hash::HashTable;
use super::list::delete_fluid_list;
use super::list::fluid_list_insert_at;
use super::list::fluid_list_nth;
use super::list::fluid_list_prepend;
use super::list::fluid_list_remove;
use super::list::fluid_list_size;
use super::list::List;
use super::modulator::fluid_mod_set_amount;
use super::modulator::fluid_mod_set_dest;
use super::modulator::fluid_mod_set_source1;
use super::modulator::fluid_mod_set_source2;
use super::modulator::Mod;
use super::reverb::ReverbModel;
use super::settings::fluid_settings_getint;
use super::settings::fluid_settings_getnum;
use super::settings::fluid_settings_register_int;
use super::settings::fluid_settings_register_num;
use super::settings::fluid_settings_register_str;
use super::settings::fluid_settings_setint;
use super::settings::fluid_settings_str_equal;
use super::sfont::Preset;
use super::sfont::Sample;
use super::sfont::SoundFont;
use super::sfont::SoundfontLoader;
use super::sys::fluid_error;
use super::sys::fluid_sys_config;
use super::tuning::fluid_tuning_get_name;
use super::tuning::fluid_tuning_set_all;
use super::tuning::fluid_tuning_set_name;
use super::tuning::fluid_tuning_set_octave;
use super::tuning::fluid_tuning_set_pitch;
use super::tuning::new_fluid_tuning;
use super::tuning::Tuning;
use super::voice::delete_fluid_voice;
use super::voice::fluid_voice_add_mod;
use super::voice::fluid_voice_get_channel;
use super::voice::fluid_voice_get_id;
use super::voice::fluid_voice_init;
use super::voice::fluid_voice_is_playing;
use super::voice::fluid_voice_kill_excl;
use super::voice::fluid_voice_modulate;
use super::voice::fluid_voice_modulate_all;
use super::voice::fluid_voice_noteoff;
use super::voice::fluid_voice_off;
use super::voice::fluid_voice_set_gain;
use super::voice::fluid_voice_set_param;
use super::voice::fluid_voice_start;
use super::voice::Voice;
use super::voice::fluid_voice_write;
use super::voice::new_fluid_voice;
use super::voice::FluidVoiceAddMod;
use std::ffi::CStr;

pub type Settings = HashTable;
#[derive(Clone)]
pub struct Synth {
    pub(crate) settings: *mut Settings,
    polyphony: i32,
    with_reverb: libc::c_char,
    with_chorus: libc::c_char,
    verbose: libc::c_char,
    dump: libc::c_char,
    sample_rate: f64,
    midi_channels: i32,
    audio_channels: i32,
    audio_groups: i32,
    effects_channels: i32,
    state: u32,
    ticks: u32,
    loaders: *mut List,
    sfont: *mut List,
    sfont_id: u32,
    bank_offsets: *mut List,
    gain: f64,
    channel: Vec<Channel>,
    num_channels: i32,
    nvoice: i32,
    voice: *mut *mut Voice,
    noteid: u32,
    storeid: u32,
    nbuf: i32,
    left_buf: *mut *mut f32,
    right_buf: *mut *mut f32,
    fx_left_buf: *mut *mut f32,
    fx_right_buf: *mut *mut f32,
    reverb: ReverbModel,
    chorus: Chorus,
    cur: i32,
    dither_index: i32,
    outbuf: [libc::c_char; 256],
    tuning: *mut *mut *mut Tuning,
    cur_tuning: *mut Tuning,
    pub(crate) min_note_length_ticks: u32,
}

impl Synth {
    pub fn new(settings: *mut Settings) -> Result<Self, &'static str> {
        unsafe {
            let mut current_block: u64;
            let mut i: i32 = 0;
            let loader;
        
            if FLUID_SYNTH_INITIALIZED == 0 as i32 {
                fluid_synth_init();
            }

            let mut synth = Self {
                settings: 0 as _,
                polyphony: 0 as _,
                with_reverb: 0 as _,
                with_chorus: 0 as _,
                verbose: 0 as _,
                dump: 0 as _,
                sample_rate: 0 as _,
                midi_channels: 0 as _,
                audio_channels: 0 as _,
                audio_groups: 0 as _,
                effects_channels: 0 as _,
                state: 0 as _,
                ticks: 0 as _,
                loaders: 0 as _,
                sfont: 0 as _,
                sfont_id: 0 as _,
                bank_offsets: 0 as _,
                gain: 0 as _,
                channel: Vec::new(),
                num_channels: 0 as _,
                nvoice: 0 as _,
                voice: 0 as _,
                noteid: 0 as _,
                storeid: 0 as _,
                nbuf: 0 as _,
                left_buf: 0 as _,
                right_buf: 0 as _,
                fx_left_buf: 0 as _,
                fx_right_buf: 0 as _,
                reverb: ReverbModel::new(),
                chorus: Chorus::new(44100f32),
                cur: 0 as _,
                dither_index: 0 as _,
                outbuf: [0; 256],
                tuning: 0 as _,
                cur_tuning: 0 as _,
                min_note_length_ticks: 0 as _,
            };

            synth.settings = settings;
            synth.with_reverb = fluid_settings_str_equal(
                settings,
                b"synth.reverb.active\x00" as *const u8 as *const libc::c_char,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as libc::c_char;
            synth.with_chorus = fluid_settings_str_equal(
                settings,
                b"synth.chorus.active\x00" as *const u8 as *const libc::c_char,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as libc::c_char;
            synth.verbose = fluid_settings_str_equal(
                settings,
                b"synth.verbose\x00" as *const u8 as *const libc::c_char,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as libc::c_char;
            synth.dump = fluid_settings_str_equal(
                settings,
                b"synth.dump\x00" as *const u8 as *const libc::c_char,
                b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            ) as libc::c_char;
            fluid_settings_getint(
                settings,
                b"synth.polyphony\x00" as *const u8 as *const libc::c_char,
                &mut synth.polyphony,
            );
            fluid_settings_getnum(
                settings,
                b"synth.sample-rate\x00" as *const u8 as *const libc::c_char,
                &mut synth.sample_rate,
            );
            fluid_settings_getint(
                settings,
                b"synth.midi-channels\x00" as *const u8 as *const libc::c_char,
                &mut synth.midi_channels,
            );
            fluid_settings_getint(
                settings,
                b"synth.audio-channels\x00" as *const u8 as *const libc::c_char,
                &mut synth.audio_channels,
            );
            fluid_settings_getint(
                settings,
                b"synth.audio-groups\x00" as *const u8 as *const libc::c_char,
                &mut synth.audio_groups,
            );
            fluid_settings_getint(
                settings,
                b"synth.effects-channels\x00" as *const u8 as *const libc::c_char,
                &mut synth.effects_channels,
            );
            fluid_settings_getnum(
                settings,
                b"synth.gain\x00" as *const u8 as *const libc::c_char,
                &mut synth.gain,
            );
            fluid_settings_getint(
                settings,
                b"synth.min-note-length\x00" as *const u8 as *const libc::c_char,
                &mut i,
            );
            synth.min_note_length_ticks =
                (i as f64 * synth.sample_rate / 1000.0f32 as f64) as u32;
            fluid_settings_register_num(
                settings,
                b"synth.gain\x00" as *const u8 as *const libc::c_char,
                0.2f32 as f64,
                0.0f32 as f64,
                10.0f32 as f64,
                0 as i32,
                ::std::mem::transmute::<
                    Option<unsafe fn(_: *mut Synth, _: *mut libc::c_char, _: f64) -> i32>,
                    NumUpdateFn,
                >(Some(
                    fluid_synth_update_gain
                        as unsafe fn(_: *mut Synth, _: *mut libc::c_char, _: f64) -> i32,
                )),
                &mut synth as *mut Self as *mut libc::c_void,
            );
            fluid_settings_register_int(
                settings,
                b"synth.polyphony\x00" as *const u8 as *const libc::c_char,
                synth.polyphony,
                16 as i32,
                4096 as i32,
                0 as i32,
                ::std::mem::transmute::<
                    Option<
                        unsafe fn(
                            _: *mut Synth,
                            _: *mut libc::c_char,
                            _: i32,
                        ) -> i32,
                    >,
                    IntUpdateFn,
                >(Some(
                    fluid_synth_update_polyphony
                        as unsafe fn(
                            _: *mut Synth,
                            _: *mut libc::c_char,
                            _: i32,
                        ) -> i32,
                )),
                &mut synth as *mut Self as *mut libc::c_void,
            );
            if synth.midi_channels % 16 as i32 != 0 as i32 {
                let n: i32 = synth.midi_channels / 16 as i32;
                synth.midi_channels = (n + 1 as i32) * 16 as i32;
                fluid_settings_setint(
                    settings,
                    b"synth.midi-channels\x00" as *const u8 as *const libc::c_char,
                    synth.midi_channels,
                );
                fluid_log!(FLUID_WARN,
                        "Requested number of MIDI channels is not a multiple of 16. I\'ll increase the number of channels to the next multiple.",
                        );
            }
            if synth.audio_channels < 1 as i32 {
                fluid_log!(
                    FLUID_WARN,
                    "Requested number of audio channels is smaller than 1. Changing this setting to 1.",
                );
                synth.audio_channels = 1 as i32
            } else if synth.audio_channels > 128 as i32 {
                fluid_log!(
                    FLUID_WARN,
                    "Requested number of audio channels is too big ({}). Limiting this setting to 128.",
                    synth.audio_channels
                );
                synth.audio_channels = 128 as i32
            }
            if synth.audio_groups < 1 as i32 {
                fluid_log!(
                    FLUID_WARN,
                    "Requested number of audio groups is smaller than 1. Changing this setting to 1.",
                );
                synth.audio_groups = 1 as i32
            } else if synth.audio_groups > 128 as i32 {
                fluid_log!(
                    FLUID_WARN,
                    "Requested number of audio groups is too big ({}). Limiting this setting to 128.",
                    synth.audio_groups
                );
                synth.audio_groups = 128 as i32
            }
            if synth.effects_channels != 2 as i32 {
                fluid_log!(
                    FLUID_WARN,
                    "Invalid number of effects channels ({}).Setting effects channels to 2.",
                    synth.effects_channels
                );
                synth.effects_channels = 2 as i32
            }
            synth.nbuf = synth.audio_channels;
            if synth.audio_groups > synth.nbuf {
                synth.nbuf = synth.audio_groups
            }
            synth.state = FLUID_SYNTH_PLAYING as i32 as u32;
            synth.sfont = 0 as *mut List;
            synth.noteid = 0 as i32 as u32;
            synth.ticks = 0 as i32 as u32;
            synth.tuning = 0 as *mut *mut *mut Tuning;
            loader = new_fluid_defsfloader();
            if loader.is_null() {
                fluid_log!(FLUID_WARN, "Failed to create the default SoundFont loader",);
            } else {
                fluid_synth_add_sfloader(&mut synth, loader);
            }
            for i in 0..synth.midi_channels {
                synth.channel.push(new_fluid_channel(&synth, i));
            }
            synth.nvoice = synth.polyphony;
            synth.voice = libc::malloc(
                (synth.nvoice as libc::size_t)
                    .wrapping_mul(::std::mem::size_of::<*mut Voice>() as libc::size_t),
            ) as *mut *mut Voice;
            if !synth.voice.is_null() {
                i = 0 as i32;
                loop {
                    if !(i < synth.nvoice) {
                        current_block = 17441561948628420366;
                        break;
                    }
                    let ref mut fresh1 = *synth.voice.offset(i as isize);
                    *fresh1 = new_fluid_voice(synth.sample_rate as f32);
                    if (*synth.voice.offset(i as isize)).is_null() {
                        current_block = 2776114520721993823;
                        break;
                    }
                    i += 1
                }
                match current_block {
                    2776114520721993823 => {}
                    _ => {
                        synth.left_buf = 0 as *mut *mut f32;
                        synth.right_buf = 0 as *mut *mut f32;
                        synth.fx_left_buf = 0 as *mut *mut f32;
                        synth.fx_right_buf = 0 as *mut *mut f32;
                        synth.left_buf = 
                            libc::malloc(
                                (synth.nbuf as libc::size_t).wrapping_mul(
                                    ::std::mem::size_of::<*mut f32>() as libc::size_t,
                                ),
                            ) as *mut *mut f32;
                        synth.right_buf =
                            libc::malloc(
                                (synth.nbuf as libc::size_t).wrapping_mul(
                                    ::std::mem::size_of::<*mut f32>() as libc::size_t,
                                ),
                            ) as *mut *mut f32;
                        if synth.left_buf.is_null() || synth.right_buf.is_null() {
                            fluid_log!(FLUID_ERR, "Out of memory",);
                        } else {
                            libc::memset(
                                synth.left_buf as *mut libc::c_void,
                                0 as i32,
                                (synth.nbuf as libc::size_t).wrapping_mul(
                                    ::std::mem::size_of::<*mut f32>() as libc::size_t,
                                ),
                            );
                            libc::memset(
                                synth.right_buf as *mut libc::c_void,
                                0 as i32,
                                (synth.nbuf as libc::size_t).wrapping_mul(
                                    ::std::mem::size_of::<*mut f32>() as libc::size_t,
                                ),
                            );
                            i = 0 as i32;
                            loop {
                                if !(i < synth.nbuf) {
                                    current_block = 178030534879405462;
                                    break;
                                }
                                let ref mut fresh2 = *synth.left_buf.offset(i as isize);
                                *fresh2 = libc::malloc(
                                    (64 as i32 as libc::size_t).wrapping_mul(
                                        ::std::mem::size_of::<f32>() as libc::size_t,
                                    ),
                                ) as *mut f32;
                                let ref mut fresh3 = *synth.right_buf.offset(i as isize);
                                *fresh3 = libc::malloc(
                                    (64 as i32 as libc::size_t).wrapping_mul(
                                        ::std::mem::size_of::<f32>() as libc::size_t,
                                    ),
                                ) as *mut f32;
                                if (*synth.left_buf.offset(i as isize)).is_null()
                                    || (*synth.right_buf.offset(i as isize)).is_null()
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
                                    synth.fx_left_buf = libc::malloc(
                                        (synth.effects_channels as libc::size_t)
                                            .wrapping_mul(::std::mem::size_of::<*mut f32>()
                                                as libc::size_t),
                                    )
                                        as *mut *mut f32;
                                    synth.fx_right_buf = libc::malloc(
                                        (synth.effects_channels as libc::size_t)
                                            .wrapping_mul(::std::mem::size_of::<*mut f32>()
                                                as libc::size_t),
                                    )
                                        as *mut *mut f32;
                                    if synth.fx_left_buf.is_null()
                                        || synth.fx_right_buf.is_null()
                                    {
                                        fluid_log!(FLUID_ERR, "Out of memory",);
                                    } else {
                                        libc::memset(
                                            synth.fx_left_buf as *mut libc::c_void,
                                            0 as i32,
                                            (2 as i32 as libc::size_t).wrapping_mul(
                                                ::std::mem::size_of::<*mut f32>()
                                                    as libc::size_t,
                                            ),
                                        );
                                        libc::memset(
                                            synth.fx_right_buf as *mut libc::c_void,
                                            0 as i32,
                                            (2 as i32 as libc::size_t).wrapping_mul(
                                                ::std::mem::size_of::<*mut f32>()
                                                    as libc::size_t,
                                            ),
                                        );
                                        i = 0 as i32;
                                        loop {
                                            if !(i < synth.effects_channels) {
                                                current_block = 7739940392431776979;
                                                break;
                                            }
                                            let ref mut fresh4 =
                                                *synth.fx_left_buf.offset(i as isize);
                                            *fresh4 = libc::malloc(
                                                (64 as i32 as libc::size_t)
                                                    .wrapping_mul(::std::mem::size_of::<f32>()
                                                        as libc::size_t),
                                            )
                                                as *mut f32;
                                            let ref mut fresh5 =
                                                *synth.fx_right_buf.offset(i as isize);
                                            *fresh5 = libc::malloc(
                                                (64 as i32 as libc::size_t)
                                                    .wrapping_mul(::std::mem::size_of::<f32>()
                                                        as libc::size_t),
                                            )
                                                as *mut f32;
                                            if (*synth.fx_left_buf.offset(i as isize))
                                                .is_null()
                                                || (*synth.fx_right_buf.offset(i as isize))
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
                                                synth.cur = 64 as i32;
                                                synth.dither_index = 0 as i32;
                                                synth.reverb = ReverbModel::new();
                                                fluid_synth_set_reverb(
                                                    &mut synth,
                                                    0.2f32 as f64,
                                                    0.0f32 as f64,
                                                    0.5f32 as f64,
                                                    0.9f32 as f64,
                                                );
                                                synth.chorus = Chorus::new(
                                                    synth.sample_rate as f32,
                                                );
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
                                                        &mut synth,
                                                        9 as i32,
                                                        128 as i32
                                                            as u32,
                                                    );
                                                }
                                                return Ok(synth);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            return Err("failed");
        }
    }
}

pub const FLUID_OK: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
pub struct BankOffset {
    pub sfont_id: i32,
    pub offset: i32,
}
pub const FLUID_SYNTH_STOPPED: SynthStatus = 3;
pub const FLUID_FAILED: C2RustUnnamed = -1;
pub const FLUID_SYNTH_PLAYING: SynthStatus = 1;
pub type IntUpdateFn =
    Option<unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: i32) -> i32>;
pub const FLUID_VOICE_SUSTAINED: VoiceStatus = 2;
pub const FLUID_VOICE_ON: VoiceStatus = 1;
pub type NumUpdateFn =
    Option<unsafe fn(_: *mut libc::c_void, _: *const libc::c_char, _: f64) -> i32>;
pub const GEN_PITCH: GenType = 59;
pub const FLUID_MOD_POSITIVE: ModFlags = 0;
pub const FLUID_MOD_UNIPOLAR: ModFlags = 0;
pub const FLUID_MOD_LINEAR: ModFlags = 0;
pub const FLUID_MOD_GC: ModFlags = 0;
pub const FLUID_MOD_PITCHWHEELSENS: ModSrc = 16;
pub const FLUID_MOD_BIPOLAR: ModFlags = 2;
pub const FLUID_MOD_PITCHWHEEL: ModSrc = 14;
pub const GEN_CHORUSSEND: GenType = 15;
pub const FLUID_MOD_CC: ModFlags = 16;
pub const GEN_REVERBSEND: GenType = 16;
pub const GEN_ATTENUATION: GenType = 48;
pub const FLUID_MOD_NEGATIVE: ModFlags = 1;
pub const FLUID_MOD_CONCAVE: ModFlags = 4;
pub const GEN_PAN: GenType = 17;
pub const GEN_VIBLFOTOPITCH: GenType = 6;
pub const FLUID_MOD_CHANNELPRESSURE: ModSrc = 13;
pub const GEN_FILTERFC: GenType = 8;
pub const FLUID_MOD_SWITCH: ModFlags = 12;
pub const FLUID_MOD_VELOCITY: ModSrc = 2;
pub const FLUID_VOICE_OFF: VoiceStatus = 3;
pub const FLUID_VOICE_CLEAN: VoiceStatus = 0;
pub const FLUID_VOICE_ENVRELEASE: VoiceEnvelopeIndex = 5;
pub const FLUID_MOD_KEYPRESSURE: ModSrc = 10;
pub const GEN_LAST: GenType = 60;
pub const FLUID_VOICE_DEFAULT: FluidVoiceAddMod = 2;
pub const FLUID_VOICE_ENVATTACK: VoiceEnvelopeIndex = 1;
pub const GEN_EXCLUSIVECLASS: GenType = 57;
pub type ModFlags = u32;
pub type ModSrc = u32;
pub type GenType = u32;
pub type C2RustUnnamed = i32;
#[derive(Copy, Clone)]
pub struct ReverbModelPreset {
    pub name: *mut libc::c_char,
    pub roomsize: f32,
    pub damp: f32,
    pub width: f32,
    pub level: f32,
}
pub type VoiceStatus = u32;
pub type VoiceEnvelopeIndex = u32;
pub type SynthStatus = u32;
static mut FLUID_SYNTH_INITIALIZED: i32 = 0 as i32;

pub static mut DEFAULT_VEL2ATT_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_VEL2FILTER_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_AT2VIBLFO_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_MOD2VIBLFO_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_ATT_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_PAN_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_EXPR_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_REVERB_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_CHORUS_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub static mut DEFAULT_PITCH_BEND_MOD: Mod = Mod {
    dest: 0,
    src1: 0,
    flags1: 0,
    src2: 0,
    flags2: 0,
    amount: 0.,
    next: 0 as *const Mod as *mut Mod,
};

pub unsafe fn fluid_synth_settings(settings: *mut Settings) {
    fluid_settings_register_str(
        settings,
        b"synth.verbose\x00" as *const u8 as *const libc::c_char,
        b"no\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.dump\x00" as *const u8 as *const libc::c_char,
        b"no\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.reverb.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.chorus.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.ladspa.active\x00" as *const u8 as *const libc::c_char,
        b"no\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"midi.portname\x00" as *const u8 as *const libc::c_char,
        b"\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_str(
        settings,
        b"synth.drums-channel.active\x00" as *const u8 as *const libc::c_char,
        b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.polyphony\x00" as *const u8 as *const libc::c_char,
        256 as i32,
        16 as i32,
        4096 as i32,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.midi-channels\x00" as *const u8 as *const libc::c_char,
        16 as i32,
        16 as i32,
        256 as i32,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_num(
        settings,
        b"synth.gain\x00" as *const u8 as *const libc::c_char,
        0.2f32 as f64,
        0.0f32 as f64,
        10.0f32 as f64,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.audio-channels\x00" as *const u8 as *const libc::c_char,
        1 as i32,
        1 as i32,
        256 as i32,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.audio-groups\x00" as *const u8 as *const libc::c_char,
        1 as i32,
        1 as i32,
        256 as i32,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.effects-channels\x00" as *const u8 as *const libc::c_char,
        2 as i32,
        2 as i32,
        2 as i32,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_num(
        settings,
        b"synth.sample-rate\x00" as *const u8 as *const libc::c_char,
        44100.0f32 as f64,
        22050.0f32 as f64,
        96000.0f32 as f64,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
    fluid_settings_register_int(
        settings,
        b"synth.min-note-length\x00" as *const u8 as *const libc::c_char,
        10 as i32,
        0 as i32,
        65535 as i32,
        0 as i32,
        None,
        0 as *mut libc::c_void,
    );
}

pub fn fluid_version(
    major: &mut i32,
    minor: &mut i32,
    micro: &mut i32,
) {
    *major = 1 as i32;
    *minor = 2 as i32;
    *micro = 1 as i32;
}

unsafe fn fluid_synth_init() {
    FLUID_SYNTH_INITIALIZED += 1;
    fluid_dsp_float_config();
    fluid_sys_config();
    init_dither();
    fluid_mod_set_source1(
        &mut DEFAULT_VEL2ATT_MOD,
        FLUID_MOD_VELOCITY as i32,
        FLUID_MOD_GC as i32
            | FLUID_MOD_CONCAVE as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_NEGATIVE as i32,
    );
    fluid_mod_set_source2(&mut DEFAULT_VEL2ATT_MOD, 0 as i32, 0 as i32);
    fluid_mod_set_dest(&mut DEFAULT_VEL2ATT_MOD, GEN_ATTENUATION as i32);
    fluid_mod_set_amount(&mut DEFAULT_VEL2ATT_MOD, 960.0f64);
    fluid_mod_set_source1(
        &mut DEFAULT_VEL2FILTER_MOD,
        FLUID_MOD_VELOCITY as i32,
        FLUID_MOD_GC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_NEGATIVE as i32,
    );
    fluid_mod_set_source2(
        &mut DEFAULT_VEL2FILTER_MOD,
        FLUID_MOD_VELOCITY as i32,
        FLUID_MOD_GC as i32
            | FLUID_MOD_SWITCH as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_dest(&mut DEFAULT_VEL2FILTER_MOD, GEN_FILTERFC as i32);
    fluid_mod_set_amount(&mut DEFAULT_VEL2FILTER_MOD, -(2400 as i32) as f64);
    fluid_mod_set_source1(
        &mut DEFAULT_AT2VIBLFO_MOD,
        FLUID_MOD_CHANNELPRESSURE as i32,
        FLUID_MOD_GC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_source2(
        &mut DEFAULT_AT2VIBLFO_MOD,
        0 as i32,
        0 as i32,
    );
    fluid_mod_set_dest(&mut DEFAULT_AT2VIBLFO_MOD, GEN_VIBLFOTOPITCH as i32);
    fluid_mod_set_amount(&mut DEFAULT_AT2VIBLFO_MOD, 50 as i32 as f64);
    fluid_mod_set_source1(
        &mut DEFAULT_MOD2VIBLFO_MOD,
        1 as i32,
        FLUID_MOD_CC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_source2(
        &mut DEFAULT_MOD2VIBLFO_MOD,
        0 as i32,
        0 as i32,
    );
    fluid_mod_set_dest(
        &mut DEFAULT_MOD2VIBLFO_MOD,
        GEN_VIBLFOTOPITCH as i32,
    );
    fluid_mod_set_amount(&mut DEFAULT_MOD2VIBLFO_MOD, 50 as i32 as f64);
    fluid_mod_set_source1(
        &mut DEFAULT_ATT_MOD,
        7 as i32,
        FLUID_MOD_CC as i32
            | FLUID_MOD_CONCAVE as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_NEGATIVE as i32,
    );
    fluid_mod_set_source2(&mut DEFAULT_ATT_MOD, 0 as i32, 0 as i32);
    fluid_mod_set_dest(&mut DEFAULT_ATT_MOD, GEN_ATTENUATION as i32);
    fluid_mod_set_amount(&mut DEFAULT_ATT_MOD, 960.0f64);
    fluid_mod_set_source1(
        &mut DEFAULT_PAN_MOD,
        10 as i32,
        FLUID_MOD_CC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_BIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_source2(&mut DEFAULT_PAN_MOD, 0 as i32, 0 as i32);
    fluid_mod_set_dest(&mut DEFAULT_PAN_MOD, GEN_PAN as i32);
    fluid_mod_set_amount(&mut DEFAULT_PAN_MOD, 500.0f64);
    fluid_mod_set_source1(
        &mut DEFAULT_EXPR_MOD,
        11 as i32,
        FLUID_MOD_CC as i32
            | FLUID_MOD_CONCAVE as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_NEGATIVE as i32,
    );
    fluid_mod_set_source2(&mut DEFAULT_EXPR_MOD, 0 as i32, 0 as i32);
    fluid_mod_set_dest(&mut DEFAULT_EXPR_MOD, GEN_ATTENUATION as i32);
    fluid_mod_set_amount(&mut DEFAULT_EXPR_MOD, 960.0f64);
    fluid_mod_set_source1(
        &mut DEFAULT_REVERB_MOD,
        91 as i32,
        FLUID_MOD_CC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_source2(&mut DEFAULT_REVERB_MOD, 0 as i32, 0 as i32);
    fluid_mod_set_dest(&mut DEFAULT_REVERB_MOD, GEN_REVERBSEND as i32);
    fluid_mod_set_amount(&mut DEFAULT_REVERB_MOD, 200 as i32 as f64);
    fluid_mod_set_source1(
        &mut DEFAULT_CHORUS_MOD,
        93 as i32,
        FLUID_MOD_CC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_source2(&mut DEFAULT_CHORUS_MOD, 0 as i32, 0 as i32);
    fluid_mod_set_dest(&mut DEFAULT_CHORUS_MOD, GEN_CHORUSSEND as i32);
    fluid_mod_set_amount(&mut DEFAULT_CHORUS_MOD, 200 as i32 as f64);
    fluid_mod_set_source1(
        &mut DEFAULT_PITCH_BEND_MOD,
        FLUID_MOD_PITCHWHEEL as i32,
        FLUID_MOD_GC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_BIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_source2(
        &mut DEFAULT_PITCH_BEND_MOD,
        FLUID_MOD_PITCHWHEELSENS as i32,
        FLUID_MOD_GC as i32
            | FLUID_MOD_LINEAR as i32
            | FLUID_MOD_UNIPOLAR as i32
            | FLUID_MOD_POSITIVE as i32,
    );
    fluid_mod_set_dest(&mut DEFAULT_PITCH_BEND_MOD, GEN_PITCH as i32);
    fluid_mod_set_amount(&mut DEFAULT_PITCH_BEND_MOD, 12700.0f64);
}

pub unsafe fn fluid_synth_set_sample_rate(
    synth: *mut Synth,
    sample_rate: f32,
) {
    (*synth).sample_rate = sample_rate as f64;

    let mut i;
    i = 0 as i32;
    while i < (*synth).nvoice {
        delete_fluid_voice(*(*synth).voice.offset(i as isize));
        let ref mut fresh6 = *(*synth).voice.offset(i as isize);
        *fresh6 = new_fluid_voice((*synth).sample_rate as f32);
        i += 1
    }
    (*synth).chorus.delete();
    (*synth).chorus = Chorus::new((*synth).sample_rate as f32);
}

impl Drop for Synth {
    fn drop(&mut self) {
        unsafe {
            let mut i;
            let mut k;
            let mut list;
            let mut sfont;
            let mut bank_offset;
            let mut loader;
            self.state = FLUID_SYNTH_STOPPED as i32 as u32;
            if !self.voice.is_null() {
                i = 0 as i32;
                while i < self.nvoice {
                    if !(*self.voice.offset(i as isize)).is_null()
                        && fluid_voice_is_playing(*self.voice.offset(i as isize)) != 0
                    {
                        fluid_voice_off(*self.voice.offset(i as isize));
                    }
                    i += 1
                }
            }
            list = self.sfont;
            while !list.is_null() {
                sfont = if !list.is_null() {
                    (*list).data
                } else {
                    0 as *mut libc::c_void
                } as *mut SoundFont;
                if !sfont.is_null() && (*sfont).free.is_some() {
                    Some((*sfont).free.expect("non-null function pointer"))
                        .expect("non-null function pointer")(sfont);
                } else {
                };
                list = if !list.is_null() {
                    (*list).next
                } else {
                    0 as *mut List
                }
            }
            delete_fluid_list(self.sfont);
            list = self.bank_offsets;
            while !list.is_null() {
                bank_offset = if !list.is_null() {
                    (*list).data
                } else {
                    0 as *mut libc::c_void
                } as *mut BankOffset;
                libc::free(bank_offset as *mut libc::c_void);
                list = if !list.is_null() {
                    (*list).next
                } else {
                    0 as *mut List
                }
            }
            delete_fluid_list(self.bank_offsets);
            list = self.loaders;
            while !list.is_null() {
                loader = if !list.is_null() {
                    (*list).data
                } else {
                    0 as *mut libc::c_void
                } as *mut SoundfontLoader;
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
                    0 as *mut List
                }
            }
            delete_fluid_list(self.loaders);
            if !self.voice.is_null() {
                i = 0 as i32;
                while i < self.nvoice {
                    if !(*self.voice.offset(i as isize)).is_null() {
                        delete_fluid_voice(*self.voice.offset(i as isize));
                    }
                    i += 1
                }
                libc::free(self.voice as *mut libc::c_void);
            }
            if !self.left_buf.is_null() {
                i = 0 as i32;
                while i < self.nbuf {
                    if !(*self.left_buf.offset(i as isize)).is_null() {
                        libc::free(*self.left_buf.offset(i as isize) as *mut libc::c_void);
                    }
                    i += 1
                }
                libc::free(self.left_buf as *mut libc::c_void);
            }
            if !self.right_buf.is_null() {
                i = 0 as i32;
                while i < self.nbuf {
                    if !(*self.right_buf.offset(i as isize)).is_null() {
                        libc::free(*self.right_buf.offset(i as isize) as *mut libc::c_void);
                    }
                    i += 1
                }
                libc::free(self.right_buf as *mut libc::c_void);
            }
            if !self.fx_left_buf.is_null() {
                i = 0 as i32;
                while i < 2 as i32 {
                    if !(*self.fx_left_buf.offset(i as isize)).is_null() {
                        libc::free(*self.fx_left_buf.offset(i as isize) as *mut libc::c_void);
                    }
                    i += 1
                }
                libc::free(self.fx_left_buf as *mut libc::c_void);
            }
            if !self.fx_right_buf.is_null() {
                i = 0 as i32;
                while i < 2 as i32 {
                    if !(*self.fx_right_buf.offset(i as isize)).is_null() {
                        libc::free(*self.fx_right_buf.offset(i as isize) as *mut libc::c_void);
                    }
                    i += 1
                }
                libc::free(self.fx_right_buf as *mut libc::c_void);
            }
            self.chorus.delete();
            if !self.tuning.is_null() {
                i = 0 as i32;
                while i < 128 as i32 {
                    if !(*self.tuning.offset(i as isize)).is_null() {
                        k = 0 as i32;
                        while k < 128 as i32 {
                            if !(*(*self.tuning.offset(i as isize)).offset(k as isize)).is_null() {
                                libc::free(*(*self.tuning.offset(i as isize)).offset(k as isize)
                                    as *mut libc::c_void);
                            }
                            k += 1
                        }
                        libc::free(*self.tuning.offset(i as isize) as *mut libc::c_void);
                    }
                    i += 1
                }
                libc::free(self.tuning as *mut libc::c_void);
            }
        }
    }
}

pub unsafe fn fluid_synth_error() -> *mut libc::c_char {
    return fluid_error();
}

pub unsafe fn fluid_synth_noteon(
    synth: *mut Synth,
    chan: i32,
    key: i32,
    vel: i32,
) -> i32 {
    let channel;
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if vel == 0 as i32 {
        return fluid_synth_noteoff(synth, chan, key);
    }
    channel = &(*synth).channel[chan as usize];
    if channel.preset.is_null() {
        if (*synth).verbose != 0 {
            fluid_log!(
                FLUID_INFO,
                "noteon\t{}\t{}\t{}\t{}\t{}\t\t{}\t{}\t{}",
                chan,
                key,
                vel,
                0,
                ((*synth).ticks as f32 / 44100.0f32),
                0.0f32,
                0,
                "channel has no preset"
            );
        }
        return FLUID_FAILED as i32;
    }
    fluid_synth_release_voice_on_same_note(synth, chan, key);
    let fresh7 = (*synth).noteid;
    (*synth).noteid = (*synth).noteid.wrapping_add(1);
    return fluid_synth_start(
        synth,
        fresh7,
        channel.preset,
        0 as i32,
        chan,
        key,
        vel,
    );
}

pub unsafe fn fluid_synth_noteoff(
    synth: *mut Synth,
    chan: i32,
    key: i32,
) -> i32 {
    let mut i;
    let mut voice;
    let mut status: i32 = FLUID_FAILED as i32;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as i32 == FLUID_VOICE_ON as i32
            && (*voice).volenv_section < FLUID_VOICE_ENVRELEASE as i32
            && (*voice).chan as i32 == chan
            && (*voice).key as i32 == key
        {
            if (*synth).verbose != 0 {
                let mut used_voices: i32 = 0 as i32;
                let mut k;
                k = 0 as i32;
                while k < (*synth).polyphony {
                    if !((**(*synth).voice.offset(k as isize)).status as i32
                        == FLUID_VOICE_CLEAN as i32
                        || (**(*synth).voice.offset(k as isize)).status as i32
                            == FLUID_VOICE_OFF as i32)
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
                    0 as i32,
                    (*voice).id,
                    ((*voice).start_time.wrapping_add((*voice).ticks) as f32 / 44100.0f32)
                        as f64,
                    ((*voice).ticks as f32 / 44100.0f32) as f64,
                    used_voices
                );
            }
            fluid_voice_noteoff(voice, &*synth);
            status = FLUID_OK as i32
        }
        i += 1
    }
    return status;
}

pub unsafe fn fluid_synth_damp_voices(synth: *mut Synth, chan: i32) -> i32 {
    let mut i;
    let mut voice;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as i32 == chan
            && (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32
        {
            fluid_voice_noteoff(voice, &*synth);
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_cc(
    synth: *mut Synth,
    chan: i32,
    num: i32,
    val: i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if num < 0 as i32 || num >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Ctrl out of range",);
        return FLUID_FAILED as i32;
    }
    if val < 0 as i32 || val >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Value out of range",);
        return FLUID_FAILED as i32;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "cc\t{}\t{}\t{}", chan, num, val);
    }
    fluid_channel_cc(&mut (*synth).channel[chan as usize], &mut *synth, num, val);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_get_cc(
    synth: *const Synth,
    chan: i32,
    num: i32,
    pval: *mut i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if num < 0 as i32 || num >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Ctrl out of range",);
        return FLUID_FAILED as i32;
    }
    *pval = (*synth).channel[chan as usize].cc[num as usize] as i32;
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_all_notes_off(
    synth: *mut Synth,
    chan: i32,
) -> i32 {
    let mut i;
    let mut voice;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if ((*voice).status as i32 == FLUID_VOICE_ON as i32
            || (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32)
            && (*voice).chan as i32 == chan
        {
            fluid_voice_noteoff(voice, &*synth);
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_all_sounds_off(
    synth: *mut Synth,
    chan: i32,
) -> i32 {
    let mut i;
    let mut voice;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if ((*voice).status as i32 == FLUID_VOICE_ON as i32
            || (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32)
            && (*voice).chan as i32 == chan
        {
            fluid_voice_off(voice);
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_system_reset(synth: *mut Synth) -> i32 {
    let mut i;
    let mut voice;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as i32 == FLUID_VOICE_ON as i32
            || (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32
        {
            fluid_voice_off(voice);
        }
        i += 1
    }
    i = 0 as i32;
    while i < (*synth).midi_channels {
        fluid_channel_reset(&mut (*synth).channel[i as usize], &*synth);
        i += 1
    }
    (*synth).chorus.reset();
    (*synth).reverb.reset();
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_modulate_voices(
    synth: *mut Synth,
    chan: i32,
    is_cc: i32,
    ctrl: i32,
) -> i32 {
    let mut i;
    let mut voice;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as i32 == chan {
            fluid_voice_modulate(voice, is_cc, ctrl);
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_modulate_voices_all(
    synth: *mut Synth,
    chan: i32,
) -> i32 {
    let mut i;
    let mut voice;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as i32 == chan {
            fluid_voice_modulate_all(voice);
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_channel_pressure(
    synth: *mut Synth,
    chan: i32,
    val: i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "channelpressure\t{}\t{}", chan, val);
    }
    fluid_channel_pressure(&mut (*synth).channel[chan as usize], &mut *synth, val);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_key_pressure(
    synth: *mut Synth,
    chan: i32,
    key: i32,
    val: i32,
) -> i32 {
    let mut result: i32 = FLUID_OK as i32;
    if key < 0 as i32 || key > 127 as i32 {
        return FLUID_FAILED as i32;
    }
    if val < 0 as i32 || val > 127 as i32 {
        return FLUID_FAILED as i32;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "keypressure\t{}\t{}\t{}", chan, key, val);
    }
    (*synth).channel[chan as usize].key_pressure[key as usize] = val as libc::c_char;
    let mut voice;
    let mut i;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as i32 == chan && (*voice).key as i32 == key {
            result = fluid_voice_modulate(
                voice,
                0 as i32,
                FLUID_MOD_KEYPRESSURE as i32,
            );
            if result != FLUID_OK as i32 {
                break;
            }
        }
        i += 1
    }
    return result;
}

pub unsafe fn fluid_synth_pitch_bend(
    synth: *mut Synth,
    chan: i32,
    val: i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "pitchb\t{}\t{}", chan, val);
    }
    fluid_channel_pitch_bend(&mut (*synth).channel[chan as usize], &mut *synth, val);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_get_pitch_bend(
    synth: *const Synth,
    chan: i32,
    ppitch_bend: *mut i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    *ppitch_bend = (*synth).channel[chan as usize].pitch_bend as i32;
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_pitch_wheel_sens(
    synth: *mut Synth,
    chan: i32,
    val: i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "pitchsens\t{}\t{}", chan, val);
    }
    fluid_channel_pitch_wheel_sens(&mut (*synth).channel[chan as usize], &mut *synth, val);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_get_pitch_wheel_sens(
    synth: *const Synth,
    chan: i32,
    pval: *mut i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    *pval = (*synth).channel[chan as usize].pitch_wheel_sensitivity as i32;
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_get_preset(
    synth: *mut Synth,
    sfontnum: u32,
    banknum: u32,
    prognum: u32,
) -> *mut Preset {
    let preset;
    let sfont;
    let offset;
    sfont = fluid_synth_get_sfont_by_id(synth, sfontnum);
    if !sfont.is_null() {
        offset = fluid_synth_get_bank_offset(synth, sfontnum as i32);
        preset = Some((*sfont).get_preset.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            sfont,
            banknum.wrapping_sub(offset as u32),
            prognum,
        );
        if !preset.is_null() {
            return preset;
        }
    }
    return 0 as *mut Preset;
}

pub unsafe fn fluid_synth_find_preset(
    synth: *const Synth,
    banknum: u32,
    prognum: u32,
) -> *mut Preset {
    let mut preset;
    let mut sfont;
    let mut list: *mut List = (*synth).sfont;
    let mut offset;
    while !list.is_null() {
        sfont = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut SoundFont;
        offset = fluid_synth_get_bank_offset(synth, (*sfont).id as i32);
        preset = Some((*sfont).get_preset.expect("non-null function pointer"))
            .expect("non-null function pointer")(
            sfont,
            banknum.wrapping_sub(offset as u32),
            prognum,
        );
        if !preset.is_null() {
            (*preset).sfont = sfont;
            return preset;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        }
    }
    return 0 as *mut Preset;
}

pub unsafe fn fluid_synth_program_change(
    synth: *mut Synth,
    chan: i32,
    prognum: i32,
) -> i32 {
    let mut preset;
    let channel;
    let banknum;
    let sfont_id;
    let mut subst_bank;
    let mut subst_prog;
    if prognum < 0 as i32
        || prognum >= 128 as i32
        || chan < 0 as i32
        || chan >= (*synth).midi_channels
    {
        fluid_log!(
            FLUID_ERR,
            "Index out of range (chan={}, prog={})",
            chan,
            prognum
        );
        return FLUID_FAILED as i32;
    }
    channel = &mut (*synth).channel[chan as usize];
    banknum = fluid_channel_get_banknum(channel);
    fluid_channel_set_prognum(channel, prognum);
    if (*synth).verbose != 0 {
        fluid_log!(FLUID_INFO, "prog\t{}\t{}\t{}", chan, banknum, prognum);
    }
    if (*channel).channum == 9 as i32
        && fluid_settings_str_equal(
            (*synth).settings,
            b"synth.drums-channel.active\x00" as *const u8 as *const libc::c_char,
            b"yes\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        ) != 0
    {
        preset = fluid_synth_find_preset(
            synth,
            128 as i32 as u32,
            prognum as u32,
        )
    } else {
        preset = fluid_synth_find_preset(synth, banknum, prognum as u32)
    }
    if preset.is_null() {
        subst_bank = banknum as i32;
        subst_prog = prognum;
        if banknum != 128 as i32 as u32 {
            subst_bank = 0 as i32;
            preset = fluid_synth_find_preset(
                synth,
                0 as i32 as u32,
                prognum as u32,
            );
            if preset.is_null() && prognum != 0 as i32 {
                preset = fluid_synth_find_preset(
                    synth,
                    0 as i32 as u32,
                    0 as i32 as u32,
                );
                subst_prog = 0 as i32
            }
        } else {
            preset = fluid_synth_find_preset(
                synth,
                128 as i32 as u32,
                0 as i32 as u32,
            );
            subst_prog = 0 as i32
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
        0 as i32 as u32
    };
    fluid_channel_set_sfontnum(channel, sfont_id);
    fluid_channel_set_preset(channel, preset);
    return FLUID_OK as i32;
}

pub fn fluid_synth_bank_select(
    synth: *mut Synth,
    chan: i32,
    bank: u32,
) -> i32 {
    unsafe {
        if chan >= 0 as i32 && chan < (*synth).midi_channels {
            fluid_channel_set_banknum(&mut (*synth).channel[chan as usize], bank);
            return FLUID_OK as i32;
        }
        return FLUID_FAILED as i32;
    }
}

pub unsafe fn fluid_synth_sfont_select(
    synth: *mut Synth,
    chan: i32,
    sfont_id: u32,
) -> i32 {
    if chan >= 0 as i32 && chan < (*synth).midi_channels {
        fluid_channel_set_sfontnum(&mut (*synth).channel[chan as usize], sfont_id);
        return FLUID_OK as i32;
    }
    return FLUID_FAILED as i32;
}

pub unsafe fn fluid_synth_get_program(
    synth: *const Synth,
    chan: i32,
    sfont_id: *mut u32,
    bank_num: *mut u32,
    preset_num: *mut u32,
) -> i32 {
    let channel;
    if chan >= 0 as i32 && chan < (*synth).midi_channels {
        channel = &(*synth).channel[chan as usize];
        *sfont_id = fluid_channel_get_sfontnum(channel);
        *bank_num = fluid_channel_get_banknum(channel);
        *preset_num = fluid_channel_get_prognum(channel) as u32;
        return FLUID_OK as i32;
    }
    return FLUID_FAILED as i32;
}

pub unsafe fn fluid_synth_program_select(
    synth: *mut Synth,
    chan: i32,
    sfont_id: u32,
    bank_num: u32,
    preset_num: u32,
) -> i32 {
    let preset;
    let channel;
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_ERR, "Channel number out of range (chan={})", chan);
        return FLUID_FAILED as i32;
    }
    channel = &mut (*synth).channel[chan as usize];
    preset = fluid_synth_get_preset(synth, sfont_id, bank_num, preset_num);
    if preset.is_null() {
        fluid_log!(
            FLUID_ERR,
            "There is no preset with bank number {} and preset number {} in SoundFont {}",
            bank_num,
            preset_num,
            sfont_id
        );
        return FLUID_FAILED as i32;
    }
    fluid_channel_set_sfontnum(channel, sfont_id);
    fluid_channel_set_banknum(channel, bank_num);
    fluid_channel_set_prognum(channel, preset_num as i32);
    fluid_channel_set_preset(channel, preset);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_update_presets(synth: *mut Synth) {
    let mut chan;
    let mut channel;
    chan = 0 as i32;
    while chan < (*synth).midi_channels {
        channel = &mut (*synth).channel[chan as usize];
        fluid_channel_set_preset(
            channel,
            fluid_synth_get_preset(
                synth,
                fluid_channel_get_sfontnum(channel),
                fluid_channel_get_banknum(channel),
                fluid_channel_get_prognum(channel) as u32,
            ),
        );
        chan += 1
    }
}

pub unsafe fn fluid_synth_update_gain(
    synth: *mut Synth,
    _name: *mut libc::c_char,
    value: f64,
) -> i32 {
    fluid_synth_set_gain(synth, value as f32);
    return 0 as i32;
}

pub unsafe fn fluid_synth_set_gain(synth: *mut Synth, mut gain: f32) {
    let mut i;
    gain = if gain < 0.0f32 {
        0.0f32
    } else if gain > 10.0f32 {
        10.0f32
    } else {
        gain
    };
    (*synth).gain = gain as f64;
    i = 0 as i32;
    while i < (*synth).polyphony {
        let voice: *mut Voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as i32 == FLUID_VOICE_ON as i32
            || (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32
        {
            fluid_voice_set_gain(voice, gain);
        }
        i += 1
    }
}

pub unsafe fn fluid_synth_get_gain(synth: *const Synth) -> f32 {
    return (*synth).gain as f32;
}

pub unsafe fn fluid_synth_update_polyphony(
    synth: *mut Synth,
    _name: *mut libc::c_char,
    value: i32,
) -> i32 {
    fluid_synth_set_polyphony(synth, value);
    return 0 as i32;
}

pub unsafe fn fluid_synth_set_polyphony(
    synth: *mut Synth,
    polyphony: i32,
) -> i32 {
    let mut i;
    if polyphony < 1 as i32 || polyphony > (*synth).nvoice {
        return FLUID_FAILED as i32;
    }
    i = polyphony;
    while i < (*synth).nvoice {
        let voice: *mut Voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as i32 == FLUID_VOICE_ON as i32
            || (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32
        {
            fluid_voice_off(voice);
        }
        i += 1
    }
    (*synth).polyphony = polyphony;
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_get_polyphony(synth: *const Synth) -> i32 {
    return (*synth).polyphony;
}

pub unsafe fn fluid_synth_get_internal_bufsize(_synth: *const Synth) -> i32 {
    return 64 as i32;
}

pub unsafe fn fluid_synth_program_reset(synth: *mut Synth) -> i32 {
    let mut i;
    i = 0 as i32;
    while i < (*synth).midi_channels {
        fluid_synth_program_change(
            synth,
            i,
            fluid_channel_get_prognum(&(*synth).channel[i as usize]),
        );
        i += 1
    }
    return FLUID_OK as i32;
}

pub fn fluid_synth_set_reverb(
    synth: *mut Synth,
    roomsize: f64,
    damping: f64,
    width: f64,
    level: f64,
) {
    unsafe {
        (*synth).reverb.set_room_size(roomsize as f32);
        (*synth).reverb.set_damp(damping as f32);
        (*synth).reverb.set_width(width as f32);
        (*synth).reverb.set_level(level as f32);
    }
}

pub unsafe fn fluid_synth_set_chorus(
    synth: *mut Synth,
    nr: i32,
    level: f64,
    speed: f64,
    depth_ms: f64,
    type_0: i32,
) {
    (*synth).chorus.set_nr(nr);
    (*synth).chorus.set_level(level as f32);
    (*synth).chorus.set_speed_hz(speed as f32);
    (*synth).chorus.set_depth_ms(depth_ms as f32);
    (*synth).chorus.set_type(type_0);
    (*synth).chorus.update();
}

pub unsafe fn fluid_synth_write_float(
    synth: *mut Synth,
    len: i32,
    lout: *mut libc::c_void,
    loff: i32,
    lincr: i32,
    rout: *mut libc::c_void,
    roff: i32,
    rincr: i32,
) -> i32 {
    let mut i;
    let mut j;
    let mut k;
    let mut l;
    let left_out: *mut f32 = lout as *mut f32;
    let right_out: *mut f32 = rout as *mut f32;
    let left_in: *mut f32 = *(*synth).left_buf.offset(0 as i32 as isize);
    let right_in: *mut f32 = *(*synth).right_buf.offset(0 as i32 as isize);
    if (*synth).state != FLUID_SYNTH_PLAYING as i32 as u32 {
        return 0 as i32;
    }
    l = (*synth).cur;
    i = 0 as i32;
    j = loff;
    k = roff;
    while i < len {
        if l == 64 as i32 {
            fluid_synth_one_block(synth, 0 as i32);
            l = 0 as i32
        }
        *left_out.offset(j as isize) = *left_in.offset(l as isize);
        *right_out.offset(k as isize) = *right_in.offset(l as isize);
        i += 1;
        l += 1;
        j += lincr;
        k += rincr
    }
    (*synth).cur = l;
    return 0 as i32;
}
static mut RAND_TABLE: [[f32; 48000]; 2] = [[0.; 48000]; 2];
unsafe fn init_dither() {
    let mut d;
    let mut dp;
    let mut c;
    let mut i;
    c = 0 as i32;
    while c < 2 as i32 {
        dp = 0 as i32 as f32;
        i = 0 as i32;
        while i < 48000 as i32 - 1 as i32 {
            d = libc::rand() as f32 / 2147483647 as i32 as f32 - 0.5f32;
            RAND_TABLE[c as usize][i as usize] = d - dp;
            dp = d;
            i += 1
        }
        RAND_TABLE[c as usize][(48000 as i32 - 1 as i32) as usize] =
            0 as i32 as f32 - dp;
        c += 1
    }
}
unsafe fn roundi(x: f32) -> i32 {
    if x >= 0.0f32 {
        return (x + 0.5f32) as i32;
    } else {
        return (x - 0.5f32) as i32;
    };
}

pub unsafe fn fluid_synth_write_s16(
    synth: *mut Synth,
    len: i32,
    lout: *mut libc::c_void,
    loff: i32,
    lincr: i32,
    rout: *mut libc::c_void,
    roff: i32,
    rincr: i32,
) -> i32 {
    let mut i;
    let mut j;
    let mut k;
    let mut cur;
    let left_out: *mut i16 = lout as *mut i16;
    let right_out: *mut i16 = rout as *mut i16;
    let left_in: *mut f32 = *(*synth).left_buf.offset(0 as i32 as isize);
    let right_in: *mut f32 = *(*synth).right_buf.offset(0 as i32 as isize);
    let mut left_sample;
    let mut right_sample;
    let mut di: i32 = (*synth).dither_index;
    if (*synth).state != FLUID_SYNTH_PLAYING as i32 as u32 {
        return 0 as i32;
    }
    cur = (*synth).cur;
    i = 0 as i32;
    j = loff;
    k = roff;
    while i < len {
        if cur == 64 as i32 {
            fluid_synth_one_block(synth, 0 as i32);
            cur = 0 as i32
        }
        left_sample = roundi(
            *left_in.offset(cur as isize) * 32766.0f32
                + RAND_TABLE[0 as i32 as usize][di as usize],
        ) as f32;
        right_sample = roundi(
            *right_in.offset(cur as isize) * 32766.0f32
                + RAND_TABLE[1 as i32 as usize][di as usize],
        ) as f32;
        di += 1;
        if di >= 48000 as i32 {
            di = 0 as i32
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
        *left_out.offset(j as isize) = left_sample as i16;
        *right_out.offset(k as isize) = right_sample as i16;
        i += 1;
        cur += 1;
        j += lincr;
        k += rincr
    }
    (*synth).cur = cur;
    (*synth).dither_index = di;
    return 0 as i32;
}

pub unsafe fn fluid_synth_one_block(
    synth: *mut Synth,
    do_not_mix_fx_to_out: i32,
) -> i32 {
    let mut i;
    let mut auchan;
    let mut voice;
    let mut left_buf;
    let mut right_buf;
    let reverb_buf;
    let chorus_buf;
    let byte_size: i32 = (64 as i32 as libc::size_t)
        .wrapping_mul(::std::mem::size_of::<f32>() as libc::size_t)
        as i32;
    i = 0 as i32;
    while i < (*synth).nbuf {
        libc::memset(
            *(*synth).left_buf.offset(i as isize) as *mut libc::c_void,
            0 as i32,
            byte_size as libc::size_t,
        );
        libc::memset(
            *(*synth).right_buf.offset(i as isize) as *mut libc::c_void,
            0 as i32,
            byte_size as libc::size_t,
        );
        i += 1
    }
    i = 0 as i32;
    while i < (*synth).effects_channels {
        libc::memset(
            *(*synth).fx_left_buf.offset(i as isize) as *mut libc::c_void,
            0 as i32,
            byte_size as libc::size_t,
        );
        libc::memset(
            *(*synth).fx_right_buf.offset(i as isize) as *mut libc::c_void,
            0 as i32,
            byte_size as libc::size_t,
        );
        i += 1
    }
    reverb_buf = if (*synth).with_reverb as i32 != 0 {
        *(*synth).fx_left_buf.offset(0 as i32 as isize)
    } else {
        0 as *mut f32
    };
    chorus_buf = if (*synth).with_chorus as i32 != 0 {
        *(*synth).fx_left_buf.offset(1 as i32 as isize)
    } else {
        0 as *mut f32
    };
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as i32 == FLUID_VOICE_ON as i32
            || (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32
        {
            auchan = fluid_channel_get_num(fluid_voice_get_channel(voice).as_ref().unwrap());
            auchan %= (*synth).audio_groups;
            left_buf = *(*synth).left_buf.offset(auchan as isize);
            right_buf = *(*synth).right_buf.offset(auchan as isize);
            fluid_voice_write(voice, &*synth, left_buf, right_buf, reverb_buf, chorus_buf);
        }
        i += 1
    }
    if do_not_mix_fx_to_out != 0 {
        if !reverb_buf.is_null() {
            (*synth).reverb.process_replace(
                reverb_buf,
                *(*synth).fx_left_buf.offset(0 as i32 as isize),
                *(*synth).fx_right_buf.offset(0 as i32 as isize),
            );
        }
        if !chorus_buf.is_null() {
            (*synth).chorus.process_replace(
                chorus_buf,
                *(*synth).fx_left_buf.offset(1 as i32 as isize),
                *(*synth).fx_right_buf.offset(1 as i32 as isize),
            );
        }
    } else {
        if !reverb_buf.is_null() {
            (*synth).reverb.process_mix(
                reverb_buf,
                *(*synth).left_buf.offset(0 as i32 as isize),
                *(*synth).right_buf.offset(0 as i32 as isize),
            );
        }
        if !chorus_buf.is_null() {
            (*synth).chorus.process_mix(
                chorus_buf,
                *(*synth).left_buf.offset(0 as i32 as isize),
                *(*synth).right_buf.offset(0 as i32 as isize),
            );
        }
    }
    (*synth).ticks = (*synth)
        .ticks
        .wrapping_add(64);
    return 0 as i32;
}

pub unsafe fn fluid_synth_free_voice_by_kill(synth: *mut Synth) -> *mut Voice {
    let mut i;
    let mut best_prio: f32 = 999999.0f32;
    let mut this_voice_prio;
    let mut voice;
    let mut best_voice_index: i32 = -(1 as i32);
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).status as i32 == FLUID_VOICE_CLEAN as i32
            || (*voice).status as i32 == FLUID_VOICE_OFF as i32
        {
            return voice;
        }
        this_voice_prio = 10000.0f32;
        if (*voice).chan as i32 == 0xff as i32 {
            this_voice_prio = (this_voice_prio as f64 - 2000.0f64) as f32
        }
        if (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32 {
            this_voice_prio -= 1000 as i32 as f32
        }
        this_voice_prio -= (*synth).noteid.wrapping_sub(fluid_voice_get_id(voice)) as f32;
        if (*voice).volenv_section != FLUID_VOICE_ENVATTACK as i32 {
            this_voice_prio =
                (this_voice_prio as f64 + (*voice).volenv_val as f64 * 1000.0f64) as f32
        }
        if this_voice_prio < best_prio {
            best_voice_index = i;
            best_prio = this_voice_prio
        }
        i += 1
    }
    if best_voice_index < 0 as i32 {
        return 0 as *mut Voice;
    }
    voice = *(*synth).voice.offset(best_voice_index as isize);
    fluid_voice_off(voice);
    return voice;
}

pub unsafe fn fluid_synth_alloc_voice(
    synth: *mut Synth,
    sample: *mut Sample,
    chan: i32,
    key: i32,
    vel: i32,
) -> *mut Voice {
    let mut i;
    let mut k;
    let mut voice: *mut Voice = 0 as *mut Voice;
    let channel;
    i = 0 as i32;
    while i < (*synth).polyphony {
        if (**(*synth).voice.offset(i as isize)).status as i32
            == FLUID_VOICE_CLEAN as i32
            || (**(*synth).voice.offset(i as isize)).status as i32
                == FLUID_VOICE_OFF as i32
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
        return 0 as *mut Voice;
    }
    if (*synth).verbose != 0 {
        k = 0 as i32;
        i = 0 as i32;
        while i < (*synth).polyphony {
            if !((**(*synth).voice.offset(i as isize)).status as i32
                == FLUID_VOICE_CLEAN as i32
                || (**(*synth).voice.offset(i as isize)).status as i32
                    == FLUID_VOICE_OFF as i32)
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
            ((*synth).ticks as f32 / 44100.0f32) as f64,
            0.0f32 as f64,
            k
        );
    }
    if chan >= 0 as i32 {
        channel = &mut (*synth).channel[chan as usize]
    } else {
        fluid_log!(FLUID_WARN, "Channel should be valid",);
        return 0 as *mut Voice;
    }
    if fluid_voice_init(
        voice,
        sample,
        channel,
        key,
        vel,
        (*synth).storeid,
        (*synth).ticks,
        (*synth).gain as f32,
    ) != FLUID_OK as i32
    {
        fluid_log!(FLUID_WARN, "Failed to initialize voice",);
        return 0 as *mut Voice;
    }
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_VEL2ATT_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_VEL2FILTER_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_AT2VIBLFO_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_MOD2VIBLFO_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_ATT_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_PAN_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_EXPR_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_REVERB_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_CHORUS_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    fluid_voice_add_mod(
        voice,
        &mut DEFAULT_PITCH_BEND_MOD,
        FLUID_VOICE_DEFAULT as i32,
    );
    return voice;
}

pub unsafe fn fluid_synth_kill_by_exclusive_class(
    synth: *mut Synth,
    new_voice: *mut Voice,
) {
    let mut i;
    let excl_class: i32 = ((*new_voice).gen[GEN_EXCLUSIVECLASS as i32 as usize].val
        as f32
        + (*new_voice).gen[GEN_EXCLUSIVECLASS as i32 as usize].mod_0 as f32
        + (*new_voice).gen[GEN_EXCLUSIVECLASS as i32 as usize].nrpn as f32)
        as i32;
    if excl_class == 0 as i32 {
        return;
    }
    i = 0 as i32;
    while i < (*synth).polyphony {
        let existing_voice: *mut Voice = *(*synth).voice.offset(i as isize);
        if (*existing_voice).status as i32 == FLUID_VOICE_ON as i32
            || (*existing_voice).status as i32 == FLUID_VOICE_SUSTAINED as i32
        {
            if !((*existing_voice).chan as i32 != (*new_voice).chan as i32) {
                if !(((*existing_voice).gen[GEN_EXCLUSIVECLASS as i32 as usize].val as f32
                    + (*existing_voice).gen[GEN_EXCLUSIVECLASS as i32 as usize].mod_0
                        as f32
                    + (*existing_voice).gen[GEN_EXCLUSIVECLASS as i32 as usize].nrpn as f32)
                    as i32
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

pub unsafe fn fluid_synth_start_voice(synth: *mut Synth, voice: *mut Voice) {
    fluid_synth_kill_by_exclusive_class(synth, voice);
    fluid_voice_start(voice);
}

pub fn fluid_synth_add_sfloader(
    synth: *mut Synth,
    loader: *mut SoundfontLoader,
) {
    unsafe {
        (*synth).loaders = fluid_list_prepend((*synth).loaders, loader as *mut libc::c_void);
    }
}

pub unsafe fn fluid_synth_sfload(
    synth: *mut Synth,
    filename: *const libc::c_char,
    reset_presets: i32,
) -> i32 {
    let mut sfont;
    let list;
    let loader;
    if filename.is_null() {
        fluid_log!(FLUID_ERR, "Invalid filename",);
        return FLUID_FAILED as i32;
    }
    list = (*synth).loaders;
    if !list.is_null() {
        loader = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut SoundfontLoader;
        sfont = Some((*loader).load.expect("non-null function pointer"))
            .expect("non-null function pointer")(loader, filename);
        if sfont.is_null() {
            return -(1 as i32);
        }
        (*synth).sfont_id = (*synth).sfont_id.wrapping_add(1);
        (*sfont).id = (*synth).sfont_id;
        (*synth).sfont = fluid_list_prepend((*synth).sfont, sfont as *mut libc::c_void);
        if reset_presets != 0 {
            fluid_synth_program_reset(synth);
        }
        return (*sfont).id as i32;
    }
    fluid_log!(
        FLUID_ERR,
        "Failed to load SoundFont \"{}\"",
        CStr::from_ptr(filename).to_str().unwrap()
    );
    return -(1 as i32);
}

pub unsafe fn fluid_synth_sfunload(
    synth: *mut Synth,
    id: u32,
    reset_presets: i32,
) -> i32 {
    let sfont: *mut SoundFont = fluid_synth_get_sfont_by_id(synth, id);
    if sfont.is_null() {
        fluid_log!(FLUID_ERR, "No SoundFont with id = {}", id);
        return FLUID_FAILED as i32;
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
        0 as i32
    }) != 0 as i32
    {
        let r: i32 = if !sfont.is_null() && (*sfont).free.is_some() {
            Some((*sfont).free.expect("non-null function pointer"))
                .expect("non-null function pointer")(sfont)
        } else {
            0 as i32
        };
        if r == 0 as i32 {
            fluid_log!(FLUID_DBG as i32, "Unloaded SoundFont",);
        }
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_sfreload(synth: *mut Synth, id: u32) -> i32 {
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    let mut sfont;
    let mut index: i32 = 0 as i32;
    let mut list;
    let mut loader;
    sfont = fluid_synth_get_sfont_by_id(synth, id);
    if sfont.is_null() {
        fluid_log!(FLUID_ERR, "No SoundFont with id = {}", id);
        return FLUID_FAILED as i32;
    }
    list = (*synth).sfont;
    while !list.is_null() {
        if sfont
            == (if !list.is_null() {
                (*list).data
            } else {
                0 as *mut libc::c_void
            }) as *mut SoundFont
        {
            break;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        };
        index += 1
    }
    libc::strcpy(
        filename.as_mut_ptr(),
        Some((*sfont).get_name.expect("non-null function pointer"))
            .expect("non-null function pointer")(sfont),
    );
    if fluid_synth_sfunload(synth, id, 0 as i32) != FLUID_OK as i32 {
        return FLUID_FAILED as i32;
    }
    list = (*synth).loaders;
    while !list.is_null() {
        loader = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut SoundfontLoader;
        sfont = Some((*loader).load.expect("non-null function pointer"))
            .expect("non-null function pointer")(loader, filename.as_mut_ptr());
        if !sfont.is_null() {
            (*sfont).id = id;
            (*synth).sfont =
                fluid_list_insert_at((*synth).sfont, index, sfont as *mut libc::c_void);
            fluid_synth_update_presets(synth);
            return (*sfont).id as i32;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        }
    }
    fluid_log!(
        FLUID_ERR,
        "Failed to load SoundFont \"{}\"",
        CStr::from_ptr(filename.as_ptr()).to_str().unwrap()
    );
    return -(1 as i32);
}

pub unsafe fn fluid_synth_remove_sfont(synth: *mut Synth, sfont: *mut SoundFont) {
    let sfont_id: i32 = (*sfont).id as i32;
    (*synth).sfont = fluid_list_remove((*synth).sfont, sfont as *mut libc::c_void);
    fluid_synth_remove_bank_offset(synth, sfont_id);
    fluid_synth_program_reset(synth);
}

pub unsafe fn fluid_synth_sfcount(synth: *const Synth) -> i32 {
    return fluid_list_size((*synth).sfont);
}

pub unsafe fn fluid_synth_get_sfont(
    synth: *const Synth,
    num: u32,
) -> *mut SoundFont {
    return if !fluid_list_nth((*synth).sfont, num as i32).is_null() {
        (*fluid_list_nth((*synth).sfont, num as i32)).data
    } else {
        0 as *mut libc::c_void
    } as *mut SoundFont;
}

pub unsafe fn fluid_synth_get_sfont_by_id(
    synth: *const Synth,
    id: u32,
) -> *mut SoundFont {
    let mut list: *mut List = (*synth).sfont;
    let mut sfont;
    while !list.is_null() {
        sfont = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut SoundFont;
        if (*sfont).id == id {
            return sfont;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        }
    }
    return 0 as *mut SoundFont;
}

pub unsafe fn fluid_synth_get_channel_preset(
    synth: *const Synth,
    chan: i32,
) -> *mut Preset {
    if chan >= 0 as i32 && chan < (*synth).midi_channels {
        return fluid_channel_get_preset(&(*synth).channel[chan as usize]);
    }
    return 0 as *mut Preset;
}

pub unsafe fn fluid_synth_set_reverb_on(synth: *mut Synth, on: i32) {
    (*synth).with_reverb = on as libc::c_char;
}

pub unsafe fn fluid_synth_set_chorus_on(synth: *mut Synth, on: i32) {
    (*synth).with_chorus = on as libc::c_char;
}

pub unsafe fn fluid_synth_get_chorus_nr(synth: *const Synth) -> i32 {
    return (*synth).chorus.get_nr();
}

pub unsafe fn fluid_synth_get_chorus_level(synth: *const Synth) -> f64 {
    return (*synth).chorus.get_level() as f64;
}

pub unsafe fn fluid_synth_get_chorus_speed_hz(synth: *const Synth) -> f64 {
    return (*synth).chorus.get_speed_hz() as f64;
}

pub unsafe fn fluid_synth_get_chorus_depth_ms(synth: *const Synth) -> f64 {
    return (*synth).chorus.get_depth_ms() as f64;
}

pub unsafe fn fluid_synth_get_chorus_type(synth: *const Synth) -> i32 {
    return (*synth).chorus.get_type();
}

pub unsafe fn fluid_synth_get_reverb_roomsize(synth: *const Synth) -> f64 {
    return (*synth).reverb.get_room_size() as f64;
}

pub unsafe fn fluid_synth_get_reverb_damp(synth: *const Synth) -> f64 {
    return (*synth).reverb.get_damp() as f64;
}

pub unsafe fn fluid_synth_get_reverb_level(synth: *const Synth) -> f64 {
    return (*synth).reverb.get_level() as f64;
}

pub unsafe fn fluid_synth_get_reverb_width(synth: *const Synth) -> f64 {
    return (*synth).reverb.get_width() as f64;
}

pub unsafe fn fluid_synth_release_voice_on_same_note(
    synth: *mut Synth,
    chan: i32,
    key: i32,
) {
    let mut i;
    let mut voice;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if ((*voice).status as i32 == FLUID_VOICE_ON as i32
            || (*voice).status as i32 == FLUID_VOICE_SUSTAINED as i32)
            && (*voice).chan as i32 == chan
            && (*voice).key as i32 == key
            && fluid_voice_get_id(voice) != (*synth).noteid
        {
            fluid_voice_noteoff(voice, &*synth);
        }
        i += 1
    }
}

pub unsafe fn fluid_synth_set_interp_method(
    synth: *mut Synth,
    chan: i32,
    interp_method: i32,
) -> i32 {
    let mut i;
    i = 0 as i32;
    while i < (*synth).midi_channels {
        if chan < 0 as i32
            || fluid_channel_get_num(&(*synth).channel[chan as usize]) == chan
        {
            fluid_channel_set_interp_method(&mut (*synth).channel[chan as usize], interp_method);
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_count_midi_channels(synth: *const Synth) -> i32 {
    return (*synth).midi_channels;
}

pub unsafe fn fluid_synth_count_audio_channels(synth: *const Synth) -> i32 {
    return (*synth).audio_channels;
}

pub unsafe fn fluid_synth_count_audio_groups(synth: *const Synth) -> i32 {
    return (*synth).audio_groups;
}

pub unsafe fn fluid_synth_count_effects_channels(synth: *const Synth) -> i32 {
    return (*synth).effects_channels;
}
unsafe fn fluid_synth_get_tuning(
    synth: *const Synth,
    bank: i32,
    prog: i32,
) -> *mut Tuning {
    if bank < 0 as i32 || bank >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Bank number out of range",);
        return 0 as *mut Tuning;
    }
    if prog < 0 as i32 || prog >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Program number out of range",);
        return 0 as *mut Tuning;
    }
    if (*synth).tuning.is_null()
        || (*(*synth).tuning.offset(bank as isize)).is_null()
        || (*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).is_null()
    {
        fluid_log!(FLUID_WARN, "No tuning at bank {}, prog {}", bank, prog);
        return 0 as *mut Tuning;
    }
    return *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
}
unsafe fn fluid_synth_create_tuning(
    synth: *mut Synth,
    bank: i32,
    prog: i32,
    name: *const libc::c_char,
) -> *mut Tuning {
    if bank < 0 as i32 || bank >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Bank number out of range",);
        return 0 as *mut Tuning;
    }
    if prog < 0 as i32 || prog >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Program number out of range",);
        return 0 as *mut Tuning;
    }
    if (*synth).tuning.is_null() {
        (*synth).tuning = libc::malloc(
            (128 as i32 as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut *mut Tuning>() as libc::size_t),
        ) as *mut *mut *mut Tuning;
        if (*synth).tuning.is_null() {
            fluid_log!(FLUID_PANIC as i32, "Out of memory",);
            return 0 as *mut Tuning;
        }
        libc::memset(
            (*synth).tuning as *mut libc::c_void,
            0 as i32,
            (128 as i32 as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut *mut Tuning>() as libc::size_t),
        );
    }
    if (*(*synth).tuning.offset(bank as isize)).is_null() {
        let ref mut fresh31 = *(*synth).tuning.offset(bank as isize);
        *fresh31 = libc::malloc(
            (128 as i32 as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut Tuning>() as libc::size_t),
        ) as *mut *mut Tuning;
        if (*(*synth).tuning.offset(bank as isize)).is_null() {
            fluid_log!(FLUID_PANIC as i32, "Out of memory",);
            return 0 as *mut Tuning;
        }
        libc::memset(
            *(*synth).tuning.offset(bank as isize) as *mut libc::c_void,
            0 as i32,
            (128 as i32 as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<*mut Tuning>() as libc::size_t),
        );
    }
    if (*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).is_null() {
        let ref mut fresh32 = *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
        *fresh32 = new_fluid_tuning(name, bank, prog);
        if (*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).is_null() {
            return 0 as *mut Tuning;
        }
    }
    if fluid_tuning_get_name((*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).as_ref().unwrap())
        .is_null()
        || libc::strcmp(
            fluid_tuning_get_name((*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).as_ref().unwrap()),
            name,
        ) != 0 as i32
    {
        fluid_tuning_set_name(
            (*(*(*synth).tuning.offset(bank as isize)).offset(prog as isize)).as_mut().unwrap(),
            name,
        );
    }
    return *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
}

pub unsafe fn fluid_synth_create_key_tuning(
    synth: *mut Synth,
    bank: i32,
    prog: i32,
    name: *const libc::c_char,
    pitch: *mut f64,
) -> i32 {
    let tuning: *mut Tuning = fluid_synth_create_tuning(synth, bank, prog, name);
    if tuning.is_null() {
        return FLUID_FAILED as i32;
    }
    if !pitch.is_null() {
        fluid_tuning_set_all(tuning.as_mut().unwrap(), pitch);
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_create_octave_tuning(
    synth: *mut Synth,
    bank: i32,
    prog: i32,
    name: *const libc::c_char,
    pitch: *const f64,
) -> i32 {
    let tuning;
    if synth.is_null() {
        return FLUID_FAILED as i32;
    }
    if !(bank >= 0 as i32 && bank < 128 as i32) {
        return FLUID_FAILED as i32;
    }
    if !(prog >= 0 as i32 && prog < 128 as i32) {
        return FLUID_FAILED as i32;
    }
    if name.is_null() {
        return FLUID_FAILED as i32;
    }
    if pitch.is_null() {
        return FLUID_FAILED as i32;
    }
    tuning = fluid_synth_create_tuning(synth, bank, prog, name);
    if tuning.is_null() {
        return FLUID_FAILED as i32;
    }
    fluid_tuning_set_octave(tuning.as_mut().unwrap(), pitch);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_activate_octave_tuning(
    synth: *mut Synth,
    bank: i32,
    prog: i32,
    name: *const libc::c_char,
    pitch: *const f64,
    _apply: i32,
) -> i32 {
    return fluid_synth_create_octave_tuning(synth, bank, prog, name, pitch);
}

pub unsafe fn fluid_synth_tune_notes(
    synth: *mut Synth,
    bank: i32,
    prog: i32,
    len: i32,
    key: *mut i32,
    pitch: *mut f64,
    _apply: i32,
) -> i32 {
    let mut tuning;
    let mut i;
    if synth.is_null() {
        return FLUID_FAILED as i32;
    }
    if !(bank >= 0 as i32 && bank < 128 as i32) {
        return FLUID_FAILED as i32;
    }
    if !(prog >= 0 as i32 && prog < 128 as i32) {
        return FLUID_FAILED as i32;
    }
    if !(len > 0 as i32) {
        return FLUID_FAILED as i32;
    }
    if key.is_null() {
        return FLUID_FAILED as i32;
    }
    if pitch.is_null() {
        return FLUID_FAILED as i32;
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
        return FLUID_FAILED as i32;
    }
    i = 0 as i32;
    while i < len {
        fluid_tuning_set_pitch(tuning.as_mut().unwrap(), *key.offset(i as isize), *pitch.offset(i as isize));
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_select_tuning(
    synth: *mut Synth,
    chan: i32,
    bank: i32,
    prog: i32,
) -> i32 {
    let tuning;
    if synth.is_null() {
        return FLUID_FAILED as i32;
    }
    if !(bank >= 0 as i32 && bank < 128 as i32) {
        return FLUID_FAILED as i32;
    }
    if !(prog >= 0 as i32 && prog < 128 as i32) {
        return FLUID_FAILED as i32;
    }
    tuning = fluid_synth_get_tuning(synth, bank, prog);
    if tuning.is_null() {
        return FLUID_FAILED as i32;
    }
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    let ref mut fresh33 = (*synth).channel[chan as usize].tuning;
    *fresh33 = *(*(*synth).tuning.offset(bank as isize)).offset(prog as isize);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_activate_tuning(
    synth: *mut Synth,
    chan: i32,
    bank: i32,
    prog: i32,
    _apply: i32,
) -> i32 {
    return fluid_synth_select_tuning(synth, chan, bank, prog);
}

pub unsafe fn fluid_synth_reset_tuning(
    synth: *mut Synth,
    chan: i32,
) -> i32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    let ref mut fresh34 = (*synth).channel[chan as usize].tuning;
    *fresh34 = 0 as *mut Tuning;
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_tuning_iteration_start(synth: *mut Synth) {
    (*synth).cur_tuning = 0 as *mut Tuning;
}

pub unsafe fn fluid_synth_tuning_iteration_next(
    synth: *mut Synth,
    bank: *mut i32,
    prog: *mut i32,
) -> i32 {
    let mut b: i32 = 0 as i32;
    let mut p: i32 = 0 as i32;
    if (*synth).tuning.is_null() {
        return 0 as i32;
    }
    if !(*synth).cur_tuning.is_null() {
        b = (*(*synth).cur_tuning).bank;
        p = 1 as i32 + (*(*synth).cur_tuning).prog;
        if p >= 128 as i32 {
            p = 0 as i32;
            b += 1
        }
    }
    while b < 128 as i32 {
        if !(*(*synth).tuning.offset(b as isize)).is_null() {
            while p < 128 as i32 {
                if !(*(*(*synth).tuning.offset(b as isize)).offset(p as isize)).is_null() {
                    (*synth).cur_tuning = *(*(*synth).tuning.offset(b as isize)).offset(p as isize);
                    *bank = b;
                    *prog = p;
                    return 1 as i32;
                }
                p += 1
            }
        }
        p = 0 as i32;
        b += 1
    }
    return 0 as i32;
}

pub unsafe fn fluid_synth_tuning_dump(
    synth: *const Synth,
    bank: i32,
    prog: i32,
    name: *mut libc::c_char,
    len: i32,
    pitch: *mut f64,
) -> i32 {
    let tuning: *mut Tuning = fluid_synth_get_tuning(synth, bank, prog);
    if tuning.is_null() {
        return FLUID_FAILED as i32;
    }
    if !name.is_null() {
        libc::strncpy(
            name,
            fluid_tuning_get_name(tuning.as_ref().unwrap()),
            (len - 1 as i32) as libc::size_t,
        );
        *name.offset((len - 1 as i32) as isize) = 0 as i32 as libc::c_char
    }
    if !pitch.is_null() {
        libc::memcpy(
            pitch as *mut libc::c_void,
            &mut *(*tuning)
                .pitch
                .as_mut_ptr()
                .offset(0 as i32 as isize) as *mut f64 as *const libc::c_void,
            (128 as i32 as libc::size_t)
                .wrapping_mul(::std::mem::size_of::<f64>() as libc::size_t),
        );
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_get_settings(synth: *mut Synth) -> *mut Settings {
    return (*synth).settings;
}

pub unsafe fn fluid_synth_set_gen(
    synth: *mut Synth,
    chan: i32,
    param: i32,
    value: f32,
) -> i32 {
    let mut i;
    let mut voice;
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if param < 0 as i32 || param >= GEN_LAST as i32 {
        fluid_log!(FLUID_WARN, "Parameter number out of range",);
        return FLUID_FAILED as i32;
    }
    (*synth).channel[chan as usize].gen[param as usize] = value;
    (*synth).channel[chan as usize].gen_abs[param as usize] =
        0 as i32 as libc::c_char;
    i = 0 as i32;
    while i < (*synth).polyphony {
        voice = *(*synth).voice.offset(i as isize);
        if (*voice).chan as i32 == chan {
            fluid_voice_set_param(voice, param, value, 0 as i32);
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_synth_get_gen(
    synth: *const Synth,
    chan: i32,
    param: i32,
) -> f32 {
    if chan < 0 as i32 || chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return 0.0f32;
    }
    if param < 0 as i32 || param >= GEN_LAST as i32 {
        fluid_log!(FLUID_WARN, "Parameter number out of range",);
        return 0.0f32;
    }
    return (*synth).channel[chan as usize].gen[param as usize];
}

pub unsafe fn fluid_synth_start(
    synth: *mut Synth,
    id: u32,
    preset: *mut Preset,
    _audio_chan: i32,
    midi_chan: i32,
    key: i32,
    vel: i32,
) -> i32 {
    let r;
    if midi_chan < 0 as i32 || midi_chan >= (*synth).midi_channels {
        fluid_log!(FLUID_WARN, "Channel out of range",);
        return FLUID_FAILED as i32;
    }
    if key < 0 as i32 || key >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Key out of range",);
        return FLUID_FAILED as i32;
    }
    if vel <= 0 as i32 || vel >= 128 as i32 {
        fluid_log!(FLUID_WARN, "Velocity out of range",);
        return FLUID_FAILED as i32;
    }
    (*synth).storeid = id;
    r = Some((*preset).noteon.expect("non-null function pointer"))
        .expect("non-null function pointer")(preset, synth, midi_chan, key, vel);
    return r;
}

pub unsafe fn fluid_synth_get_bank_offset0(
    synth: *const Synth,
    sfont_id: i32,
) -> *const BankOffset {
    let mut list = (*synth).bank_offsets as *const List;
    let mut offset;
    while !list.is_null() {
        offset = if !list.is_null() {
            (*list).data
        } else {
            0 as *const libc::c_void
        } as *const BankOffset;
        if (*offset).sfont_id == sfont_id {
            return offset;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *const List
        }
    }
    return 0 as *const BankOffset;
}

pub unsafe fn fluid_synth_get_mut_bank_offset0(
    synth: *mut Synth,
    sfont_id: i32,
) -> *mut BankOffset {
    let mut list = (*synth).bank_offsets;
    let mut offset;
    while !list.is_null() {
        offset = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut BankOffset;
        if (*offset).sfont_id == sfont_id {
            return offset;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        }
    }
    return 0 as *mut BankOffset;
}


pub unsafe fn fluid_synth_set_bank_offset(
    synth: *mut Synth,
    sfont_id: i32,
    offset: i32,
) -> i32 {
    let mut bank_offset;
    bank_offset = fluid_synth_get_mut_bank_offset0(synth, sfont_id);
    if bank_offset.is_null() {
        bank_offset = libc::malloc(::std::mem::size_of::<BankOffset>() as libc::size_t)
            as *mut BankOffset;
        if bank_offset.is_null() {
            return -(1 as i32);
        }
        (*bank_offset).sfont_id = sfont_id;
        (*bank_offset).offset = offset;
        (*synth).bank_offsets =
            fluid_list_prepend((*synth).bank_offsets, bank_offset as *mut libc::c_void)
    } else {
        (*bank_offset).offset = offset
    }
    return 0 as i32;
}

pub unsafe fn fluid_synth_get_bank_offset(
    synth: *const Synth,
    sfont_id: i32,
) -> i32 {
    let bank_offset;
    bank_offset = fluid_synth_get_bank_offset0(synth, sfont_id);
    return if bank_offset.is_null() {
        0 as i32
    } else {
        (*bank_offset).offset
    };
}

pub unsafe fn fluid_synth_remove_bank_offset(synth: *mut Synth, sfont_id: i32) {
    let bank_offset;
    bank_offset = fluid_synth_get_bank_offset0(synth, sfont_id);
    if !bank_offset.is_null() {
        (*synth).bank_offsets =
            fluid_list_remove((*synth).bank_offsets, bank_offset as *mut libc::c_void)
    };
}
