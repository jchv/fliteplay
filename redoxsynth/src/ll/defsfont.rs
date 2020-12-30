use super::gen::fluid_gen_set_default_values;
use super::gen::Gen;
use super::modulator::fluid_mod_delete;
use super::modulator::fluid_mod_new;
use super::modulator::fluid_mod_test_identity;
use super::modulator::Mod;
use super::sfont::FileApi;
use super::sfont::Preset;
use super::sfont::Sample;
use super::sfont::SoundFont;
use super::sfont::SoundFontLoader;
use super::synth::Synth;
use super::voice::fluid_voice_add_mod;
use super::voice::fluid_voice_gen_incr;
use super::voice::fluid_voice_gen_set;
use super::voice::fluid_voice_optimize_sample;
use super::voice::FluidVoiceAddMod;
use super::voice::Voice;
use std::{
    cmp::Ordering,
    ffi::{CStr, CString},
};
pub const FLUID_OK: i32 = 0;
pub const FLUID_FAILED: i32 = -1;
#[derive(Clone)]
#[repr(C)]
pub struct DefSFont {
    filename: Vec<u8>,
    samplepos: u32,
    samplesize: u32,
    sampledata: *mut i16,
    sample: Vec<*mut Sample>,
    preset: *mut DefPreset,
    iter_cur: *mut DefPreset,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct DefPreset {
    next: *mut DefPreset,
    sfont: *mut DefSFont,
    name: [u8; 21],
    bank: u32,
    num: u32,
    global_zone: *mut PresetZone,
    zone: *mut PresetZone,
}
#[derive(Clone)]
#[repr(C)]
pub struct PresetZone {
    next: *mut PresetZone,
    name: Vec<u8>,
    inst: *mut Instrument,
    keylo: i32,
    keyhi: i32,
    vello: i32,
    velhi: i32,
    gen: [Gen; 60],
    mod_0: *mut Mod,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Instrument {
    name: [u8; 21],
    global_zone: *mut InstrumentZone,
    zone: *mut InstrumentZone,
}
#[derive(Clone)]
#[repr(C)]
pub struct InstrumentZone {
    next: *mut InstrumentZone,
    name: Vec<u8>,
    sample: *mut Sample,
    keylo: i32,
    keyhi: i32,
    vello: i32,
    velhi: i32,
    gen: [Gen; 60],
    mod_0: *mut Mod,
}
#[derive(Clone)]
#[repr(C)]
pub struct SFData {
    version: SFVersion,
    romver: SFVersion,
    samplepos: u32,
    samplesize: u32,
    fname: Vec<u8>,
    sffd: *mut libc::FILE,
    info: Vec<Vec<u8>>,
    preset: Vec<*mut SFPreset>,
    inst: Vec<*mut SFInst>,
    sample: Vec<*mut SFSample>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFVersion {
    major: u16,
    minor: u16,
}
#[derive(Clone)]
#[repr(C)]
pub struct SFInst {
    name: [u8; 21],
    zone: Vec<*mut SFZone>,
}

#[derive(Copy, Clone)]
enum InstSamp {
    Inst(*mut SFInst),
    Sample(*mut SFSample),
    Int(i32),
    None,
}

impl InstSamp {
    pub fn is_none(&self) -> bool {
        match self {
            InstSamp::None => true,
            _ => false,
        }
    }

    pub fn unwrap_sample(&self) -> *mut SFSample {
        match self {
            InstSamp::Sample(ptr) => *ptr,
            _ => panic!("mismatch"),
        }
    }

    pub fn unwrap_inst(&self) -> *mut SFInst {
        match self {
            InstSamp::Inst(ptr) => *ptr,
            _ => panic!("mismatch"),
        }
    }

