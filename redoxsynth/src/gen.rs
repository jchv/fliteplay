#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
use crate::channel::_fluid_channel_t;
use crate::sfont::_fluid_preset_t;
use crate::sfont::_fluid_sfont_t;
use crate::synth::_fluid_synth_t;
use crate::fluid_tuning::_fluid_tuning_t;
pub type fluid_synth_t = _fluid_synth_t;
pub type fluid_sfont_t = _fluid_sfont_t;
pub type fluid_preset_t = _fluid_preset_t;
pub type fluid_gen_type = libc::c_uint;
pub const GEN_LAST: fluid_gen_type = 60;
pub const GEN_PITCH: fluid_gen_type = 59;
pub const GEN_OVERRIDEROOTKEY: fluid_gen_type = 58;
pub const GEN_EXCLUSIVECLASS: fluid_gen_type = 57;
pub const GEN_SCALETUNE: fluid_gen_type = 56;
pub const GEN_RESERVED3: fluid_gen_type = 55;
pub const GEN_SAMPLEMODE: fluid_gen_type = 54;
pub const GEN_SAMPLEID: fluid_gen_type = 53;
pub const GEN_FINETUNE: fluid_gen_type = 52;
pub const GEN_COARSETUNE: fluid_gen_type = 51;
pub const GEN_ENDLOOPADDRCOARSEOFS: fluid_gen_type = 50;
pub const GEN_RESERVED2: fluid_gen_type = 49;
pub const GEN_ATTENUATION: fluid_gen_type = 48;
pub const GEN_VELOCITY: fluid_gen_type = 47;
pub const GEN_KEYNUM: fluid_gen_type = 46;
pub const GEN_STARTLOOPADDRCOARSEOFS: fluid_gen_type = 45;
pub const GEN_VELRANGE: fluid_gen_type = 44;
pub const GEN_KEYRANGE: fluid_gen_type = 43;
pub const GEN_RESERVED1: fluid_gen_type = 42;
pub const GEN_INSTRUMENT: fluid_gen_type = 41;
pub const GEN_KEYTOVOLENVDECAY: fluid_gen_type = 40;
pub const GEN_KEYTOVOLENVHOLD: fluid_gen_type = 39;
pub const GEN_VOLENVRELEASE: fluid_gen_type = 38;
pub const GEN_VOLENVSUSTAIN: fluid_gen_type = 37;
pub const GEN_VOLENVDECAY: fluid_gen_type = 36;
pub const GEN_VOLENVHOLD: fluid_gen_type = 35;
pub const GEN_VOLENVATTACK: fluid_gen_type = 34;
pub const GEN_VOLENVDELAY: fluid_gen_type = 33;
pub const GEN_KEYTOMODENVDECAY: fluid_gen_type = 32;
pub const GEN_KEYTOMODENVHOLD: fluid_gen_type = 31;
pub const GEN_MODENVRELEASE: fluid_gen_type = 30;
pub const GEN_MODENVSUSTAIN: fluid_gen_type = 29;
pub const GEN_MODENVDECAY: fluid_gen_type = 28;
pub const GEN_MODENVHOLD: fluid_gen_type = 27;
pub const GEN_MODENVATTACK: fluid_gen_type = 26;
pub const GEN_MODENVDELAY: fluid_gen_type = 25;
pub const GEN_VIBLFOFREQ: fluid_gen_type = 24;
pub const GEN_VIBLFODELAY: fluid_gen_type = 23;
pub const GEN_MODLFOFREQ: fluid_gen_type = 22;
pub const GEN_MODLFODELAY: fluid_gen_type = 21;
pub const GEN_UNUSED4: fluid_gen_type = 20;
pub const GEN_UNUSED3: fluid_gen_type = 19;
pub const GEN_UNUSED2: fluid_gen_type = 18;
pub const GEN_PAN: fluid_gen_type = 17;
pub const GEN_REVERBSEND: fluid_gen_type = 16;
pub const GEN_CHORUSSEND: fluid_gen_type = 15;
pub const GEN_UNUSED1: fluid_gen_type = 14;
pub const GEN_MODLFOTOVOL: fluid_gen_type = 13;
pub const GEN_ENDADDRCOARSEOFS: fluid_gen_type = 12;
pub const GEN_MODENVTOFILTERFC: fluid_gen_type = 11;
pub const GEN_MODLFOTOFILTERFC: fluid_gen_type = 10;
pub const GEN_FILTERQ: fluid_gen_type = 9;
pub const GEN_FILTERFC: fluid_gen_type = 8;
pub const GEN_MODENVTOPITCH: fluid_gen_type = 7;
pub const GEN_VIBLFOTOPITCH: fluid_gen_type = 6;
pub const GEN_MODLFOTOPITCH: fluid_gen_type = 5;
pub const GEN_STARTADDRCOARSEOFS: fluid_gen_type = 4;
pub const GEN_ENDLOOPADDROFS: fluid_gen_type = 3;
pub const GEN_STARTLOOPADDROFS: fluid_gen_type = 2;
pub const GEN_ENDADDROFS: fluid_gen_type = 1;
pub const GEN_STARTADDROFS: fluid_gen_type = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_gen_t {
    pub flags: libc::c_uchar,
    pub val: libc::c_double,
    pub mod_0: libc::c_double,
    pub nrpn: libc::c_double,
}
pub type fluid_gen_t = _fluid_gen_t;
pub type fluid_gen_flags = libc::c_uint;
pub const GEN_ABS_NRPN: fluid_gen_flags = 2;
pub const GEN_SET: fluid_gen_flags = 1;
pub const GEN_UNUSED: fluid_gen_flags = 0;
pub const FLUID_OK: C2RustUnnamed = 0;
pub type fluid_gen_info_t = _fluid_gen_info_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _fluid_gen_info_t {
    pub num: libc::c_char,
    pub init: libc::c_char,
    pub nrpn_scale: libc::c_char,
    pub min: libc::c_float,
    pub max: libc::c_float,
    pub def: libc::c_float,
}
pub type fluid_real_t = libc::c_float;
pub type C2RustUnnamed = libc::c_int;
pub const FLUID_FAILED: C2RustUnnamed = -1;

