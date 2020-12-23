use crate::ll::channel::fluid_channel_get_interp_method;
use crate::ll::channel::fluid_channel_get_num;
use crate::ll::channel::Channel;
use crate::ll::conv::fluid_act2hz;
use crate::ll::conv::fluid_atten2amp;
use crate::ll::conv::fluid_cb2amp;
use crate::ll::conv::fluid_ct2hz;
use crate::ll::conv::fluid_ct2hz_real;
use crate::ll::conv::fluid_pan;
use crate::ll::conv::fluid_tc2sec;
use crate::ll::conv::fluid_tc2sec_attack;
use crate::ll::conv::fluid_tc2sec_delay;
use crate::ll::conv::fluid_tc2sec_release;
use crate::ll::dsp_float::fluid_dsp_float_interpolate_4th_order;
use crate::ll::dsp_float::fluid_dsp_float_interpolate_7th_order;
use crate::ll::dsp_float::fluid_dsp_float_interpolate_linear;
use crate::ll::dsp_float::fluid_dsp_float_interpolate_none;
use crate::ll::gen::fluid_gen_init;
use crate::ll::gen::fluid_gen_t;
use crate::ll::modulator::fluid_mod_clone;
use crate::ll::modulator::fluid_mod_get_dest;
use crate::ll::modulator::fluid_mod_get_value;
use crate::ll::modulator::fluid_mod_t;
use crate::ll::modulator::fluid_mod_test_identity;
use crate::ll::sfont::Sample;
use crate::ll::tuning::Tuning;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_voice_t {
    pub id: libc::c_uint,
    pub status: libc::c_uchar,
    pub chan: libc::c_uchar,
    pub key: libc::c_uchar,
    pub vel: libc::c_uchar,
    pub channel: *mut Channel,
    pub gen: [fluid_gen_t; 60],
    pub mod_0: [fluid_mod_t; 64],
    pub mod_count: libc::c_int,
    pub has_looped: libc::c_int,
    pub sample: *mut Sample,
    pub check_sample_sanity_flag: libc::c_int,
    pub output_rate: f32,
    pub start_time: libc::c_uint,
    pub ticks: libc::c_uint,
    pub noteoff_ticks: libc::c_uint,
    pub amp: f32,
    pub phase: Phase,
    pub phase_incr: f32,
    pub amp_incr: f32,
    pub dsp_buf: *mut f32,
    pub pitch: f32,
    pub attenuation: f32,
    pub min_attenuation_c_b: f32,
    pub root_pitch: f32,
    pub start: libc::c_int,
    pub end: libc::c_int,
    pub loopstart: libc::c_int,
    pub loopend: libc::c_int,
    pub synth_gain: f32,
    pub volenv_data: [fluid_env_data_t; 7],
    pub volenv_count: libc::c_uint,
    pub volenv_section: libc::c_int,
    pub volenv_val: f32,
    pub amplitude_that_reaches_noise_floor_nonloop: f32,
    pub amplitude_that_reaches_noise_floor_loop: f32,
    pub modenv_data: [fluid_env_data_t; 7],
    pub modenv_count: libc::c_uint,
    pub modenv_section: libc::c_int,
    pub modenv_val: f32,
    pub modenv_to_fc: f32,
    pub modenv_to_pitch: f32,
    pub modlfo_val: f32,
    pub modlfo_delay: libc::c_uint,
    pub modlfo_incr: f32,
    pub modlfo_to_fc: f32,
    pub modlfo_to_pitch: f32,
    pub modlfo_to_vol: f32,
    pub viblfo_val: f32,
    pub viblfo_delay: libc::c_uint,
    pub viblfo_incr: f32,
    pub viblfo_to_pitch: f32,
    pub fres: f32,
    pub last_fres: f32,
    pub q_lin: f32,
    pub filter_gain: f32,
    pub hist1: f32,
    pub hist2: f32,
    pub filter_startup: libc::c_int,
    pub b02: f32,
    pub b1: f32,
    pub a1: f32,
    pub a2: f32,
    pub b02_incr: f32,
    pub b1_incr: f32,
    pub a1_incr: f32,
    pub a2_incr: f32,
    pub filter_coeff_incr_count: libc::c_int,
    pub pan: f32,
    pub amp_left: f32,
    pub amp_right: f32,
    pub reverb_send: f32,
    pub amp_reverb: f32,
    pub chorus_send: f32,
    pub amp_chorus: f32,
    pub interp_method: libc::c_int,
    pub debug: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_env_data_t {
    pub count: libc::c_uint,
    pub coeff: f32,
    pub incr: f32,
    pub min: f32,
    pub max: f32,
}
pub type Phase = libc::c_ulonglong;
pub type C2RustUnnamed = libc::c_uint;
pub const FLUID_SAMPLE_DONE: C2RustUnnamed = 2;
pub type ModFlags = libc::c_uint;
pub const FLUID_MOD_CC: ModFlags = 16;
pub const FLUID_MOD_BIPOLAR: ModFlags = 2;
pub type ModSrc = libc::c_uint;
pub const FLUID_MOD_PITCHWHEEL: ModSrc = 14;
pub type GenType = libc::c_uint;
pub const GEN_PITCH: GenType = 59;
pub const GEN_OVERRIDEROOTKEY: GenType = 58;
pub const GEN_EXCLUSIVECLASS: GenType = 57;
pub const GEN_SCALETUNE: GenType = 56;
pub const GEN_SAMPLEMODE: GenType = 54;
pub const GEN_FINETUNE: GenType = 52;
pub const GEN_COARSETUNE: GenType = 51;
pub const GEN_ENDLOOPADDRCOARSEOFS: GenType = 50;
pub const GEN_ATTENUATION: GenType = 48;
pub const GEN_VELOCITY: GenType = 47;
pub const GEN_KEYNUM: GenType = 46;
pub const GEN_STARTLOOPADDRCOARSEOFS: GenType = 45;
pub const GEN_KEYTOVOLENVDECAY: GenType = 40;
pub const GEN_KEYTOVOLENVHOLD: GenType = 39;
pub const GEN_VOLENVRELEASE: GenType = 38;
pub const GEN_VOLENVSUSTAIN: GenType = 37;
pub const GEN_VOLENVDECAY: GenType = 36;
pub const GEN_VOLENVHOLD: GenType = 35;
pub const GEN_VOLENVATTACK: GenType = 34;
pub const GEN_VOLENVDELAY: GenType = 33;
pub const GEN_KEYTOMODENVDECAY: GenType = 32;
pub const GEN_KEYTOMODENVHOLD: GenType = 31;
pub const GEN_MODENVRELEASE: GenType = 30;
pub const GEN_MODENVSUSTAIN: GenType = 29;
pub const GEN_MODENVDECAY: GenType = 28;
pub const GEN_MODENVHOLD: GenType = 27;
pub const GEN_MODENVATTACK: GenType = 26;
pub const GEN_MODENVDELAY: GenType = 25;
pub const GEN_VIBLFOFREQ: GenType = 24;
pub const GEN_VIBLFODELAY: GenType = 23;
pub const GEN_MODLFOFREQ: GenType = 22;
pub const GEN_MODLFODELAY: GenType = 21;
pub const GEN_PAN: GenType = 17;
pub const GEN_REVERBSEND: GenType = 16;
pub const GEN_CHORUSSEND: GenType = 15;
pub const GEN_MODLFOTOVOL: GenType = 13;
pub const GEN_ENDADDRCOARSEOFS: GenType = 12;
pub const GEN_MODENVTOFILTERFC: GenType = 11;
pub const GEN_MODLFOTOFILTERFC: GenType = 10;
pub const GEN_FILTERQ: GenType = 9;
pub const GEN_FILTERFC: GenType = 8;
pub const GEN_MODENVTOPITCH: GenType = 7;
pub const GEN_VIBLFOTOPITCH: GenType = 6;
pub const GEN_MODLFOTOPITCH: GenType = 5;
pub const GEN_STARTADDRCOARSEOFS: GenType = 4;
pub const GEN_ENDLOOPADDROFS: GenType = 3;
pub const GEN_STARTLOOPADDROFS: GenType = 2;
pub const GEN_ENDADDROFS: GenType = 1;
pub const GEN_STARTADDROFS: GenType = 0;
pub type GenFlags = libc::c_uint;
pub const GEN_ABS_NRPN: GenFlags = 2;
pub const GEN_SET: GenFlags = 1;
pub const FLUID_VOICE_ENVRELEASE: VoiceEnvelopeIndex = 5;
pub const FLUID_VOICE_ENVDECAY: VoiceEnvelopeIndex = 3;
pub const FLUID_VOICE_ENVHOLD: VoiceEnvelopeIndex = 2;
pub const FLUID_VOICE_ENVATTACK: VoiceEnvelopeIndex = 1;
pub const FLUID_VOICE_ENVDELAY: VoiceEnvelopeIndex = 0;
pub type FluidVoiceAddMod = libc::c_uint;
pub const FLUID_VOICE_ADD: FluidVoiceAddMod = 1;
pub const FLUID_VOICE_OVERWRITE: FluidVoiceAddMod = 0;
pub const FLUID_VOICE_SUSTAINED: VoiceStatus = 2;
pub const FLUID_VOICE_ON: VoiceStatus = 1;
pub const FLUID_OK: libc::c_int = 0;
pub type VoiceStatus = libc::c_uint;
pub const FLUID_VOICE_OFF: VoiceStatus = 3;
pub const FLUID_VOICE_CLEAN: VoiceStatus = 0;
pub type VoiceEnvelopeIndex = libc::c_uint;
pub const FLUID_VOICE_ENVFINISHED: VoiceEnvelopeIndex = 6;
pub const FLUID_VOICE_ENVSUSTAIN: VoiceEnvelopeIndex = 4;
pub const FLUID_LOOP_DURING_RELEASE: LoopMode = 1;
pub const FLUID_LOOP_UNTIL_RELEASE: LoopMode = 3;
pub const FLUID_UNLOOPED: LoopMode = 0;
pub const SUSTAIN_SWITCH: MidiControlChange = 64;
pub type MidiControlChange = libc::c_uint;
pub type LoopMode = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn new_fluid_voice(output_rate: f32) -> *mut fluid_voice_t {
    let voice: *mut fluid_voice_t;
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
    (*voice).channel = 0 as *mut Channel;
    (*voice).sample = 0 as *mut Sample;
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
pub unsafe extern "C" fn delete_fluid_voice(voice: *mut fluid_voice_t) -> libc::c_int {
    if voice.is_null() {
        return FLUID_OK as libc::c_int;
    }
    libc::free(voice as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_init(
    mut voice: *mut fluid_voice_t,
    sample: *mut Sample,
    channel: *mut Channel,
    key: libc::c_int,
    vel: libc::c_int,
    id: libc::c_uint,
    start_time: libc::c_uint,
    gain: f32,
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
    (*voice).last_fres = -(1 as libc::c_int) as f32;
    (*voice).filter_startup = 1 as libc::c_int;
    (*voice).interp_method = fluid_channel_get_interp_method((*voice).channel);
    (*voice).volenv_count = 0 as libc::c_int as libc::c_uint;
    (*voice).volenv_section = 0 as libc::c_int;
    (*voice).volenv_val = 0.0f32;
    (*voice).amp = 0.0f32;
    (*voice).modenv_count = 0 as libc::c_int as libc::c_uint;
    (*voice).modenv_section = 0 as libc::c_int;
    (*voice).modenv_val = 0.0f32;
    (*voice).modlfo_val = 0.0f64 as f32;
    (*voice).viblfo_val = 0.0f32;
    (*voice).hist1 = 0 as libc::c_int as f32;
    (*voice).hist2 = 0 as libc::c_int as f32;
    fluid_gen_init(
        &mut *(*voice).gen.as_mut_ptr().offset(0 as libc::c_int as isize),
        channel,
    );
    (*voice).synth_gain = gain;
    if ((*voice).synth_gain as f64) < 0.0000001f64 {
        (*voice).synth_gain = 0.0000001f64 as f32
    }
    (*voice).amplitude_that_reaches_noise_floor_nonloop =
        (0.00003f64 / (*voice).synth_gain as f64) as f32;
    (*voice).amplitude_that_reaches_noise_floor_loop =
        (0.00003f64 / (*voice).synth_gain as f64) as f32;
    (*(*voice).sample).refcount = (*(*voice).sample).refcount.wrapping_add(1);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_set(
    mut voice: *mut fluid_voice_t,
    i: libc::c_int,
    val: libc::c_float,
) {
    (*voice).gen[i as usize].val = val as f64;
    (*voice).gen[i as usize].flags = GEN_SET as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_incr(
    mut voice: *mut fluid_voice_t,
    i: libc::c_int,
    val: libc::c_float,
) {
    (*voice).gen[i as usize].val += val as f64;
    (*voice).gen[i as usize].flags = GEN_SET as libc::c_int as libc::c_uchar;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_get(
    voice: *mut fluid_voice_t,
    gen: libc::c_int,
) -> libc::c_float {
    return (*voice).gen[gen as usize].val as libc::c_float;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_gen_value(
    voice: *mut fluid_voice_t,
    num: libc::c_int,
) -> f32 {
    if (*voice).gen[num as usize].flags as libc::c_int == GEN_ABS_NRPN as libc::c_int {
        return (*voice).gen[num as usize].nrpn as f32;
    } else {
        return ((*voice).gen[num as usize].val
            + (*voice).gen[num as usize].mod_0
            + (*voice).gen[num as usize].nrpn) as f32;
    };
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_write(
    mut voice: *mut fluid_voice_t,
    dsp_left_buf: *mut f32,
    dsp_right_buf: *mut f32,
    dsp_reverb_buf: *mut f32,
    dsp_chorus_buf: *mut f32,
) -> libc::c_int {
    let current_block: u64;
    let mut fres;
    let target_amp;
    let count;
    let mut dsp_buf: [f32; 64] = [0.; 64];
    let mut env_data;
    let mut x;
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
        if (*voice).modlfo_val as f64 > 1.0f64 {
            (*voice).modlfo_incr = -(*voice).modlfo_incr;
            (*voice).modlfo_val = 2.0f64 as f32 - (*voice).modlfo_val
        } else if ((*voice).modlfo_val as f64) < -1.0f64 {
            (*voice).modlfo_incr = -(*voice).modlfo_incr;
            (*voice).modlfo_val = -2.0f64 as f32 - (*voice).modlfo_val
        }
    }
    if (*voice).ticks >= (*voice).viblfo_delay {
        (*voice).viblfo_val += (*voice).viblfo_incr;
        if (*voice).viblfo_val > 1.0f64 as f32 {
            (*voice).viblfo_incr = -(*voice).viblfo_incr;
            (*voice).viblfo_val = 2.0f64 as f32 - (*voice).viblfo_val
        } else if ((*voice).viblfo_val as f64) < -1.0f64 {
            (*voice).viblfo_incr = -(*voice).viblfo_incr;
            (*voice).viblfo_val = -2.0f64 as f32 - (*voice).viblfo_val
        }
    }
    if !((*voice).volenv_section == FLUID_VOICE_ENVDELAY as libc::c_int) {
        if (*voice).volenv_section == FLUID_VOICE_ENVATTACK as libc::c_int {
            target_amp = fluid_atten2amp((*voice).attenuation)
                * fluid_cb2amp((*voice).modlfo_val * -(*voice).modlfo_to_vol)
                * (*voice).volenv_val;
            current_block = 576355610076403033;
        } else {
            let amplitude_that_reaches_noise_floor;
            let amp_max;
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
            amp_max = fluid_atten2amp((*voice).min_attenuation_c_b) * (*voice).volenv_val;
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
                        (*voice).phase_incr = 1 as libc::c_int as f32
                    }
                    fres = fluid_ct2hz(
                        (*voice).fres
                            + (*voice).modlfo_val * (*voice).modlfo_to_fc
                            + (*voice).modenv_val * (*voice).modenv_to_fc,
                    );
                    if fres > 0.45f32 * (*voice).output_rate {
                        fres = 0.45f32 * (*voice).output_rate
                    } else if fres < 5 as libc::c_int as libc::c_float {
                        fres = 5 as libc::c_int as f32
                    }
                    if f64::abs((fres - (*voice).last_fres) as f64) > 0.01f64 {
                        let omega: f32 = (2.0f64
                            * std::f64::consts::PI
                            * (fres / (*voice).output_rate) as f64)
                            as f32;
                        let sin_coeff: f32 = f64::sin(omega.into()) as f32;
                        let cos_coeff: f32 = f64::cos(omega.into()) as f32;
                        let alpha_coeff: f32 = sin_coeff / (2.0f32 * (*voice).q_lin);
                        let a0_inv: f32 = 1.0f32 / (1.0f32 + alpha_coeff);
                        let a1_temp: f32 = -2.0f32 * cos_coeff * a0_inv;
                        let a2_temp: f32 = (1.0f32 - alpha_coeff) * a0_inv;
                        let b1_temp: f32 =
                            (1.0f32 - cos_coeff) * a0_inv * (*voice).filter_gain;
                        let b02_temp: f32 = b1_temp * 0.5f32;
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
    count: libc::c_int,
    dsp_left_buf: *mut f32,
    dsp_right_buf: *mut f32,
    dsp_reverb_buf: *mut f32,
    dsp_chorus_buf: *mut f32,
) {
    let mut dsp_hist1: f32 = (*voice).hist1;
    let mut dsp_hist2: f32 = (*voice).hist2;
    let mut dsp_a1: f32 = (*voice).a1;
    let mut dsp_a2: f32 = (*voice).a2;
    let mut dsp_b02: f32 = (*voice).b02;
    let mut dsp_b1: f32 = (*voice).b1;
    let dsp_a1_incr: f32 = (*voice).a1_incr;
    let dsp_a2_incr: f32 = (*voice).a2_incr;
    let dsp_b02_incr: f32 = (*voice).b02_incr;
    let dsp_b1_incr: f32 = (*voice).b1_incr;
    let mut dsp_filter_coeff_incr_count: libc::c_int = (*voice).filter_coeff_incr_count;
    let dsp_buf: *mut f32 = (*voice).dsp_buf;
    let mut dsp_centernode;
    let mut dsp_i;
    let mut v;
    if f64::abs(dsp_hist1 as f64) < 1e-20f64 {
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
    if -0.5f64 < (*voice).pan as f64 && ((*voice).pan as f64) < 0.5f64 {
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
        if (*voice).amp_left as f64 != 0.0f64 {
            dsp_i = 0 as libc::c_int;
            while dsp_i < count {
                let ref mut fresh3 = *dsp_left_buf.offset(dsp_i as isize);
                *fresh3 += (*voice).amp_left * *dsp_buf.offset(dsp_i as isize);
                dsp_i += 1
            }
        }
        if (*voice).amp_right as f64 != 0.0f64 {
            dsp_i = 0 as libc::c_int;
            while dsp_i < count {
                let ref mut fresh4 = *dsp_right_buf.offset(dsp_i as isize);
                *fresh4 += (*voice).amp_right * *dsp_buf.offset(dsp_i as isize);
                dsp_i += 1
            }
        }
    }
    if !dsp_reverb_buf.is_null() && (*voice).amp_reverb as f64 != 0.0f64 {
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
    voice: *mut fluid_voice_t,
) -> *mut Channel {
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
    let mut i;
    let list_of_generators_to_initialize: [libc::c_int; 35] = [
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
        let mod_0: *mut fluid_mod_t =
            &mut *(*voice).mod_0.as_mut_ptr().offset(i as isize) as *mut fluid_mod_t;
        let modval: f32 = fluid_mod_get_value(mod_0, (*voice).channel, voice);
        let dest_gen_index: libc::c_int = (*mod_0).dest as libc::c_int;
        let mut dest_gen: *mut fluid_gen_t =
            &mut *(*voice).gen.as_mut_ptr().offset(dest_gen_index as isize) as *mut fluid_gen_t;
        (*dest_gen).mod_0 += modval as f64;
        i += 1
    }
    if !(*(*voice).channel).tuning.is_null() {
        let tuning: *mut Tuning = (*(*voice).channel).tuning;
        (*voice).gen[GEN_PITCH as libc::c_int as usize].val = (*tuning).pitch
            [60 as libc::c_int as usize]
            + (*voice).gen[GEN_SCALETUNE as libc::c_int as usize].val / 100.0f32 as f64
                * ((*tuning).pitch[(*voice).key as usize]
                    - (*tuning).pitch[60 as libc::c_int as usize])
    } else {
        (*voice).gen[GEN_PITCH as libc::c_int as usize].val =
            (*voice).gen[GEN_SCALETUNE as libc::c_int as usize].val
                * ((*voice).key as libc::c_int as libc::c_float - 60.0f32) as f64
                + (100.0f32 * 60.0f32) as f64
    }
    i = 0 as libc::c_int;
    while list_of_generators_to_initialize[i as usize] != -(1 as libc::c_int) {
        fluid_voice_update_param(voice, list_of_generators_to_initialize[i as usize]);
        i += 1
    }
    (*voice).min_attenuation_c_b = fluid_voice_get_lower_boundary_for_attenuation(voice);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn calculate_hold_decay_buffers(
    voice: *mut fluid_voice_t,
    gen_base: libc::c_int,
    gen_key2base: libc::c_int,
    is_decay: libc::c_int,
) -> libc::c_int {
    let mut timecents;
    let seconds;
    let buffers;
    timecents = (((*voice).gen[gen_base as usize].val as f32
        + (*voice).gen[gen_base as usize].mod_0 as f32
        + (*voice).gen[gen_base as usize].nrpn as f32) as f64
        + ((*voice).gen[gen_key2base as usize].val as f32
            + (*voice).gen[gen_key2base as usize].mod_0 as f32
            + (*voice).gen[gen_key2base as usize].nrpn as f32) as f64
            * (60.0f64 - (*voice).key as libc::c_int as f64))
        as f32;
    if is_decay != 0 {
        if timecents as f64 > 8000.0f64 {
            timecents = 8000.0f64 as f32
        }
    } else {
        if timecents > 5000 as libc::c_int as libc::c_float {
            timecents = 5000.0f64 as f32
        }
        if timecents as f64 <= -32768.0f64 {
            return 0 as libc::c_int;
        }
    }
    if (timecents as f64) < -12000.0f64 {
        timecents = -12000.0f64 as f32
    }
    seconds = fluid_tc2sec(timecents);
    buffers = (((*voice).output_rate * seconds / 64 as libc::c_int as f32)
        as f64
        + 0.5f64) as libc::c_int;
    return buffers;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_update_param(
    mut voice: *mut fluid_voice_t,
    gen: libc::c_int,
) {
    let mut q_d_b;
    let mut x;
    let mut y;
    let mut count;
    // Alternate attenuation scale used by EMU10K1 cards when setting the attenuation at the preset or instrument level within the SoundFont bank.
    static mut ALT_ATTENUATION_SCALE: libc::c_float = 0.4f64 as libc::c_float;
    let current_block_195: u64;
    match gen {
        17 => {
            (*voice).pan = (*voice).gen[GEN_PAN as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_PAN as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_PAN as libc::c_int as usize].nrpn as f32;
            (*voice).amp_left =
                fluid_pan((*voice).pan, 1 as libc::c_int) * (*voice).synth_gain / 32768.0f32;
            (*voice).amp_right =
                fluid_pan((*voice).pan, 0 as libc::c_int) * (*voice).synth_gain / 32768.0f32;
            current_block_195 = 5267916556966421873;
        }
        48 => {
            (*voice).attenuation = (*voice).gen[GEN_ATTENUATION as libc::c_int as usize].val
                as f32
                * ALT_ATTENUATION_SCALE
                + (*voice).gen[GEN_ATTENUATION as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_ATTENUATION as libc::c_int as usize].nrpn as f32;
            (*voice).attenuation = if ((*voice).attenuation as f64) < 0.0f64 {
                0.0f64
            } else if (*voice).attenuation as f64 > 1440.0f64 {
                1440.0f64
            } else {
                (*voice).attenuation as f64
            } as f32;
            current_block_195 = 5267916556966421873;
        }
        59 | 51 | 52 => {
            (*voice).pitch = (*voice).gen[GEN_PITCH as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_PITCH as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_PITCH as libc::c_int as usize].nrpn as f32
                + 100.0f32
                    * ((*voice).gen[GEN_COARSETUNE as libc::c_int as usize].val as f32
                        + (*voice).gen[GEN_COARSETUNE as libc::c_int as usize].mod_0
                            as f32
                        + (*voice).gen[GEN_COARSETUNE as libc::c_int as usize].nrpn
                            as f32)
                + ((*voice).gen[GEN_FINETUNE as libc::c_int as usize].val as f32
                    + (*voice).gen[GEN_FINETUNE as libc::c_int as usize].mod_0 as f32
                    + (*voice).gen[GEN_FINETUNE as libc::c_int as usize].nrpn as f32);
            current_block_195 = 5267916556966421873;
        }
        16 => {
            (*voice).reverb_send = ((*voice).gen[GEN_REVERBSEND as libc::c_int as usize].val
                as f32
                + (*voice).gen[GEN_REVERBSEND as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_REVERBSEND as libc::c_int as usize].nrpn as f32)
                / 1000.0f32;
            (*voice).reverb_send = if ((*voice).reverb_send as f64) < 0.0f64 {
                0.0f64
            } else if (*voice).reverb_send as f64 > 1.0f64 {
                1.0f64
            } else {
                (*voice).reverb_send as f64
            } as f32;
            (*voice).amp_reverb = (*voice).reverb_send * (*voice).synth_gain / 32768.0f32;
            current_block_195 = 5267916556966421873;
        }
        15 => {
            (*voice).chorus_send = ((*voice).gen[GEN_CHORUSSEND as libc::c_int as usize].val
                as f32
                + (*voice).gen[GEN_CHORUSSEND as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_CHORUSSEND as libc::c_int as usize].nrpn as f32)
                / 1000.0f32;
            (*voice).chorus_send = if ((*voice).chorus_send as f64) < 0.0f64 {
                0.0f64
            } else if (*voice).chorus_send as f64 > 1.0f64 {
                1.0f64
            } else {
                (*voice).chorus_send as f64
            } as f32;
            (*voice).amp_chorus = (*voice).chorus_send * (*voice).synth_gain / 32768.0f32;
            current_block_195 = 5267916556966421873;
        }
        58 => {
            if (*voice).gen[GEN_OVERRIDEROOTKEY as libc::c_int as usize].val
                > -(1 as libc::c_int) as f64
            {
                //FIXME: use flag instead of -1
                (*voice).root_pitch = ((*voice).gen[GEN_OVERRIDEROOTKEY as libc::c_int as usize]
                    .val
                    * 100.0f32 as f64
                    - (*(*voice).sample).pitchadj as f64)
                    as f32
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
            (*voice).fres = (*voice).gen[GEN_FILTERFC as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_FILTERFC as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_FILTERFC as libc::c_int as usize].nrpn as f32;
            (*voice).last_fres = -1.0f32;
            current_block_195 = 5267916556966421873;
        }
        9 => {
            q_d_b = (((*voice).gen[GEN_FILTERQ as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_FILTERQ as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_FILTERQ as libc::c_int as usize].nrpn as f32)
                / 10.0f32) as f64;
            q_d_b = if q_d_b < 0.0f32 as f64 {
                0.0f32 as f64
            } else if q_d_b > 96.0f32 as f64 {
                96.0f32 as f64
            } else {
                q_d_b
            };
            q_d_b -= 3.01f32 as f64;
            (*voice).q_lin = f64::powf(10.0f32 as f64, q_d_b / 20.0f32 as f64)
                as f32;
            (*voice).filter_gain =
                (1.0f64 / f64::sqrt((*voice).q_lin as f64)) as f32;
            (*voice).last_fres = -1.0f64 as f32;
            current_block_195 = 5267916556966421873;
        }
        5 => {
            (*voice).modlfo_to_pitch = (*voice).gen[GEN_MODLFOTOPITCH as libc::c_int as usize].val
                as f32
                + (*voice).gen[GEN_MODLFOTOPITCH as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODLFOTOPITCH as libc::c_int as usize].nrpn as f32;
            (*voice).modlfo_to_pitch = if ((*voice).modlfo_to_pitch as f64) < -12000.0f64
            {
                -12000.0f64
            } else if (*voice).modlfo_to_pitch as f64 > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).modlfo_to_pitch as f64
            } as f32;
            current_block_195 = 5267916556966421873;
        }
        13 => {
            (*voice).modlfo_to_vol = (*voice).gen[GEN_MODLFOTOVOL as libc::c_int as usize].val
                as f32
                + (*voice).gen[GEN_MODLFOTOVOL as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODLFOTOVOL as libc::c_int as usize].nrpn as f32;
            (*voice).modlfo_to_vol = if ((*voice).modlfo_to_vol as f64) < -960.0f64 {
                -960.0f64
            } else if (*voice).modlfo_to_vol as f64 > 960.0f64 {
                960.0f64
            } else {
                (*voice).modlfo_to_vol as f64
            } as f32;
            current_block_195 = 5267916556966421873;
        }
        10 => {
            (*voice).modlfo_to_fc = (*voice).gen[GEN_MODLFOTOFILTERFC as libc::c_int as usize].val
                as f32
                + (*voice).gen[GEN_MODLFOTOFILTERFC as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODLFOTOFILTERFC as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_MODLFODELAY as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_MODLFODELAY as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODLFODELAY as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_MODLFOFREQ as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_MODLFOFREQ as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODLFOFREQ as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_VIBLFOFREQ as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_VIBLFOFREQ as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_VIBLFOFREQ as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_VIBLFODELAY as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_VIBLFODELAY as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_VIBLFODELAY as libc::c_int as usize].nrpn as f32;
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
                as f32
                + (*voice).gen[GEN_VIBLFOTOPITCH as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_VIBLFOTOPITCH as libc::c_int as usize].nrpn as f32;
            (*voice).viblfo_to_pitch = if ((*voice).viblfo_to_pitch as f64) < -12000.0f64
            {
                -12000.0f64
            } else if (*voice).viblfo_to_pitch as f64 > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).viblfo_to_pitch as f64
            } as f32;
            current_block_195 = 5267916556966421873;
        }
        46 => {
            x = (*voice).gen[GEN_KEYNUM as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_KEYNUM as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_KEYNUM as libc::c_int as usize].nrpn as f32;
            if x >= 0 as libc::c_int as libc::c_float {
                (*voice).key = x as libc::c_uchar
            }
            current_block_195 = 5267916556966421873;
        }
        47 => {
            x = (*voice).gen[GEN_VELOCITY as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_VELOCITY as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_VELOCITY as libc::c_int as usize].nrpn as f32;
            if x > 0 as libc::c_int as libc::c_float {
                (*voice).vel = x as libc::c_uchar
            }
            current_block_195 = 5267916556966421873;
        }
        7 => {
            (*voice).modenv_to_pitch = (*voice).gen[GEN_MODENVTOPITCH as libc::c_int as usize].val
                as f32
                + (*voice).gen[GEN_MODENVTOPITCH as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODENVTOPITCH as libc::c_int as usize].nrpn as f32;
            (*voice).modenv_to_pitch = if ((*voice).modenv_to_pitch as f64) < -12000.0f64
            {
                -12000.0f64
            } else if (*voice).modenv_to_pitch as f64 > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).modenv_to_pitch as f64
            } as f32;
            current_block_195 = 5267916556966421873;
        }
        11 => {
            (*voice).modenv_to_fc = (*voice).gen[GEN_MODENVTOFILTERFC as libc::c_int as usize].val
                as f32
                + (*voice).gen[GEN_MODENVTOFILTERFC as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODENVTOFILTERFC as libc::c_int as usize].nrpn as f32;
            (*voice).modenv_to_fc = if ((*voice).modenv_to_fc as f64) < -12000.0f64 {
                -12000.0f64
            } else if (*voice).modenv_to_fc as f64 > 12000.0f64 {
                12000.0f64
            } else {
                (*voice).modenv_to_fc as f64
            } as f32;
            current_block_195 = 5267916556966421873;
        }
        0 | 4 => {
            if !(*voice).sample.is_null() {
                (*voice).start = (*(*voice).sample)
                    .start
                    .wrapping_add(
                        ((*voice).gen[GEN_STARTADDROFS as libc::c_int as usize].val as f32
                            + (*voice).gen[GEN_STARTADDROFS as libc::c_int as usize].mod_0
                                as f32
                            + (*voice).gen[GEN_STARTADDROFS as libc::c_int as usize].nrpn
                                as f32) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_STARTADDRCOARSEOFS as libc::c_int as usize].val
                                as f32
                                + (*voice).gen[GEN_STARTADDRCOARSEOFS as libc::c_int as usize].mod_0
                                    as f32
                                + (*voice).gen[GEN_STARTADDRCOARSEOFS as libc::c_int as usize].nrpn
                                    as f32) as libc::c_int)
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
                        ((*voice).gen[GEN_ENDADDROFS as libc::c_int as usize].val as f32
                            + (*voice).gen[GEN_ENDADDROFS as libc::c_int as usize].mod_0
                                as f32
                            + (*voice).gen[GEN_ENDADDROFS as libc::c_int as usize].nrpn
                                as f32) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_ENDADDRCOARSEOFS as libc::c_int as usize].val
                                as f32
                                + (*voice).gen[GEN_ENDADDRCOARSEOFS as libc::c_int as usize].mod_0
                                    as f32
                                + (*voice).gen[GEN_ENDADDRCOARSEOFS as libc::c_int as usize].nrpn
                                    as f32) as libc::c_int)
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
                            as f32
                            + (*voice).gen[GEN_STARTLOOPADDROFS as libc::c_int as usize].mod_0
                                as f32
                            + (*voice).gen[GEN_STARTLOOPADDROFS as libc::c_int as usize].nrpn
                                as f32) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize].val
                                as f32
                                + (*voice).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .mod_0 as f32
                                + (*voice).gen[GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .nrpn as f32)
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
                            as f32
                            + (*voice).gen[GEN_ENDLOOPADDROFS as libc::c_int as usize].mod_0
                                as f32
                            + (*voice).gen[GEN_ENDLOOPADDROFS as libc::c_int as usize].nrpn
                                as f32) as libc::c_int
                            as libc::c_uint,
                    )
                    .wrapping_add(
                        (32768 as libc::c_int
                            * ((*voice).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize].val
                                as f32
                                + (*voice).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .mod_0 as f32
                                + (*voice).gen[GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as usize]
                                    .nrpn as f32)
                                as libc::c_int) as libc::c_uint,
                    ) as libc::c_int;
                (*voice).check_sample_sanity_flag = (1 as libc::c_int) << 0 as libc::c_int
            }
            current_block_195 = 5267916556966421873;
        }
        33 => {
            x = (*voice).gen[GEN_VOLENVDELAY as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_VOLENVDELAY as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_VOLENVDELAY as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_VOLENVATTACK as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_VOLENVATTACK as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_VOLENVATTACK as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_VOLENVRELEASE as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_VOLENVRELEASE as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_VOLENVRELEASE as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_MODENVDELAY as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_MODENVDELAY as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODENVDELAY as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_MODENVATTACK as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_MODENVATTACK as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODENVATTACK as libc::c_int as usize].nrpn as f32;
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
            x = (*voice).gen[GEN_MODENVRELEASE as libc::c_int as usize].val as f32
                + (*voice).gen[GEN_MODENVRELEASE as libc::c_int as usize].mod_0 as f32
                + (*voice).gen[GEN_MODENVRELEASE as libc::c_int as usize].nrpn as f32;
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
                    (-1.0f32 / count as libc::c_float) as f64
                } else {
                    0.0f64
                } as f32;
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
                    * ((*voice).gen[GEN_MODENVSUSTAIN as libc::c_int as usize].val as f32
                        + (*voice).gen[GEN_MODENVSUSTAIN as libc::c_int as usize].mod_0
                            as f32
                        + (*voice).gen[GEN_MODENVSUSTAIN as libc::c_int as usize].nrpn
                            as f32);
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
                    * ((*voice).gen[GEN_VOLENVSUSTAIN as libc::c_int as usize].val as f32
                        + (*voice).gen[GEN_VOLENVSUSTAIN as libc::c_int as usize].mod_0
                            as f32
                        + (*voice).gen[GEN_VOLENVSUSTAIN as libc::c_int as usize].nrpn
                            as f32);
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
    cc: libc::c_int,
    ctrl: libc::c_int,
) -> libc::c_int {
    let mut i;
    let mut k;
    let mut mod_0;
    let mut gen;
    let mut modval;
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
            modval = 0.0f64 as f32;
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
            (*voice).gen[gen as usize].mod_0 = modval as f64;
            fluid_voice_update_param(voice, gen);
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_modulate_all(mut voice: *mut fluid_voice_t) -> libc::c_int {
    let mut mod_0;
    let mut i;
    let mut k;
    let mut gen;
    let mut modval;
    i = 0 as libc::c_int;
    while i < (*voice).mod_count {
        mod_0 = &mut *(*voice).mod_0.as_mut_ptr().offset(i as isize) as *mut fluid_mod_t;
        gen = fluid_mod_get_dest(mod_0);
        modval = 0.0f64 as f32;
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
        (*voice).gen[gen as usize].mod_0 = modval as f64;
        fluid_voice_update_param(voice, gen);
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_noteoff(mut voice: *mut fluid_voice_t) -> libc::c_int {
    let at_tick;
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
                let lfo: f32 = (*voice).modlfo_val * -(*voice).modlfo_to_vol;
                let amp: f32 = ((*voice).volenv_val as f64
                    * f64::powf(
                        10.0f64,
                        (lfo / -(200 as libc::c_int) as libc::c_float) as f64,
                    )) as f32;
                let mut env_value: f32 =
                    -((-(200 as libc::c_int) as f64 * f64::ln(amp as f64)
                        / f64::ln(10.0f64)
                        - lfo as f64)
                        / 960.0f64
                        - 1 as libc::c_int as f64) as f32;
                env_value = if (env_value as f64) < 0.0f64 {
                    0.0f64
                } else if env_value as f64 > 1.0f64 {
                    1.0f64
                } else {
                    env_value as f64
                } as f32;
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
        (*voice).sample = 0 as *mut Sample
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn FluidVoiceAddMod(
    mut voice: *mut fluid_voice_t,
    mod_0: *mut fluid_mod_t,
    mode: libc::c_int,
) {
    let mut i;
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
pub unsafe extern "C" fn fluid_voice_get_id(voice: *mut fluid_voice_t) -> libc::c_uint {
    return (*voice).id;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_is_playing(voice: *mut fluid_voice_t) -> libc::c_int {
    return ((*voice).status as libc::c_int == FLUID_VOICE_ON as libc::c_int
        || (*voice).status as libc::c_int == FLUID_VOICE_SUSTAINED as libc::c_int)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_get_lower_boundary_for_attenuation(
    voice: *mut fluid_voice_t,
) -> f32 {
    let mut i;
    let mut mod_0;
    let mut possible_att_reduction_c_b: f32 = 0 as libc::c_int as f32;
    let mut lower_bound;
    i = 0 as libc::c_int;
    while i < (*voice).mod_count {
        mod_0 = &mut *(*voice).mod_0.as_mut_ptr().offset(i as isize) as *mut fluid_mod_t;
        if (*mod_0).dest as libc::c_int == GEN_ATTENUATION as libc::c_int
            && ((*mod_0).flags1 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0
                || (*mod_0).flags2 as libc::c_int & FLUID_MOD_CC as libc::c_int != 0)
        {
            let current_val: f32 = fluid_mod_get_value(mod_0, (*voice).channel, voice);
            let mut v: f32 = f64::abs((*mod_0).amount) as f32;
            if (*mod_0).src1 as libc::c_int == FLUID_MOD_PITCHWHEEL as libc::c_int
                || (*mod_0).flags1 as libc::c_int & FLUID_MOD_BIPOLAR as libc::c_int != 0
                || (*mod_0).flags2 as libc::c_int & FLUID_MOD_BIPOLAR as libc::c_int != 0
                || (*mod_0).amount < 0 as libc::c_int as f64
            {
                v = (v as f64 * -1.0f64) as f32
            } else {
                v = 0 as libc::c_int as f32
            }
            if current_val > v {
                possible_att_reduction_c_b += current_val - v
            }
        }
        i += 1
    }
    lower_bound = (*voice).attenuation - possible_att_reduction_c_b;
    if lower_bound < 0 as libc::c_int as libc::c_float {
        lower_bound = 0 as libc::c_int as f32
    }
    return lower_bound;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_check_sample_sanity(mut voice: *mut fluid_voice_t) {
    let min_index_nonloop: libc::c_int = (*(*voice).sample).start as libc::c_int;
    let max_index_nonloop: libc::c_int = (*(*voice).sample).end as libc::c_int;
    let min_index_loop: libc::c_int =
        (*(*voice).sample).start as libc::c_int + 0 as libc::c_int;
    let max_index_loop: libc::c_int =
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
        let temp: libc::c_int = (*voice).start;
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
            let temp_0: libc::c_int = (*voice).loopstart;
            (*voice).loopstart = (*voice).loopend;
            (*voice).loopend = temp_0
        }
        if (*voice).loopend < (*voice).loopstart + 2 as libc::c_int {
            (*voice).gen[GEN_SAMPLEMODE as libc::c_int as usize].val =
                FLUID_UNLOOPED as libc::c_int as f64
        }
        if (*voice).loopstart >= (*(*voice).sample).loopstart as libc::c_int
            && (*voice).loopend <= (*(*voice).sample).loopend as libc::c_int
        {
            if (*(*voice).sample).amplitude_that_reaches_noise_floor_is_valid != 0 {
                (*voice).amplitude_that_reaches_noise_floor_loop =
                    ((*(*voice).sample).amplitude_that_reaches_noise_floor
                        / (*voice).synth_gain as f64) as f32
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
                    FLUID_UNLOOPED as libc::c_int as f64
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
        let index_in_sample: libc::c_int =
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
    gen: libc::c_int,
    nrpn_value: f32,
    abs: libc::c_int,
) -> libc::c_int {
    (*voice).gen[gen as usize].nrpn = nrpn_value as f64;
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
    mut gain: f32,
) -> libc::c_int {
    if (gain as f64) < 0.0000001f64 {
        gain = 0.0000001f64 as f32
    }
    (*voice).synth_gain = gain;
    (*voice).amp_left = fluid_pan((*voice).pan, 1 as libc::c_int) * gain / 32768.0f32;
    (*voice).amp_right = fluid_pan((*voice).pan, 0 as libc::c_int) * gain / 32768.0f32;
    (*voice).amp_reverb = (*voice).reverb_send * gain / 32768.0f32;
    (*voice).amp_chorus = (*voice).chorus_send * gain / 32768.0f32;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_voice_optimize_sample(mut s: *mut Sample) -> libc::c_int {
    let mut peak_max: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut peak_min: libc::c_short = 0 as libc::c_int as libc::c_short;
    let mut peak;
    let normalized_amplitude_during_loop;
    let result;
    let mut i;
    if (*s).valid == 0 || (*s).sampletype & 0x10 as libc::c_int != 0 {
        return FLUID_OK as libc::c_int;
    }
    if (*s).amplitude_that_reaches_noise_floor_is_valid == 0 {
        i = (*s).loopstart as libc::c_int;
        while i < (*s).loopend as libc::c_int {
            let val: libc::c_short = *(*s).data.offset(i as isize);
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
            (peak as f32 as f64 / 32768.0f64) as f32;
        result = 0.00003f64 / normalized_amplitude_during_loop as f64;
        (*s).amplitude_that_reaches_noise_floor = result;
        (*s).amplitude_that_reaches_noise_floor_is_valid = 1 as libc::c_int
    }
    return FLUID_OK as libc::c_int;
}