    pub fn unwrap_int(&self) -> i32 {
        match self {
            InstSamp::Int(val) => *val,
            _ => panic!("mismatch"),
        }
    }
}

#[derive(Clone)]
#[repr(C)]
pub struct SFZone {
    instsamp: InstSamp,
    gen: Vec<*mut SFGen>,
    mod_0: Vec<*mut SFMod>,
}
#[derive(Clone)]
#[repr(C)]
pub struct SFPreset {
    name: [u8; 21],
    prenum: u16,
    bank: u16,
    libr: u32,
    genre: u32,
    morph: u32,
    zone: Vec<*mut SFZone>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFMod {
    src: u16,
    dest: u16,
    amount: i16,
    amtsrc: u16,
    trans: u16,
}
pub const FLUID_MOD_SWITCH: ModFlags = 12;
pub const FLUID_MOD_CONVEX: ModFlags = 8;
pub const FLUID_MOD_CONCAVE: ModFlags = 4;
pub const FLUID_MOD_LINEAR: ModFlags = 0;
pub const FLUID_MOD_UNIPOLAR: ModFlags = 0;
pub const FLUID_MOD_BIPOLAR: ModFlags = 2;
pub const FLUID_MOD_POSITIVE: ModFlags = 0;
pub const FLUID_MOD_NEGATIVE: ModFlags = 1;
pub const FLUID_MOD_GC: ModFlags = 0;
pub const FLUID_MOD_CC: ModFlags = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFSample {
    name: [u8; 21],
    samfile: u8,
    start: u32,
    end: u32,
    loopstart: u32,
    loopend: u32,
    samplerate: u32,
    origpitch: u8,
    pitchadj: libc::c_schar,
    sampletype: u16,
}
pub const GEN_SET: GenFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFGen {
    id: u16,
    amount: SFGenAmount,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SFGenAmount {
    sword: i16,
    uword: u16,
    range: SFGenAmountRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFGenAmountRange {
    lo: u8,
    hi: u8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFChunk {
    id: u32,
    size: u32,
}
pub const SHDR_ID: u32 = 28;
pub const UNKN_ID: u32 = 0;
pub const GEN_RESERVED3: u32 = 55;
pub const GEN_RESERVED2: u32 = 49;
pub const GEN_RESERVED1: u32 = 42;
pub const GEN_UNUSED4: u32 = 20;
pub const GEN_UNUSED3: u32 = 19;
pub const GEN_UNUSED2: u32 = 18;
pub const GEN_UNUSED1: u32 = 14;
pub const GEN_DUMMY: u32 = 59;
pub const GEN_SAMPLE_ID: u32 = 53;
pub const GEN_VEL_RANGE: u32 = 44;
pub const GEN_KEY_RANGE: u32 = 43;
pub const IGEN_ID: u32 = 27;
pub const IMOD_ID: u32 = 26;
pub const IBAG_ID: u32 = 25;
pub const IHDR_ID: u32 = 24;
pub const GEN_OVERRIDE_ROOT_KEY: u32 = 58;
pub const GEN_EXCLUSIVE_CLASS: u32 = 57;
pub const GEN_SAMPLE_MODES: u32 = 54;
pub const GEN_END_LOOP_ADDR_COARSE_OFS: u32 = 50;
pub const GEN_VELOCITY: u32 = 47;
pub const GEN_KEYNUM: u32 = 46;
pub const GEN_START_LOOP_ADDR_COARSE_OFS: u32 = 45;
pub const GEN_END_ADDR_COARSE_OFS: u32 = 12;
pub const GEN_START_ADDR_COARSE_OFS: u32 = 4;
pub const GEN_END_LOOP_ADDR_OFS: u32 = 3;
pub const GEN_START_LOOP_ADDR_OFS: u32 = 2;
pub const GEN_END_ADDR_OFS: u32 = 1;
pub const GEN_START_ADDR_OFS: u32 = 0;
pub const GEN_INSTRUMENT: u32 = 41;
pub const PGEN_ID: u32 = 23;
pub const PMOD_ID: u32 = 22;
pub const PBAG_ID: u32 = 21;
pub const PHDR_ID: u32 = 20;
pub const PDTA_ID: u32 = 6;
pub const LIST_ID: u32 = 2;
pub const SMPL_ID: u32 = 19;
pub const SDTA_ID: u32 = 5;
pub const ICMT_ID: u32 = 16;
pub const IVER_ID: u32 = 11;
pub const IFIL_ID: u32 = 7;
pub const INFO_ID: u32 = 4;
pub const SFBK_ID: u32 = 3;
pub const RIFF_ID: u32 = 1;
pub const FLUID_VOICE_ADD: FluidVoiceAddMod = 1;
pub const GEN_OVERRIDEROOTKEY: GenType = 58;
pub const GEN_EXCLUSIVECLASS: GenType = 57;
pub const GEN_SAMPLEMODE: GenType = 54;
pub const GEN_ENDLOOPADDRCOARSEOFS: GenType = 50;
pub const GEN_STARTLOOPADDRCOARSEOFS: GenType = 45;
pub const GEN_ENDADDRCOARSEOFS: GenType = 12;
pub const GEN_STARTADDRCOARSEOFS: GenType = 4;
pub const GEN_ENDLOOPADDROFS: GenType = 3;
pub const GEN_STARTLOOPADDROFS: GenType = 2;
pub const GEN_ENDADDROFS: GenType = 1;
pub const GEN_STARTADDROFS: GenType = 0;
pub const GEN_LAST: GenType = 60;
pub const FLUID_VOICE_OVERWRITE: FluidVoiceAddMod = 0;
pub type ModFlags = u32;
pub type GenType = u32;
pub type GenFlags = u32;
unsafe fn default_fopen(_fileapi: *mut FileApi, path: &[u8]) -> *mut libc::c_void {
    return libc::fopen(path.as_ptr() as _, b"rb\x00" as *const u8 as *const i8)
        as *mut libc::c_void;
}
unsafe fn default_fclose(handle: *mut libc::c_void) -> i32 {
    return libc::fclose(handle as *mut libc::FILE);
}
unsafe fn default_ftell(handle: *mut libc::c_void) -> isize {
    return libc::ftell(handle as *mut libc::FILE) as _;
}
unsafe fn safe_fread(buf: *mut libc::c_void, count: i32, handle: *mut libc::c_void) -> i32 {
    if libc::fread(buf, count as libc::size_t, 1, handle as *mut libc::FILE) != 1 as libc::size_t {
        if libc::feof(handle as *mut libc::FILE) != 0 {
            gerr!(ErrEof as i32, "EOF while attemping to read {} bytes", count);
        } else {
            fluid_log!(FLUID_ERR, "File read failed",);
        }
        return FLUID_FAILED as i32;
    }
    return FLUID_OK as i32;
}
unsafe fn safe_fseek(handle: *mut libc::c_void, ofs: isize, whence: i32) -> i32 {
    if libc::fseek(handle as *mut libc::FILE, ofs as _, whence) != 0 as i32 {
        fluid_log!(
            FLUID_ERR,
            "File seek failed with offset = {} and whence = {}",
            ofs,
            whence
        );
        return FLUID_FAILED as i32;
    }
    return FLUID_OK as i32;
}
static mut DEFAULT_FILEAPI: FileApi = {
    FileApi {
        data: 0 as *const libc::c_void as *mut libc::c_void,
        free: None,
        fopen: Some(default_fopen as unsafe fn(_: *mut FileApi, _: &[u8]) -> *mut libc::c_void),
        fread: Some(
            safe_fread as unsafe fn(_: *mut libc::c_void, _: i32, _: *mut libc::c_void) -> i32,
        ),
        fseek: Some(safe_fseek as unsafe fn(_: *mut libc::c_void, _: isize, _: i32) -> i32),
        fclose: Some(default_fclose as unsafe fn(_: *mut libc::c_void) -> i32),
        ftell: Some(default_ftell as unsafe fn(_: *mut libc::c_void) -> isize),
    }
};
static mut FLUID_DEFAULT_FILEAPI: *mut FileApi =
    unsafe { &DEFAULT_FILEAPI as *const FileApi as *mut FileApi };

pub unsafe fn fluid_set_default_fileapi(fileapi: *mut FileApi) {
    if !FLUID_DEFAULT_FILEAPI.is_null() && (*FLUID_DEFAULT_FILEAPI).free.is_some() {
        Some(
            (*FLUID_DEFAULT_FILEAPI)
                .free
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(FLUID_DEFAULT_FILEAPI);
    }
    FLUID_DEFAULT_FILEAPI = if fileapi.is_null() {
        &DEFAULT_FILEAPI as *const FileApi as *mut FileApi
    } else {
        fileapi
    };
}

pub fn new_fluid_defsfloader() -> *mut SoundFontLoader {
    unsafe {
        let mut loader: *mut SoundFontLoader;
        loader = libc::malloc(::std::mem::size_of::<SoundFontLoader>() as libc::size_t)
            as *mut SoundFontLoader;
        if loader.is_null() {
            fluid_log!(FLUID_ERR, "Out of memory",);
            return 0 as *mut SoundFontLoader;
        }
        (*loader).data = 0 as *mut libc::c_void;
        (*loader).fileapi = FLUID_DEFAULT_FILEAPI;
        (*loader).free = Some(delete_fluid_defsfloader as _);
        (*loader).load = Some(fluid_defsfloader_load as _);
        return loader;
    }
}

pub unsafe fn delete_fluid_defsfloader(loader: *mut SoundFontLoader) -> i32 {
    if !loader.is_null() {
        libc::free(loader as *mut libc::c_void);
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defsfloader_load(
    loader: *mut SoundFontLoader,
    filename: &[u8],
) -> Option<SoundFont> {
    let defsfont = new_fluid_defsfont();
    let mut sfont = SoundFont {
        data: Box::new(defsfont),
        id: 0 as _,
        free: Some(fluid_defsfont_sfont_delete as _),
        get_name: Some(fluid_defsfont_sfont_get_name as _),
        get_preset: Some(fluid_defsfont_sfont_get_preset as _),
        iteration_start: Some(fluid_defsfont_sfont_iteration_start as _),
        iteration_next: Some(fluid_defsfont_sfont_iteration_next as _),
    };
    if fluid_defsfont_load(
        sfont.data.downcast_mut::<DefSFont>().unwrap(),
        filename,
        (*loader).fileapi,
    ) == FLUID_FAILED as i32
    {
        delete_fluid_defsfont(sfont.data.downcast_mut::<DefSFont>().unwrap());
        return None;
    }
    return Some(sfont);
}

pub unsafe fn fluid_defsfont_sfont_delete(sfont: *mut SoundFont) -> i32 {
    match (*sfont).data.downcast_mut::<DefSFont>() {
        Some(defsfont) => {
            if delete_fluid_defsfont(defsfont) != 0 as i32 {
                return -(1 as i32);
            }
            libc::free(sfont as *mut libc::c_void);
        }
        None => {}
    }
    return 0;
}

pub unsafe fn fluid_defsfont_sfont_get_name(sfont: *const SoundFont) -> Option<Vec<u8>> {
    return (*sfont)
        .data
        .downcast_ref::<DefSFont>()
        .map(|defsfont| fluid_defsfont_get_name(defsfont));
}

pub unsafe fn fluid_defsfont_sfont_get_preset(
    sfont: *const SoundFont,
    bank: u32,
    prenum: u32,
) -> *mut Preset {
    match (*sfont).data.downcast_ref::<DefSFont>() {
        Some(defsfont) => {
            let mut preset: *mut Preset;
            let defpreset: *mut DefPreset;
            defpreset = fluid_defsfont_get_preset(defsfont, bank, prenum);
            if defpreset.is_null() {
                return 0 as *mut Preset;
            }
            preset = libc::malloc(::std::mem::size_of::<Preset>() as libc::size_t) as *mut Preset;
            if preset.is_null() {
                fluid_log!(FLUID_ERR, "Out of memory",);
                return 0 as *mut Preset;
            }
            (*preset).sfont = sfont;
            (*preset).data = defpreset as *mut libc::c_void;
            (*preset).free =
                Some(fluid_defpreset_preset_delete as unsafe fn(_: *mut Preset) -> i32);
            (*preset).get_name =
                Some(fluid_defpreset_preset_get_name as unsafe fn(_: *const Preset) -> Vec<u8>);
            (*preset).get_banknum =
                Some(fluid_defpreset_preset_get_banknum as unsafe fn(_: *const Preset) -> i32);
            (*preset).get_num =
                Some(fluid_defpreset_preset_get_num as unsafe fn(_: *const Preset) -> i32);
            (*preset).noteon = Some(
                fluid_defpreset_preset_noteon
                    as unsafe fn(_: *mut Preset, _: &mut Synth, _: i32, _: i32, _: i32) -> i32,
            );
            (*preset).notify = None;
            return preset;
        }
        None => {
            return 0 as _;
        }
    }
}

pub unsafe fn fluid_defsfont_sfont_iteration_start(sfont: *mut SoundFont) {
    match (*sfont).data.downcast_mut::<DefSFont>() {
        Some(defsfont) => {
            fluid_defsfont_iteration_start(defsfont);
        }
        None => {}
    }
}

pub unsafe fn fluid_defsfont_sfont_iteration_next(
    sfont: *mut SoundFont,
    mut preset: *mut Preset,
) -> i32 {
    (*preset).free = Some(fluid_defpreset_preset_delete as unsafe fn(_: *mut Preset) -> i32);
    (*preset).get_name =
        Some(fluid_defpreset_preset_get_name as unsafe fn(_: *const Preset) -> Vec<u8>);
    (*preset).get_banknum =
        Some(fluid_defpreset_preset_get_banknum as unsafe fn(_: *const Preset) -> i32);
    (*preset).get_num = Some(fluid_defpreset_preset_get_num as unsafe fn(_: *const Preset) -> i32);
    (*preset).noteon = Some(
        fluid_defpreset_preset_noteon
            as unsafe fn(_: *mut Preset, _: &mut Synth, _: i32, _: i32, _: i32) -> i32,
    );
    (*preset).notify = None;
    match (*sfont).data.downcast_mut::<DefSFont>() {
        Some(defsfont) => {
            return fluid_defsfont_iteration_next(defsfont, preset);
        }
        None => {
            return 0;
        }
    }
}

pub unsafe fn fluid_defpreset_preset_delete(preset: *mut Preset) -> i32 {
    libc::free(preset as *mut libc::c_void);
    return 0 as i32;
}

pub unsafe fn fluid_defpreset_preset_get_name(preset: *const Preset) -> Vec<u8> {
    return fluid_defpreset_get_name((*preset).data as *mut DefPreset);
}

pub unsafe fn fluid_defpreset_preset_get_banknum(preset: *const Preset) -> i32 {
    return fluid_defpreset_get_banknum((*preset).data as *mut DefPreset);
}

pub unsafe fn fluid_defpreset_preset_get_num(preset: *const Preset) -> i32 {
    return fluid_defpreset_get_num((*preset).data as *mut DefPreset);
}

pub unsafe fn fluid_defpreset_preset_noteon(
    preset: *mut Preset,
    synth: &mut Synth,
    chan: i32,
    key: i32,
    vel: i32,
) -> i32 {
    return fluid_defpreset_noteon((*preset).data as *mut DefPreset, synth, chan, key, vel);
}

pub unsafe fn new_fluid_defsfont() -> DefSFont {
    return DefSFont {
        filename: Vec::new(),
        samplepos: 0 as _,
        samplesize: 0 as _,
        sample: Vec::new(),
        sampledata: 0 as _,
        preset: 0 as _,
        iter_cur: 0 as _,
    };
}

pub unsafe fn delete_fluid_defsfont(mut sfont: *mut DefSFont) -> i32 {
    let mut preset: *mut DefPreset;
    for sample in (*sfont).sample.iter() {
        if (**sample).refcount != 0 as i32 as u32 {
            return -(1 as i32);
        }
    }
    for sample in (*sfont).sample.iter() {
        delete_fluid_sample(*sample);
    }
    if !(*sfont).sampledata.is_null() {
        libc::free((*sfont).sampledata as *mut libc::c_void);
    }
    preset = (*sfont).preset;
    while !preset.is_null() {
        (*sfont).preset = (*preset).next;
        delete_fluid_defpreset(preset);
        preset = (*sfont).preset
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defsfont_get_name(sfont: *const DefSFont) -> Vec<u8> {
    return (*sfont).filename.to_vec();
}

pub static mut PRESET_CALLBACK: Option<unsafe fn(_: u32, _: u32, _: &[u8]) -> ()> = None;

pub unsafe fn fluid_defsfont_load(
    mut sfont: *mut DefSFont,
    file: &[u8],
    fapi: *mut FileApi,
) -> i32 {
    let sfdata: *mut SFData;
    let mut sample: *mut Sample;
    let mut preset: *mut DefPreset;
    (*sfont).filename = file.to_vec();
    sfdata = sfload_file(file, fapi);
    if sfdata.is_null() {
        fluid_log!(FLUID_ERR, "Couldn't load soundfont file",);
        return FLUID_FAILED;
    }
    (*sfont).samplepos = (*sfdata).samplepos;
    (*sfont).samplesize = (*sfdata).samplesize;
    if fluid_defsfont_load_sampledata(sfont, fapi) != FLUID_OK {
        sfont_close(sfdata, fapi);
        return FLUID_FAILED;
    }
    for sfsample in (*sfdata).sample.iter() {
        sample = new_fluid_sample();
        if sample.is_null() {
            sfont_close(sfdata, fapi);
            return FLUID_FAILED;
        }
        if fluid_sample_import_sfont(sample, *sfsample, sfont) != FLUID_OK {
            sfont_close(sfdata, fapi);
            return FLUID_FAILED;
        }
        fluid_defsfont_add_sample(sfont, sample);
        fluid_voice_optimize_sample(sample);
    }
    for sfpreset in (*sfdata).preset.iter() {
        preset = new_fluid_defpreset(sfont);
        if preset.is_null() {
            sfont_close(sfdata, fapi);
            return FLUID_FAILED;
        }
        if fluid_defpreset_import_sfont(preset, *sfpreset, sfont) != FLUID_OK {
            sfont_close(sfdata, fapi);
            return FLUID_FAILED;
        }
        fluid_defsfont_add_preset(sfont, preset);
        if PRESET_CALLBACK.is_some() {
            PRESET_CALLBACK.expect("non-null function pointer")(
                (*preset).bank,
                (*preset).num,
                &(*preset).name,
            );
        }
    }
    sfont_close(sfdata, fapi);
    return FLUID_OK;
}

pub unsafe fn fluid_defsfont_add_sample(sfont: *mut DefSFont, sample: *mut Sample) -> i32 {
    (*sfont).sample.push(sample);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defsfont_add_preset(
    mut sfont: *mut DefSFont,
    mut preset: *mut DefPreset,
) -> i32 {
    let mut cur: *mut DefPreset;
    let mut prev: *mut DefPreset;
    if (*sfont).preset.is_null() {
        (*preset).next = 0 as *mut DefPreset;
        (*sfont).preset = preset
    } else {
        cur = (*sfont).preset;
        prev = 0 as *mut DefPreset;
        while !cur.is_null() {
            if (*preset).bank < (*cur).bank
                || (*preset).bank == (*cur).bank && (*preset).num < (*cur).num
            {
                if prev.is_null() {
                    (*preset).next = cur;
                    (*sfont).preset = preset
                } else {
                    (*preset).next = cur;
                    (*prev).next = preset
                }
                return FLUID_OK as i32;
            }
            prev = cur;
            cur = (*cur).next
        }
        (*preset).next = 0 as *mut DefPreset;
        (*prev).next = preset
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defsfont_load_sampledata(mut sfont: *mut DefSFont, fapi: *mut FileApi) -> i32 {
    let fd: *mut libc::FILE;
    let mut endian: u16;
    fd = (*fapi).fopen.expect("non-null function pointer")(fapi, &(*sfont).filename)
        as *mut libc::FILE;
    if fd.is_null() {
        fluid_log!(FLUID_ERR, "Can't open soundfont file",);
        return FLUID_FAILED as i32;
    }
    if (*fapi).fseek.expect("non-null function pointer")(
        fd as *mut libc::c_void,
        (*sfont).samplepos as isize,
        0 as i32,
    ) == FLUID_FAILED as i32
    {
        libc::perror(b"error\x00" as *const u8 as *const i8);
        fluid_log!(FLUID_ERR, "Failed to seek position in data file",);
        return FLUID_FAILED as i32;
    }
    (*sfont).sampledata = libc::malloc((*sfont).samplesize as libc::size_t) as *mut i16;
    if (*sfont).sampledata.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return FLUID_FAILED as i32;
    }
    if (*fapi).fread.expect("non-null function pointer")(
        (*sfont).sampledata as *mut libc::c_void,
        (*sfont).samplesize as i32,
        fd as *mut libc::c_void,
    ) == FLUID_FAILED as i32
    {
        fluid_log!(FLUID_ERR, "Failed to read sample data",);
        return FLUID_FAILED as i32;
    }
    (*fapi).fclose.expect("non-null function pointer")(fd as *mut libc::c_void);
    endian = 0x100 as i32 as u16;
    if *(&mut endian as *mut u16 as *mut i8).offset(0 as i32 as isize) != 0 {
        let cbuf: *mut u8;
        let mut hi: u8;
        let mut lo: u8;
        let mut i: u32;
        let mut j: u32;
        let mut s: i16;
        cbuf = (*sfont).sampledata as *mut u8;
        i = 0 as i32 as u32;
        j = 0 as i32 as u32;
        while j < (*sfont).samplesize {
            let fresh0 = j;
            j = j.wrapping_add(1);
            lo = *cbuf.offset(fresh0 as isize);
            let fresh1 = j;
            j = j.wrapping_add(1);
            hi = *cbuf.offset(fresh1 as isize);
            s = ((hi as i32) << 8 as i32 | lo as i32) as i16;
            *(*sfont).sampledata.offset(i as isize) = s;
            i = i.wrapping_add(1)
        }
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defsfont_get_sample(sfont: *mut DefSFont, s: &[u8]) -> *mut Sample {
    for sample in (*sfont).sample.iter() {
        if libc::strcmp((**sample).name.as_ptr() as _, s.as_ptr() as _) == 0 as i32 {
            return *sample;
        }
    }
    return 0 as *mut Sample;
}

pub unsafe fn fluid_defsfont_get_preset(
    sfont: *const DefSFont,
    bank: u32,
    num: u32,
) -> *mut DefPreset {
    let mut preset: *mut DefPreset = (*sfont).preset;
    while !preset.is_null() {
        if (*preset).bank == bank && (*preset).num == num {
            return preset;
        }
        preset = (*preset).next
    }
    return 0 as *mut DefPreset;
}

pub unsafe fn fluid_defsfont_iteration_start(mut sfont: *mut DefSFont) {
    (*sfont).iter_cur = (*sfont).preset;
}

pub unsafe fn fluid_defsfont_iteration_next(
    mut sfont: *mut DefSFont,
    mut preset: *mut Preset,
) -> i32 {
    if (*sfont).iter_cur.is_null() {
        return 0 as i32;
    }
    (*preset).data = (*sfont).iter_cur as *mut libc::c_void;
    (*sfont).iter_cur = fluid_defpreset_next((*sfont).iter_cur);
    return 1 as i32;
}

pub unsafe fn new_fluid_defpreset(sfont: *mut DefSFont) -> *mut DefPreset {
    let mut preset: *mut DefPreset =
        libc::malloc(::std::mem::size_of::<DefPreset>() as libc::size_t) as *mut DefPreset;
    if preset.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut DefPreset;
    }
    (*preset).next = 0 as *mut DefPreset;
    (*preset).sfont = sfont;
    (*preset).name = [0; 21];
    (*preset).bank = 0 as i32 as u32;
    (*preset).num = 0 as i32 as u32;
    (*preset).global_zone = 0 as *mut PresetZone;
    (*preset).zone = 0 as *mut PresetZone;
    return preset;
}

pub unsafe fn delete_fluid_defpreset(mut preset: *mut DefPreset) -> i32 {
    let mut err: i32 = FLUID_OK as i32;
    let mut zone: *mut PresetZone;
    if !(*preset).global_zone.is_null() {
        if delete_fluid_preset_zone((*preset).global_zone) != FLUID_OK as i32 {
            err = FLUID_FAILED as i32
        }
        (*preset).global_zone = 0 as *mut PresetZone
    }
    zone = (*preset).zone;
    while !zone.is_null() {
        (*preset).zone = (*zone).next;
        if delete_fluid_preset_zone(zone) != FLUID_OK as i32 {
            err = FLUID_FAILED as i32
        }
        zone = (*preset).zone
    }
    libc::free(preset as *mut libc::c_void);
    return err;
}

pub unsafe fn fluid_defpreset_get_banknum(preset: *mut DefPreset) -> i32 {
    return (*preset).bank as i32;
}

pub unsafe fn fluid_defpreset_get_num(preset: *mut DefPreset) -> i32 {
    return (*preset).num as i32;
}

pub unsafe fn fluid_defpreset_get_name(preset: *mut DefPreset) -> Vec<u8> {
    return (*preset).name.to_vec();
}

pub unsafe fn fluid_defpreset_next(preset: *mut DefPreset) -> *mut DefPreset {
    return (*preset).next;
}

pub unsafe fn fluid_defpreset_noteon(
    preset: *mut DefPreset,
    synth: &mut Synth,
    chan: i32,
    key: i32,
    vel: i32,
) -> i32 {
    let mut preset_zone: *mut PresetZone;
    let global_preset_zone: *mut PresetZone;
    let mut inst: *mut Instrument;
    let mut inst_zone: *mut InstrumentZone;
    let mut global_inst_zone: *mut InstrumentZone;
    let mut sample: *mut Sample;
    let mut voice: *mut Voice;
    let mut mod_0: *mut Mod;
    let mut mod_list: [*mut Mod; 64] = [0 as *mut Mod; 64];
    let mut mod_list_count: i32;
    let mut i: i32;
    global_preset_zone = fluid_defpreset_get_global_zone(preset);
    preset_zone = fluid_defpreset_get_zone(preset);
    while !preset_zone.is_null() {
        if fluid_preset_zone_inside_range(preset_zone, key, vel) != 0 {
            inst = fluid_preset_zone_get_inst(preset_zone);
            global_inst_zone = fluid_inst_get_global_zone(inst);
            inst_zone = fluid_inst_get_zone(inst);
            while !inst_zone.is_null() {
                sample = fluid_inst_zone_get_sample(inst_zone);
                if fluid_sample_in_rom(sample) != 0 || sample.is_null() {
                    inst_zone = fluid_inst_zone_next(inst_zone)
                } else {
                    if fluid_inst_zone_inside_range(inst_zone, key, vel) != 0 && !sample.is_null() {
                        voice = synth.alloc_voice(sample, chan, key, vel);
                        if voice.is_null() {
                            return FLUID_FAILED as i32;
                        }
                        i = 0 as i32;
                        while i < GEN_LAST as i32 {
                            if (*inst_zone).gen[i as usize].flags != 0 {
                                fluid_voice_gen_set(
                                    voice,
                                    i,
                                    (*inst_zone).gen[i as usize].val as f32,
                                );
                            } else if !global_inst_zone.is_null()
                                && (*global_inst_zone).gen[i as usize].flags as i32 != 0
                            {
                                fluid_voice_gen_set(
                                    voice,
                                    i,
                                    (*global_inst_zone).gen[i as usize].val as f32,
                                );
                            }
                            i += 1
                        }
                        mod_list_count = 0 as i32;
                        if !global_inst_zone.is_null() {
                            mod_0 = (*global_inst_zone).mod_0;
                            while !mod_0.is_null() {
                                let fresh2 = mod_list_count;
                                mod_list_count = mod_list_count + 1;
                                mod_list[fresh2 as usize] = mod_0;
                                mod_0 = (*mod_0).next
                            }
                        }
                        mod_0 = (*inst_zone).mod_0;
                        while !mod_0.is_null() {
                            i = 0 as i32;
                            while i < mod_list_count {
                                if !mod_list[i as usize].is_null()
                                    && fluid_mod_test_identity(
                                        mod_0.as_ref().unwrap(),
                                        mod_list[i as usize].as_ref().unwrap(),
                                    ) != 0
                                {
                                    mod_list[i as usize] = 0 as *mut Mod
                                }
                                i += 1
                            }
                            let fresh3 = mod_list_count;
                            mod_list_count = mod_list_count + 1;
                            mod_list[fresh3 as usize] = mod_0;
                            mod_0 = (*mod_0).next
                        }
                        i = 0 as i32;
                        while i < mod_list_count {
                            mod_0 = mod_list[i as usize];
                            if !mod_0.is_null() {
                                fluid_voice_add_mod(
                                    voice,
                                    mod_0.as_ref().unwrap(),
                                    FLUID_VOICE_OVERWRITE as i32,
                                );
                            }
                            i += 1
                        }
                        i = 0 as i32;
                        while i < GEN_LAST as i32 {
                            if i != GEN_STARTADDROFS as i32
                                && i != GEN_ENDADDROFS as i32
                                && i != GEN_STARTLOOPADDROFS as i32
                                && i != GEN_ENDLOOPADDROFS as i32
                                && i != GEN_STARTADDRCOARSEOFS as i32
                                && i != GEN_ENDADDRCOARSEOFS as i32
                                && i != GEN_STARTLOOPADDRCOARSEOFS as i32
                                && i != GEN_KEYNUM as i32
                                && i != GEN_VELOCITY as i32
                                && i != GEN_ENDLOOPADDRCOARSEOFS as i32
                                && i != GEN_SAMPLEMODE as i32
                                && i != GEN_EXCLUSIVECLASS as i32
                                && i != GEN_OVERRIDEROOTKEY as i32
                            {
                                if (*preset_zone).gen[i as usize].flags != 0 {
                                    fluid_voice_gen_incr(
                                        voice,
                                        i,
                                        (*preset_zone).gen[i as usize].val as f32,
                                    );
                                } else if !global_preset_zone.is_null()
                                    && (*global_preset_zone).gen[i as usize].flags as i32 != 0
                                {
                                    fluid_voice_gen_incr(
                                        voice,
                                        i,
                                        (*global_preset_zone).gen[i as usize].val as f32,
                                    );
                                }
                            }
                            i += 1
                        }
                        mod_list_count = 0 as i32;
                        if !global_preset_zone.is_null() {
                            mod_0 = (*global_preset_zone).mod_0;
                            while !mod_0.is_null() {
                                let fresh4 = mod_list_count;
                                mod_list_count = mod_list_count + 1;
                                mod_list[fresh4 as usize] = mod_0;
                                mod_0 = (*mod_0).next
                            }
                        }
                        mod_0 = (*preset_zone).mod_0;
                        while !mod_0.is_null() {
                            i = 0 as i32;
                            while i < mod_list_count {
                                if !mod_list[i as usize].is_null()
                                    && fluid_mod_test_identity(
                                        mod_0.as_ref().unwrap(),
                                        mod_list[i as usize].as_ref().unwrap(),
                                    ) != 0
                                {
                                    mod_list[i as usize] = 0 as *mut Mod
                                }
                                i += 1
                            }
                            let fresh5 = mod_list_count;
                            mod_list_count = mod_list_count + 1;
                            mod_list[fresh5 as usize] = mod_0;
                            mod_0 = (*mod_0).next
                        }
                        i = 0 as i32;
                        while i < mod_list_count {
                            mod_0 = mod_list[i as usize];
                            if !mod_0.is_null() && (*mod_0).amount != 0 as i32 as f64 {
                                fluid_voice_add_mod(
                                    voice,
                                    mod_0.as_ref().unwrap(),
                                    FLUID_VOICE_ADD as i32,
                                );
                            }
                            i += 1
                        }
                        synth.start_voice(voice);
                    }
                    inst_zone = fluid_inst_zone_next(inst_zone)
                }
            }
        }
        preset_zone = fluid_preset_zone_next(preset_zone)
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defpreset_set_global_zone(
    mut preset: *mut DefPreset,
    zone: *mut PresetZone,
) -> i32 {
    (*preset).global_zone = zone;
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defpreset_import_sfont(
    mut preset: *mut DefPreset,
    sfpreset: *mut SFPreset,
    sfont: *mut DefSFont,
) -> i32 {
    let mut zone: *mut PresetZone;
    let mut count: i32;
    let mut zone_name: [u8; 256] = [0; 256];
    if (*sfpreset).name[0] != 0 {
        (*preset).name = (*sfpreset).name;
    } else {
        libc::strcpy(
            (*preset).name.as_mut_ptr() as _,
            CString::new(format!(
                "Bank{},Preset{}",
                (*sfpreset).bank,
                (*sfpreset).prenum
            ))
            .unwrap()
            .as_c_str()
            .as_ptr(),
        );
    }
    (*preset).bank = (*sfpreset).bank as u32;
    (*preset).num = (*sfpreset).prenum as u32;
    count = 0 as i32;
    for sfzone in (*sfpreset).zone.iter() {
        libc::strcpy(
            zone_name.as_mut_ptr() as _,
            CString::new(format!(
                "{}/{}",
                CStr::from_ptr((*preset).name.as_ptr() as _)
                    .to_str()
                    .unwrap(),
                count,
            ))
            .unwrap()
            .as_c_str()
            .as_ptr(),
        );
        zone = new_fluid_preset_zone(&zone_name);
        if zone.is_null() {
            return FLUID_FAILED as i32;
        }
        if fluid_preset_zone_import_sfont(zone, *sfzone, sfont) != FLUID_OK as i32 {
            return FLUID_FAILED as i32;
        }
        if count == 0 as i32 && fluid_preset_zone_get_inst(zone).is_null() {
            fluid_defpreset_set_global_zone(preset, zone);
        } else if fluid_defpreset_add_zone(preset, zone) != FLUID_OK as i32 {
            return FLUID_FAILED as i32;
        }
        count += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defpreset_add_zone(
    mut preset: *mut DefPreset,
    mut zone: *mut PresetZone,
) -> i32 {
    if (*preset).zone.is_null() {
        (*zone).next = 0 as *mut PresetZone;
        (*preset).zone = zone
    } else {
        (*zone).next = (*preset).zone;
        (*preset).zone = zone
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_defpreset_get_zone(preset: *mut DefPreset) -> *mut PresetZone {
    return (*preset).zone;
}

pub unsafe fn fluid_defpreset_get_global_zone(preset: *mut DefPreset) -> *mut PresetZone {
    return (*preset).global_zone;
}

pub unsafe fn fluid_preset_zone_next(preset: *mut PresetZone) -> *mut PresetZone {
    return (*preset).next;
}

pub unsafe fn new_fluid_preset_zone(name: &[u8]) -> *mut PresetZone {
    let mut zone: *mut PresetZone;
    zone = libc::malloc(::std::mem::size_of::<PresetZone>() as libc::size_t) as *mut PresetZone;
    if zone.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut PresetZone;
    }
    libc::memset(zone as _, 0, std::mem::size_of::<PresetZone>() as _);
    (*zone).next = 0 as *mut PresetZone;
    (*zone).name = name.to_vec();
    (*zone).inst = 0 as *mut Instrument;
    (*zone).keylo = 0 as i32;
    (*zone).keyhi = 128 as i32;
    (*zone).vello = 0 as i32;
    (*zone).velhi = 128 as i32;
    fluid_gen_set_default_values(&mut *(*zone).gen.as_mut_ptr().offset(0 as i32 as isize));
    (*zone).mod_0 = 0 as *mut Mod;
    return zone;
}

pub unsafe fn delete_fluid_preset_zone(zone: *mut PresetZone) -> i32 {
    let mut mod_0: *mut Mod;
    let mut tmp: *mut Mod;
    mod_0 = (*zone).mod_0;
    while !mod_0.is_null() {
        tmp = mod_0;
        mod_0 = (*mod_0).next;
        fluid_mod_delete(tmp.as_mut().unwrap());
    }
    if !(*zone).inst.is_null() {
        delete_fluid_inst((*zone).inst);
    }
    libc::free(zone as *mut libc::c_void);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_preset_zone_import_sfont(
    mut zone: *mut PresetZone,
    sfzone: *mut SFZone,
    sfont: *mut DefSFont,
) -> i32 {
    let mut count: i32;
    count = 0 as i32;
    for sfgen in (*sfzone).gen.iter() {
        match (**sfgen).id as i32 {
            43 => {
                (*zone).keylo = (**sfgen).amount.range.lo as i32;
                (*zone).keyhi = (**sfgen).amount.range.hi as i32
            }
            44 => {
                (*zone).vello = (**sfgen).amount.range.lo as i32;
                (*zone).velhi = (**sfgen).amount.range.hi as i32
            }
            _ => {
                (*zone).gen[(**sfgen).id as usize].val = (**sfgen).amount.sword as f32 as f64;
                (*zone).gen[(**sfgen).id as usize].flags = GEN_SET as i32 as u8
            }
        }
        count += 1
    }
    if !(*sfzone).instsamp.is_none() && !(*sfzone).instsamp.unwrap_inst().is_null() {
        (*zone).inst = new_fluid_inst();
        if (*zone).inst.is_null() {
            fluid_log!(FLUID_ERR, "Out of memory",);
            return FLUID_FAILED as i32;
        }
        if fluid_inst_import_sfont(
            (*zone).inst,
            (*sfzone).instsamp.unwrap_inst() as *mut SFInst,
            sfont,
        ) != FLUID_OK as i32
        {
            return FLUID_FAILED as i32;
        }
    }
    count = 0 as i32;
    for mod_src in (*sfzone).mod_0.iter() {
        let mut mod_dest: *mut Mod = fluid_mod_new();
        let mut type_0: i32;
        if mod_dest.is_null() {
            return FLUID_FAILED as i32;
        }
        (*mod_dest).next = 0 as *mut Mod;
        (*mod_dest).amount = (**mod_src).amount as f64;
        (*mod_dest).src1 = ((**mod_src).src as i32 & 127 as i32) as u8;
        (*mod_dest).flags1 = 0 as i32 as u8;
        if (**mod_src).src as i32 & (1 as i32) << 7 as i32 != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_CC as i32) as u8
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_GC as i32) as u8
        }
        if (**mod_src).src as i32 & (1 as i32) << 8 as i32 != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_NEGATIVE as i32) as u8
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_POSITIVE as i32) as u8
        }
        if (**mod_src).src as i32 & (1 as i32) << 9 as i32 != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_BIPOLAR as i32) as u8
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_UNIPOLAR as i32) as u8
        }
        type_0 = (**mod_src).src as i32 >> 10 as i32;
        type_0 &= 63 as i32;
        if type_0 == 0 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_LINEAR as i32) as u8
        } else if type_0 == 1 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_CONCAVE as i32) as u8
        } else if type_0 == 2 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_CONVEX as i32) as u8
        } else if type_0 == 3 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_SWITCH as i32) as u8
        } else {
            (*mod_dest).amount = 0 as i32 as f64
        }
        (*mod_dest).dest = (**mod_src).dest as u8;
        (*mod_dest).src2 = ((**mod_src).amtsrc as i32 & 127 as i32) as u8;
        (*mod_dest).flags2 = 0 as i32 as u8;
        if (**mod_src).amtsrc as i32 & (1 as i32) << 7 as i32 != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_CC as i32) as u8
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_GC as i32) as u8
        }
        if (**mod_src).amtsrc as i32 & (1 as i32) << 8 as i32 != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_NEGATIVE as i32) as u8
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_POSITIVE as i32) as u8
        }
        if (**mod_src).amtsrc as i32 & (1 as i32) << 9 as i32 != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_BIPOLAR as i32) as u8
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_UNIPOLAR as i32) as u8
        }
        type_0 = (**mod_src).amtsrc as i32 >> 10 as i32;
        type_0 &= 63 as i32;
        if type_0 == 0 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_LINEAR as i32) as u8
        } else if type_0 == 1 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_CONCAVE as i32) as u8
        } else if type_0 == 2 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_CONVEX as i32) as u8
        } else if type_0 == 3 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_SWITCH as i32) as u8
        } else {
            (*mod_dest).amount = 0 as i32 as f64
        }
        if (**mod_src).trans as i32 != 0 as i32 {
            (*mod_dest).amount = 0 as i32 as f64
        }
        if count == 0 as i32 {
            (*zone).mod_0 = mod_dest
        } else {
            let mut last_mod: *mut Mod = (*zone).mod_0;
            while !(*last_mod).next.is_null() {
                last_mod = (*last_mod).next
            }
            (*last_mod).next = mod_dest
        }
        count += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_preset_zone_get_inst(zone: *mut PresetZone) -> *mut Instrument {
    return (*zone).inst;
}

