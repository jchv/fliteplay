#![allow(warnings)]

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

mod channel;
mod chorus;
mod conv;
mod defsfont;
mod dsp_float;
mod gen;
mod hash;
mod list;
mod modulator;
mod ramsfont;
mod reverb;
mod settings;
mod sfont;
mod synth;
mod fluid_sys;
mod fluid_tuning;
mod fluid_voice;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