pub type fluid_tuning_t = _fluid_tuning_t;
pub type fluid_channel_t = _fluid_channel_t;

#[no_mangle]
pub static mut fluid_gen_info: [fluid_gen_info_t; 60] = [
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_STARTADDROFS as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1e10f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_ENDADDROFS as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1e10f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_STARTLOOPADDROFS as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1e10f32,
            max: 1e10f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_ENDLOOPADDROFS as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1e10f32,
            max: 1e10f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_STARTADDRCOARSEOFS as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1e10f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODLFOTOPITCH as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 12000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VIBLFOTOPITCH as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 12000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVTOPITCH as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 12000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_FILTERFC as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: 1500.0f32,
            max: 13500.0f32,
            def: 13500.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_FILTERQ as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 960.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODLFOTOFILTERFC as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 12000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVTOFILTERFC as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 12000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_ENDADDRCOARSEOFS as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1e10f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODLFOTOVOL as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -960.0f32,
            max: 960.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_UNUSED1 as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_CHORUSSEND as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_REVERBSEND as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_PAN as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -500.0f32,
            max: 500.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_UNUSED2 as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_UNUSED3 as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_UNUSED4 as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODLFODELAY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 5000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODLFOFREQ as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 4 as libc::c_int as libc::c_char,
            min: -16000.0f32,
            max: 4500.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VIBLFODELAY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 5000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VIBLFOFREQ as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 4 as libc::c_int as libc::c_char,
            min: -16000.0f32,
            max: 4500.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVDELAY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 5000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVATTACK as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 8000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVHOLD as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 5000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVDECAY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 8000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVSUSTAIN as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1000.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_MODENVRELEASE as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 8000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_KEYTOMODENVHOLD as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1200.0f32,
            max: 1200.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_KEYTOMODENVDECAY as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1200.0f32,
            max: 1200.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VOLENVDELAY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 5000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VOLENVATTACK as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 8000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VOLENVHOLD as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 5000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VOLENVDECAY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 8000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VOLENVSUSTAIN as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1440.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VOLENVRELEASE as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 2 as libc::c_int as libc::c_char,
            min: -12000.0f32,
            max: 8000.0f32,
            def: -12000.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_KEYTOVOLENVHOLD as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1200.0f32,
            max: 1200.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_KEYTOVOLENVDECAY as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1200.0f32,
            max: 1200.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_INSTRUMENT as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_RESERVED1 as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_KEYRANGE as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 127.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VELRANGE as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 127.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1e10f32,
            max: 1e10f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_KEYNUM as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 127.0f32,
            def: -1.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_VELOCITY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 127.0f32,
            def: -1.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_ATTENUATION as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1440.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_RESERVED2 as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -1e10f32,
            max: 1e10f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_COARSETUNE as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -120.0f32,
            max: 120.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_FINETUNE as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: -99.0f32,
            max: 99.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_SAMPLEID as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_SAMPLEMODE as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_RESERVED3 as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_SCALETUNE as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 1 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 1200.0f32,
            def: 100.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_EXCLUSIVECLASS as libc::c_int as libc::c_char,
            init: 0 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 0.0f32,
            def: 0.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_OVERRIDEROOTKEY as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 127.0f32,
            def: -1.0f32,
        };
        init
    },
    {
        let mut init = _fluid_gen_info_t {
            num: GEN_PITCH as libc::c_int as libc::c_char,
            init: 1 as libc::c_int as libc::c_char,
            nrpn_scale: 0 as libc::c_int as libc::c_char,
            min: 0.0f32,
            max: 127.0f32,
            def: 0.0f32,
        };
        init
    },
];