pub unsafe fn fluid_preset_zone_inside_range(zone: *mut PresetZone, key: i32, vel: i32) -> i32 {
    return ((*zone).keylo <= key
        && (*zone).keyhi >= key
        && (*zone).vello <= vel
        && (*zone).velhi >= vel) as i32;
}

pub unsafe fn new_fluid_inst() -> *mut Instrument {
    let mut inst: *mut Instrument =
        libc::malloc(::std::mem::size_of::<Instrument>() as libc::size_t) as *mut Instrument;
    if inst.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut Instrument;
    }
    (*inst).name = [0; 21];
    (*inst).global_zone = 0 as *mut InstrumentZone;
    (*inst).zone = 0 as *mut InstrumentZone;
    return inst;
}

pub unsafe fn delete_fluid_inst(mut inst: *mut Instrument) -> i32 {
    let mut zone: *mut InstrumentZone;
    let mut err: i32 = FLUID_OK as i32;
    if !(*inst).global_zone.is_null() {
        if delete_fluid_inst_zone((*inst).global_zone) != FLUID_OK as i32 {
            err = FLUID_FAILED as i32
        }
        (*inst).global_zone = 0 as *mut InstrumentZone
    }
    zone = (*inst).zone;
    while !zone.is_null() {
        (*inst).zone = (*zone).next;
        if delete_fluid_inst_zone(zone) != FLUID_OK as i32 {
            err = FLUID_FAILED as i32
        }
        zone = (*inst).zone
    }
    libc::free(inst as *mut libc::c_void);
    return err;
}

