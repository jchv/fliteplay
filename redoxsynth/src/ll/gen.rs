use super::channel::Channel;
pub type GenType = u32;
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
pub type GenFlags = u32;
pub const GEN_ABS_NRPN: GenFlags = 2;
pub const GEN_UNUSED: GenFlags = 0;
pub const FLUID_OK: C2RustUnnamed = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GenInfo {
    pub num: libc::c_char,
    pub init: libc::c_char,
    pub nrpn_scale: libc::c_char,
    pub min: f32,
    pub max: f32,
    pub def: f32,
}
pub type C2RustUnnamed = i32;

pub static mut GEN_INFO: [GenInfo; 60] = [
    GenInfo {
        num: GEN_STARTADDROFS as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_ENDADDROFS as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1e10f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_STARTLOOPADDROFS as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_ENDLOOPADDROFS as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_STARTADDRCOARSEOFS as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODLFOTOPITCH as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_VIBLFOTOPITCH as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODENVTOPITCH as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_FILTERFC as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: 1500.0f32,
        max: 13500.0f32,
        def: 13500.0f32,
    },
    GenInfo {
        num: GEN_FILTERQ as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 960.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODLFOTOFILTERFC as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODENVTOFILTERFC as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 12000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_ENDADDRCOARSEOFS as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1e10f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODLFOTOVOL as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -960.0f32,
        max: 960.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_UNUSED1 as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_CHORUSSEND as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_REVERBSEND as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_PAN as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -500.0f32,
        max: 500.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_UNUSED2 as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_UNUSED3 as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_UNUSED4 as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODLFODELAY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_MODLFOFREQ as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 4 as i32 as libc::c_char,
        min: -16000.0f32,
        max: 4500.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_VIBLFODELAY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_VIBLFOFREQ as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 4 as i32 as libc::c_char,
        min: -16000.0f32,
        max: 4500.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODENVDELAY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_MODENVATTACK as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_MODENVHOLD as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_MODENVDECAY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_MODENVSUSTAIN as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1000.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_MODENVRELEASE as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_KEYTOMODENVHOLD as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_KEYTOMODENVDECAY as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_VOLENVDELAY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_VOLENVATTACK as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_VOLENVHOLD as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 5000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_VOLENVDECAY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_VOLENVSUSTAIN as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1440.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_VOLENVRELEASE as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 2 as i32 as libc::c_char,
        min: -12000.0f32,
        max: 8000.0f32,
        def: -12000.0f32,
    },
    GenInfo {
        num: GEN_KEYTOVOLENVHOLD as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_KEYTOVOLENVDECAY as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1200.0f32,
        max: 1200.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_INSTRUMENT as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_RESERVED1 as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_KEYRANGE as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_VELRANGE as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_STARTLOOPADDRCOARSEOFS as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_KEYNUM as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: -1.0f32,
    },
    GenInfo {
        num: GEN_VELOCITY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: -1.0f32,
    },
    GenInfo {
        num: GEN_ATTENUATION as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1440.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_RESERVED2 as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_ENDLOOPADDRCOARSEOFS as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -1e10f32,
        max: 1e10f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_COARSETUNE as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -120.0f32,
        max: 120.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_FINETUNE as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: -99.0f32,
        max: 99.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_SAMPLEID as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_SAMPLEMODE as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_RESERVED3 as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_SCALETUNE as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 1 as i32 as libc::c_char,
        min: 0.0f32,
        max: 1200.0f32,
        def: 100.0f32,
    },
    GenInfo {
        num: GEN_EXCLUSIVECLASS as i32 as libc::c_char,
        init: 0 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 0.0f32,
        def: 0.0f32,
    },
    GenInfo {
        num: GEN_OVERRIDEROOTKEY as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: -1.0f32,
    },
    GenInfo {
        num: GEN_PITCH as i32 as libc::c_char,
        init: 1 as i32 as libc::c_char,
        nrpn_scale: 0 as i32 as libc::c_char,
        min: 0.0f32,
        max: 127.0f32,
        def: 0.0f32,
    },
];

pub unsafe fn fluid_gen_set_default_values(gen: *mut fluid_gen_t) -> i32 {
    let mut i: i32;
    i = 0 as i32;
    while i < GEN_LAST as i32 {
        (*gen.offset(i as isize)).flags = GEN_UNUSED as i32 as libc::c_uchar;
        (*gen.offset(i as isize)).mod_0 = 0.0f64;
        (*gen.offset(i as isize)).nrpn = 0.0f64;
        (*gen.offset(i as isize)).val = GEN_INFO[i as usize].def as f64;
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_gen_init(gen: *mut fluid_gen_t, channel: *mut Channel) -> i32 {
    let mut i: i32;
    fluid_gen_set_default_values(gen);
    i = 0 as i32;
    while i < GEN_LAST as i32 {
        (*gen.offset(i as isize)).nrpn = (*channel).gen[i as usize] as f64;
        if (*channel).gen_abs[i as usize] != 0 {
            (*gen.offset(i as isize)).flags = GEN_ABS_NRPN as i32 as libc::c_uchar
        }
        i += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_gen_scale_nrpn(gen: i32, data: i32) -> f32 {
    let mut value: f32 = data as f32 - 8192.0f32;
    value = if value < -(8192 as i32) as f32 {
        -(8192 as i32) as f32
    } else if value > 8192 as i32 as f32 {
        8192 as i32 as f32
    } else {
        value
    };
    return value * GEN_INFO[gen as usize].nrpn_scale as f32;
}
