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
mod fluid_dsp_float;
mod fluid_gen;
mod fluid_hash;
mod fluid_list;
mod fluid_mod;
mod fluid_ramsfont;
mod fluid_rev;
mod fluid_settings;
mod fluid_sfont;
mod fluid_synth;
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