pub unsafe fn fluid_inst_set_global_zone(
    mut inst: *mut Instrument,
    zone: *mut InstrumentZone,
) -> i32 {
    (*inst).global_zone = zone;
    return FLUID_OK as i32;
}

pub unsafe fn fluid_inst_import_sfont(
    inst: *mut Instrument,
    sfinst: *mut SFInst,
    sfont: *mut DefSFont,
) -> i32 {
    let mut zone: *mut InstrumentZone;
    let mut zone_name: [u8; 256] = [0; 256];
    let mut count: i32;
    if libc::strlen((*sfinst).name.as_ptr() as _) > 0 {
        libc::strcpy((*inst).name.as_mut_ptr() as _, (*sfinst).name.as_ptr() as _);
    } else {
        libc::strcpy(
            (*inst).name.as_mut_ptr() as _,
            b"<untitled>\x00" as *const u8 as *const i8,
        );
    }
    count = 0 as i32;
    for sfzone in (*sfinst).zone.iter() {
        libc::strcpy(
            zone_name.as_mut_ptr() as _,
            CString::new(format!(
                "{}/{}",
                CStr::from_ptr((*inst).name.as_ptr() as _).to_str().unwrap(),
                count,
            ))
            .unwrap()
            .as_c_str()
            .as_ptr(),
        );
        zone = new_fluid_inst_zone(&zone_name);
        if zone.is_null() {
            return FLUID_FAILED as i32;
        }
        if fluid_inst_zone_import_sfont(zone, *sfzone, sfont) != FLUID_OK as i32 {
            return FLUID_FAILED as i32;
        }
        if count == 0 as i32 && fluid_inst_zone_get_sample(zone).is_null() {
            fluid_inst_set_global_zone(inst, zone);
        } else if fluid_inst_add_zone(inst, zone) != FLUID_OK as i32 {
            return FLUID_FAILED as i32;
        }
        count += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_inst_add_zone(mut inst: *mut Instrument, mut zone: *mut InstrumentZone) -> i32 {
    if (*inst).zone.is_null() {
        (*zone).next = 0 as *mut InstrumentZone;
        (*inst).zone = zone
    } else {
        (*zone).next = (*inst).zone;
        (*inst).zone = zone
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_inst_get_zone(inst: *mut Instrument) -> *mut InstrumentZone {
    return (*inst).zone;
}

pub unsafe fn fluid_inst_get_global_zone(inst: *mut Instrument) -> *mut InstrumentZone {
    return (*inst).global_zone;
}

pub unsafe fn new_fluid_inst_zone(name: &[u8]) -> *mut InstrumentZone {
    let mut zone: *mut InstrumentZone;
    zone = libc::malloc(::std::mem::size_of::<InstrumentZone>() as libc::size_t)
        as *mut InstrumentZone;
    libc::memset(zone as _, 0, std::mem::size_of::<InstrumentZone>() as _);
    if zone.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut InstrumentZone;
    }
    (*zone).next = 0 as *mut InstrumentZone;
    (*zone).name = name.to_vec();
    (*zone).sample = 0 as *mut Sample;
    (*zone).keylo = 0 as i32;
    (*zone).keyhi = 128 as i32;
    (*zone).vello = 0 as i32;
    (*zone).velhi = 128 as i32;
    fluid_gen_set_default_values(&mut *(*zone).gen.as_mut_ptr().offset(0 as i32 as isize));
    (*zone).mod_0 = 0 as *mut Mod;
    return zone;
}

pub unsafe fn delete_fluid_inst_zone(zone: *mut InstrumentZone) -> i32 {
    let mut mod_0: *mut Mod;
    let mut tmp: *mut Mod;
    mod_0 = (*zone).mod_0;
    while !mod_0.is_null() {
        tmp = mod_0;
        mod_0 = (*mod_0).next;
        fluid_mod_delete(tmp.as_mut().unwrap());
    }
    libc::free(zone as *mut libc::c_void);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_inst_zone_next(zone: *mut InstrumentZone) -> *mut InstrumentZone {
    return (*zone).next;
}

pub unsafe fn fluid_inst_zone_import_sfont(
    mut zone: *mut InstrumentZone,
    sfzone: *mut SFZone,
    sfont: *mut DefSFont,
) -> i32 {
    let mut count: i32;
    count = 0 as i32;
    for sfgen in (*sfzone).gen.iter() {
        match (**sfgen).id as i32 {
            43 => {
                (*zone).keylo = (**sfgen).amount.range.lo as i32;
                (*zone).keyhi = (**sfgen).amount.range.hi as i32
            }
            44 => {
                (*zone).vello = (**sfgen).amount.range.lo as i32;
                (*zone).velhi = (**sfgen).amount.range.hi as i32
            }
            _ => {
                (*zone).gen[(**sfgen).id as usize].val = (**sfgen).amount.sword as f32 as f64;
                (*zone).gen[(**sfgen).id as usize].flags = GEN_SET as i32 as u8
            }
        }
        count += 1
    }
    if !(*sfzone).instsamp.is_none() && !(*sfzone).instsamp.unwrap_sample().is_null() {
        (*zone).sample =
            fluid_defsfont_get_sample(sfont, &(*((*sfzone).instsamp.unwrap_sample())).name);
        if (*zone).sample.is_null() {
            fluid_log!(FLUID_ERR, "Couldn't find sample name",);
            return FLUID_FAILED as i32;
        }
    }
    count = 0 as i32;
    for mod_src in (*sfzone).mod_0.iter() {
        let mut type_0: i32;
        let mut mod_dest: *mut Mod;
        mod_dest = fluid_mod_new();
        if mod_dest.is_null() {
            return FLUID_FAILED as i32;
        }
        (*mod_dest).next = 0 as *mut Mod;
        (*mod_dest).amount = (**mod_src).amount as f64;
        (*mod_dest).src1 = ((**mod_src).src as i32 & 127 as i32) as u8;
        (*mod_dest).flags1 = 0 as i32 as u8;
        if (**mod_src).src as i32 & (1 as i32) << 7 as i32 != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_CC as i32) as u8
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_GC as i32) as u8
        }
        if (**mod_src).src as i32 & (1 as i32) << 8 as i32 != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_NEGATIVE as i32) as u8
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_POSITIVE as i32) as u8
        }
        if (**mod_src).src as i32 & (1 as i32) << 9 as i32 != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_BIPOLAR as i32) as u8
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_UNIPOLAR as i32) as u8
        }
        type_0 = (**mod_src).src as i32 >> 10 as i32;
        type_0 &= 63 as i32;
        if type_0 == 0 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_LINEAR as i32) as u8
        } else if type_0 == 1 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_CONCAVE as i32) as u8
        } else if type_0 == 2 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_CONVEX as i32) as u8
        } else if type_0 == 3 as i32 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as i32 | FLUID_MOD_SWITCH as i32) as u8
        } else {
            (*mod_dest).amount = 0 as i32 as f64
        }
        (*mod_dest).dest = (**mod_src).dest as u8;
        (*mod_dest).src2 = ((**mod_src).amtsrc as i32 & 127 as i32) as u8;
        (*mod_dest).flags2 = 0 as i32 as u8;
        if (**mod_src).amtsrc as i32 & (1 as i32) << 7 as i32 != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_CC as i32) as u8
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_GC as i32) as u8
        }
        if (**mod_src).amtsrc as i32 & (1 as i32) << 8 as i32 != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_NEGATIVE as i32) as u8
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_POSITIVE as i32) as u8
        }
        if (**mod_src).amtsrc as i32 & (1 as i32) << 9 as i32 != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_BIPOLAR as i32) as u8
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_UNIPOLAR as i32) as u8
        }
        type_0 = (**mod_src).amtsrc as i32 >> 10 as i32;
        type_0 &= 63 as i32;
        if type_0 == 0 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_LINEAR as i32) as u8
        } else if type_0 == 1 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_CONCAVE as i32) as u8
        } else if type_0 == 2 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_CONVEX as i32) as u8
        } else if type_0 == 3 as i32 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as i32 | FLUID_MOD_SWITCH as i32) as u8
        } else {
            (*mod_dest).amount = 0 as i32 as f64
        }
        if (**mod_src).trans as i32 != 0 as i32 {
            (*mod_dest).amount = 0 as i32 as f64
        }
        if count == 0 as i32 {
            (*zone).mod_0 = mod_dest
        } else {
            let mut last_mod: *mut Mod = (*zone).mod_0;
            while !(*last_mod).next.is_null() {
                last_mod = (*last_mod).next
            }
            (*last_mod).next = mod_dest
        }
        count += 1
    }
    return FLUID_OK as i32;
}