#[no_mangle]
pub unsafe extern "C" fn fluid_gen_set_default_values(mut gen: *mut fluid_gen_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < GEN_LAST as libc::c_int {
        (*gen.offset(i as isize)).flags = GEN_UNUSED as libc::c_int as libc::c_uchar;
        (*gen.offset(i as isize)).mod_0 = 0.0f64;
        (*gen.offset(i as isize)).nrpn = 0.0f64;
        (*gen.offset(i as isize)).val = fluid_gen_info[i as usize].def as libc::c_double;
        i += 1
    }
    return FLUID_OK as libc::c_int;
}

#[no_mangle]
pub unsafe extern "C" fn fluid_gen_init(
    mut gen: *mut fluid_gen_t,
    mut channel: *mut fluid_channel_t,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    fluid_gen_set_default_values(gen);
    i = 0 as libc::c_int;
    while i < GEN_LAST as libc::c_int {
        (*gen.offset(i as isize)).nrpn = (*channel).gen[i as usize] as libc::c_double;
        if (*channel).gen_abs[i as usize] != 0 {
            (*gen.offset(i as isize)).flags = GEN_ABS_NRPN as libc::c_int as libc::c_uchar
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_gen_scale(
    mut gen: libc::c_int,
    mut value: libc::c_float,
) -> fluid_real_t {
    return fluid_gen_info[gen as usize].min
        + value * (fluid_gen_info[gen as usize].max - fluid_gen_info[gen as usize].min);
}

#[no_mangle]
pub unsafe extern "C" fn fluid_gen_scale_nrpn(
    mut gen: libc::c_int,
    mut data: libc::c_int,
) -> fluid_real_t {
    let mut value: fluid_real_t = data as libc::c_float - 8192.0f32;
    value = if value < -(8192 as libc::c_int) as libc::c_float {
        -(8192 as libc::c_int) as libc::c_float
    } else if value > 8192 as libc::c_int as libc::c_float {
        8192 as libc::c_int as libc::c_float
    } else {
        value
    };
    return value * fluid_gen_info[gen as usize].nrpn_scale as libc::c_float;
}
