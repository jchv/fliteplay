use crate::{ll, Chan, Status, Synth};
use num_derive::FromPrimitive;

/**
Generator (effect) numbers

See also _SoundFont 2.01 specifications section 8.1.3_
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum GenParam {
    /** Sample start address offset (0-32767) */
    StartAddrOfs = ll::gen::GEN_STARTADDROFS,
    /**< Sample end address offset (-32767-0) */
    EndAddrOfs = ll::gen::GEN_ENDADDROFS,
    /**< Sample loop start address offset (-32767-32767) */
    StartLoopAddOfs = ll::gen::GEN_STARTLOOPADDROFS,
    /**< Sample loop end address offset (-32767-32767) */
    EndLoopAddrOfs = ll::gen::GEN_ENDLOOPADDROFS,
    /** Sample start address coarse offset (X 32768) */
    StartAddrCoarseOfs = ll::gen::GEN_STARTADDRCOARSEOFS,
    /** Modulation LFO to pitch */
    ModLfoToPitch = ll::gen::GEN_MODLFOTOPITCH,
    /** Vibrato LFO to pitch */
    VibLfoToPitch = ll::gen::GEN_VIBLFOTOPITCH,
    /** Modulation envelope to pitch */
    ModEnvToPitch = ll::gen::GEN_MODENVTOPITCH,
    /** Filter cutoff */
    FilterFc = ll::gen::GEN_FILTERFC,
    /** Filter Q */
    FilterQ = ll::gen::GEN_FILTERQ,
    /** Modulation LFO to filter cutoff */
    ModLfoToFilterFc = ll::gen::GEN_MODLFOTOFILTERFC,
    /** Modulation envelope to filter cutoff */
    ModEnvToFilterFc = ll::gen::GEN_MODENVTOFILTERFC,
    /** Sample end address coarse offset (X 32768) */
    EndAddrCoarseOfs = ll::gen::GEN_ENDADDRCOARSEOFS,
    /** Modulation LFO to volume */
    ModLfoToVol = ll::gen::GEN_MODLFOTOVOL,
    /** Chorus send amount */
    ChorussEnd = ll::gen::GEN_CHORUSSEND,
    /** Reverb send amount */
    ReverbsEnd = ll::gen::GEN_REVERBSEND,
    /** Stereo panning */
    Pan = ll::gen::GEN_PAN,
    /** Modulation LFO delay */
    ModLfoDelay = ll::gen::GEN_MODLFODELAY,
    /** Modulation LFO frequency */
    ModLfoFreq = ll::gen::GEN_MODLFOFREQ,
    /** Vibrato LFO delay */
    Viblfodelay = ll::gen::GEN_VIBLFODELAY,
    /** Vibrato LFO frequency */
    VibLfoFreq = ll::gen::GEN_VIBLFOFREQ,
    /** Modulation envelope delay */
    ModEnvDelay = ll::gen::GEN_MODENVDELAY,
    /** Modulation envelope attack */
    ModEnvAttack = ll::gen::GEN_MODENVATTACK,
    /** Modulation envelope hold */
    ModEnvHold = ll::gen::GEN_MODENVHOLD,
    /** Modulation envelope decay */
    ModEnvDecay = ll::gen::GEN_MODENVDECAY,
    /** Modulation envelope sustain */
    ModEnvSustain = ll::gen::GEN_MODENVSUSTAIN,
    /** Modulation envelope release */
    ModEnvRelease = ll::gen::GEN_MODENVRELEASE,
    /** Key to modulation envelope hold */
    KeyToModEnvHold = ll::gen::GEN_KEYTOMODENVHOLD,
    /** Key to modulation envelope decay */
    KeyToModEnvDecay = ll::gen::GEN_KEYTOMODENVDECAY,
    /** Volume envelope delay */
    VolEnvDelay = ll::gen::GEN_VOLENVDELAY,
    /** Volume envelope attack */
    VolEnvAttack = ll::gen::GEN_VOLENVATTACK,
    /** Volume envelope hold */
    VolEnvHold = ll::gen::GEN_VOLENVHOLD,
    /** Volume envelope decay */
    VolEnvDecay = ll::gen::GEN_VOLENVDECAY,
    /** Volume envelope sustain */
    VolEnvSustain = ll::gen::GEN_VOLENVSUSTAIN,
    /** Volume envelope release */
    VolEnvRelease = ll::gen::GEN_VOLENVRELEASE,
    /** Key to volume envelope hold */
    KeyToVolEnvHold = ll::gen::GEN_KEYTOVOLENVHOLD,
    /** Key to volume envelope decay */
    KeyToVolEnvDecay = ll::gen::GEN_KEYTOVOLENVDECAY,
    /** Instrument ID (shouldn't be set by user) */
    Instrument = ll::gen::GEN_INSTRUMENT,
    /** MIDI note range */
    Keyrange = ll::gen::GEN_KEYRANGE,
    /** MIDI velocity range */
    Velrange = ll::gen::GEN_VELRANGE,
    /** Sample start loop address coarse offset (X 32768) */
    Startloopaddrcoarseofs = ll::gen::GEN_STARTLOOPADDRCOARSEOFS,
    /** Fixed MIDI note number */
    Keynum = ll::gen::GEN_KEYNUM,
    /** Fixed MIDI velocity value */
    Velocity = ll::gen::GEN_VELOCITY,
    /** Initial volume attenuation */
    Attenuation = ll::gen::GEN_ATTENUATION,
    /** Sample end loop address coarse offset (X 32768) */
    EndLoopAddrCoarseOfs = ll::gen::GEN_ENDLOOPADDRCOARSEOFS,
    /** Coarse tuning */
    CoarseTune = ll::gen::GEN_COARSETUNE,
    /** Fine tuning */
    FineTune = ll::gen::GEN_FINETUNE,
    /** Sample ID (shouldn't be set by user) */
    SampleId = ll::gen::GEN_SAMPLEID,
    /** Sample mode flags */
    SampleMode = ll::gen::GEN_SAMPLEMODE,
    /** Scale tuning */
    ScaleTune = ll::gen::GEN_SCALETUNE,
    /** Exclusive class number */
    ExclusiveClass = ll::gen::GEN_EXCLUSIVECLASS,
    /** Sample root note override */
    OverrideRootKey = ll::gen::GEN_OVERRIDEROOTKEY,

    /** Pitch (NOTE: Not a real SoundFont generator)

    The initial pitch is not a "standard" generator. It is not
    mentioned in the list of generator in the SF2 specifications. It
    is used, however, as the destination for the default pitch wheel
    modulator.
     */
    Pitch = ll::gen::GEN_PITCH,
}

/**
Generator interface
 */
impl Synth {
    /**
    Change the value of a generator. This function allows to control
    all synthesis parameters in real-time. The changes are additive,
    i.e. they add up to the existing parameter value. This function is
    similar to sending an NRPN message to the synthesizer. The
    function accepts a float as the value of the parameter. The
    parameter numbers and ranges are described in the SoundFont 2.01
    specification, paragraph 8.1.3, page 48.
     */
    pub fn set_gen(&mut self, chan: Chan, param: GenParam, value: f32) -> Status {
        Synth::zero_ok(unsafe { self.handle.set_gen(chan as _, param as _, value) })
    }

    /**
    Retreive the value of a generator. This function returns the value
    set by a previous call 'set_gen()' or by an NRPN message.

    Returns the value of the generator.
     */
    pub fn get_gen(&self, chan: Chan, param: GenParam) -> f32 {
        unsafe { self.handle.get_gen(chan as _, param as _) }
    }
}