pub unsafe fn fluid_inst_zone_get_sample(zone: *mut InstrumentZone) -> *mut Sample {
    return (*zone).sample;
}

pub unsafe fn fluid_inst_zone_inside_range(zone: *mut InstrumentZone, key: i32, vel: i32) -> i32 {
    return ((*zone).keylo <= key
        && (*zone).keyhi >= key
        && (*zone).vello <= vel
        && (*zone).velhi >= vel) as i32;
}

pub unsafe fn new_fluid_sample() -> *mut Sample {
    let mut sample: *mut Sample;
    sample = libc::malloc(::std::mem::size_of::<Sample>() as libc::size_t) as *mut Sample;
    if sample.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut Sample;
    }
    libc::memset(
        sample as *mut libc::c_void,
        0 as i32,
        ::std::mem::size_of::<Sample>() as libc::size_t,
    );
    (*sample).valid = 1 as i32;
    return sample;
}

pub unsafe fn delete_fluid_sample(sample: *mut Sample) -> i32 {
    libc::free(sample as *mut libc::c_void);
    return FLUID_OK as i32;
}

pub unsafe fn fluid_sample_in_rom(sample: *mut Sample) -> i32 {
    return (*sample).sampletype & 0x8000 as i32;
}

pub unsafe fn fluid_sample_import_sfont(
    mut sample: *mut Sample,
    sfsample: *mut SFSample,
    sfont: *mut DefSFont,
) -> i32 {
    libc::strcpy(
        (*sample).name.as_mut_ptr() as _,
        (*sfsample).name.as_ptr() as _,
    );
    (*sample).data = (*sfont).sampledata;
    (*sample).start = (*sfsample).start;
    (*sample).end = (*sfsample).start.wrapping_add((*sfsample).end);
    (*sample).loopstart = (*sfsample).start.wrapping_add((*sfsample).loopstart);
    (*sample).loopend = (*sfsample).start.wrapping_add((*sfsample).loopend);
    (*sample).samplerate = (*sfsample).samplerate;
    (*sample).origpitch = (*sfsample).origpitch as i32;
    (*sample).pitchadj = (*sfsample).pitchadj as i32;
    (*sample).sampletype = (*sfsample).sampletype as i32;
    if ((*sample).sampletype & 0x10 as i32) != 0 {
        // vorbis?
        return FLUID_OK;
    }
    if (*sample).sampletype & 0x8000 as i32 != 0 {
        (*sample).valid = 0 as i32;
        fluid_log!(
            FLUID_WARN,
            "Ignoring sample: can\'t use ROM samples",
            //(*sample).name
        );
    }
    if (*sample).end.wrapping_sub((*sample).start) < 8 as i32 as u32 {
        (*sample).valid = 0 as i32;
        fluid_log!(
            FLUID_WARN,
            "Ignoring sample: too few sample data points",
            //(*sample).name
        );
    }
    return FLUID_OK as i32;
}

pub static IDLIST: &[u8; 113] =
    b"RIFFLISTsfbkINFOsdtapdtaifilisngINAMiromiverICRDIENGIPRDICOPICMTISFTsnamsmplphdrpbagpmodpgeninstibagimodigenshdr\x00";
static mut SDTACHUNK_SIZE: u32 = 0;
unsafe fn chunkid(id: u32) -> i32 {
    let mut i: u32;
    let mut p: *const u32;
    p = IDLIST as *const [u8; 113] as *const u32;
    i = 0 as i32 as u32;
    while (i as usize)
        < (::std::mem::size_of::<[u8; 113]>() as usize)
            .wrapping_div(::std::mem::size_of::<i32>() as usize)
    {
        if *p == id {
            return i.wrapping_add(1 as i32 as u32) as i32;
        }
        i = i.wrapping_add(1);
        p = p.offset(1 as i32 as isize)
    }
    return UNKN_ID as i32;
}

