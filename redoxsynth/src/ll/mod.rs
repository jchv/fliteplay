#[macro_use]
mod redoxsynth_macros {
    macro_rules! fluid_log {
        ($log_level:expr, $fmt_string:expr, $( $arg:expr ),*) => {
            println!($fmt_string, $( $arg ),*);
        }
    }

    macro_rules! gerr {
        ($err:expr, $fmt_string:expr, $( $arg:expr ),*) => {
            { println!($fmt_string, $( $arg ),*); 0 }
        }
    }
}

pub const SEEK_SET: u32 = 0;
pub const SEEK_CUR: u32 = 1;
pub const SEEK_END: u32 = 2;
pub const HINT_BOUNDED_BELOW: u32 = 1;
pub const HINT_BOUNDED_ABOVE: u32 = 2;
pub const HINT_TOGGLED: u32 = 4;
pub const HINT_SAMPLE_RATE: u32 = 8;
pub const HINT_LOGARITHMIC: u32 = 16;
pub const HINT_INTEGER: u32 = 32;
pub const HINT_FILENAME: u32 = 1;
pub const HINT_OPTIONLIST: u32 = 2;
pub const REVERB_DEFAULT_ROOMSIZE: f64 = 0.2;
pub const REVERB_DEFAULT_DAMP: f64 = 0.0;
pub const REVERB_DEFAULT_WIDTH: f64 = 0.5;
pub const REVERB_DEFAULT_LEVEL: f64 = 0.9;
pub const CHORUS_DEFAULT_N: u32 = 3;
pub const CHORUS_DEFAULT_LEVEL: f64 = 2.0;
pub const CHORUS_DEFAULT_SPEED: f64 = 0.3;
pub const CHORUS_DEFAULT_DEPTH: f64 = 8.0;

pub const OK: i32 = 0;
pub const FAILED: i32 = -1;

pub(crate) mod channel;
pub(crate) mod chorus;
pub(crate) mod conv;
pub(crate) mod defsfont;
pub(crate) mod dsp_float;
pub(crate) mod gen;
pub(crate) mod hash;
pub(crate) mod list;
pub(crate) mod modulator;
pub(crate) mod reverb;
pub(crate) mod settings;
pub(crate) mod sfont;
pub(crate) mod synth;
pub(crate) mod sys;
pub(crate) mod tuning;
pub(crate) mod voice;
