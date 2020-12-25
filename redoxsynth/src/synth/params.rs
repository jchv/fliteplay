use crate::{ll, Status, Synth};
use num_derive::FromPrimitive;

/* Flags to choose the interpolation method */
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, FromPrimitive)]
#[repr(u32)]
pub enum InterpMethod {
    /**
    No interpolation: Fastest, but questionable audio quality
     */
    None = 0,
    /**
    Straight-line interpolation: A bit slower, reasonable audio quality
     */
    Linear = 1,
    /**
    Fourth-order interpolation: Requires 50% of the whole DSP processing time, good quality
    (default)
     */
    FourthOrder = 4,
    /**
    Seventh-order interpolation
     */
    SeventhOrder = 7,
}

impl Default for InterpMethod {
    fn default() -> Self {
        Self::FourthOrder
    }
}

/**
Synthesis parameters
 */
impl Synth {
    /**
    Set the master gain
     */
    pub fn set_gain(&mut self, gain: f32) {
        unsafe {
            ll::synth::fluid_synth_set_gain(&mut self.handle, gain);
        }
    }

    /**
    Get the master gain
     */
    pub fn get_gain(&self) -> f32 {
        unsafe { ll::synth::fluid_synth_get_gain(&self.handle) }
    }

    /**
    Set the polyphony limit (FluidSynth >= 1.0.6)
     */
    pub fn set_polyphony(&mut self, polyphony: u32) -> Status {
        Synth::zero_ok(unsafe { ll::synth::fluid_synth_set_polyphony(&mut self.handle, polyphony as _) })
    }

    /**
    Get the polyphony limit (FluidSynth >= 1.0.6)
     */
    pub fn get_polyphony(&self) -> u32 {
        unsafe { ll::synth::fluid_synth_get_polyphony(&self.handle) as _ }
    }

    /**
    Get the internal buffer size. The internal buffer size if not the
    same thing as the buffer size specified in the
    settings. Internally, the synth *always* uses a specific buffer
    size independent of the buffer size used by the audio driver. The
    internal buffer size is normally 64 samples. The reason why it
    uses an internal buffer size is to allow audio drivers to call the
    synthesizer with a variable buffer length. The internal buffer
    size is useful for client who want to optimize their buffer sizes.
     */
    pub fn get_internal_buffer_size(&self) -> usize {
        unsafe { ll::synth::fluid_synth_get_internal_bufsize(&self.handle) as _ }
    }

    /** Set the interpolation method for one channel (`Some(chan)`) or all channels (`None`) */
    pub fn set_interp_method(&mut self, chan: Option<u32>, interp_method: InterpMethod) -> Status {
        let chan = if let Some(chan) = chan { chan as _ } else { -1 };
        Synth::zero_ok(unsafe {
            ll::synth::fluid_synth_set_interp_method(&mut self.handle, chan, interp_method as _)
        })
    }
}