pub unsafe fn sfload_file(fname: &[u8], fapi: *mut FileApi) -> *mut SFData {
    let mut sf: *mut SFData;
    let fd: *mut libc::c_void;
    let mut fsize: i32 = 0 as i32;
    let mut err: i32 = 0 as i32;
    fd = (*fapi).fopen.expect("non-null function pointer")(fapi, fname);
    if fd.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Unable to open file \"{}\"",
            CStr::from_ptr(fname.as_ptr() as *const i8)
                .to_str()
                .unwrap()
        );
        return 0 as *mut SFData;
    }
    sf = libc::malloc(::std::mem::size_of::<SFData>() as libc::size_t) as *mut SFData;
    if sf.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        err = (0 as i32 == 0) as i32
    }
    if err == 0 {
        libc::memset(
            sf as *mut libc::c_void,
            0 as i32,
            ::std::mem::size_of::<SFData>() as libc::size_t,
        );
        (*sf).fname = fname.to_vec();
        (*sf).sffd = fd as *mut libc::FILE
    }
    if err == 0
        && (*fapi).fseek.expect("non-null function pointer")(fd, 0 as isize, 2 as i32)
            == FLUID_FAILED as i32
    {
        err = (0 as i32 == 0) as i32;
        fluid_log!(FLUID_ERR, "Seek to end of file failed",);
    }
    if err == 0 && {
        fsize = (*fapi).ftell.expect("non-null function pointer")(fd) as i32;
        (fsize) == FLUID_FAILED as i32
    } {
        err = (0 as i32 == 0) as i32;
        fluid_log!(FLUID_ERR, "Get end of file position failed",);
    }
    if err == 0 {
        (*fapi).fseek.expect("non-null function pointer")(fd, 0 as i32 as isize, 0 as i32);
    }
    if err == 0 && load_body(fsize as u32, sf, fd, fapi) == 0 {
        err = (0 as i32 == 0) as i32
    }
    if err != 0 {
        if !sf.is_null() {
            sfont_close(sf, fapi);
        }
        return 0 as *mut SFData;
    }
    return sf;
}
unsafe fn load_body(size: u32, sf: *mut SFData, fd: *mut libc::c_void, fapi: *mut FileApi) -> i32 {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            &mut chunk as *mut SFChunk as *mut libc::c_void,
            8 as i32,
            fd,
        ) == FLUID_FAILED
        {
            return 0 as i32;
        }
        (*(&mut chunk as *mut SFChunk)).size = (*(&mut chunk as *mut SFChunk)).size;
    });
    if chunkid(chunk.id) != RIFF_ID as i32 {
        fluid_log!(FLUID_ERR, "Not a RIFF file",);
        return 0 as i32;
    }
    if (*fapi).fread.expect("non-null function pointer")(
        &mut chunk.id as *mut u32 as *mut libc::c_void,
        4 as i32,
        fd,
    ) == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    if chunkid(chunk.id) != SFBK_ID as i32 {
        fluid_log!(FLUID_ERR, "Not a sound font file",);
        return 0 as i32;
    }
    if chunk.size != size.wrapping_sub(8 as i32 as u32) {
        gerr!(ErrCorr, "Sound font file size mismatch",);
        return 0 as i32;
    }
    if read_listchunk(&mut chunk, fd, fapi) == 0 {
        return 0 as i32;
    }
    if chunkid(chunk.id) != INFO_ID as i32 {
        return gerr!(ErrCorr, "Invalid ID found when expecting INFO chunk",);
    }
    if process_info(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if read_listchunk(&mut chunk, fd, fapi) == 0 {
        return 0 as i32;
    }
    if chunkid(chunk.id) != SDTA_ID as i32 {
        return gerr!(ErrCorr, "Invalid ID found when expecting SAMPLE chunk",);
    }
    if process_sdta(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if read_listchunk(&mut chunk, fd, fapi) == 0 {
        return 0 as i32;
    }
    if chunkid(chunk.id) != PDTA_ID as i32 {
        return gerr!(ErrCorr, "Invalid ID found when expecting HYDRA chunk",);
    }
    if process_pdta(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if fixup_pgen(sf) == 0 {
        return 0 as i32;
    }
    if fixup_igen(sf) == 0 {
        return 0 as i32;
    }
    if fixup_sample(sf) == 0 {
        return 0 as i32;
    }
    (*sf).preset.sort_by(|a, b| {
        let cmp = sfont_preset_compare_func(*a as *mut libc::c_void, *b as *mut libc::c_void);
        if cmp < 0 {
            return Ordering::Less;
        } else if cmp > 0 {
            return Ordering::Greater;
        } else {
            return Ordering::Equal;
        }
    });
    return 1 as i32;
}
unsafe fn read_listchunk(
    mut chunk: *mut SFChunk,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            chunk as *mut libc::c_void,
            8 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        (*chunk).size = (*chunk).size;
    });
    if chunkid((*chunk).id) != LIST_ID as i32 {
        return gerr!(ErrCorr, "Invalid chunk id in level 0 parse",);
    }
    if (*fapi).fread.expect("non-null function pointer")(
        &mut (*chunk).id as *mut u32 as *mut libc::c_void,
        4 as i32,
        fd,
    ) == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    (*chunk).size = (*chunk).size.wrapping_sub(4 as i32 as u32);
    return 1 as i32;
}
unsafe fn process_info(
    mut size: i32,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    let mut id: u8;
    let mut ver: u16;
    while size > 0 as i32 {
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut chunk as *mut SFChunk as *mut libc::c_void,
                8 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*(&mut chunk as *mut SFChunk)).size = (*(&mut chunk as *mut SFChunk)).size;
        });
        size -= 8 as i32;
        id = chunkid(chunk.id) as u8;
        if id as i32 == IFIL_ID as i32 {
            if chunk.size != 4 as i32 as u32 {
                return gerr!(ErrCorr, "Sound font version info chunk has invalid size",);
            }
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                ver = _temp as i16 as u16;
            });
            (*sf).version.major = ver;
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                ver = _temp as i16 as u16;
            });
            (*sf).version.minor = ver;
            if ((*sf).version.major as i32) < 2 as i32 {
                fluid_log!(
                    FLUID_ERR,
                    "Sound font version is {}.{} which is not supported, convert to version 2.0x",
                    (*sf).version.major,
                    (*sf).version.minor
                );
                return 0 as i32;
            }
            if (*sf).version.major as i32 > 2 as i32 {
                fluid_log!(FLUID_WARN,
                          "Sound font version is {}.{} which is newer than what this version of FLUID Synth was designed for (v2.0x)",
                          (*sf).version.major,
                          (*sf).version.minor);
                return 0 as i32;
            }
        } else if id as i32 == IVER_ID as i32 {
            if chunk.size != 4 as i32 as u32 {
                return gerr!(ErrCorr, "ROM version info chunk has invalid size",);
            }
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                ver = _temp as i16 as u16;
            });
            (*sf).romver.major = ver;
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                ver = _temp as i16 as u16;
            });
            (*sf).romver.minor = ver
        } else if id as i32 != UNKN_ID as i32 {
            if id as i32 != ICMT_ID as i32 && chunk.size > 256 as i32 as u32
                || chunk.size > 65536 as i32 as u32
                || chunk.size.wrapping_rem(2 as i32 as u32) != 0
            {
                return gerr!(
                    ErrCorr,
                    "INFO sub chunk {} has invalid chunk size of {} bytes",
                    chunk.id,
                    chunk.size
                );
            }
            let mut item = Vec::new();
            item.resize(chunk.size as usize + 1, 0);
            item[0] = id;
            if (*fapi).fread.expect("non-null function pointer")(
                item.as_mut_ptr().offset(1) as _,
                chunk.size as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            item[chunk.size as usize - 1] = 0;
            (*sf).info.push(item);
        } else {
            return gerr!(ErrCorr, "Invalid chunk id in INFO chunk",);
        }
        size = (size as u32).wrapping_sub(chunk.size) as i32 as i32
    }
    if size < 0 as i32 {
        return gerr!(ErrCorr, "INFO chunk size mismatch",);
    }
    return 1 as i32;
}
unsafe fn process_sdta(
    mut size: i32,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    if size == 0 as i32 {
        return 1 as i32;
    }
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            &mut chunk as *mut SFChunk as *mut libc::c_void,
            8 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        (*(&mut chunk as *mut SFChunk)).size = (*(&mut chunk as *mut SFChunk)).size;
    });
    size -= 8 as i32;
    if chunkid(chunk.id) != SMPL_ID as i32 {
        return gerr!(ErrCorr, "Expected SMPL chunk found invalid id instead",);
    }
    if (size as u32).wrapping_sub(chunk.size) != 0 as i32 as u32 {
        return gerr!(ErrCorr, "SDTA chunk size mismatch",);
    }
    (*sf).samplepos = (*fapi).ftell.expect("non-null function pointer")(fd) as u32;
    SDTACHUNK_SIZE = chunk.size;
    (*sf).samplesize = chunk.size;
    if (*fapi).fseek.expect("non-null function pointer")(fd, chunk.size as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe fn pdtahelper(
    expid: u32,
    reclen: u32,
    mut chunk: *mut SFChunk,
    size: *mut i32,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let id: u32;
    let expstr: *mut i8;
    expstr = IDLIST.as_ptr().offset(
        expid
            .wrapping_sub(1 as i32 as u32)
            .wrapping_mul(4 as i32 as u32) as isize,
    ) as *mut i8;
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            chunk as *mut libc::c_void,
            8 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        (*chunk).size = (*chunk).size;
    });
    *size -= 8 as i32;
    id = chunkid((*chunk).id) as u32;
    if id != expid {
        return gerr!(
            ErrCorr,
            "Expected PDTA sub-chunk \"{}\" found invalid id instead",
            CStr::from_ptr(expstr).to_str().unwrap()
        );
    }
    if (*chunk).size.wrapping_rem(reclen) != 0 {
        return gerr!(
            ErrCorr,
            "\"{}\" chunk size is not a multiple of {} bytes",
            CStr::from_ptr(expstr).to_str().unwrap(),
            reclen
        );
    }
    *size = (*size as u32).wrapping_sub((*chunk).size) as i32 as i32;
    if *size < 0 as i32 {
        return gerr!(
            ErrCorr,
            "\"{}\" chunk size exceeds remaining PDTA chunk size",
            CStr::from_ptr(expstr).to_str().unwrap()
        );
    }
    return 1 as i32;
}
unsafe fn process_pdta(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    if pdtahelper(
        PHDR_ID as i32 as u32,
        38 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_phdr(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        PBAG_ID as i32 as u32,
        4 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_pbag(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        PMOD_ID as i32 as u32,
        10 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_pmod(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        PGEN_ID as i32 as u32,
        4 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_pgen(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        IHDR_ID as i32 as u32,
        22 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_ihdr(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        IBAG_ID as i32 as u32,
        4 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_ibag(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        IMOD_ID as i32 as u32,
        10 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_imod(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        IGEN_ID as i32 as u32,
        4 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_igen(chunk.size as i32, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    if pdtahelper(
        SHDR_ID as i32 as u32,
        46 as i32 as u32,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as i32;
    }
    if load_shdr(chunk.size, sf, fd, fapi) == 0 {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe fn load_phdr(size: i32, sf: *mut SFData, fd: *mut libc::c_void, fapi: *mut FileApi) -> i32 {
    let mut i: i32;
    let mut i2: i32;
    let mut p: *mut SFPreset;
    let mut pr: *mut SFPreset = 0 as *mut SFPreset;
    let mut zndx: u16;
    let mut pzndx: u16 = 0 as i32 as u16;
    if size % 38 as i32 != 0 || size == 0 as i32 {
        return gerr!(ErrCorr, "Preset header chunk size is invalid",);
    }
    i = size / 38 as i32 - 1 as i32;
    if i == 0 as i32 {
        fluid_log!(FLUID_WARN, "File contains no presets",);
        if (*fapi).fseek.expect("non-null function pointer")(fd, 38 as i32 as isize, 1 as i32)
            == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        return 1 as i32;
    }
    while i > 0 as i32 {
        p = libc::malloc(::std::mem::size_of::<SFPreset>() as libc::size_t) as *mut SFPreset;
        libc::memset(p as _, 0, std::mem::size_of::<SFPreset>() as libc::size_t);
        (*sf).preset.push(p);
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut (*p).name as *mut [u8; 21] as *mut libc::c_void,
                20 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).name[20] = 0;
        });
        ({
            let mut _temp: u16 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u16 as *mut libc::c_void,
                2 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).prenum = _temp as i16 as u16;
        });
        ({
            let mut _temp: u16 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u16 as *mut libc::c_void,
                2 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).bank = _temp as i16 as u16;
        });
        ({
            let mut _temp: u16 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u16 as *mut libc::c_void,
                2 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            zndx = _temp as i16 as u16;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).libr = _temp as i32 as u32;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).genre = _temp as i32 as u32;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).morph = _temp as i32 as u32;
        });
        if !pr.is_null() {
            if (zndx as i32) < pzndx as i32 {
                return gerr!(ErrCorr, "Preset header indices not monotonic",);
            }
            i2 = zndx as i32 - pzndx as i32;
            loop {
                let fresh6 = i2;
                i2 = i2 - 1;
                if !(fresh6 != 0) {
                    break;
                }
                (*pr).zone.insert(0, 0 as _);
            }
        } else if zndx as i32 > 0 as i32 {
            fluid_log!(
                FLUID_WARN,
                "{} preset zones not referenced, discarding",
                zndx
            );
        }
        pr = p;
        pzndx = zndx;
        i -= 1
    }
    if (*fapi).fseek.expect("non-null function pointer")(fd, 24 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    ({
        let mut _temp: u16 = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut u16 as *mut libc::c_void,
            2 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        zndx = _temp as i16 as u16;
    });
    if (*fapi).fseek.expect("non-null function pointer")(fd, 12 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    if (zndx as i32) < pzndx as i32 {
        return gerr!(ErrCorr, "Preset header indices not monotonic",);
    }
    i2 = zndx as i32 - pzndx as i32;
    loop {
        let fresh7 = i2;
        i2 = i2 - 1;
        if !(fresh7 != 0) {
            break;
        }
        (*pr).zone.insert(0, 0 as _);
    }
    return 1 as i32;
}
unsafe fn load_pbag(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut pz: *mut SFZone = 0 as *mut SFZone;
    let mut genndx: u16;
    let mut modndx: u16;
    let mut pgenndx: u16 = 0 as i32 as u16;
    let mut pmodndx: u16 = 0 as i32 as u16;
    let mut i: u16;
    if size % 4 as i32 != 0 || size == 0 as i32 {
        return gerr!(ErrCorr, "Preset bag chunk size is invalid",);
    }
    for preset in (*sf).preset.iter() {
        for z in (**preset).zone.iter_mut() {
            size -= 4 as i32;
            if size < 0 as i32 {
                return gerr!(ErrCorr, "Preset bag chunk size mismatch",);
            }
            *z = libc::malloc(::std::mem::size_of::<SFZone>() as libc::size_t) as *mut SFZone;
            libc::memset(*z as _, 0, ::std::mem::size_of::<SFZone>() as _);
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                genndx = _temp as i16 as u16;
            });
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                modndx = _temp as i16 as u16;
            });
            (**z).instsamp = InstSamp::None;
            if !pz.is_null() {
                if (genndx as i32) < pgenndx as i32 {
                    return gerr!(ErrCorr, "Preset bag generator indices not monotonic",);
                }
                if (modndx as i32) < pmodndx as i32 {
                    return gerr!(ErrCorr, "Preset bag modulator indices not monotonic",);
                }
                i = (genndx as i32 - pgenndx as i32) as u16;
                loop {
                    let fresh8 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh8 != 0) {
                        break;
                    }
                    (*pz).gen.insert(0, 0 as _);
                }
                i = (modndx as i32 - pmodndx as i32) as u16;
                loop {
                    let fresh9 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh9 != 0) {
                        break;
                    }
                    (*pz).mod_0.insert(0, 0 as _);
                }
            }
            pz = *z;
            pgenndx = genndx;
            pmodndx = modndx;
        }
    }
    size -= 4 as i32;
    if size != 0 as i32 {
        return gerr!(ErrCorr, "Preset bag chunk size mismatch",);
    }
    ({
        let mut _temp: u16 = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut u16 as *mut libc::c_void,
            2 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        genndx = _temp as i16 as u16;
    });
    ({
        let mut _temp: u16 = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut u16 as *mut libc::c_void,
            2 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        modndx = _temp as i16 as u16;
    });
    if pz.is_null() {
        if genndx as i32 > 0 as i32 {
            fluid_log!(FLUID_WARN, "No preset generators and terminal index not 0",);
        }
        if modndx as i32 > 0 as i32 {
            fluid_log!(FLUID_WARN, "No preset modulators and terminal index not 0",);
        }
        return 1 as i32;
    }
    if (genndx as i32) < pgenndx as i32 {
        return gerr!(ErrCorr, "Preset bag generator indices not monotonic",);
    }
    if (modndx as i32) < pmodndx as i32 {
        return gerr!(ErrCorr, "Preset bag modulator indices not monotonic",);
    }
    i = (genndx as i32 - pgenndx as i32) as u16;
    loop {
        let fresh10 = i;
        i = i.wrapping_sub(1);
        if !(fresh10 != 0) {
            break;
        }
        (*pz).gen.insert(0, 0 as _);
    }
    i = (modndx as i32 - pmodndx as i32) as u16;
    loop {
        let fresh11 = i;
        i = i.wrapping_sub(1);
        if !(fresh11 != 0) {
            break;
        }
        (*pz).mod_0.insert(0, 0 as _);
    }
    return 1 as i32;
}
unsafe fn load_pmod(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    for preset in (*sf).preset.iter() {
        for z in (**preset).zone.iter() {
            for m in (**z).mod_0.iter_mut() {
                size -= 10 as i32;
                if size < 0 as i32 {
                    return gerr!(ErrCorr, "Preset modulator chunk size mismatch",);
                }
                *m = libc::malloc(::std::mem::size_of::<SFMod>() as libc::size_t) as *mut SFMod;
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).src = _temp as i16 as u16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).dest = _temp as i16 as u16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).amount = _temp as i16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).amtsrc = _temp as i16 as u16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).trans = _temp as i16 as u16;
                });
            }
        }
    }
    if size == 0 as i32 {
        return 1 as i32;
    }
    size -= 10 as i32;
    if size != 0 as i32 {
        return gerr!(ErrCorr, "Preset modulator chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(fd, 10 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe fn load_pgen(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut p3;
    let mut dup;
    let hz: usize = 0;
    let mut g: *mut SFGen;
    let mut genval: SFGenAmount = SFGenAmount { sword: 0 };
    let mut genid: u16;
    let mut level: i32;
    let mut skip: i32;
    let mut drop_0: i32;
    let mut gzone: i32;
    let mut discarded: i32;
    for preset in (*sf).preset.iter() {
        gzone = 0 as i32;
        discarded = 0 as i32;
        for (i, z) in (**preset).zone.iter_mut().enumerate() {
            level = 0 as i32;
            p3 = 0;
            while p3 < (**z).gen.len() {
                dup = None;
                skip = 0 as i32;
                drop_0 = 0 as i32;
                size -= 4 as i32;
                if size < 0 as i32 {
                    return gerr!(ErrCorr, "Preset generator chunk size mismatch",);
                }
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    genid = _temp as i16 as u16;
                });
                if genid as i32 == GEN_KEY_RANGE as i32 {
                    if level == 0 as i32 {
                        level = 1 as i32;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                    } else {
                        skip = (0 as i32 == 0) as i32
                    }
                } else if genid as i32 == GEN_VEL_RANGE as i32 {
                    if level <= 1 as i32 {
                        level = 2 as i32;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                    } else {
                        skip = (0 as i32 == 0) as i32
                    }
                } else if genid as i32 == GEN_INSTRUMENT as i32 {
                    level = 3 as i32;
                    ({
                        let mut _temp: u16 = 0;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut _temp as *mut u16 as *mut libc::c_void,
                            2 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                        genval.uword = _temp as i16 as u16;
                    });
                    let ref mut fresh12 = (**z).instsamp;
                    *fresh12 = InstSamp::Int(genval.uword as i32 + 1 as i32);
                    break;
                } else {
                    level = 2 as i32;
                    if gen_validp(genid as i32) != 0 {
                        ({
                            let mut _temp: u16 = 0;
                            if (*fapi).fread.expect("non-null function pointer")(
                                &mut _temp as *mut u16 as *mut libc::c_void,
                                2 as i32,
                                fd,
                            ) == FLUID_FAILED as i32
                            {
                                return 0 as i32;
                            }
                            genval.sword = _temp as i16;
                        });
                        dup = (**z)
                            .gen
                            .iter()
                            .position(|x| !(*x).is_null() && (**x).id == genid);
                    } else {
                        skip = (0 as i32 == 0) as i32
                    }
                }
                if skip == 0 {
                    if dup.is_none() {
                        g = libc::malloc(::std::mem::size_of::<SFGen>() as libc::size_t)
                            as *mut SFGen;
                        (**z).gen[p3] = g;
                        (*g).id = genid
                    } else {
                        g = (**z).gen[dup.unwrap()];
                        drop_0 = (0 as i32 == 0) as i32
                    }
                    (*g).amount = genval
                } else {
                    discarded = (0 as i32 == 0) as i32;
                    drop_0 = (0 as i32 == 0) as i32;
                    if (*fapi).fseek.expect("non-null function pointer")(
                        fd,
                        2 as i32 as isize,
                        1 as i32,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                }
                if drop_0 == 0 {
                    p3 += 1;
                } else {
                    (**z).gen.remove(p3);
                }
            }
            if level == 3 as i32 {
                (**z).gen.remove(p3);
            } else if gzone == 0 {
                gzone = (0 as i32 == 0) as i32;
                if hz != i {
                    (**preset).zone[hz] = *z;
                    continue;
                }
            } else {
                fluid_log!(
                    FLUID_WARN,
                    "Preset \"{}\": Discarding invalid global zone",
                    CStr::from_ptr((**preset).name.as_ptr() as *const i8)
                        .to_str()
                        .unwrap()
                );
                sfont_free_zone(*z);
                *z = 0 as _;
            }
            while p3 < (**z).gen.len() {
                discarded = (0 as i32 == 0) as i32;
                size -= 4 as i32;
                if size < 0 as i32 {
                    return gerr!(ErrCorr, "Preset generator chunk size mismatch",);
                }
                if (*fapi).fseek.expect("non-null function pointer")(
                    fd,
                    4 as i32 as isize,
                    1 as i32,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                {
                    (**z).gen.remove(p3);
                }
            }
        }
        if discarded != 0 {
            fluid_log!(
                FLUID_WARN,
                "Preset \"{}\": Some invalid generators were discarded",
                CStr::from_ptr((**preset).name.as_ptr() as *const i8)
                    .to_str()
                    .unwrap()
            );
        }
    }
    if size == 0 as i32 {
        return 1 as i32;
    }
    size -= 4 as i32;
    if size != 0 as i32 {
        return gerr!(ErrCorr, "Preset generator chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(fd, 4 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe fn load_ihdr(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut i: i32;
    let mut i2: i32;
    let mut p: *mut SFInst;
    let mut pr: *mut SFInst = 0 as *mut SFInst;
    let mut zndx: u16;
    let mut pzndx: u16 = 0 as i32 as u16;
    if size % 22 as i32 != 0 || size == 0 as i32 {
        return gerr!(ErrCorr, "Instrument header has invalid size",);
    }
    size = size / 22 as i32 - 1 as i32;
    if size == 0 as i32 {
        fluid_log!(FLUID_WARN, "File contains no instruments",);
        if (*fapi).fseek.expect("non-null function pointer")(fd, 22 as i32 as isize, 1 as i32)
            == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        return 1 as i32;
    }
    i = 0 as i32;
    while i < size {
        p = libc::malloc(::std::mem::size_of::<SFInst>() as libc::size_t) as *mut SFInst;
        libc::memset(p as _, 0, ::std::mem::size_of::<SFInst>() as libc::size_t);
        (*sf).inst.push(p);
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut (*p).name as *mut [u8; 21] as *mut libc::c_void,
                20 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).name[20] = 0;
        });
        ({
            let mut _temp: u16 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u16 as *mut libc::c_void,
                2 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            zndx = _temp as i16 as u16;
        });
        if !pr.is_null() {
            if (zndx as i32) < pzndx as i32 {
                return gerr!(ErrCorr, "Instrument header indices not monotonic",);
            }
            i2 = zndx as i32 - pzndx as i32;
            loop {
                let fresh13 = i2;
                i2 = i2 - 1;
                if !(fresh13 != 0) {
                    break;
                }
                (*pr).zone.insert(0, 0 as _)
            }
        } else if zndx as i32 > 0 as i32 {
            fluid_log!(
                FLUID_WARN,
                "{} instrument zones not referenced, discarding",
                zndx
            );
        }
        pzndx = zndx;
        pr = p;
        i += 1
    }
    if (*fapi).fseek.expect("non-null function pointer")(fd, 20 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    ({
        let mut _temp: u16 = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut u16 as *mut libc::c_void,
            2 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        zndx = _temp as i16 as u16;
    });
    if (zndx as i32) < pzndx as i32 {
        return gerr!(ErrCorr, "Instrument header indices not monotonic",);
    }
    i2 = zndx as i32 - pzndx as i32;
    loop {
        let fresh14 = i2;
        i2 = i2 - 1;
        if !(fresh14 != 0) {
            break;
        }
        (*pr).zone.insert(0, 0 as _)
    }
    return 1 as i32;
}
unsafe fn load_ibag(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut pz: *mut SFZone = 0 as *mut SFZone;
    let mut genndx;
    let mut modndx;
    let mut pgenndx: u16 = 0 as i32 as u16;
    let mut pmodndx: u16 = 0 as i32 as u16;
    let mut i;
    if size % 4 as i32 != 0 || size == 0 as i32 {
        return gerr!(ErrCorr, "Instrument bag chunk size is invalid",);
    }
    for inst in (*sf).inst.iter() {
        for z in (**inst).zone.iter_mut() {
            size -= 4 as i32;
            if size < 0 as i32 {
                return gerr!(ErrCorr, "Instrument bag chunk size mismatch",);
            }
            *z = libc::malloc(::std::mem::size_of::<SFZone>() as libc::size_t) as *mut SFZone;
            libc::memset(*z as _, 0, std::mem::size_of::<SFZone>() as _);
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                genndx = _temp as i16 as u16;
            });
            ({
                let mut _temp: u16 = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut u16 as *mut libc::c_void,
                    2 as i32,
                    fd,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                modndx = _temp as i16 as u16;
            });
            (**z).instsamp = InstSamp::None;
            if !pz.is_null() {
                if (genndx as i32) < pgenndx as i32 {
                    return gerr!(ErrCorr, "Instrument generator indices not monotonic",);
                }
                if (modndx as i32) < pmodndx as i32 {
                    return gerr!(ErrCorr, "Instrument modulator indices not monotonic",);
                }
                i = genndx as i32 - pgenndx as i32;
                loop {
                    let fresh15 = i;
                    i = i - 1;
                    if !(fresh15 != 0) {
                        break;
                    }
                    (*pz).gen.insert(0, 0 as _);
                }
                i = modndx as i32 - pmodndx as i32;
                loop {
                    let fresh16 = i;
                    i = i - 1;
                    if !(fresh16 != 0) {
                        break;
                    }
                    (*pz).mod_0.insert(0, 0 as _);
                }
            }
            pz = *z;
            pgenndx = genndx;
            pmodndx = modndx;
        }
    }
    size -= 4 as i32;
    if size != 0 as i32 {
        return gerr!(ErrCorr, "Instrument chunk size mismatch",);
    }
    ({
        let mut _temp: u16 = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut u16 as *mut libc::c_void,
            2 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        genndx = _temp as i16 as u16;
    });
    ({
        let mut _temp: u16 = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut u16 as *mut libc::c_void,
            2 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        modndx = _temp as i16 as u16;
    });
    if pz.is_null() {
        if genndx as i32 > 0 as i32 {
            fluid_log!(
                FLUID_WARN,
                "No instrument generators and terminal index not 0",
            );
        }
        if modndx as i32 > 0 as i32 {
            fluid_log!(
                FLUID_WARN,
                "No instrument modulators and terminal index not 0",
            );
        }
        return 1 as i32;
    }
    if (genndx as i32) < pgenndx as i32 {
        return gerr!(ErrCorr, "Instrument generator indices not monotonic",);
    }
    if (modndx as i32) < pmodndx as i32 {
        return gerr!(ErrCorr, "Instrument modulator indices not monotonic",);
    }
    i = genndx as i32 - pgenndx as i32;
    loop {
        let fresh17 = i;
        i = i - 1;
        if !(fresh17 != 0) {
            break;
        }
        (*pz).gen.insert(0, 0 as _);
    }
    i = modndx as i32 - pmodndx as i32;
    loop {
        let fresh18 = i;
        i = i - 1;
        if !(fresh18 != 0) {
            break;
        }
        (*pz).mod_0.insert(0, 0 as _);
    }
    return 1 as i32;
}
unsafe fn load_imod(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    for inst in (*sf).inst.iter() {
        for zone in (**inst).zone.iter() {
            for m in (**zone).mod_0.iter_mut() {
                size -= 10 as i32;
                if size < 0 as i32 {
                    return gerr!(ErrCorr, "Instrument modulator chunk size mismatch",);
                }
                *m = libc::malloc(::std::mem::size_of::<SFMod>() as libc::size_t) as *mut SFMod;
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).src = _temp as i16 as u16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).dest = _temp as i16 as u16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).amount = _temp as i16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).amtsrc = _temp as i16 as u16;
                });
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    (**m).trans = _temp as i16 as u16;
                });
            }
        }
    }
    if size == 0 as i32 {
        return 1 as i32;
    }
    size -= 10 as i32;
    if size != 0 as i32 {
        return gerr!(ErrCorr, "Instrument modulator chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(fd, 10 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe fn load_igen(
    mut size: i32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut p3;
    let mut dup;
    let mut hz: usize;
    let mut g: *mut SFGen;
    let mut genval: SFGenAmount = SFGenAmount { sword: 0 };
    let mut genid: u16;
    let mut level: i32;
    let mut skip: i32;
    let mut drop_0: i32;
    let mut gzone: i32;
    let mut discarded: i32;
    for inst in (*sf).inst.iter() {
        gzone = 0 as i32;
        discarded = 0 as i32;
        hz = 0;
        for (i, z) in (**inst).zone.iter().enumerate() {
            level = 0 as i32;
            p3 = 0;
            while p3 < (**z).gen.len() {
                dup = None;
                skip = 0 as i32;
                drop_0 = 0 as i32;
                size -= 4 as i32;
                if size < 0 as i32 {
                    return gerr!(ErrCorr, "IGEN chunk size mismatch",);
                }
                ({
                    let mut _temp: u16 = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut u16 as *mut libc::c_void,
                        2 as i32,
                        fd,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                    genid = _temp as i16 as u16;
                });
                if genid as i32 == GEN_KEY_RANGE as i32 {
                    if level == 0 as i32 {
                        level = 1 as i32;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                    } else {
                        skip = (0 as i32 == 0) as i32
                    }
                } else if genid as i32 == GEN_VEL_RANGE as i32 {
                    if level <= 1 as i32 {
                        level = 2 as i32;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut u8 as *mut libc::c_void,
                            1 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                    } else {
                        skip = (0 as i32 == 0) as i32
                    }
                } else if genid as i32 == GEN_SAMPLE_ID as i32 {
                    level = 3 as i32;
                    ({
                        let mut _temp: u16 = 0;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut _temp as *mut u16 as *mut libc::c_void,
                            2 as i32,
                            fd,
                        ) == FLUID_FAILED as i32
                        {
                            return 0 as i32;
                        }
                        genval.uword = _temp as i16 as u16;
                    });
                    let ref mut fresh19 = (**z).instsamp;
                    *fresh19 = InstSamp::Int(genval.uword as i32 + 1 as i32);
                    break;
                } else {
                    level = 2 as i32;
                    if gen_valid(genid as i32) != 0 {
                        ({
                            let mut _temp: u16 = 0;
                            if (*fapi).fread.expect("non-null function pointer")(
                                &mut _temp as *mut u16 as *mut libc::c_void,
                                2 as i32,
                                fd,
                            ) == FLUID_FAILED as i32
                            {
                                return 0 as i32;
                            }
                            genval.sword = _temp as i16;
                        });
                        dup = (**z)
                            .gen
                            .iter()
                            .position(|x| !(*x).is_null() && (**x).id == genid);
                    } else {
                        skip = (0 as i32 == 0) as i32
                    }
                }
                if skip == 0 {
                    if dup.is_none() {
                        g = libc::malloc(::std::mem::size_of::<SFGen>() as libc::size_t)
                            as *mut SFGen;
                        (**z).gen[p3] = g;
                        (*g).id = genid
                    } else {
                        g = (**z).gen[dup.unwrap()];
                        drop_0 = (0 as i32 == 0) as i32
                    }
                    (*g).amount = genval
                } else {
                    discarded = (0 as i32 == 0) as i32;
                    drop_0 = (0 as i32 == 0) as i32;
                    if (*fapi).fseek.expect("non-null function pointer")(
                        fd,
                        2 as i32 as isize,
                        1 as i32,
                    ) == FLUID_FAILED as i32
                    {
                        return 0 as i32;
                    }
                }
                if drop_0 == 0 {
                    p3 += 1;
                } else {
                    (**z).gen.remove(p3);
                }
            }
            if level == 3 as i32 {
                {
                    (**z).gen.remove(p3);
                }
            } else if gzone == 0 {
                gzone = (0 as i32 == 0) as i32;
                if hz != i {
                    fluid_log!(
                        FLUID_WARN,
                        "Instrument \"{}\": Global zone is not first zone",
                        CStr::from_ptr((**inst).name.as_ptr() as *const i8)
                            .to_str()
                            .unwrap()
                    );
                    (**inst).zone[hz] = *z;
                    continue;
                }
            } else {
                fluid_log!(
                    FLUID_WARN,
                    "Instrument \"{}\": Discarding invalid global zone",
                    CStr::from_ptr((**inst).name.as_ptr() as *const i8)
                        .to_str()
                        .unwrap()
                );
                // TODO: can't remove item from list yet
                sfont_free_zone((**inst).zone[hz]);
                (**inst).zone[hz] = 0 as _;
            }
            while p3 < (**z).gen.len() {
                discarded = (0 as i32 == 0) as i32;
                size -= 4 as i32;
                if size < 0 as i32 {
                    return gerr!(ErrCorr, "Instrument generator chunk size mismatch",);
                }
                if (*fapi).fseek.expect("non-null function pointer")(
                    fd,
                    4 as i32 as isize,
                    1 as i32,
                ) == FLUID_FAILED as i32
                {
                    return 0 as i32;
                }
                {
                    (**z).gen.remove(p3);
                }
            }
        }
        if discarded != 0 {
            fluid_log!(
                FLUID_WARN,
                "Instrument \"{}\": Some invalid generators were discarded",
                CStr::from_ptr((**inst).name.as_ptr() as *const i8)
                    .to_str()
                    .unwrap()
            );
        }
    }
    if size == 0 as i32 {
        return 1 as i32;
    }
    size -= 4 as i32;
    if size != 0 as i32 {
        return gerr!(ErrCorr, "IGEN chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(fd, 4 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe fn load_shdr(
    mut size: u32,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut FileApi,
) -> i32 {
    let mut i: u32;
    let mut p: *mut SFSample;
    if size.wrapping_rem(46 as i32 as u32) != 0 || size == 0 as i32 as u32 {
        return gerr!(ErrCorr, "Sample header has invalid size",);
    }
    size = size
        .wrapping_div(46 as i32 as u32)
        .wrapping_sub(1 as i32 as u32);
    if size == 0 as i32 as u32 {
        fluid_log!(FLUID_WARN, "File contains no samples",);
        if (*fapi).fseek.expect("non-null function pointer")(fd, 46 as i32 as isize, 1 as i32)
            == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        return 1 as i32;
    }
    i = 0 as i32 as u32;
    while i < size {
        p = libc::malloc(::std::mem::size_of::<SFSample>() as libc::size_t) as *mut SFSample;
        (*sf).sample.push(p);
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut (*p).name as *mut [u8; 21] as *mut libc::c_void,
                20 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).name[20] = 0;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).start = _temp as i32 as u32;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).end = _temp as i32 as u32;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).loopstart = _temp as i32 as u32;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).loopend = _temp as i32 as u32;
        });
        ({
            let mut _temp: u32 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u32 as *mut libc::c_void,
                4 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).samplerate = _temp as i32 as u32;
        });
        if (*fapi).fread.expect("non-null function pointer")(
            &mut (*p).origpitch as *mut u8 as *mut libc::c_void,
            1 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        if (*fapi).fread.expect("non-null function pointer")(
            &mut (*p).pitchadj as *mut libc::c_schar as *mut libc::c_void,
            1 as i32,
            fd,
        ) == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        if (*fapi).fseek.expect("non-null function pointer")(fd, 2 as i32 as isize, 1 as i32)
            == FLUID_FAILED as i32
        {
            return 0 as i32;
        }
        ({
            let mut _temp: u16 = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut u16 as *mut libc::c_void,
                2 as i32,
                fd,
            ) == FLUID_FAILED as i32
            {
                return 0 as i32;
            }
            (*p).sampletype = _temp as i16 as u16;
        });
        (*p).samfile = 0 as i32 as u8;
        i = i.wrapping_add(1)
    }
    if (*fapi).fseek.expect("non-null function pointer")(fd, 46 as i32 as isize, 1 as i32)
        == FLUID_FAILED as i32
    {
        return 0 as i32;
    }
    return 1 as i32;
}
unsafe fn fixup_pgen(sf: *mut SFData) -> i32 {
    let mut p3;
    let mut i: i32;
    for preset in (*sf).preset.iter() {
        for z in (**preset).zone.iter() {
            if !(**z).instsamp.is_none() {
                i = (**z).instsamp.unwrap_int();
                p3 = (*sf).inst.get(i as usize - 1);
                if p3.is_none() {
                    return gerr!(
                        ErrCorr,
                        "Preset {} {}: Invalid instrument reference",
                        (**preset).bank,
                        (**preset).prenum
                    );
                }
                (**z).instsamp = InstSamp::Inst(*p3.unwrap())
            } else {
                (**z).instsamp = InstSamp::None
            }
        }
    }
    return 1 as i32;
}
unsafe fn fixup_igen(sf: *mut SFData) -> i32 {
    let mut p3;
    let mut i: i32;
    for inst in (*sf).inst.iter() {
        for z in (**inst).zone.iter() {
            if !(**z).instsamp.is_none() {
                i = (**z).instsamp.unwrap_int();
                p3 = (*sf).sample.get(i as usize - 1);
                if p3.is_none() {
                    return gerr!(
                        ErrCorr,
                        "Instrument \"{}\": Invalid sample reference",
                        CStr::from_ptr((**inst).name.as_ptr() as *const i8)
                            .to_str()
                            .unwrap()
                    );
                }
                (**z).instsamp = InstSamp::Sample(*p3.unwrap())
            }
        }
    }
    return 1 as i32;
}
unsafe fn fixup_sample(sf: *mut SFData) -> i32 {
    for sam in (*sf).sample.iter() {
        if (**sam).sampletype as i32 & 0x8000 as i32 == 0 && (**sam).end > SDTACHUNK_SIZE
            || (**sam).start > (**sam).end.wrapping_sub(4 as i32 as u32)
        {
            fluid_log!(FLUID_WARN,
                      "Sample \'{}\' start/end file positions are invalid, disabling and will not be saved", 
                      CStr::from_ptr(
                          (**sam).name.as_ptr() as *const i8
                      )
                      .to_str()
                      .unwrap());
            (**sam).loopend = 0 as i32 as u32;
            (**sam).loopstart = (**sam).loopend;
            (**sam).end = (**sam).loopstart;
            (**sam).start = (**sam).end;
            return 1 as i32;
        } else {
            if !((**sam).sampletype as i32 & 0x10 as i32 != 0) {
                if (**sam).loopend > (**sam).end
                    || (**sam).loopstart >= (**sam).loopend
                    || (**sam).loopstart <= (**sam).start
                {
                    if (**sam).end.wrapping_sub((**sam).start) >= 20 as i32 as u32 {
                        (**sam).loopstart = (**sam).start.wrapping_add(8 as i32 as u32);
                        (**sam).loopend = (**sam).end.wrapping_sub(8 as i32 as u32)
                    } else {
                        (**sam).loopstart = (**sam).start.wrapping_add(1 as i32 as u32);
                        (**sam).loopend = (**sam).end.wrapping_sub(1 as i32 as u32)
                    }
                }
            }
        }
        (**sam).end = (**sam)
            .end
            .wrapping_sub((**sam).start.wrapping_add(1 as i32 as u32));
        (**sam).loopstart = (**sam).loopstart.wrapping_sub((**sam).start);
        (**sam).loopend = (**sam).loopend.wrapping_sub((**sam).start);
    }
    return 1 as i32;
}

