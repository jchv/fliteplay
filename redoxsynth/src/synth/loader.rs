use crate::{ll, Loader, Synth};

impl Synth {
    /**
    Add a SoundFont loader to the synthesizer. Note that SoundFont
    loader don't necessarily load SoundFonts. They can load any type
    of wavetable data but export a SoundFont interface.
     */
    pub fn add_sfloader(&mut self, loader: Loader) {
        ll::synth::fluid_synth_add_sfloader(&mut self.handle, loader.into_ptr());
    }
}
