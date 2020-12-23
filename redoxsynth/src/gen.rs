use crate::channel::Channel;
pub type GenType = libc::c_uint;
pub const GEN_LAST: GenType = 60;
pub const GEN_PITCH: GenType = 59;
pub const GEN_OVERRIDEROOTKEY: GenType = 58;
pub const GEN_EXCLUSIVECLASS: GenType = 57;
pub const GEN_SCALETUNE: GenType = 56;
pub const GEN_RESERVED3: GenType = 55;
pub const GEN_SAMPLEMODE: GenType = 54;
pub const GEN_SAMPLEID: GenType = 53;
pub const GEN_FINETUNE: GenType = 52;
pub const GEN_COARSETUNE: GenType = 51;
pub const GEN_ENDLOOPADDRCOARSEOFS: GenType = 50;
pub const GEN_RESERVED2: GenType = 49;
pub const GEN_ATTENUATION: GenType = 48;
pub const GEN_VELOCITY: GenType = 47;
pub const GEN_KEYNUM: GenType = 46;
pub const GEN_STARTLOOPADDRCOARSEOFS: GenType = 45;
pub const GEN_VELRANGE: GenType = 44;
pub const GEN_KEYRANGE: GenType = 43;
pub const GEN_RESERVED1: GenType = 42;
pub const GEN_INSTRUMENT: GenType = 41;
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
pub const GEN_UNUSED4: GenType = 20;
pub const GEN_UNUSED3: GenType = 19;
pub const GEN_UNUSED2: GenType = 18;
pub const GEN_PAN: GenType = 17;
pub const GEN_REVERBSEND: GenType = 16;
pub const GEN_CHORUSSEND: GenType = 15;
pub const GEN_UNUSED1: GenType = 14;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_gen_t {
    pub flags: libc::c_uchar,
    pub val: f64,
    pub mod_0: f64,
    pub nrpn: f64,
}
pub type GenFlags = libc::c_uint;
pub const GEN_ABS_NRPN: GenFlags = 2;
pub const GEN_UNUSED: GenFlags = 0;
pub const FLUID_OK: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_gen_info_t {
    pub num: libc::c_char,
    pub init: libc::c_char,
    pub nrpn_scale: libc::c_char,
    pub min: libc::c_float,
    pub max: libc::c_float,
    pub def: libc::c_float,
}
pub type C2RustUnnamed = libc::c_int;
#[no_mangle]
pub static mut fluid_gen_info: [fluid_gen_info_t; 60] = [
    fluid_gen_info_t {
        num: GEN_STARTADDROFS as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_ENDADDROFS as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1e10f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_STARTLOOPADDROFS as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_ENDLOOPADDROFS as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_STARTADDRCOARSEOFS as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODLFOTOPITCH as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VIBLFOTOPITCH as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVTOPITCH as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_FILTERFC as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: 1500.0f32,
        max: 13500.0f32,
        def: 13500.0f32,
    },
    fluid_gen_info_t {
        num: GEN_FILTERQ as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 960.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODLFOTOFILTERFC as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVTOFILTERFC as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_ENDADDRCOARSEOFS as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1e10f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODLFOTOVOL as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -960.0f32,
        max: 960.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_UNUSED1 as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_CHORUSSEND as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_REVERBSEND as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_PAN as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -500.0f32,
        max: 500.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_UNUSED2 as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_UNUSED3 as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_UNUSED4 as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODLFODELAY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODLFOFREQ as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 4 as libc::c_int as libc::c_char,
        min: -16000.0f32,
        max: 4500.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VIBLFODELAY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VIBLFOFREQ as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 4 as libc::c_int as libc::c_char,
        min: -16000.0f32,
        max: 4500.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVDELAY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVATTACK as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVHOLD as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVDECAY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVSUSTAIN as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1000.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_MODENVRELEASE as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_KEYTOMODENVHOLD as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_KEYTOMODENVDECAY as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VOLENVDELAY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VOLENVATTACK as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VOLENVHOLD as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VOLENVDECAY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VOLENVSUSTAIN as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1440.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VOLENVRELEASE as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 2 as libc::c_int as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    fluid_gen_info_t {
        num: GEN_KEYTOVOLENVHOLD as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_KEYTOVOLENVDECAY as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_INSTRUMENT as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_RESERVED1 as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_KEYRANGE as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VELRANGE as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_STARTLOOPADDRCOARSEOFS as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_KEYNUM as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: -1.0f32,
    },
    fluid_gen_info_t {
        num: GEN_VELOCITY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: -1.0f32,
    },
    fluid_gen_info_t {
        num: GEN_ATTENUATION as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1440.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_RESERVED2 as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_ENDLOOPADDRCOARSEOFS as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_COARSETUNE as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -120.0f32,
        max: 120.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_FINETUNE as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: -99.0f32,
        max: 99.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_SAMPLEID as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_SAMPLEMODE as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_RESERVED3 as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_SCALETUNE as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 1 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 1200.0f32,
        def: 100.0f32,
    },
    fluid_gen_info_t {
        num: GEN_EXCLUSIVECLASS as libc::c_int as libc::c_char,
        init: 0 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    fluid_gen_info_t {
        num: GEN_OVERRIDEROOTKEY as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: -1.0f32,
    },
    fluid_gen_info_t {
        num: GEN_PITCH as libc::c_int as libc::c_char,
        init: 1 as libc::c_int as libc::c_char,
        nrpn_scale: 0 as libc::c_int as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: 0.0f32,
    },
];
#[no_mangle]
pub unsafe extern "C" fn fluid_gen_set_default_values(gen: *mut fluid_gen_t) -> libc::c_int {
    let mut i: libc::c_int;
    i = 0 as libc::c_int;
    while i < GEN_LAST as libc::c_int {
        (*gen.offset(i as isize)).flags = GEN_UNUSED as libc::c_int as libc::c_uchar;
        (*gen.offset(i as isize)).mod_0 = 0.0f64;
        (*gen.offset(i as isize)).nrpn = 0.0f64;
        (*gen.offset(i as isize)).val = fluid_gen_info[i as usize].def as f64;
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_gen_init(
    gen: *mut fluid_gen_t,
    channel: *mut Channel,
) -> libc::c_int {
    let mut i: libc::c_int;
    fluid_gen_set_default_values(gen);
    i = 0 as libc::c_int;
    while i < GEN_LAST as libc::c_int {
        (*gen.offset(i as isize)).nrpn = (*channel).gen[i as usize] as f64;
        if (*channel).gen_abs[i as usize] != 0 {
            (*gen.offset(i as isize)).flags = GEN_ABS_NRPN as libc::c_int as libc::c_uchar
        }
        i += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_gen_scale(
    gen: libc::c_int,
    value: libc::c_float,
) -> f32 {
    return fluid_gen_info[gen as usize].min
        + value * (fluid_gen_info[gen as usize].max - fluid_gen_info[gen as usize].min);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_gen_scale_nrpn(
    gen: libc::c_int,
    data: libc::c_int,
) -> f32 {
    let mut value: f32 = data as libc::c_float - 8192.0f32;
    value = if value < -(8192 as libc::c_int) as libc::c_float {
        -(8192 as libc::c_int) as libc::c_float
    } else if value > 8192 as libc::c_int as libc::c_float {
        8192 as libc::c_int as libc::c_float
    } else {
        value
    };
    return value * fluid_gen_info[gen as usize].nrpn_scale as libc::c_float;
}