pub static mut BADGEN: [u16; 8] = [
    GEN_UNUSED1 as i32 as u16,
    GEN_UNUSED2 as i32 as u16,
    GEN_UNUSED3 as i32 as u16,
    GEN_UNUSED4 as i32 as u16,
    GEN_RESERVED1 as i32 as u16,
    GEN_RESERVED2 as i32 as u16,
    GEN_RESERVED3 as i32 as u16,
    0 as i32 as u16,
];

pub static mut BADPGEN: [u16; 14] = [
    GEN_START_ADDR_OFS as i32 as u16,
    GEN_END_ADDR_OFS as i32 as u16,
    GEN_START_LOOP_ADDR_OFS as i32 as u16,
    GEN_END_LOOP_ADDR_OFS as i32 as u16,
    GEN_START_ADDR_COARSE_OFS as i32 as u16,
    GEN_END_ADDR_COARSE_OFS as i32 as u16,
    GEN_START_LOOP_ADDR_COARSE_OFS as i32 as u16,
    GEN_KEYNUM as i32 as u16,
    GEN_VELOCITY as i32 as u16,
    GEN_END_LOOP_ADDR_COARSE_OFS as i32 as u16,
    GEN_SAMPLE_MODES as i32 as u16,
    GEN_EXCLUSIVE_CLASS as i32 as u16,
    GEN_OVERRIDE_ROOT_KEY as i32 as u16,
    0 as i32 as u16,
];

