use std::{
    fs::File,
    io::Write,
};
use byte_slice_cast::AsByteSlice;
use fluidlite::{Settings, Synth};
use redoxsynth as _;
fn main() {
    let settings = Settings::new().unwrap();
    let synth = Synth::new(settings).unwrap();
    synth.sfload("gm.sf2", true).unwrap();
    let mut buffer = [0i16; 44100 * 2];
    let mut file = File::create("soundfont-sample.pcm").unwrap();
    synth.note_on(0, 60, 127).unwrap();
    synth.write(buffer.as_mut()).unwrap();
    file.write(buffer.as_byte_slice()).unwrap();
    synth.note_off(0, 60).unwrap();
    synth.write(buffer.as_mut()).unwrap();
    file.write(buffer.as_byte_slice()).unwrap();
}