pub unsafe fn sfont_close(sf: *mut SFData, fapi: *mut FileApi) {
    if !(*sf).sffd.is_null() {
        (*fapi).fclose.expect("non-null function pointer")((*sf).sffd as *mut libc::c_void);
    }

    for preset in (*sf).preset.iter() {
        for z in (**preset).zone.iter() {
            sfont_free_zone(*z);
        }
        (**preset).zone.clear();
    }

    for inst in (*sf).inst.iter() {
        for z in (**inst).zone.iter() {
            sfont_free_zone(*z);
        }
        (**inst).zone.clear();
    }
    (*sf).inst.clear();

    for sample in (*sf).sample.iter() {
        libc::free(*sample as *mut libc::c_void);
    }
    (*sf).sample.clear();
}

pub unsafe fn sfont_free_zone(zone: *mut SFZone) {
    if zone.is_null() {
        return;
    }
    for gen in (*zone).gen.iter() {
        libc::free(*gen as _);
    }
    (*zone).gen.clear();

    for m in (*zone).mod_0.iter() {
        libc::free(*m as _);
    }
    (*zone).mod_0.clear();

    libc::free(zone as *mut libc::c_void);
}

pub unsafe fn sfont_preset_compare_func(a: *mut libc::c_void, b: *mut libc::c_void) -> i32 {
    let aval: i32;
    let bval: i32;
    aval =
        ((*(a as *mut SFPreset)).bank as i32) << 16 as i32 | (*(a as *mut SFPreset)).prenum as i32;
    bval =
        ((*(b as *mut SFPreset)).bank as i32) << 16 as i32 | (*(b as *mut SFPreset)).prenum as i32;
    return aval - bval;
}

pub unsafe fn gen_valid(gen: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    if gen > GEN_DUMMY as i32 - 1 as i32 {
        return 0 as i32;
    }
    while BADGEN[i as usize] as i32 != 0 && BADGEN[i as usize] as i32 != gen {
        i += 1
    }
    return (BADGEN[i as usize] as i32 == 0 as i32) as i32;
}

pub unsafe fn gen_validp(gen: i32) -> i32 {
    let mut i: i32 = 0 as i32;
    if gen_valid(gen) == 0 {
        return 0 as i32;
    }
    while BADPGEN[i as usize] as i32 != 0 && BADPGEN[i as usize] as i32 != gen as u16 as i32 {
        i += 1
    }
    return (BADPGEN[i as usize] as i32 == 0 as i32) as i32;
}
