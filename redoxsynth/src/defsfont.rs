use crate::list::CompareFn;
use crate::gen::fluid_gen_set_default_values;
use crate::gen::fluid_gen_t;
use crate::list::delete1_fluid_list;
use crate::list::delete_fluid_list;
use crate::list::fluid_list_append;
use crate::list::fluid_list_nth;
use crate::list::fluid_list_prepend;
use crate::list::fluid_list_remove;
use crate::list::fluid_list_remove_link;
use crate::list::fluid_list_sort;
use crate::list::List;
use crate::modulator::fluid_mod_delete;
use crate::modulator::fluid_mod_new;
use crate::modulator::fluid_mod_t;
use crate::modulator::fluid_mod_test_identity;
use crate::sfont::fluid_fileapi_t;
use crate::sfont::fluid_preset_t;
use crate::sfont::fluid_sample_t;
use crate::sfont::fluid_sfloader_t;
use crate::sfont::fluid_sfont_t;
use crate::synth::fluid_synth_alloc_voice;
use crate::synth::fluid_synth_start_voice;
use crate::synth::fluid_synth_t;
use crate::voice::fluid_voice_add_mod;
use crate::voice::fluid_voice_gen_incr;
use crate::voice::fluid_voice_gen_set;
use crate::voice::fluid_voice_optimize_sample;
use crate::voice::fluid_voice_t;
use std::ffi::{CStr, CString};
pub const FLUID_OK: libc::c_int = 0;
pub const FLUID_FAILED: libc::c_int = -1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_defsfont_t {
    pub filename: *mut libc::c_char,
    pub samplepos: libc::c_uint,
    pub samplesize: libc::c_uint,
    pub sampledata: *mut libc::c_short,
    pub sample: *mut List,
    pub preset: *mut fluid_defpreset_t,
    pub iter_preset: fluid_preset_t,
    pub iter_cur: *mut fluid_defpreset_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_defpreset_t {
    pub next: *mut fluid_defpreset_t,
    pub sfont: *mut fluid_defsfont_t,
    pub name: [libc::c_char; 21],
    pub bank: libc::c_uint,
    pub num: libc::c_uint,
    pub global_zone: *mut fluid_preset_zone_t,
    pub zone: *mut fluid_preset_zone_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_preset_zone_t {
    pub next: *mut fluid_preset_zone_t,
    pub name: *mut libc::c_char,
    pub inst: *mut fluid_inst_t,
    pub keylo: libc::c_int,
    pub keyhi: libc::c_int,
    pub vello: libc::c_int,
    pub velhi: libc::c_int,
    pub gen: [fluid_gen_t; 60],
    pub mod_0: *mut fluid_mod_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_inst_t {
    pub name: [libc::c_char; 21],
    pub global_zone: *mut fluid_inst_zone_t,
    pub zone: *mut fluid_inst_zone_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fluid_inst_zone_t {
    pub next: *mut fluid_inst_zone_t,
    pub name: *mut libc::c_char,
    pub sample: *mut fluid_sample_t,
    pub keylo: libc::c_int,
    pub keyhi: libc::c_int,
    pub vello: libc::c_int,
    pub velhi: libc::c_int,
    pub gen: [fluid_gen_t; 60],
    pub mod_0: *mut fluid_mod_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFData {
    pub version: SFVersion,
    pub romver: SFVersion,
    pub samplepos: libc::c_uint,
    pub samplesize: libc::c_uint,
    pub fname: *mut libc::c_char,
    pub sffd: *mut libc::FILE,
    pub info: *mut List,
    pub preset: *mut List,
    pub inst: *mut List,
    pub sample: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFVersion {
    pub major: libc::c_ushort,
    pub minor: libc::c_ushort,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFInst {
    pub name: [libc::c_char; 21],
    pub zone: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFZone {
    pub instsamp: *mut List,
    pub gen: *mut List,
    pub mod_0: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFPreset {
    pub name: [libc::c_char; 21],
    pub prenum: libc::c_ushort,
    pub bank: libc::c_ushort,
    pub libr: libc::c_uint,
    pub genre: libc::c_uint,
    pub morph: libc::c_uint,
    pub zone: *mut List,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFMod {
    pub src: libc::c_ushort,
    pub dest: libc::c_ushort,
    pub amount: libc::c_short,
    pub amtsrc: libc::c_ushort,
    pub trans: libc::c_ushort,
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
    pub name: [libc::c_char; 21],
    pub samfile: libc::c_uchar,
    pub start: libc::c_uint,
    pub end: libc::c_uint,
    pub loopstart: libc::c_uint,
    pub loopend: libc::c_uint,
    pub samplerate: libc::c_uint,
    pub origpitch: libc::c_uchar,
    pub pitchadj: libc::c_schar,
    pub sampletype: libc::c_ushort,
}
pub const GEN_SET: GenFlags = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFGen {
    pub id: libc::c_ushort,
    pub amount: SFGenAmount,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union SFGenAmount {
    pub sword: libc::c_short,
    pub uword: libc::c_ushort,
    pub range: SFGenAmountRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFGenAmountRange {
    pub lo: libc::c_uchar,
    pub hi: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SFChunk {
    pub id: libc::c_uint,
    pub size: libc::c_uint,
}
pub const SHDR_ID: libc::c_uint = 28;
pub const UNKN_ID: libc::c_uint = 0;
pub const GEN_RESERVED3: libc::c_uint = 55;
pub const GEN_RESERVED2: libc::c_uint = 49;
pub const GEN_RESERVED1: libc::c_uint = 42;
pub const GEN_UNUSED4: libc::c_uint = 20;
pub const GEN_UNUSED3: libc::c_uint = 19;
pub const GEN_UNUSED2: libc::c_uint = 18;
pub const GEN_UNUSED1: libc::c_uint = 14;
pub const GEN_DUMMY: libc::c_uint = 59;
pub const GEN_SAMPLE_ID: libc::c_uint = 53;
pub const GEN_VEL_RANGE: libc::c_uint = 44;
pub const GEN_KEY_RANGE: libc::c_uint = 43;
pub const IGEN_ID: libc::c_uint = 27;
pub const IMOD_ID: libc::c_uint = 26;
pub const IBAG_ID: libc::c_uint = 25;
pub const IHDR_ID: libc::c_uint = 24;
pub const GEN_OVERRIDE_ROOT_KEY: libc::c_uint = 58;
pub const GEN_EXCLUSIVE_CLASS: libc::c_uint = 57;
pub const GEN_SAMPLE_MODES: libc::c_uint = 54;
pub const GEN_END_LOOP_ADDR_COARSE_OFS: libc::c_uint = 50;
pub const GEN_VELOCITY: libc::c_uint = 47;
pub const GEN_KEYNUM: libc::c_uint = 46;
pub const GEN_START_LOOP_ADDR_COARSE_OFS: libc::c_uint = 45;
pub const GEN_END_ADDR_COARSE_OFS: libc::c_uint = 12;
pub const GEN_START_ADDR_COARSE_OFS: libc::c_uint = 4;
pub const GEN_END_LOOP_ADDR_OFS: libc::c_uint = 3;
pub const GEN_START_LOOP_ADDR_OFS: libc::c_uint = 2;
pub const GEN_END_ADDR_OFS: libc::c_uint = 1;
pub const GEN_START_ADDR_OFS: libc::c_uint = 0;
pub const GEN_INSTRUMENT: libc::c_uint = 41;
pub const PGEN_ID: libc::c_uint = 23;
pub const PMOD_ID: libc::c_uint = 22;
pub const PBAG_ID: libc::c_uint = 21;
pub const PHDR_ID: libc::c_uint = 20;
pub const PDTA_ID: libc::c_uint = 6;
pub const LIST_ID: libc::c_uint = 2;
pub const SMPL_ID: libc::c_uint = 19;
pub const SDTA_ID: libc::c_uint = 5;
pub const ICMT_ID: libc::c_uint = 16;
pub const IVER_ID: libc::c_uint = 11;
pub const IFIL_ID: libc::c_uint = 7;
pub const INFO_ID: libc::c_uint = 4;
pub const SFBK_ID: libc::c_uint = 3;
pub const RIFF_ID: libc::c_uint = 1;
pub const FLUID_VOICE_ADD: fluid_voice_add_mod = 1;
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
pub const FLUID_VOICE_OVERWRITE: fluid_voice_add_mod = 0;
pub type ModFlags = libc::c_uint;
pub type GenType = libc::c_uint;
pub type GenFlags = libc::c_uint;
unsafe extern "C" fn default_fopen(
    _fileapi: *mut fluid_fileapi_t,
    path: *const libc::c_char,
) -> *mut libc::c_void {
    return libc::fopen(path, b"rb\x00" as *const u8 as *const libc::c_char) as *mut libc::c_void;
}
unsafe extern "C" fn default_fclose(handle: *mut libc::c_void) -> libc::c_int {
    return libc::fclose(handle as *mut libc::FILE);
}
unsafe extern "C" fn default_ftell(handle: *mut libc::c_void) -> libc::c_long {
    return libc::ftell(handle as *mut libc::FILE);
}
unsafe extern "C" fn safe_fread(
    buf: *mut libc::c_void,
    count: libc::c_int,
    handle: *mut libc::c_void,
) -> libc::c_int {
    if libc::fread(buf, count as libc::size_t, 1, handle as *mut libc::FILE) != 1 as libc::size_t {
        if libc::feof(handle as *mut libc::FILE) != 0 {
            gerr!(
                ErrEof as libc::c_int,
                "EOF while attemping to read {} bytes",
                count
            );
        } else {
            fluid_log!(FLUID_ERR, "File read failed",);
        }
        return FLUID_FAILED as libc::c_int;
    }
    return FLUID_OK as libc::c_int;
}
unsafe extern "C" fn safe_fseek(
    handle: *mut libc::c_void,
    ofs: libc::c_long,
    whence: libc::c_int,
) -> libc::c_int {
    if libc::fseek(handle as *mut libc::FILE, ofs, whence) != 0 as libc::c_int {
        fluid_log!(
            FLUID_ERR,
            "File seek failed with offset = {} and whence = {}",
            ofs,
            whence
        );
        return FLUID_FAILED as libc::c_int;
    }
    return FLUID_OK as libc::c_int;
}
static mut DEFAULT_FILEAPI: fluid_fileapi_t = {
    fluid_fileapi_t {
        data: 0 as *const libc::c_void as *mut libc::c_void,
        free: None,
        fopen: Some(
            default_fopen
                as unsafe extern "C" fn(
                    _: *mut fluid_fileapi_t,
                    _: *const libc::c_char,
                ) -> *mut libc::c_void,
        ),
        fread: Some(
            safe_fread
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_int,
                    _: *mut libc::c_void,
                ) -> libc::c_int,
        ),
        fseek: Some(
            safe_fseek
                as unsafe extern "C" fn(
                    _: *mut libc::c_void,
                    _: libc::c_long,
                    _: libc::c_int,
                ) -> libc::c_int,
        ),
        fclose: Some(
            default_fclose as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int,
        ),
        ftell: Some(
            default_ftell as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long,
        ),
    }
};
static mut FLUID_DEFAULT_FILEAPI: *mut fluid_fileapi_t =
    unsafe { &DEFAULT_FILEAPI as *const fluid_fileapi_t as *mut fluid_fileapi_t };
#[no_mangle]
pub unsafe extern "C" fn fluid_init_default_fileapi(mut fileapi: *mut fluid_fileapi_t) {
    (*fileapi).data = 0 as *mut libc::c_void;
    (*fileapi).free = None;
    (*fileapi).fopen = Some(
        default_fopen
            as unsafe extern "C" fn(
                _: *mut fluid_fileapi_t,
                _: *const libc::c_char,
            ) -> *mut libc::c_void,
    );
    (*fileapi).fread = Some(
        safe_fread
            as unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: libc::c_int,
                _: *mut libc::c_void,
            ) -> libc::c_int,
    );
    (*fileapi).fseek = Some(
        safe_fseek
            as unsafe extern "C" fn(
                _: *mut libc::c_void,
                _: libc::c_long,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    (*fileapi).fclose =
        Some(default_fclose as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int);
    (*fileapi).ftell =
        Some(default_ftell as unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_set_default_fileapi(fileapi: *mut fluid_fileapi_t) {
    if !FLUID_DEFAULT_FILEAPI.is_null() && (*FLUID_DEFAULT_FILEAPI).free.is_some() {
        Some(
            (*FLUID_DEFAULT_FILEAPI)
                .free
                .expect("non-null function pointer"),
        )
        .expect("non-null function pointer")(FLUID_DEFAULT_FILEAPI);
    }
    FLUID_DEFAULT_FILEAPI = if fileapi.is_null() {
        &DEFAULT_FILEAPI as *const fluid_fileapi_t as *mut fluid_fileapi_t
    } else {
        fileapi
    };
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_defsfloader() -> *mut fluid_sfloader_t {
    let mut loader: *mut fluid_sfloader_t;
    loader = libc::malloc(::std::mem::size_of::<fluid_sfloader_t>() as libc::size_t)
        as *mut fluid_sfloader_t;
    if loader.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_sfloader_t;
    }
    (*loader).data = 0 as *mut libc::c_void;
    (*loader).fileapi = FLUID_DEFAULT_FILEAPI;
    (*loader).free = Some(
        delete_fluid_defsfloader as unsafe extern "C" fn(_: *mut fluid_sfloader_t) -> libc::c_int,
    );
    (*loader).load = Some(
        fluid_defsfloader_load
            as unsafe extern "C" fn(
                _: *mut fluid_sfloader_t,
                _: *const libc::c_char,
            ) -> *mut fluid_sfont_t,
    );
    return loader;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_defsfloader(
    loader: *mut fluid_sfloader_t,
) -> libc::c_int {
    if !loader.is_null() {
        libc::free(loader as *mut libc::c_void);
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfloader_load(
    loader: *mut fluid_sfloader_t,
    filename: *const libc::c_char,
) -> *mut fluid_sfont_t {
    let defsfont: *mut fluid_defsfont_t;
    let mut sfont: *mut fluid_sfont_t;
    defsfont = new_fluid_defsfont();
    if defsfont.is_null() {
        return 0 as *mut fluid_sfont_t;
    }
    sfont = if !(*loader).data.is_null() {
        (*loader).data as *mut fluid_sfont_t
    } else {
        libc::malloc(::std::mem::size_of::<fluid_sfont_t>() as libc::size_t) as *mut fluid_sfont_t
    };
    if sfont.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_sfont_t;
    }
    (*sfont).data = defsfont as *mut libc::c_void;
    (*sfont).free = Some(
        fluid_defsfont_sfont_delete as unsafe extern "C" fn(_: *mut fluid_sfont_t) -> libc::c_int,
    );
    (*sfont).get_name = Some(
        fluid_defsfont_sfont_get_name
            as unsafe extern "C" fn(_: *mut fluid_sfont_t) -> *mut libc::c_char,
    );
    (*sfont).get_preset = Some(
        fluid_defsfont_sfont_get_preset
            as unsafe extern "C" fn(
                _: *mut fluid_sfont_t,
                _: libc::c_uint,
                _: libc::c_uint,
            ) -> *mut fluid_preset_t,
    );
    (*sfont).iteration_start = Some(
        fluid_defsfont_sfont_iteration_start as unsafe extern "C" fn(_: *mut fluid_sfont_t) -> (),
    );
    (*sfont).iteration_next = Some(
        fluid_defsfont_sfont_iteration_next
            as unsafe extern "C" fn(_: *mut fluid_sfont_t, _: *mut fluid_preset_t) -> libc::c_int,
    );
    if fluid_defsfont_load(defsfont, filename, (*loader).fileapi) == FLUID_FAILED as libc::c_int {
        delete_fluid_defsfont(defsfont);
        return 0 as *mut fluid_sfont_t;
    }
    return sfont;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_sfont_delete(sfont: *mut fluid_sfont_t) -> libc::c_int {
    if delete_fluid_defsfont((*sfont).data as *mut fluid_defsfont_t) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    libc::free(sfont as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_sfont_get_name(
    sfont: *mut fluid_sfont_t,
) -> *mut libc::c_char {
    return fluid_defsfont_get_name((*sfont).data as *mut fluid_defsfont_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_sfont_get_preset(
    sfont: *mut fluid_sfont_t,
    bank: libc::c_uint,
    prenum: libc::c_uint,
) -> *mut fluid_preset_t {
    let mut preset: *mut fluid_preset_t;
    let defpreset: *mut fluid_defpreset_t;
    defpreset = fluid_defsfont_get_preset((*sfont).data as *mut fluid_defsfont_t, bank, prenum);
    if defpreset.is_null() {
        return 0 as *mut fluid_preset_t;
    }
    preset = libc::malloc(::std::mem::size_of::<fluid_preset_t>() as libc::size_t)
        as *mut fluid_preset_t;
    if preset.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_preset_t;
    }
    (*preset).sfont = sfont;
    (*preset).data = defpreset as *mut libc::c_void;
    (*preset).free = Some(
        fluid_defpreset_preset_delete
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_name = Some(
        fluid_defpreset_preset_get_name
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> *mut libc::c_char,
    );
    (*preset).get_banknum = Some(
        fluid_defpreset_preset_get_banknum
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_num = Some(
        fluid_defpreset_preset_get_num
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).noteon = Some(
        fluid_defpreset_preset_noteon
            as unsafe extern "C" fn(
                _: *mut fluid_preset_t,
                _: *mut fluid_synth_t,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    (*preset).notify = None;
    return preset;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_sfont_iteration_start(sfont: *mut fluid_sfont_t) {
    fluid_defsfont_iteration_start((*sfont).data as *mut fluid_defsfont_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_sfont_iteration_next(
    sfont: *mut fluid_sfont_t,
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    (*preset).free = Some(
        fluid_defpreset_preset_delete
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_name = Some(
        fluid_defpreset_preset_get_name
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> *mut libc::c_char,
    );
    (*preset).get_banknum = Some(
        fluid_defpreset_preset_get_banknum
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).get_num = Some(
        fluid_defpreset_preset_get_num
            as unsafe extern "C" fn(_: *mut fluid_preset_t) -> libc::c_int,
    );
    (*preset).noteon = Some(
        fluid_defpreset_preset_noteon
            as unsafe extern "C" fn(
                _: *mut fluid_preset_t,
                _: *mut fluid_synth_t,
                _: libc::c_int,
                _: libc::c_int,
                _: libc::c_int,
            ) -> libc::c_int,
    );
    (*preset).notify = None;
    return fluid_defsfont_iteration_next((*sfont).data as *mut fluid_defsfont_t, preset);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_preset_delete(
    preset: *mut fluid_preset_t,
) -> libc::c_int {
    libc::free(preset as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_preset_get_name(
    preset: *mut fluid_preset_t,
) -> *mut libc::c_char {
    return fluid_defpreset_get_name((*preset).data as *mut fluid_defpreset_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_preset_get_banknum(
    preset: *mut fluid_preset_t,
) -> libc::c_int {
    return fluid_defpreset_get_banknum((*preset).data as *mut fluid_defpreset_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_preset_get_num(
    preset: *mut fluid_preset_t,
) -> libc::c_int {
    return fluid_defpreset_get_num((*preset).data as *mut fluid_defpreset_t);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_preset_noteon(
    preset: *mut fluid_preset_t,
    synth: *mut fluid_synth_t,
    chan: libc::c_int,
    key: libc::c_int,
    vel: libc::c_int,
) -> libc::c_int {
    return fluid_defpreset_noteon(
        (*preset).data as *mut fluid_defpreset_t,
        synth,
        chan,
        key,
        vel,
    );
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_defsfont() -> *mut fluid_defsfont_t {
    let mut sfont: *mut fluid_defsfont_t;
    sfont = libc::malloc(::std::mem::size_of::<fluid_defsfont_t>() as libc::size_t)
        as *mut fluid_defsfont_t;
    if sfont.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_defsfont_t;
    }
    (*sfont).filename = 0 as *mut libc::c_char;
    (*sfont).samplepos = 0 as libc::c_int as libc::c_uint;
    (*sfont).samplesize = 0 as libc::c_int as libc::c_uint;
    (*sfont).sample = 0 as *mut List;
    (*sfont).sampledata = 0 as *mut libc::c_short;
    (*sfont).preset = 0 as *mut fluid_defpreset_t;
    return sfont;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_defsfont(mut sfont: *mut fluid_defsfont_t) -> libc::c_int {
    let mut list: *mut List;
    let mut preset: *mut fluid_defpreset_t;
    let mut sample: *mut fluid_sample_t;
    list = (*sfont).sample;
    while !list.is_null() {
        sample = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sample_t;
        if (*sample).refcount != 0 as libc::c_int as libc::c_uint {
            return -(1 as libc::c_int);
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        }
    }
    if !(*sfont).filename.is_null() {
        libc::free((*sfont).filename as *mut libc::c_void);
    }
    list = (*sfont).sample;
    while !list.is_null() {
        delete_fluid_sample(if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sample_t);
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        }
    }
    if !(*sfont).sample.is_null() {
        delete_fluid_list((*sfont).sample);
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
    libc::free(sfont as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_get_name(
    sfont: *mut fluid_defsfont_t,
) -> *mut libc::c_char {
    return (*sfont).filename;
}
#[no_mangle]
pub static mut preset_callback: Option<
    unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint, _: *mut libc::c_char) -> (),
> = None;
#[no_mangle]
pub unsafe extern "C" fn fluid_synth_set_preset_callback(callback: *mut libc::c_void) {
    preset_callback = ::std::mem::transmute::<
        *mut libc::c_void,
        Option<unsafe extern "C" fn(_: libc::c_uint, _: libc::c_uint, _: *mut libc::c_char) -> ()>,
    >(callback);
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_load(
    mut sfont: *mut fluid_defsfont_t,
    file: *const libc::c_char,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut current_block: u64;
    let sfdata: *mut SFData;
    let mut p: *mut List;
    let mut sfpreset: *mut SFPreset;
    let mut sfsample: *mut SFSample;
    let mut sample: *mut fluid_sample_t;
    let mut preset: *mut fluid_defpreset_t;
    (*sfont).filename = libc::malloc(libc::strlen(file) + 1) as *mut libc::c_char;
    if (*sfont).filename.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return FLUID_FAILED as libc::c_int;
    }
    libc::strcpy((*sfont).filename, file);
    sfdata = sfload_file(file, fapi);
    if sfdata.is_null() {
        fluid_log!(FLUID_ERR, "Couldn't load soundfont file",);
        return FLUID_FAILED as libc::c_int;
    }
    (*sfont).samplepos = (*sfdata).samplepos;
    (*sfont).samplesize = (*sfdata).samplesize;
    if !(fluid_defsfont_load_sampledata(sfont, fapi) != FLUID_OK as libc::c_int) {
        p = (*sfdata).sample;
        loop {
            if p.is_null() {
                current_block = 11194104282611034094;
                break;
            }
            sfsample = (*p).data as *mut SFSample;
            sample = new_fluid_sample();
            if sample.is_null() {
                current_block = 12140413667747225274;
                break;
            }
            if fluid_sample_import_sfont(sample, sfsample, sfont) != FLUID_OK as libc::c_int {
                current_block = 12140413667747225274;
                break;
            }
            fluid_defsfont_add_sample(sfont, sample);
            fluid_voice_optimize_sample(sample);
            p = if !p.is_null() {
                (*p).next
            } else {
                0 as *mut List
            }
        }
        match current_block {
            12140413667747225274 => {}
            _ => {
                p = (*sfdata).preset;
                loop {
                    if p.is_null() {
                        current_block = 14434620278749266018;
                        break;
                    }
                    sfpreset = (*p).data as *mut SFPreset;
                    preset = new_fluid_defpreset(sfont);
                    if preset.is_null() {
                        current_block = 12140413667747225274;
                        break;
                    }
                    if fluid_defpreset_import_sfont(preset, sfpreset, sfont)
                        != FLUID_OK as libc::c_int
                    {
                        current_block = 12140413667747225274;
                        break;
                    }
                    fluid_defsfont_add_preset(sfont, preset);
                    if preset_callback.is_some() {
                        preset_callback.expect("non-null function pointer")(
                            (*preset).bank,
                            (*preset).num,
                            (*preset).name.as_mut_ptr(),
                        );
                    }
                    p = if !p.is_null() {
                        (*p).next
                    } else {
                        0 as *mut List
                    }
                }
                match current_block {
                    12140413667747225274 => {}
                    _ => {
                        sfont_close(sfdata, fapi);
                        return FLUID_OK as libc::c_int;
                    }
                }
            }
        }
    }
    sfont_close(sfdata, fapi);
    return FLUID_FAILED as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_add_sample(
    mut sfont: *mut fluid_defsfont_t,
    sample: *mut fluid_sample_t,
) -> libc::c_int {
    (*sfont).sample = fluid_list_append((*sfont).sample, sample as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_add_preset(
    mut sfont: *mut fluid_defsfont_t,
    mut preset: *mut fluid_defpreset_t,
) -> libc::c_int {
    let mut cur: *mut fluid_defpreset_t;
    let mut prev: *mut fluid_defpreset_t;
    if (*sfont).preset.is_null() {
        (*preset).next = 0 as *mut fluid_defpreset_t;
        (*sfont).preset = preset
    } else {
        cur = (*sfont).preset;
        prev = 0 as *mut fluid_defpreset_t;
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
                return FLUID_OK as libc::c_int;
            }
            prev = cur;
            cur = (*cur).next
        }
        (*preset).next = 0 as *mut fluid_defpreset_t;
        (*prev).next = preset
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_load_sampledata(
    mut sfont: *mut fluid_defsfont_t,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let fd: *mut libc::FILE;
    let mut endian: libc::c_ushort;
    fd = (*fapi).fopen.expect("non-null function pointer")(fapi, (*sfont).filename) as *mut libc::FILE;
    if fd.is_null() {
        fluid_log!(FLUID_ERR, "Can't open soundfont file",);
        return FLUID_FAILED as libc::c_int;
    }
    if (*fapi).fseek.expect("non-null function pointer")(
        fd as *mut libc::c_void,
        (*sfont).samplepos as libc::c_long,
        0 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        libc::perror(b"error\x00" as *const u8 as *const libc::c_char);
        fluid_log!(FLUID_ERR, "Failed to seek position in data file",);
        return FLUID_FAILED as libc::c_int;
    }
    (*sfont).sampledata = libc::malloc((*sfont).samplesize as libc::size_t) as *mut libc::c_short;
    if (*sfont).sampledata.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return FLUID_FAILED as libc::c_int;
    }
    if (*fapi).fread.expect("non-null function pointer")(
        (*sfont).sampledata as *mut libc::c_void,
        (*sfont).samplesize as libc::c_int,
        fd as *mut libc::c_void,
    ) == FLUID_FAILED as libc::c_int
    {
        fluid_log!(FLUID_ERR, "Failed to read sample data",);
        return FLUID_FAILED as libc::c_int;
    }
    (*fapi).fclose.expect("non-null function pointer")(fd as *mut libc::c_void);
    endian = 0x100 as libc::c_int as libc::c_ushort;
    if *(&mut endian as *mut libc::c_ushort as *mut libc::c_char).offset(0 as libc::c_int as isize)
        != 0
    {
        let cbuf: *mut libc::c_uchar;
        let mut hi: libc::c_uchar;
        let mut lo: libc::c_uchar;
        let mut i: libc::c_uint;
        let mut j: libc::c_uint;
        let mut s: libc::c_short;
        cbuf = (*sfont).sampledata as *mut libc::c_uchar;
        i = 0 as libc::c_int as libc::c_uint;
        j = 0 as libc::c_int as libc::c_uint;
        while j < (*sfont).samplesize {
            let fresh0 = j;
            j = j.wrapping_add(1);
            lo = *cbuf.offset(fresh0 as isize);
            let fresh1 = j;
            j = j.wrapping_add(1);
            hi = *cbuf.offset(fresh1 as isize);
            s = ((hi as libc::c_int) << 8 as libc::c_int | lo as libc::c_int) as libc::c_short;
            *(*sfont).sampledata.offset(i as isize) = s;
            i = i.wrapping_add(1)
        }
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_get_sample(
    sfont: *mut fluid_defsfont_t,
    s: *mut libc::c_char,
) -> *mut fluid_sample_t {
    let mut list: *mut List;
    let mut sample: *mut fluid_sample_t;
    list = (*sfont).sample;
    while !list.is_null() {
        sample = if !list.is_null() {
            (*list).data
        } else {
            0 as *mut libc::c_void
        } as *mut fluid_sample_t;
        if libc::strcmp((*sample).name.as_mut_ptr(), s) == 0 as libc::c_int {
            return sample;
        }
        list = if !list.is_null() {
            (*list).next
        } else {
            0 as *mut List
        }
    }
    return 0 as *mut fluid_sample_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_get_preset(
    sfont: *mut fluid_defsfont_t,
    bank: libc::c_uint,
    num: libc::c_uint,
) -> *mut fluid_defpreset_t {
    let mut preset: *mut fluid_defpreset_t = (*sfont).preset;
    while !preset.is_null() {
        if (*preset).bank == bank && (*preset).num == num {
            return preset;
        }
        preset = (*preset).next
    }
    return 0 as *mut fluid_defpreset_t;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_iteration_start(mut sfont: *mut fluid_defsfont_t) {
    (*sfont).iter_cur = (*sfont).preset;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defsfont_iteration_next(
    mut sfont: *mut fluid_defsfont_t,
    mut preset: *mut fluid_preset_t,
) -> libc::c_int {
    if (*sfont).iter_cur.is_null() {
        return 0 as libc::c_int;
    }
    (*preset).data = (*sfont).iter_cur as *mut libc::c_void;
    (*sfont).iter_cur = fluid_defpreset_next((*sfont).iter_cur);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_defpreset(
    sfont: *mut fluid_defsfont_t,
) -> *mut fluid_defpreset_t {
    let mut preset: *mut fluid_defpreset_t =
        libc::malloc(::std::mem::size_of::<fluid_defpreset_t>() as libc::size_t)
            as *mut fluid_defpreset_t;
    if preset.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_defpreset_t;
    }
    (*preset).next = 0 as *mut fluid_defpreset_t;
    (*preset).sfont = sfont;
    (*preset).name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*preset).bank = 0 as libc::c_int as libc::c_uint;
    (*preset).num = 0 as libc::c_int as libc::c_uint;
    (*preset).global_zone = 0 as *mut fluid_preset_zone_t;
    (*preset).zone = 0 as *mut fluid_preset_zone_t;
    return preset;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_defpreset(mut preset: *mut fluid_defpreset_t) -> libc::c_int {
    let mut err: libc::c_int = FLUID_OK as libc::c_int;
    let mut zone: *mut fluid_preset_zone_t;
    if !(*preset).global_zone.is_null() {
        if delete_fluid_preset_zone((*preset).global_zone) != FLUID_OK as libc::c_int {
            err = FLUID_FAILED as libc::c_int
        }
        (*preset).global_zone = 0 as *mut fluid_preset_zone_t
    }
    zone = (*preset).zone;
    while !zone.is_null() {
        (*preset).zone = (*zone).next;
        if delete_fluid_preset_zone(zone) != FLUID_OK as libc::c_int {
            err = FLUID_FAILED as libc::c_int
        }
        zone = (*preset).zone
    }
    libc::free(preset as *mut libc::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_get_banknum(
    preset: *mut fluid_defpreset_t,
) -> libc::c_int {
    return (*preset).bank as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_get_num(
    preset: *mut fluid_defpreset_t,
) -> libc::c_int {
    return (*preset).num as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_get_name(
    preset: *mut fluid_defpreset_t,
) -> *mut libc::c_char {
    return (*preset).name.as_mut_ptr();
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_next(
    preset: *mut fluid_defpreset_t,
) -> *mut fluid_defpreset_t {
    return (*preset).next;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_noteon(
    preset: *mut fluid_defpreset_t,
    synth: *mut fluid_synth_t,
    chan: libc::c_int,
    key: libc::c_int,
    vel: libc::c_int,
) -> libc::c_int {
    let mut preset_zone: *mut fluid_preset_zone_t;
    let global_preset_zone: *mut fluid_preset_zone_t;
    let mut inst: *mut fluid_inst_t;
    let mut inst_zone: *mut fluid_inst_zone_t;
    let mut global_inst_zone: *mut fluid_inst_zone_t;
    let mut sample: *mut fluid_sample_t;
    let mut voice: *mut fluid_voice_t;
    let mut mod_0: *mut fluid_mod_t;
    let mut mod_list: [*mut fluid_mod_t; 64] = [0 as *mut fluid_mod_t; 64];
    let mut mod_list_count: libc::c_int;
    let mut i: libc::c_int;
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
                        voice = fluid_synth_alloc_voice(synth, sample, chan, key, vel);
                        if voice.is_null() {
                            return FLUID_FAILED as libc::c_int;
                        }
                        i = 0 as libc::c_int;
                        while i < GEN_LAST as libc::c_int {
                            if (*inst_zone).gen[i as usize].flags != 0 {
                                fluid_voice_gen_set(
                                    voice,
                                    i,
                                    (*inst_zone).gen[i as usize].val as libc::c_float,
                                );
                            } else if !global_inst_zone.is_null()
                                && (*global_inst_zone).gen[i as usize].flags as libc::c_int != 0
                            {
                                fluid_voice_gen_set(
                                    voice,
                                    i,
                                    (*global_inst_zone).gen[i as usize].val as libc::c_float,
                                );
                            }
                            i += 1
                        }
                        mod_list_count = 0 as libc::c_int;
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
                            i = 0 as libc::c_int;
                            while i < mod_list_count {
                                if !mod_list[i as usize].is_null()
                                    && fluid_mod_test_identity(mod_0, mod_list[i as usize]) != 0
                                {
                                    mod_list[i as usize] = 0 as *mut fluid_mod_t
                                }
                                i += 1
                            }
                            let fresh3 = mod_list_count;
                            mod_list_count = mod_list_count + 1;
                            mod_list[fresh3 as usize] = mod_0;
                            mod_0 = (*mod_0).next
                        }
                        i = 0 as libc::c_int;
                        while i < mod_list_count {
                            mod_0 = mod_list[i as usize];
                            if !mod_0.is_null() {
                                fluid_voice_add_mod(
                                    voice,
                                    mod_0,
                                    FLUID_VOICE_OVERWRITE as libc::c_int,
                                );
                            }
                            i += 1
                        }
                        i = 0 as libc::c_int;
                        while i < GEN_LAST as libc::c_int {
                            if i != GEN_STARTADDROFS as libc::c_int
                                && i != GEN_ENDADDROFS as libc::c_int
                                && i != GEN_STARTLOOPADDROFS as libc::c_int
                                && i != GEN_ENDLOOPADDROFS as libc::c_int
                                && i != GEN_STARTADDRCOARSEOFS as libc::c_int
                                && i != GEN_ENDADDRCOARSEOFS as libc::c_int
                                && i != GEN_STARTLOOPADDRCOARSEOFS as libc::c_int
                                && i != GEN_KEYNUM as libc::c_int
                                && i != GEN_VELOCITY as libc::c_int
                                && i != GEN_ENDLOOPADDRCOARSEOFS as libc::c_int
                                && i != GEN_SAMPLEMODE as libc::c_int
                                && i != GEN_EXCLUSIVECLASS as libc::c_int
                                && i != GEN_OVERRIDEROOTKEY as libc::c_int
                            {
                                if (*preset_zone).gen[i as usize].flags != 0 {
                                    fluid_voice_gen_incr(
                                        voice,
                                        i,
                                        (*preset_zone).gen[i as usize].val as libc::c_float,
                                    );
                                } else if !global_preset_zone.is_null()
                                    && (*global_preset_zone).gen[i as usize].flags as libc::c_int
                                        != 0
                                {
                                    fluid_voice_gen_incr(
                                        voice,
                                        i,
                                        (*global_preset_zone).gen[i as usize].val as libc::c_float,
                                    );
                                }
                            }
                            i += 1
                        }
                        mod_list_count = 0 as libc::c_int;
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
                            i = 0 as libc::c_int;
                            while i < mod_list_count {
                                if !mod_list[i as usize].is_null()
                                    && fluid_mod_test_identity(mod_0, mod_list[i as usize]) != 0
                                {
                                    mod_list[i as usize] = 0 as *mut fluid_mod_t
                                }
                                i += 1
                            }
                            let fresh5 = mod_list_count;
                            mod_list_count = mod_list_count + 1;
                            mod_list[fresh5 as usize] = mod_0;
                            mod_0 = (*mod_0).next
                        }
                        i = 0 as libc::c_int;
                        while i < mod_list_count {
                            mod_0 = mod_list[i as usize];
                            if !mod_0.is_null()
                                && (*mod_0).amount != 0 as libc::c_int as f64
                            {
                                fluid_voice_add_mod(voice, mod_0, FLUID_VOICE_ADD as libc::c_int);
                            }
                            i += 1
                        }
                        fluid_synth_start_voice(synth, voice);
                    }
                    inst_zone = fluid_inst_zone_next(inst_zone)
                }
            }
        }
        preset_zone = fluid_preset_zone_next(preset_zone)
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_set_global_zone(
    mut preset: *mut fluid_defpreset_t,
    zone: *mut fluid_preset_zone_t,
) -> libc::c_int {
    (*preset).global_zone = zone;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_import_sfont(
    mut preset: *mut fluid_defpreset_t,
    sfpreset: *mut SFPreset,
    sfont: *mut fluid_defsfont_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut sfzone: *mut SFZone;
    let mut zone: *mut fluid_preset_zone_t;
    let mut count: libc::c_int;
    let mut zone_name: [libc::c_char; 256] = [0; 256];
    if libc::strlen((*sfpreset).name.as_mut_ptr()) > 0 {
        libc::strcpy((*preset).name.as_mut_ptr(), (*sfpreset).name.as_mut_ptr());
    } else {
        libc::strcpy(
            (*preset).name.as_mut_ptr(),
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
    (*preset).bank = (*sfpreset).bank as libc::c_uint;
    (*preset).num = (*sfpreset).prenum as libc::c_uint;
    p = (*sfpreset).zone;
    count = 0 as libc::c_int;
    while !p.is_null() {
        sfzone = (*p).data as *mut SFZone;
        libc::strcpy(
            zone_name.as_mut_ptr(),
            CString::new(format!(
                "{}/{}",
                CStr::from_ptr((*preset).name.as_ptr()).to_str().unwrap(),
                count,
            ))
            .unwrap()
            .as_c_str()
            .as_ptr(),
        );
        zone = new_fluid_preset_zone(zone_name.as_mut_ptr());
        if zone.is_null() {
            return FLUID_FAILED as libc::c_int;
        }
        if fluid_preset_zone_import_sfont(zone, sfzone, sfont) != FLUID_OK as libc::c_int {
            return FLUID_FAILED as libc::c_int;
        }
        if count == 0 as libc::c_int && fluid_preset_zone_get_inst(zone).is_null() {
            fluid_defpreset_set_global_zone(preset, zone);
        } else if fluid_defpreset_add_zone(preset, zone) != FLUID_OK as libc::c_int {
            return FLUID_FAILED as libc::c_int;
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        };
        count += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_add_zone(
    mut preset: *mut fluid_defpreset_t,
    mut zone: *mut fluid_preset_zone_t,
) -> libc::c_int {
    if (*preset).zone.is_null() {
        (*zone).next = 0 as *mut fluid_preset_zone_t;
        (*preset).zone = zone
    } else {
        (*zone).next = (*preset).zone;
        (*preset).zone = zone
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_get_zone(
    preset: *mut fluid_defpreset_t,
) -> *mut fluid_preset_zone_t {
    return (*preset).zone;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_defpreset_get_global_zone(
    preset: *mut fluid_defpreset_t,
) -> *mut fluid_preset_zone_t {
    return (*preset).global_zone;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_preset_zone_next(
    preset: *mut fluid_preset_zone_t,
) -> *mut fluid_preset_zone_t {
    return (*preset).next;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_preset_zone(
    name: *mut libc::c_char,
) -> *mut fluid_preset_zone_t {
    let size: libc::size_t;
    let mut zone: *mut fluid_preset_zone_t;
    zone = libc::malloc(::std::mem::size_of::<fluid_preset_zone_t>() as libc::size_t)
        as *mut fluid_preset_zone_t;
    if zone.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_preset_zone_t;
    }
    (*zone).next = 0 as *mut fluid_preset_zone_t;
    size = libc::strlen(name) + 1;
    (*zone).name = libc::malloc(size) as *mut libc::c_char;
    if (*zone).name.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        libc::free(zone as *mut libc::c_void);
        return 0 as *mut fluid_preset_zone_t;
    }
    libc::strcpy((*zone).name, name);
    (*zone).inst = 0 as *mut fluid_inst_t;
    (*zone).keylo = 0 as libc::c_int;
    (*zone).keyhi = 128 as libc::c_int;
    (*zone).vello = 0 as libc::c_int;
    (*zone).velhi = 128 as libc::c_int;
    fluid_gen_set_default_values(&mut *(*zone).gen.as_mut_ptr().offset(0 as libc::c_int as isize));
    (*zone).mod_0 = 0 as *mut fluid_mod_t;
    return zone;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_preset_zone(
    zone: *mut fluid_preset_zone_t,
) -> libc::c_int {
    let mut mod_0: *mut fluid_mod_t;
    let mut tmp: *mut fluid_mod_t;
    mod_0 = (*zone).mod_0;
    while !mod_0.is_null() {
        tmp = mod_0;
        mod_0 = (*mod_0).next;
        fluid_mod_delete(tmp);
    }
    if !(*zone).name.is_null() {
        libc::free((*zone).name as *mut libc::c_void);
    }
    if !(*zone).inst.is_null() {
        delete_fluid_inst((*zone).inst);
    }
    libc::free(zone as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_preset_zone_import_sfont(
    mut zone: *mut fluid_preset_zone_t,
    sfzone: *mut SFZone,
    sfont: *mut fluid_defsfont_t,
) -> libc::c_int {
    let mut r: *mut List;
    let mut sfgen: *mut SFGen;
    let mut count: libc::c_int;
    count = 0 as libc::c_int;
    r = (*sfzone).gen;
    while !r.is_null() {
        sfgen = (*r).data as *mut SFGen;
        match (*sfgen).id as libc::c_int {
            43 => {
                (*zone).keylo = (*sfgen).amount.range.lo as libc::c_int;
                (*zone).keyhi = (*sfgen).amount.range.hi as libc::c_int
            }
            44 => {
                (*zone).vello = (*sfgen).amount.range.lo as libc::c_int;
                (*zone).velhi = (*sfgen).amount.range.hi as libc::c_int
            }
            _ => {
                (*zone).gen[(*sfgen).id as usize].val =
                    (*sfgen).amount.sword as f32 as f64;
                (*zone).gen[(*sfgen).id as usize].flags = GEN_SET as libc::c_int as libc::c_uchar
            }
        }
        r = if !r.is_null() {
            (*r).next
        } else {
            0 as *mut List
        };
        count += 1
    }
    if !(*sfzone).instsamp.is_null() && !(*(*sfzone).instsamp).data.is_null() {
        (*zone).inst = new_fluid_inst();
        if (*zone).inst.is_null() {
            fluid_log!(FLUID_ERR, "Out of memory",);
            return FLUID_FAILED as libc::c_int;
        }
        if fluid_inst_import_sfont(
            (*zone).inst,
            (*(*sfzone).instsamp).data as *mut SFInst,
            sfont,
        ) != FLUID_OK as libc::c_int
        {
            return FLUID_FAILED as libc::c_int;
        }
    }
    count = 0 as libc::c_int;
    r = (*sfzone).mod_0;
    while !r.is_null() {
        let mod_src: *mut SFMod = (*r).data as *mut SFMod;
        let mut mod_dest: *mut fluid_mod_t = fluid_mod_new();
        let mut type_0: libc::c_int;
        if mod_dest.is_null() {
            return FLUID_FAILED as libc::c_int;
        }
        (*mod_dest).next = 0 as *mut fluid_mod_t;
        (*mod_dest).amount = (*mod_src).amount as f64;
        (*mod_dest).src1 = ((*mod_src).src as libc::c_int & 127 as libc::c_int) as libc::c_uchar;
        (*mod_dest).flags1 = 0 as libc::c_int as libc::c_uchar;
        if (*mod_src).src as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            (*mod_dest).flags1 =
                ((*mod_dest).flags1 as libc::c_int | FLUID_MOD_CC as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).flags1 =
                ((*mod_dest).flags1 as libc::c_int | FLUID_MOD_GC as libc::c_int) as libc::c_uchar
        }
        if (*mod_src).src as libc::c_int & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_NEGATIVE as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_POSITIVE as libc::c_int)
                as libc::c_uchar
        }
        if (*mod_src).src as libc::c_int & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_BIPOLAR as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int)
                as libc::c_uchar
        }
        type_0 = (*mod_src).src as libc::c_int >> 10 as libc::c_int;
        type_0 &= 63 as libc::c_int;
        if type_0 == 0 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_LINEAR as libc::c_int) as libc::c_uchar
        } else if type_0 == 1 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_CONCAVE as libc::c_int)
                as libc::c_uchar
        } else if type_0 == 2 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_CONVEX as libc::c_int) as libc::c_uchar
        } else if type_0 == 3 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_SWITCH as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).amount = 0 as libc::c_int as f64
        }
        (*mod_dest).dest = (*mod_src).dest as libc::c_uchar;
        (*mod_dest).src2 = ((*mod_src).amtsrc as libc::c_int & 127 as libc::c_int) as libc::c_uchar;
        (*mod_dest).flags2 = 0 as libc::c_int as libc::c_uchar;
        if (*mod_src).amtsrc as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            (*mod_dest).flags2 =
                ((*mod_dest).flags2 as libc::c_int | FLUID_MOD_CC as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).flags2 =
                ((*mod_dest).flags2 as libc::c_int | FLUID_MOD_GC as libc::c_int) as libc::c_uchar
        }
        if (*mod_src).amtsrc as libc::c_int & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_NEGATIVE as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_POSITIVE as libc::c_int)
                as libc::c_uchar
        }
        if (*mod_src).amtsrc as libc::c_int & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_BIPOLAR as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int)
                as libc::c_uchar
        }
        type_0 = (*mod_src).amtsrc as libc::c_int >> 10 as libc::c_int;
        type_0 &= 63 as libc::c_int;
        if type_0 == 0 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_LINEAR as libc::c_int) as libc::c_uchar
        } else if type_0 == 1 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_CONCAVE as libc::c_int)
                as libc::c_uchar
        } else if type_0 == 2 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_CONVEX as libc::c_int) as libc::c_uchar
        } else if type_0 == 3 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_SWITCH as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).amount = 0 as libc::c_int as f64
        }
        if (*mod_src).trans as libc::c_int != 0 as libc::c_int {
            (*mod_dest).amount = 0 as libc::c_int as f64
        }
        if count == 0 as libc::c_int {
            (*zone).mod_0 = mod_dest
        } else {
            let mut last_mod: *mut fluid_mod_t = (*zone).mod_0;
            while !(*last_mod).next.is_null() {
                last_mod = (*last_mod).next
            }
            (*last_mod).next = mod_dest
        }
        r = if !r.is_null() {
            (*r).next
        } else {
            0 as *mut List
        };
        count += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_preset_zone_get_inst(
    zone: *mut fluid_preset_zone_t,
) -> *mut fluid_inst_t {
    return (*zone).inst;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_preset_zone_inside_range(
    zone: *mut fluid_preset_zone_t,
    key: libc::c_int,
    vel: libc::c_int,
) -> libc::c_int {
    return ((*zone).keylo <= key
        && (*zone).keyhi >= key
        && (*zone).vello <= vel
        && (*zone).velhi >= vel) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_inst() -> *mut fluid_inst_t {
    let mut inst: *mut fluid_inst_t =
        libc::malloc(::std::mem::size_of::<fluid_inst_t>() as libc::size_t) as *mut fluid_inst_t;
    if inst.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_inst_t;
    }
    (*inst).name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    (*inst).global_zone = 0 as *mut fluid_inst_zone_t;
    (*inst).zone = 0 as *mut fluid_inst_zone_t;
    return inst;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_inst(mut inst: *mut fluid_inst_t) -> libc::c_int {
    let mut zone: *mut fluid_inst_zone_t;
    let mut err: libc::c_int = FLUID_OK as libc::c_int;
    if !(*inst).global_zone.is_null() {
        if delete_fluid_inst_zone((*inst).global_zone) != FLUID_OK as libc::c_int {
            err = FLUID_FAILED as libc::c_int
        }
        (*inst).global_zone = 0 as *mut fluid_inst_zone_t
    }
    zone = (*inst).zone;
    while !zone.is_null() {
        (*inst).zone = (*zone).next;
        if delete_fluid_inst_zone(zone) != FLUID_OK as libc::c_int {
            err = FLUID_FAILED as libc::c_int
        }
        zone = (*inst).zone
    }
    libc::free(inst as *mut libc::c_void);
    return err;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_set_global_zone(
    mut inst: *mut fluid_inst_t,
    zone: *mut fluid_inst_zone_t,
) -> libc::c_int {
    (*inst).global_zone = zone;
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_import_sfont(
    inst: *mut fluid_inst_t,
    sfinst: *mut SFInst,
    sfont: *mut fluid_defsfont_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut sfzone: *mut SFZone;
    let mut zone: *mut fluid_inst_zone_t;
    let mut zone_name: [libc::c_char; 256] = [0; 256];
    let mut count: libc::c_int;
    p = (*sfinst).zone;
    if libc::strlen((*sfinst).name.as_mut_ptr()) > 0 {
        libc::strcpy((*inst).name.as_mut_ptr(), (*sfinst).name.as_mut_ptr());
    } else {
        libc::strcpy(
            (*inst).name.as_mut_ptr(),
            b"<untitled>\x00" as *const u8 as *const libc::c_char,
        );
    }
    count = 0 as libc::c_int;
    while !p.is_null() {
        sfzone = (*p).data as *mut SFZone;
        libc::strcpy(
            zone_name.as_mut_ptr(),
            CString::new(format!(
                "{}/{}",
                CStr::from_ptr((*inst).name.as_mut_ptr()).to_str().unwrap(),
                count,
            ))
            .unwrap()
            .as_c_str()
            .as_ptr(),
        );
        zone = new_fluid_inst_zone(zone_name.as_mut_ptr());
        if zone.is_null() {
            return FLUID_FAILED as libc::c_int;
        }
        if fluid_inst_zone_import_sfont(zone, sfzone, sfont) != FLUID_OK as libc::c_int {
            return FLUID_FAILED as libc::c_int;
        }
        if count == 0 as libc::c_int && fluid_inst_zone_get_sample(zone).is_null() {
            fluid_inst_set_global_zone(inst, zone);
        } else if fluid_inst_add_zone(inst, zone) != FLUID_OK as libc::c_int {
            return FLUID_FAILED as libc::c_int;
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        };
        count += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_add_zone(
    mut inst: *mut fluid_inst_t,
    mut zone: *mut fluid_inst_zone_t,
) -> libc::c_int {
    if (*inst).zone.is_null() {
        (*zone).next = 0 as *mut fluid_inst_zone_t;
        (*inst).zone = zone
    } else {
        (*zone).next = (*inst).zone;
        (*inst).zone = zone
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_get_zone(
    inst: *mut fluid_inst_t,
) -> *mut fluid_inst_zone_t {
    return (*inst).zone;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_get_global_zone(
    inst: *mut fluid_inst_t,
) -> *mut fluid_inst_zone_t {
    return (*inst).global_zone;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_inst_zone(
    name: *mut libc::c_char,
) -> *mut fluid_inst_zone_t {
    let size: libc::size_t;
    let mut zone: *mut fluid_inst_zone_t;
    zone = libc::malloc(::std::mem::size_of::<fluid_inst_zone_t>() as libc::size_t)
        as *mut fluid_inst_zone_t;
    if zone.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_inst_zone_t;
    }
    (*zone).next = 0 as *mut fluid_inst_zone_t;
    size = libc::strlen(name) + 1;
    (*zone).name = libc::malloc(size as libc::size_t) as *mut libc::c_char;
    if (*zone).name.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        libc::free(zone as *mut libc::c_void);
        return 0 as *mut fluid_inst_zone_t;
    }
    libc::strcpy((*zone).name, name);
    (*zone).sample = 0 as *mut fluid_sample_t;
    (*zone).keylo = 0 as libc::c_int;
    (*zone).keyhi = 128 as libc::c_int;
    (*zone).vello = 0 as libc::c_int;
    (*zone).velhi = 128 as libc::c_int;
    fluid_gen_set_default_values(&mut *(*zone).gen.as_mut_ptr().offset(0 as libc::c_int as isize));
    (*zone).mod_0 = 0 as *mut fluid_mod_t;
    return zone;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_inst_zone(zone: *mut fluid_inst_zone_t) -> libc::c_int {
    let mut mod_0: *mut fluid_mod_t;
    let mut tmp: *mut fluid_mod_t;
    mod_0 = (*zone).mod_0;
    while !mod_0.is_null() {
        tmp = mod_0;
        mod_0 = (*mod_0).next;
        fluid_mod_delete(tmp);
    }
    if !(*zone).name.is_null() {
        libc::free((*zone).name as *mut libc::c_void);
    }
    libc::free(zone as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_zone_next(
    zone: *mut fluid_inst_zone_t,
) -> *mut fluid_inst_zone_t {
    return (*zone).next;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_zone_import_sfont(
    mut zone: *mut fluid_inst_zone_t,
    sfzone: *mut SFZone,
    sfont: *mut fluid_defsfont_t,
) -> libc::c_int {
    let mut r: *mut List;
    let mut sfgen: *mut SFGen;
    let mut count: libc::c_int;
    count = 0 as libc::c_int;
    r = (*sfzone).gen;
    while !r.is_null() {
        sfgen = (*r).data as *mut SFGen;
        match (*sfgen).id as libc::c_int {
            43 => {
                (*zone).keylo = (*sfgen).amount.range.lo as libc::c_int;
                (*zone).keyhi = (*sfgen).amount.range.hi as libc::c_int
            }
            44 => {
                (*zone).vello = (*sfgen).amount.range.lo as libc::c_int;
                (*zone).velhi = (*sfgen).amount.range.hi as libc::c_int
            }
            _ => {
                (*zone).gen[(*sfgen).id as usize].val =
                    (*sfgen).amount.sword as f32 as f64;
                (*zone).gen[(*sfgen).id as usize].flags = GEN_SET as libc::c_int as libc::c_uchar
            }
        }
        r = if !r.is_null() {
            (*r).next
        } else {
            0 as *mut List
        };
        count += 1
    }
    if !(*sfzone).instsamp.is_null() && !(*(*sfzone).instsamp).data.is_null() {
        (*zone).sample = fluid_defsfont_get_sample(
            sfont,
            (*((*(*sfzone).instsamp).data as *mut SFSample))
                .name
                .as_mut_ptr(),
        );
        if (*zone).sample.is_null() {
            fluid_log!(FLUID_ERR, "Couldn't find sample name",);
            return FLUID_FAILED as libc::c_int;
        }
    }
    count = 0 as libc::c_int;
    r = (*sfzone).mod_0;
    while !r.is_null() {
        let mod_src: *mut SFMod = (*r).data as *mut SFMod;
        let mut type_0: libc::c_int;
        let mut mod_dest: *mut fluid_mod_t;
        mod_dest = fluid_mod_new();
        if mod_dest.is_null() {
            return FLUID_FAILED as libc::c_int;
        }
        (*mod_dest).next = 0 as *mut fluid_mod_t;
        (*mod_dest).amount = (*mod_src).amount as f64;
        (*mod_dest).src1 = ((*mod_src).src as libc::c_int & 127 as libc::c_int) as libc::c_uchar;
        (*mod_dest).flags1 = 0 as libc::c_int as libc::c_uchar;
        if (*mod_src).src as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            (*mod_dest).flags1 =
                ((*mod_dest).flags1 as libc::c_int | FLUID_MOD_CC as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).flags1 =
                ((*mod_dest).flags1 as libc::c_int | FLUID_MOD_GC as libc::c_int) as libc::c_uchar
        }
        if (*mod_src).src as libc::c_int & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_NEGATIVE as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_POSITIVE as libc::c_int)
                as libc::c_uchar
        }
        if (*mod_src).src as libc::c_int & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_BIPOLAR as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int)
                as libc::c_uchar
        }
        type_0 = (*mod_src).src as libc::c_int >> 10 as libc::c_int;
        type_0 &= 63 as libc::c_int;
        if type_0 == 0 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_LINEAR as libc::c_int) as libc::c_uchar
        } else if type_0 == 1 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_CONCAVE as libc::c_int)
                as libc::c_uchar
        } else if type_0 == 2 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_CONVEX as libc::c_int) as libc::c_uchar
        } else if type_0 == 3 as libc::c_int {
            (*mod_dest).flags1 = ((*mod_dest).flags1 as libc::c_int
                | FLUID_MOD_SWITCH as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).amount = 0 as libc::c_int as f64
        }
        (*mod_dest).dest = (*mod_src).dest as libc::c_uchar;
        (*mod_dest).src2 = ((*mod_src).amtsrc as libc::c_int & 127 as libc::c_int) as libc::c_uchar;
        (*mod_dest).flags2 = 0 as libc::c_int as libc::c_uchar;
        if (*mod_src).amtsrc as libc::c_int & (1 as libc::c_int) << 7 as libc::c_int != 0 {
            (*mod_dest).flags2 =
                ((*mod_dest).flags2 as libc::c_int | FLUID_MOD_CC as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).flags2 =
                ((*mod_dest).flags2 as libc::c_int | FLUID_MOD_GC as libc::c_int) as libc::c_uchar
        }
        if (*mod_src).amtsrc as libc::c_int & (1 as libc::c_int) << 8 as libc::c_int != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_NEGATIVE as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_POSITIVE as libc::c_int)
                as libc::c_uchar
        }
        if (*mod_src).amtsrc as libc::c_int & (1 as libc::c_int) << 9 as libc::c_int != 0 {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_BIPOLAR as libc::c_int)
                as libc::c_uchar
        } else {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_UNIPOLAR as libc::c_int)
                as libc::c_uchar
        }
        type_0 = (*mod_src).amtsrc as libc::c_int >> 10 as libc::c_int;
        type_0 &= 63 as libc::c_int;
        if type_0 == 0 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_LINEAR as libc::c_int) as libc::c_uchar
        } else if type_0 == 1 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_CONCAVE as libc::c_int)
                as libc::c_uchar
        } else if type_0 == 2 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_CONVEX as libc::c_int) as libc::c_uchar
        } else if type_0 == 3 as libc::c_int {
            (*mod_dest).flags2 = ((*mod_dest).flags2 as libc::c_int
                | FLUID_MOD_SWITCH as libc::c_int) as libc::c_uchar
        } else {
            (*mod_dest).amount = 0 as libc::c_int as f64
        }
        if (*mod_src).trans as libc::c_int != 0 as libc::c_int {
            (*mod_dest).amount = 0 as libc::c_int as f64
        }
        if count == 0 as libc::c_int {
            (*zone).mod_0 = mod_dest
        } else {
            let mut last_mod: *mut fluid_mod_t = (*zone).mod_0;
            while !(*last_mod).next.is_null() {
                last_mod = (*last_mod).next
            }
            (*last_mod).next = mod_dest
        }
        r = if !r.is_null() {
            (*r).next
        } else {
            0 as *mut List
        };
        count += 1
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_zone_get_sample(
    zone: *mut fluid_inst_zone_t,
) -> *mut fluid_sample_t {
    return (*zone).sample;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_inst_zone_inside_range(
    zone: *mut fluid_inst_zone_t,
    key: libc::c_int,
    vel: libc::c_int,
) -> libc::c_int {
    return ((*zone).keylo <= key
        && (*zone).keyhi >= key
        && (*zone).vello <= vel
        && (*zone).velhi >= vel) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn new_fluid_sample() -> *mut fluid_sample_t {
    let mut sample: *mut fluid_sample_t;
    sample = libc::malloc(::std::mem::size_of::<fluid_sample_t>() as libc::size_t)
        as *mut fluid_sample_t;
    if sample.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        return 0 as *mut fluid_sample_t;
    }
    libc::memset(
        sample as *mut libc::c_void,
        0 as libc::c_int,
        ::std::mem::size_of::<fluid_sample_t>() as libc::size_t,
    );
    (*sample).valid = 1 as libc::c_int;
    return sample;
}
#[no_mangle]
pub unsafe extern "C" fn delete_fluid_sample(sample: *mut fluid_sample_t) -> libc::c_int {
    libc::free(sample as *mut libc::c_void);
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_sample_in_rom(sample: *mut fluid_sample_t) -> libc::c_int {
    return (*sample).sampletype & 0x8000 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fluid_sample_import_sfont(
    mut sample: *mut fluid_sample_t,
    sfsample: *mut SFSample,
    sfont: *mut fluid_defsfont_t,
) -> libc::c_int {
    libc::strcpy((*sample).name.as_mut_ptr(), (*sfsample).name.as_mut_ptr());
    (*sample).data = (*sfont).sampledata;
    (*sample).start = (*sfsample).start;
    (*sample).end = (*sfsample).start.wrapping_add((*sfsample).end);
    (*sample).loopstart = (*sfsample).start.wrapping_add((*sfsample).loopstart);
    (*sample).loopend = (*sfsample).start.wrapping_add((*sfsample).loopend);
    (*sample).samplerate = (*sfsample).samplerate;
    (*sample).origpitch = (*sfsample).origpitch as libc::c_int;
    (*sample).pitchadj = (*sfsample).pitchadj as libc::c_int;
    (*sample).sampletype = (*sfsample).sampletype as libc::c_int;
    if ((*sample).sampletype & 0x10 as libc::c_int) != 0 {
        // vorbis?
        return FLUID_OK;
    }
    if (*sample).sampletype & 0x8000 as libc::c_int != 0 {
        (*sample).valid = 0 as libc::c_int;
        fluid_log!(
            FLUID_WARN,
            "Ignoring sample: can\'t use ROM samples",
            //(*sample).name
        );
    }
    if (*sample).end.wrapping_sub((*sample).start) < 8 as libc::c_int as libc::c_uint {
        (*sample).valid = 0 as libc::c_int;
        fluid_log!(
            FLUID_WARN,
            "Ignoring sample: too few sample data points",
            //(*sample).name
        );
    }
    return FLUID_OK as libc::c_int;
}
#[no_mangle]
pub static idlist: &[u8; 113] =
    b"RIFFLISTsfbkINFOsdtapdtaifilisngINAMiromiverICRDIENGIPRDICOPICMTISFTsnamsmplphdrpbagpmodpgeninstibagimodigenshdr\x00";
static mut SDTACHUNK_SIZE: libc::c_uint = 0;
unsafe extern "C" fn chunkid(id: libc::c_uint) -> libc::c_int {
    let mut i: libc::c_uint;
    let mut p: *const libc::c_uint;
    p = idlist as *const [u8; 113] as *const libc::c_uint;
    i = 0 as libc::c_int as libc::c_uint;
    while (i as libc::c_ulong)
        < (::std::mem::size_of::<[libc::c_char; 113]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<libc::c_int>() as libc::c_ulong)
    {
        if *p == id {
            return i.wrapping_add(1 as libc::c_int as libc::c_uint) as libc::c_int;
        }
        i = i.wrapping_add(1);
        p = p.offset(1 as libc::c_int as isize)
    }
    return UNKN_ID as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn sfload_file(
    fname: *const libc::c_char,
    fapi: *mut fluid_fileapi_t,
) -> *mut SFData {
    let mut sf: *mut SFData;
    let fd: *mut libc::c_void;
    let mut fsize: libc::c_int = 0 as libc::c_int;
    let mut err: libc::c_int = 0 as libc::c_int;
    fd = (*fapi).fopen.expect("non-null function pointer")(fapi, fname);
    if fd.is_null() {
        fluid_log!(
            FLUID_ERR,
            "Unable to open file \"{}\"",
            CStr::from_ptr(fname).to_str().unwrap()
        );
        return 0 as *mut SFData;
    }
    sf = libc::malloc(::std::mem::size_of::<SFData>() as libc::size_t) as *mut SFData;
    if sf.is_null() {
        fluid_log!(FLUID_ERR, "Out of memory",);
        err = (0 as libc::c_int == 0) as libc::c_int
    }
    if err == 0 {
        libc::memset(
            sf as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<SFData>() as libc::size_t,
        );
        (*sf).fname = libc::strcpy(
            libc::malloc(libc::strlen(fname) + 1) as *mut libc::c_char,
            fname,
        );
        (*sf).sffd = fd as *mut libc::FILE
    }
    if err == 0
        && (*fapi).fseek.expect("non-null function pointer")(
            fd,
            0 as libc::c_long,
            2 as libc::c_int,
        ) == FLUID_FAILED as libc::c_int
    {
        err = (0 as libc::c_int == 0) as libc::c_int;
        fluid_log!(FLUID_ERR, "Seek to end of file failed",);
    }
    if err == 0 && {
        fsize = (*fapi).ftell.expect("non-null function pointer")(fd) as libc::c_int;
        (fsize) == FLUID_FAILED as libc::c_int
    } {
        err = (0 as libc::c_int == 0) as libc::c_int;
        fluid_log!(FLUID_ERR, "Get end of file position failed",);
    }
    if err == 0 {
        (*fapi).fseek.expect("non-null function pointer")(
            fd,
            0 as libc::c_int as libc::c_long,
            0 as libc::c_int,
        );
    }
    if err == 0 && load_body(fsize as libc::c_uint, sf, fd, fapi) == 0 {
        err = (0 as libc::c_int == 0) as libc::c_int
    }
    if err != 0 {
        if !sf.is_null() {
            sfont_close(sf, fapi);
        }
        return 0 as *mut SFData;
    }
    return sf;
}
unsafe extern "C" fn load_body(
    size: libc::c_uint,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            &mut chunk as *mut SFChunk as *mut libc::c_void,
            8 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        (*(&mut chunk as *mut SFChunk)).size = (*(&mut chunk as *mut SFChunk)).size;
    });
    if chunkid(chunk.id) != RIFF_ID as libc::c_int {
        fluid_log!(FLUID_ERR, "Not a RIFF file",);
        return 0 as libc::c_int;
    }
    if (*fapi).fread.expect("non-null function pointer")(
        &mut chunk.id as *mut libc::c_uint as *mut libc::c_void,
        4 as libc::c_int,
        fd,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if chunkid(chunk.id) != SFBK_ID as libc::c_int {
        fluid_log!(FLUID_ERR, "Not a sound font file",);
        return 0 as libc::c_int;
    }
    if chunk.size != size.wrapping_sub(8 as libc::c_int as libc::c_uint) {
        gerr!(ErrCorr, "Sound font file size mismatch",);
        return 0 as libc::c_int;
    }
    if read_listchunk(&mut chunk, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if chunkid(chunk.id) != INFO_ID as libc::c_int {
        return gerr!(ErrCorr, "Invalid ID found when expecting INFO chunk",);
    }
    if process_info(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if read_listchunk(&mut chunk, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if chunkid(chunk.id) != SDTA_ID as libc::c_int {
        return gerr!(ErrCorr, "Invalid ID found when expecting SAMPLE chunk",);
    }
    if process_sdta(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if read_listchunk(&mut chunk, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if chunkid(chunk.id) != PDTA_ID as libc::c_int {
        return gerr!(ErrCorr, "Invalid ID found when expecting HYDRA chunk",);
    }
    if process_pdta(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if fixup_pgen(sf) == 0 {
        return 0 as libc::c_int;
    }
    if fixup_igen(sf) == 0 {
        return 0 as libc::c_int;
    }
    if fixup_sample(sf) == 0 {
        return 0 as libc::c_int;
    }
    (*sf).preset = fluid_list_sort(
        (*sf).preset,
        ::std::mem::transmute::<
            Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_int>,
            CompareFn,
        >(Some(
            sfont_preset_compare_func
                as unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> libc::c_int,
        )),
    );
    return 1 as libc::c_int;
}
unsafe extern "C" fn read_listchunk(
    mut chunk: *mut SFChunk,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            chunk as *mut libc::c_void,
            8 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        (*chunk).size = (*chunk).size;
    });
    if chunkid((*chunk).id) != LIST_ID as libc::c_int {
        return gerr!(ErrCorr, "Invalid chunk id in level 0 parse",);
    }
    if (*fapi).fread.expect("non-null function pointer")(
        &mut (*chunk).id as *mut libc::c_uint as *mut libc::c_void,
        4 as libc::c_int,
        fd,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    (*chunk).size = (*chunk).size.wrapping_sub(4 as libc::c_int as libc::c_uint);
    return 1 as libc::c_int;
}
unsafe extern "C" fn process_info(
    mut size: libc::c_int,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    let mut id: libc::c_uchar;
    let mut item: *mut libc::c_char;
    let mut ver: libc::c_ushort;
    while size > 0 as libc::c_int {
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut chunk as *mut SFChunk as *mut libc::c_void,
                8 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*(&mut chunk as *mut SFChunk)).size = (*(&mut chunk as *mut SFChunk)).size;
        });
        size -= 8 as libc::c_int;
        id = chunkid(chunk.id) as libc::c_uchar;
        if id as libc::c_int == IFIL_ID as libc::c_int {
            if chunk.size != 4 as libc::c_int as libc::c_uint {
                return gerr!(ErrCorr, "Sound font version info chunk has invalid size",);
            }
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                ver = _temp as libc::c_short as libc::c_ushort;
            });
            (*sf).version.major = ver;
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                ver = _temp as libc::c_short as libc::c_ushort;
            });
            (*sf).version.minor = ver;
            if ((*sf).version.major as libc::c_int) < 2 as libc::c_int {
                fluid_log!(
                    FLUID_ERR,
                    "Sound font version is {}.{} which is not supported, convert to version 2.0x",
                    (*sf).version.major,
                    (*sf).version.minor
                );
                return 0 as libc::c_int;
            }
            if (*sf).version.major as libc::c_int > 2 as libc::c_int {
                fluid_log!(FLUID_WARN,
                          "Sound font version is {}.{} which is newer than what this version of FLUID Synth was designed for (v2.0x)",
                          (*sf).version.major,
                          (*sf).version.minor);
                return 0 as libc::c_int;
            }
        } else if id as libc::c_int == IVER_ID as libc::c_int {
            if chunk.size != 4 as libc::c_int as libc::c_uint {
                return gerr!(ErrCorr, "ROM version info chunk has invalid size",);
            }
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                ver = _temp as libc::c_short as libc::c_ushort;
            });
            (*sf).romver.major = ver;
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                ver = _temp as libc::c_short as libc::c_ushort;
            });
            (*sf).romver.minor = ver
        } else if id as libc::c_int != UNKN_ID as libc::c_int {
            if id as libc::c_int != ICMT_ID as libc::c_int
                && chunk.size > 256 as libc::c_int as libc::c_uint
                || chunk.size > 65536 as libc::c_int as libc::c_uint
                || chunk.size.wrapping_rem(2 as libc::c_int as libc::c_uint) != 0
            {
                return gerr!(
                    ErrCorr,
                    "INFO sub chunk {} has invalid chunk size of {} bytes",
                    chunk.id,
                    chunk.size
                );
            }
            item = libc::malloc(chunk.size.wrapping_add(1) as libc::size_t) as *mut libc::c_char;
            if item.is_null() {
                fluid_log!(FLUID_ERR, "Out of memory",);
                return 0 as libc::c_int;
            }
            (*sf).info = fluid_list_append((*sf).info, item as *mut libc::c_void);
            *(item as *mut libc::c_uchar) = id;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut *item.offset(1 as libc::c_int as isize) as *mut libc::c_char
                    as *mut libc::c_void,
                chunk.size as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            *item.offset(chunk.size as isize) = '\u{0}' as i32 as libc::c_char
        } else {
            return gerr!(ErrCorr, "Invalid chunk id in INFO chunk",);
        }
        size = (size as libc::c_uint).wrapping_sub(chunk.size) as libc::c_int as libc::c_int
    }
    if size < 0 as libc::c_int {
        return gerr!(ErrCorr, "INFO chunk size mismatch",);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn process_sdta(
    mut size: libc::c_int,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    if size == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            &mut chunk as *mut SFChunk as *mut libc::c_void,
            8 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        (*(&mut chunk as *mut SFChunk)).size = (*(&mut chunk as *mut SFChunk)).size;
    });
    size -= 8 as libc::c_int;
    if chunkid(chunk.id) != SMPL_ID as libc::c_int {
        return gerr!(ErrCorr, "Expected SMPL chunk found invalid id instead",);
    }
    if (size as libc::c_uint).wrapping_sub(chunk.size) != 0 as libc::c_int as libc::c_uint {
        return gerr!(ErrCorr, "SDTA chunk size mismatch",);
    }
    (*sf).samplepos = (*fapi).ftell.expect("non-null function pointer")(fd) as libc::c_uint;
    SDTACHUNK_SIZE = chunk.size;
    (*sf).samplesize = chunk.size;
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        chunk.size as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn pdtahelper(
    expid: libc::c_uint,
    reclen: libc::c_uint,
    mut chunk: *mut SFChunk,
    size: *mut libc::c_int,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let id: libc::c_uint;
    let expstr: *mut libc::c_char;
    expstr = idlist.as_ptr().offset(
        expid
            .wrapping_sub(1 as libc::c_int as libc::c_uint)
            .wrapping_mul(4 as libc::c_int as libc::c_uint) as isize,
    ) as *mut libc::c_char;
    ({
        if (*fapi).fread.expect("non-null function pointer")(
            chunk as *mut libc::c_void,
            8 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        (*chunk).size = (*chunk).size;
    });
    *size -= 8 as libc::c_int;
    id = chunkid((*chunk).id) as libc::c_uint;
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
    *size = (*size as libc::c_uint).wrapping_sub((*chunk).size) as libc::c_int as libc::c_int;
    if *size < 0 as libc::c_int {
        return gerr!(
            ErrCorr,
            "\"{}\" chunk size exceeds remaining PDTA chunk size",
            CStr::from_ptr(expstr).to_str().unwrap()
        );
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn process_pdta(
    mut size: libc::c_int,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut chunk: SFChunk = SFChunk { id: 0, size: 0 };
    if pdtahelper(
        PHDR_ID as libc::c_int as libc::c_uint,
        38 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_phdr(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        PBAG_ID as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_pbag(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        PMOD_ID as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_pmod(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        PGEN_ID as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_pgen(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        IHDR_ID as libc::c_int as libc::c_uint,
        22 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_ihdr(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        IBAG_ID as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_ibag(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        IMOD_ID as libc::c_int as libc::c_uint,
        10 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_imod(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        IGEN_ID as libc::c_int as libc::c_uint,
        4 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_igen(chunk.size as libc::c_int, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    if pdtahelper(
        SHDR_ID as libc::c_int as libc::c_uint,
        46 as libc::c_int as libc::c_uint,
        &mut chunk,
        &mut size,
        fd,
        fapi,
    ) == 0
    {
        return 0 as libc::c_int;
    }
    if load_shdr(chunk.size, sf, fd, fapi) == 0 {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_phdr(
    size: libc::c_int,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut i: libc::c_int;
    let mut i2: libc::c_int;
    let mut p: *mut SFPreset;
    let mut pr: *mut SFPreset = 0 as *mut SFPreset;
    let mut zndx: libc::c_ushort;
    let mut pzndx: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    if size % 38 as libc::c_int != 0 || size == 0 as libc::c_int {
        return gerr!(ErrCorr, "Preset header chunk size is invalid",);
    }
    i = size / 38 as libc::c_int - 1 as libc::c_int;
    if i == 0 as libc::c_int {
        fluid_log!(FLUID_WARN, "File contains no presets",);
        if (*fapi).fseek.expect("non-null function pointer")(
            fd,
            38 as libc::c_int as libc::c_long,
            1 as libc::c_int,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    while i > 0 as libc::c_int {
        p = libc::malloc(::std::mem::size_of::<SFPreset>() as libc::size_t) as *mut SFPreset;
        (*sf).preset = fluid_list_append((*sf).preset, p as *mut libc::c_void);
        (*p).zone = 0 as *mut List;
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut (*p).name as *mut [libc::c_char; 21] as *mut libc::c_void,
                20 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).name[20 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        });
        ({
            let mut _temp: libc::c_ushort = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                2 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).prenum = _temp as libc::c_short as libc::c_ushort;
        });
        ({
            let mut _temp: libc::c_ushort = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                2 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).bank = _temp as libc::c_short as libc::c_ushort;
        });
        ({
            let mut _temp: libc::c_ushort = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                2 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            zndx = _temp as libc::c_short as libc::c_ushort;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).libr = _temp as libc::c_int as libc::c_uint;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).genre = _temp as libc::c_int as libc::c_uint;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).morph = _temp as libc::c_int as libc::c_uint;
        });
        if !pr.is_null() {
            if (zndx as libc::c_int) < pzndx as libc::c_int {
                return gerr!(ErrCorr, "Preset header indices not monotonic",);
            }
            i2 = zndx as libc::c_int - pzndx as libc::c_int;
            loop {
                let fresh6 = i2;
                i2 = i2 - 1;
                if !(fresh6 != 0) {
                    break;
                }
                (*pr).zone = fluid_list_prepend((*pr).zone, 0 as *mut libc::c_void)
            }
        } else if zndx as libc::c_int > 0 as libc::c_int {
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
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        24 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    ({
        let mut _temp: libc::c_ushort = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
            2 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        zndx = _temp as libc::c_short as libc::c_ushort;
    });
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        12 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (zndx as libc::c_int) < pzndx as libc::c_int {
        return gerr!(ErrCorr, "Preset header indices not monotonic",);
    }
    i2 = zndx as libc::c_int - pzndx as libc::c_int;
    loop {
        let fresh7 = i2;
        i2 = i2 - 1;
        if !(fresh7 != 0) {
            break;
        }
        (*pr).zone = fluid_list_prepend((*pr).zone, 0 as *mut libc::c_void)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_pbag(
    mut size: libc::c_int,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut z: *mut SFZone;
    let mut pz: *mut SFZone = 0 as *mut SFZone;
    let mut genndx: libc::c_ushort;
    let mut modndx: libc::c_ushort;
    let mut pgenndx: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut pmodndx: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut i: libc::c_ushort;
    if size % 4 as libc::c_int != 0 || size == 0 as libc::c_int {
        return gerr!(ErrCorr, "Preset bag chunk size is invalid",);
    }
    p = (*sf).preset;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFPreset)).zone;
        while !p2.is_null() {
            size -= 4 as libc::c_int;
            if size < 0 as libc::c_int {
                return gerr!(ErrCorr, "Preset bag chunk size mismatch",);
            }
            z = libc::malloc(::std::mem::size_of::<SFZone>() as libc::size_t) as *mut SFZone;
            (*p2).data = z as *mut libc::c_void;
            (*z).gen = 0 as *mut List;
            (*z).mod_0 = 0 as *mut List;
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                genndx = _temp as libc::c_short as libc::c_ushort;
            });
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                modndx = _temp as libc::c_short as libc::c_ushort;
            });
            (*z).instsamp = 0 as *mut List;
            if !pz.is_null() {
                if (genndx as libc::c_int) < pgenndx as libc::c_int {
                    return gerr!(ErrCorr, "Preset bag generator indices not monotonic",);
                }
                if (modndx as libc::c_int) < pmodndx as libc::c_int {
                    return gerr!(ErrCorr, "Preset bag modulator indices not monotonic",);
                }
                i = (genndx as libc::c_int - pgenndx as libc::c_int) as libc::c_ushort;
                loop {
                    let fresh8 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh8 != 0) {
                        break;
                    }
                    (*pz).gen = fluid_list_prepend((*pz).gen, 0 as *mut libc::c_void)
                }
                i = (modndx as libc::c_int - pmodndx as libc::c_int) as libc::c_ushort;
                loop {
                    let fresh9 = i;
                    i = i.wrapping_sub(1);
                    if !(fresh9 != 0) {
                        break;
                    }
                    (*pz).mod_0 = fluid_list_prepend((*pz).mod_0, 0 as *mut libc::c_void)
                }
            }
            pz = z;
            pgenndx = genndx;
            pmodndx = modndx;
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    size -= 4 as libc::c_int;
    if size != 0 as libc::c_int {
        return gerr!(ErrCorr, "Preset bag chunk size mismatch",);
    }
    ({
        let mut _temp: libc::c_ushort = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
            2 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        genndx = _temp as libc::c_short as libc::c_ushort;
    });
    ({
        let mut _temp: libc::c_ushort = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
            2 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        modndx = _temp as libc::c_short as libc::c_ushort;
    });
    if pz.is_null() {
        if genndx as libc::c_int > 0 as libc::c_int {
            fluid_log!(FLUID_WARN, "No preset generators and terminal index not 0",);
        }
        if modndx as libc::c_int > 0 as libc::c_int {
            fluid_log!(FLUID_WARN, "No preset modulators and terminal index not 0",);
        }
        return 1 as libc::c_int;
    }
    if (genndx as libc::c_int) < pgenndx as libc::c_int {
        return gerr!(ErrCorr, "Preset bag generator indices not monotonic",);
    }
    if (modndx as libc::c_int) < pmodndx as libc::c_int {
        return gerr!(ErrCorr, "Preset bag modulator indices not monotonic",);
    }
    i = (genndx as libc::c_int - pgenndx as libc::c_int) as libc::c_ushort;
    loop {
        let fresh10 = i;
        i = i.wrapping_sub(1);
        if !(fresh10 != 0) {
            break;
        }
        (*pz).gen = fluid_list_prepend((*pz).gen, 0 as *mut libc::c_void)
    }
    i = (modndx as libc::c_int - pmodndx as libc::c_int) as libc::c_ushort;
    loop {
        let fresh11 = i;
        i = i.wrapping_sub(1);
        if !(fresh11 != 0) {
            break;
        }
        (*pz).mod_0 = fluid_list_prepend((*pz).mod_0, 0 as *mut libc::c_void)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_pmod(
    mut size: libc::c_int,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut p3: *mut List;
    let mut m: *mut SFMod;
    p = (*sf).preset;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFPreset)).zone;
        while !p2.is_null() {
            p3 = (*((*p2).data as *mut SFZone)).mod_0;
            while !p3.is_null() {
                size -= 10 as libc::c_int;
                if size < 0 as libc::c_int {
                    return gerr!(ErrCorr, "Preset modulator chunk size mismatch",);
                }
                m = libc::malloc(::std::mem::size_of::<SFMod>() as libc::size_t) as *mut SFMod;
                (*p3).data = m as *mut libc::c_void;
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).src = _temp as libc::c_short as libc::c_ushort;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).dest = _temp as libc::c_short as libc::c_ushort;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).amount = _temp as libc::c_short;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).amtsrc = _temp as libc::c_short as libc::c_ushort;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).trans = _temp as libc::c_short as libc::c_ushort;
                });
                p3 = if !p3.is_null() {
                    (*p3).next
                } else {
                    0 as *mut List
                }
            }
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    if size == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    size -= 10 as libc::c_int;
    if size != 0 as libc::c_int {
        return gerr!(ErrCorr, "Preset modulator chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        10 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_pgen(
    mut size: libc::c_int,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut p3: *mut List;
    let mut dup: *mut List;
    let mut hz: *mut *mut List = 0 as *mut *mut List;
    let mut z: *mut SFZone;
    let mut g: *mut SFGen;
    let mut genval: SFGenAmount = SFGenAmount { sword: 0 };
    let mut genid: libc::c_ushort;
    let mut level: libc::c_int;
    let mut skip: libc::c_int;
    let mut drop_0: libc::c_int;
    let mut gzone: libc::c_int;
    let mut discarded: libc::c_int;
    p = (*sf).preset;
    while !p.is_null() {
        gzone = 0 as libc::c_int;
        discarded = 0 as libc::c_int;
        p2 = (*((*p).data as *mut SFPreset)).zone;
        if !p2.is_null() {
            hz = &mut p2
        }
        while !p2.is_null() {
            level = 0 as libc::c_int;
            z = (*p2).data as *mut SFZone;
            p3 = (*z).gen;
            while !p3.is_null() {
                dup = 0 as *mut List;
                skip = 0 as libc::c_int;
                drop_0 = 0 as libc::c_int;
                size -= 4 as libc::c_int;
                if size < 0 as libc::c_int {
                    return gerr!(ErrCorr, "Preset generator chunk size mismatch",);
                }
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    genid = _temp as libc::c_short as libc::c_ushort;
                });
                if genid as libc::c_int == GEN_KEY_RANGE as libc::c_int {
                    if level == 0 as libc::c_int {
                        level = 1 as libc::c_int;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                    } else {
                        skip = (0 as libc::c_int == 0) as libc::c_int
                    }
                } else if genid as libc::c_int == GEN_VEL_RANGE as libc::c_int {
                    if level <= 1 as libc::c_int {
                        level = 2 as libc::c_int;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                    } else {
                        skip = (0 as libc::c_int == 0) as libc::c_int
                    }
                } else if genid as libc::c_int == GEN_INSTRUMENT as libc::c_int {
                    level = 3 as libc::c_int;
                    ({
                        let mut _temp: libc::c_ushort = 0;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                            2 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                        genval.uword = _temp as libc::c_short as libc::c_ushort;
                    });
                    let ref mut fresh12 = (*((*p2).data as *mut SFZone)).instsamp;
                    *fresh12 = (genval.uword as libc::c_int + 1 as libc::c_int) as libc::c_long
                        as *mut libc::c_void as *mut List;
                    break;
                } else {
                    level = 2 as libc::c_int;
                    if gen_validp(genid as libc::c_int) != 0 {
                        ({
                            let mut _temp: libc::c_ushort = 0;
                            if (*fapi).fread.expect("non-null function pointer")(
                                &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                                2 as libc::c_int,
                                fd,
                            ) == FLUID_FAILED as libc::c_int
                            {
                                return 0 as libc::c_int;
                            }
                            genval.sword = _temp as libc::c_short;
                        });
                        dup = gen_inlist(genid as libc::c_int, (*z).gen)
                    } else {
                        skip = (0 as libc::c_int == 0) as libc::c_int
                    }
                }
                if skip == 0 {
                    if dup.is_null() {
                        g = libc::malloc(::std::mem::size_of::<SFGen>() as libc::size_t)
                            as *mut SFGen;
                        (*p3).data = g as *mut libc::c_void;
                        (*g).id = genid
                    } else {
                        g = (*dup).data as *mut SFGen;
                        drop_0 = (0 as libc::c_int == 0) as libc::c_int
                    }
                    (*g).amount = genval
                } else {
                    discarded = (0 as libc::c_int == 0) as libc::c_int;
                    drop_0 = (0 as libc::c_int == 0) as libc::c_int;
                    if (*fapi).fseek.expect("non-null function pointer")(
                        fd,
                        2 as libc::c_int as libc::c_long,
                        1 as libc::c_int,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                }
                if drop_0 == 0 {
                    p3 = if !p3.is_null() {
                        (*p3).next
                    } else {
                        0 as *mut List
                    }
                } else {
                    {
                        let mut _temp: *mut List = p3;
                        p3 = if !p3.is_null() {
                            (*p3).next
                        } else {
                            0 as *mut List
                        };
                        (*z).gen = fluid_list_remove_link((*z).gen, _temp);
                        delete1_fluid_list(_temp);
                    }
                }
            }
            if level == 3 as libc::c_int {
                {
                    let mut _temp: *mut List = p3;
                    p3 = if !p3.is_null() {
                        (*p3).next
                    } else {
                        0 as *mut List
                    };
                    (*z).gen = fluid_list_remove_link((*z).gen, _temp);
                    delete1_fluid_list(_temp);
                }
            } else if gzone == 0 {
                gzone = (0 as libc::c_int == 0) as libc::c_int;
                if *hz != p2 {
                    let save: *mut libc::c_void = (*p2).data;
                    fluid_log!(
                        FLUID_WARN,
                        "Preset \"{}\": Global zone is not first zone",
                        CStr::from_ptr(
                            (*((*p).data as *const SFPreset)).name.as_ptr() as *const libc::c_char
                        )
                        .to_str()
                        .unwrap()
                    );
                    ({
                        let mut _temp: *mut List = p2;
                        p2 = if !p2.is_null() {
                            (*p2).next
                        } else {
                            0 as *mut List
                        };
                        *hz = fluid_list_remove_link(*hz, _temp);
                        delete1_fluid_list(_temp);
                    });
                    *hz = fluid_list_prepend(*hz, save);
                    continue;
                }
            } else {
                fluid_log!(
                    FLUID_WARN,
                    "Preset \"{}\": Discarding invalid global zone",
                    CStr::from_ptr(
                        (*((*p).data as *const SFPreset)).name.as_ptr() as *const libc::c_char
                    )
                    .to_str()
                    .unwrap()
                );
                sfont_zone_delete(sf, hz, (*p2).data as *mut SFZone);
            }
            while !p3.is_null() {
                discarded = (0 as libc::c_int == 0) as libc::c_int;
                size -= 4 as libc::c_int;
                if size < 0 as libc::c_int {
                    return gerr!(ErrCorr, "Preset generator chunk size mismatch",);
                }
                if (*fapi).fseek.expect("non-null function pointer")(
                    fd,
                    4 as libc::c_int as libc::c_long,
                    1 as libc::c_int,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                {
                    let mut _temp: *mut List = p3;
                    p3 = if !p3.is_null() {
                        (*p3).next
                    } else {
                        0 as *mut List
                    };
                    (*z).gen = fluid_list_remove_link((*z).gen, _temp);
                    delete1_fluid_list(_temp);
                }
            }
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        if discarded != 0 {
            fluid_log!(
                FLUID_WARN,
                "Preset \"{}\": Some invalid generators were discarded",
                CStr::from_ptr(
                    (*((*p).data as *const SFPreset)).name.as_ptr() as *const libc::c_char
                )
                .to_str()
                .unwrap()
            );
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    if size == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    size -= 4 as libc::c_int;
    if size != 0 as libc::c_int {
        return gerr!(ErrCorr, "Preset generator chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        4 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_ihdr(
    mut size: libc::c_int,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut i: libc::c_int;
    let mut i2: libc::c_int;
    let mut p: *mut SFInst;
    let mut pr: *mut SFInst = 0 as *mut SFInst;
    let mut zndx: libc::c_ushort;
    let mut pzndx: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    if size % 22 as libc::c_int != 0 || size == 0 as libc::c_int {
        return gerr!(ErrCorr, "Instrument header has invalid size",);
    }
    size = size / 22 as libc::c_int - 1 as libc::c_int;
    if size == 0 as libc::c_int {
        fluid_log!(FLUID_WARN, "File contains no instruments",);
        if (*fapi).fseek.expect("non-null function pointer")(
            fd,
            22 as libc::c_int as libc::c_long,
            1 as libc::c_int,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int;
    while i < size {
        p = libc::malloc(::std::mem::size_of::<SFInst>() as libc::size_t) as *mut SFInst;
        (*sf).inst = fluid_list_append((*sf).inst, p as *mut libc::c_void);
        (*p).zone = 0 as *mut List;
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut (*p).name as *mut [libc::c_char; 21] as *mut libc::c_void,
                20 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).name[20 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        });
        ({
            let mut _temp: libc::c_ushort = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                2 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            zndx = _temp as libc::c_short as libc::c_ushort;
        });
        if !pr.is_null() {
            if (zndx as libc::c_int) < pzndx as libc::c_int {
                return gerr!(ErrCorr, "Instrument header indices not monotonic",);
            }
            i2 = zndx as libc::c_int - pzndx as libc::c_int;
            loop {
                let fresh13 = i2;
                i2 = i2 - 1;
                if !(fresh13 != 0) {
                    break;
                }
                (*pr).zone = fluid_list_prepend((*pr).zone, 0 as *mut libc::c_void)
            }
        } else if zndx as libc::c_int > 0 as libc::c_int {
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
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        20 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    ({
        let mut _temp: libc::c_ushort = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
            2 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        zndx = _temp as libc::c_short as libc::c_ushort;
    });
    if (zndx as libc::c_int) < pzndx as libc::c_int {
        return gerr!(ErrCorr, "Instrument header indices not monotonic",);
    }
    i2 = zndx as libc::c_int - pzndx as libc::c_int;
    loop {
        let fresh14 = i2;
        i2 = i2 - 1;
        if !(fresh14 != 0) {
            break;
        }
        (*pr).zone = fluid_list_prepend((*pr).zone, 0 as *mut libc::c_void)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_ibag(
    mut size: libc::c_int,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut z: *mut SFZone;
    let mut pz: *mut SFZone = 0 as *mut SFZone;
    let mut genndx;
    let mut modndx;
    let mut pgenndx: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut pmodndx: libc::c_ushort = 0 as libc::c_int as libc::c_ushort;
    let mut i;
    if size % 4 as libc::c_int != 0 || size == 0 as libc::c_int {
        return gerr!(ErrCorr, "Instrument bag chunk size is invalid",);
    }
    p = (*sf).inst;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFInst)).zone;
        while !p2.is_null() {
            size -= 4 as libc::c_int;
            if size < 0 as libc::c_int {
                return gerr!(ErrCorr, "Instrument bag chunk size mismatch",);
            }
            z = libc::malloc(::std::mem::size_of::<SFZone>() as libc::size_t) as *mut SFZone;
            (*p2).data = z as *mut libc::c_void;
            (*z).gen = 0 as *mut List;
            (*z).mod_0 = 0 as *mut List;
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                genndx = _temp as libc::c_short as libc::c_ushort;
            });
            ({
                let mut _temp: libc::c_ushort = 0;
                if (*fapi).fread.expect("non-null function pointer")(
                    &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                    2 as libc::c_int,
                    fd,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                modndx = _temp as libc::c_short as libc::c_ushort;
            });
            (*z).instsamp = 0 as *mut List;
            if !pz.is_null() {
                if (genndx as libc::c_int) < pgenndx as libc::c_int {
                    return gerr!(ErrCorr, "Instrument generator indices not monotonic",);
                }
                if (modndx as libc::c_int) < pmodndx as libc::c_int {
                    return gerr!(ErrCorr, "Instrument modulator indices not monotonic",);
                }
                i = genndx as libc::c_int - pgenndx as libc::c_int;
                loop {
                    let fresh15 = i;
                    i = i - 1;
                    if !(fresh15 != 0) {
                        break;
                    }
                    (*pz).gen = fluid_list_prepend((*pz).gen, 0 as *mut libc::c_void)
                }
                i = modndx as libc::c_int - pmodndx as libc::c_int;
                loop {
                    let fresh16 = i;
                    i = i - 1;
                    if !(fresh16 != 0) {
                        break;
                    }
                    (*pz).mod_0 = fluid_list_prepend((*pz).mod_0, 0 as *mut libc::c_void)
                }
            }
            pz = z;
            pgenndx = genndx;
            pmodndx = modndx;
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    size -= 4 as libc::c_int;
    if size != 0 as libc::c_int {
        return gerr!(ErrCorr, "Instrument chunk size mismatch",);
    }
    ({
        let mut _temp: libc::c_ushort = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
            2 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        genndx = _temp as libc::c_short as libc::c_ushort;
    });
    ({
        let mut _temp: libc::c_ushort = 0;
        if (*fapi).fread.expect("non-null function pointer")(
            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
            2 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        modndx = _temp as libc::c_short as libc::c_ushort;
    });
    if pz.is_null() {
        if genndx as libc::c_int > 0 as libc::c_int {
            fluid_log!(
                FLUID_WARN,
                "No instrument generators and terminal index not 0",
            );
        }
        if modndx as libc::c_int > 0 as libc::c_int {
            fluid_log!(
                FLUID_WARN,
                "No instrument modulators and terminal index not 0",
            );
        }
        return 1 as libc::c_int;
    }
    if (genndx as libc::c_int) < pgenndx as libc::c_int {
        return gerr!(ErrCorr, "Instrument generator indices not monotonic",);
    }
    if (modndx as libc::c_int) < pmodndx as libc::c_int {
        return gerr!(ErrCorr, "Instrument modulator indices not monotonic",);
    }
    i = genndx as libc::c_int - pgenndx as libc::c_int;
    loop {
        let fresh17 = i;
        i = i - 1;
        if !(fresh17 != 0) {
            break;
        }
        (*pz).gen = fluid_list_prepend((*pz).gen, 0 as *mut libc::c_void)
    }
    i = modndx as libc::c_int - pmodndx as libc::c_int;
    loop {
        let fresh18 = i;
        i = i - 1;
        if !(fresh18 != 0) {
            break;
        }
        (*pz).mod_0 = fluid_list_prepend((*pz).mod_0, 0 as *mut libc::c_void)
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_imod(
    mut size: libc::c_int,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut p3: *mut List;
    let mut m: *mut SFMod;
    p = (*sf).inst;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFInst)).zone;
        while !p2.is_null() {
            p3 = (*((*p2).data as *mut SFZone)).mod_0;
            while !p3.is_null() {
                size -= 10 as libc::c_int;
                if size < 0 as libc::c_int {
                    return gerr!(ErrCorr, "Instrument modulator chunk size mismatch",);
                }
                m = libc::malloc(::std::mem::size_of::<SFMod>() as libc::size_t) as *mut SFMod;
                (*p3).data = m as *mut libc::c_void;
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).src = _temp as libc::c_short as libc::c_ushort;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).dest = _temp as libc::c_short as libc::c_ushort;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).amount = _temp as libc::c_short;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).amtsrc = _temp as libc::c_short as libc::c_ushort;
                });
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    (*m).trans = _temp as libc::c_short as libc::c_ushort;
                });
                p3 = if !p3.is_null() {
                    (*p3).next
                } else {
                    0 as *mut List
                }
            }
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    if size == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    size -= 10 as libc::c_int;
    if size != 0 as libc::c_int {
        return gerr!(ErrCorr, "Instrument modulator chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        10 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_igen(
    mut size: libc::c_int,
    sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut p3: *mut List;
    let mut dup: *mut List;
    let mut hz: *mut *mut List = 0 as *mut *mut List;
    let mut z: *mut SFZone;
    let mut g: *mut SFGen;
    let mut genval: SFGenAmount = SFGenAmount { sword: 0 };
    let mut genid: libc::c_ushort;
    let mut level: libc::c_int;
    let mut skip: libc::c_int;
    let mut drop_0: libc::c_int;
    let mut gzone: libc::c_int;
    let mut discarded: libc::c_int;
    p = (*sf).inst;
    while !p.is_null() {
        gzone = 0 as libc::c_int;
        discarded = 0 as libc::c_int;
        p2 = (*((*p).data as *mut SFInst)).zone;
        if !p2.is_null() {
            hz = &mut p2
        }
        while !p2.is_null() {
            level = 0 as libc::c_int;
            z = (*p2).data as *mut SFZone;
            p3 = (*z).gen;
            while !p3.is_null() {
                dup = 0 as *mut List;
                skip = 0 as libc::c_int;
                drop_0 = 0 as libc::c_int;
                size -= 4 as libc::c_int;
                if size < 0 as libc::c_int {
                    return gerr!(ErrCorr, "IGEN chunk size mismatch",);
                }
                ({
                    let mut _temp: libc::c_ushort = 0;
                    if (*fapi).fread.expect("non-null function pointer")(
                        &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                        2 as libc::c_int,
                        fd,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                    genid = _temp as libc::c_short as libc::c_ushort;
                });
                if genid as libc::c_int == GEN_KEY_RANGE as libc::c_int {
                    if level == 0 as libc::c_int {
                        level = 1 as libc::c_int;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                    } else {
                        skip = (0 as libc::c_int == 0) as libc::c_int
                    }
                } else if genid as libc::c_int == GEN_VEL_RANGE as libc::c_int {
                    if level <= 1 as libc::c_int {
                        level = 2 as libc::c_int;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.lo as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut genval.range.hi as *mut libc::c_uchar as *mut libc::c_void,
                            1 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                    } else {
                        skip = (0 as libc::c_int == 0) as libc::c_int
                    }
                } else if genid as libc::c_int == GEN_SAMPLE_ID as libc::c_int {
                    level = 3 as libc::c_int;
                    ({
                        let mut _temp: libc::c_ushort = 0;
                        if (*fapi).fread.expect("non-null function pointer")(
                            &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                            2 as libc::c_int,
                            fd,
                        ) == FLUID_FAILED as libc::c_int
                        {
                            return 0 as libc::c_int;
                        }
                        genval.uword = _temp as libc::c_short as libc::c_ushort;
                    });
                    let ref mut fresh19 = (*((*p2).data as *mut SFZone)).instsamp;
                    *fresh19 = (genval.uword as libc::c_int + 1 as libc::c_int) as libc::c_long
                        as *mut libc::c_void as *mut List;
                    break;
                } else {
                    level = 2 as libc::c_int;
                    if gen_valid(genid as libc::c_int) != 0 {
                        ({
                            let mut _temp: libc::c_ushort = 0;
                            if (*fapi).fread.expect("non-null function pointer")(
                                &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                                2 as libc::c_int,
                                fd,
                            ) == FLUID_FAILED as libc::c_int
                            {
                                return 0 as libc::c_int;
                            }
                            genval.sword = _temp as libc::c_short;
                        });
                        dup = gen_inlist(genid as libc::c_int, (*z).gen)
                    } else {
                        skip = (0 as libc::c_int == 0) as libc::c_int
                    }
                }
                if skip == 0 {
                    if dup.is_null() {
                        g = libc::malloc(::std::mem::size_of::<SFGen>() as libc::size_t)
                            as *mut SFGen;
                        (*p3).data = g as *mut libc::c_void;
                        (*g).id = genid
                    } else {
                        g = (*dup).data as *mut SFGen;
                        drop_0 = (0 as libc::c_int == 0) as libc::c_int
                    }
                    (*g).amount = genval
                } else {
                    discarded = (0 as libc::c_int == 0) as libc::c_int;
                    drop_0 = (0 as libc::c_int == 0) as libc::c_int;
                    if (*fapi).fseek.expect("non-null function pointer")(
                        fd,
                        2 as libc::c_int as libc::c_long,
                        1 as libc::c_int,
                    ) == FLUID_FAILED as libc::c_int
                    {
                        return 0 as libc::c_int;
                    }
                }
                if drop_0 == 0 {
                    p3 = if !p3.is_null() {
                        (*p3).next
                    } else {
                        0 as *mut List
                    }
                } else {
                    {
                        let mut _temp: *mut List = p3;
                        p3 = if !p3.is_null() {
                            (*p3).next
                        } else {
                            0 as *mut List
                        };
                        (*z).gen = fluid_list_remove_link((*z).gen, _temp);
                        delete1_fluid_list(_temp);
                    }
                }
            }
            if level == 3 as libc::c_int {
                {
                    let mut _temp: *mut List = p3;
                    p3 = if !p3.is_null() {
                        (*p3).next
                    } else {
                        0 as *mut List
                    };
                    (*z).gen = fluid_list_remove_link((*z).gen, _temp);
                    delete1_fluid_list(_temp);
                }
            } else if gzone == 0 {
                gzone = (0 as libc::c_int == 0) as libc::c_int;
                if *hz != p2 {
                    let save: *mut libc::c_void = (*p2).data;
                    fluid_log!(
                        FLUID_WARN,
                        "Instrument \"{}\": Global zone is not first zone",
                        CStr::from_ptr((*((*p).data as *const SFPreset)).name.as_ptr())
                            .to_str()
                            .unwrap()
                    );
                    ({
                        let mut _temp: *mut List = p2;
                        p2 = if !p2.is_null() {
                            (*p2).next
                        } else {
                            0 as *mut List
                        };
                        *hz = fluid_list_remove_link(*hz, _temp);
                        delete1_fluid_list(_temp);
                    });
                    *hz = fluid_list_prepend(*hz, save);
                    continue;
                }
            } else {
                fluid_log!(
                    FLUID_WARN,
                    "Instrument \"{}\": Discarding invalid global zone",
                    CStr::from_ptr((*((*p).data as *const SFInst)).name.as_ptr())
                        .to_str()
                        .unwrap()
                );
                sfont_zone_delete(sf, hz, (*p2).data as *mut SFZone);
            }
            while !p3.is_null() {
                discarded = (0 as libc::c_int == 0) as libc::c_int;
                size -= 4 as libc::c_int;
                if size < 0 as libc::c_int {
                    return gerr!(ErrCorr, "Instrument generator chunk size mismatch",);
                }
                if (*fapi).fseek.expect("non-null function pointer")(
                    fd,
                    4 as libc::c_int as libc::c_long,
                    1 as libc::c_int,
                ) == FLUID_FAILED as libc::c_int
                {
                    return 0 as libc::c_int;
                }
                {
                    let mut _temp: *mut List = p3;
                    p3 = if !p3.is_null() {
                        (*p3).next
                    } else {
                        0 as *mut List
                    };
                    (*z).gen = fluid_list_remove_link((*z).gen, _temp);
                    delete1_fluid_list(_temp);
                }
            }
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        if discarded != 0 {
            fluid_log!(
                FLUID_WARN,
                "Instrument \"{}\": Some invalid generators were discarded",
                CStr::from_ptr((*((*p).data as *const SFInst)).name.as_ptr())
                    .to_str()
                    .unwrap()
            );
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    if size == 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    size -= 4 as libc::c_int;
    if size != 0 as libc::c_int {
        return gerr!(ErrCorr, "IGEN chunk size mismatch",);
    }
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        4 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn load_shdr(
    mut size: libc::c_uint,
    mut sf: *mut SFData,
    fd: *mut libc::c_void,
    fapi: *mut fluid_fileapi_t,
) -> libc::c_int {
    let mut i: libc::c_uint;
    let mut p: *mut SFSample;
    if size.wrapping_rem(46 as libc::c_int as libc::c_uint) != 0
        || size == 0 as libc::c_int as libc::c_uint
    {
        return gerr!(ErrCorr, "Sample header has invalid size",);
    }
    size = size
        .wrapping_div(46 as libc::c_int as libc::c_uint)
        .wrapping_sub(1 as libc::c_int as libc::c_uint);
    if size == 0 as libc::c_int as libc::c_uint {
        fluid_log!(FLUID_WARN, "File contains no samples",);
        if (*fapi).fseek.expect("non-null function pointer")(
            fd,
            46 as libc::c_int as libc::c_long,
            1 as libc::c_int,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        return 1 as libc::c_int;
    }
    i = 0 as libc::c_int as libc::c_uint;
    while i < size {
        p = libc::malloc(::std::mem::size_of::<SFSample>() as libc::size_t) as *mut SFSample;
        (*sf).sample = fluid_list_append((*sf).sample, p as *mut libc::c_void);
        ({
            if (*fapi).fread.expect("non-null function pointer")(
                &mut (*p).name as *mut [libc::c_char; 21] as *mut libc::c_void,
                20 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).name[20 as libc::c_int as usize] = '\u{0}' as i32 as libc::c_char;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).start = _temp as libc::c_int as libc::c_uint;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).end = _temp as libc::c_int as libc::c_uint;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).loopstart = _temp as libc::c_int as libc::c_uint;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).loopend = _temp as libc::c_int as libc::c_uint;
        });
        ({
            let mut _temp: libc::c_uint = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_uint as *mut libc::c_void,
                4 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).samplerate = _temp as libc::c_int as libc::c_uint;
        });
        if (*fapi).fread.expect("non-null function pointer")(
            &mut (*p).origpitch as *mut libc::c_uchar as *mut libc::c_void,
            1 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if (*fapi).fread.expect("non-null function pointer")(
            &mut (*p).pitchadj as *mut libc::c_schar as *mut libc::c_void,
            1 as libc::c_int,
            fd,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        if (*fapi).fseek.expect("non-null function pointer")(
            fd,
            2 as libc::c_int as libc::c_long,
            1 as libc::c_int,
        ) == FLUID_FAILED as libc::c_int
        {
            return 0 as libc::c_int;
        }
        ({
            let mut _temp: libc::c_ushort = 0;
            if (*fapi).fread.expect("non-null function pointer")(
                &mut _temp as *mut libc::c_ushort as *mut libc::c_void,
                2 as libc::c_int,
                fd,
            ) == FLUID_FAILED as libc::c_int
            {
                return 0 as libc::c_int;
            }
            (*p).sampletype = _temp as libc::c_short as libc::c_ushort;
        });
        (*p).samfile = 0 as libc::c_int as libc::c_uchar;
        i = i.wrapping_add(1)
    }
    if (*fapi).fseek.expect("non-null function pointer")(
        fd,
        46 as libc::c_int as libc::c_long,
        1 as libc::c_int,
    ) == FLUID_FAILED as libc::c_int
    {
        return 0 as libc::c_int;
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fixup_pgen(sf: *mut SFData) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut p3: *mut List;
    let mut z: *mut SFZone;
    let mut i: libc::c_int;
    p = (*sf).preset;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFPreset)).zone;
        while !p2.is_null() {
            z = (*p2).data as *mut SFZone;
            i = (*z).instsamp as libc::c_int;
            if i != 0 {
                p3 = fluid_list_nth((*sf).inst, i - 1 as libc::c_int);
                if p3.is_null() {
                    return gerr!(
                        ErrCorr,
                        "Preset {} {}: Invalid instrument reference",
                        (*((*p).data as *mut SFPreset)).bank,
                        (*((*p).data as *mut SFPreset)).prenum
                    );
                }
                (*z).instsamp = p3
            } else {
                (*z).instsamp = 0 as *mut List
            }
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fixup_igen(sf: *mut SFData) -> libc::c_int {
    let mut p: *mut List;
    let mut p2: *mut List;
    let mut p3: *mut List;
    let mut z: *mut SFZone;
    let mut i: libc::c_int;
    p = (*sf).inst;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFInst)).zone;
        while !p2.is_null() {
            z = (*p2).data as *mut SFZone;
            i = (*z).instsamp as libc::c_int;
            if i != 0 {
                p3 = fluid_list_nth((*sf).sample, i - 1 as libc::c_int);
                if p3.is_null() {
                    return gerr!(
                        ErrCorr,
                        "Instrument \"{}\": Invalid sample reference",
                        CStr::from_ptr((*((*p).data as *const SFInst)).name.as_ptr())
                            .to_str()
                            .unwrap()
                    );
                }
                (*z).instsamp = p3
            }
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn fixup_sample(sf: *mut SFData) -> libc::c_int {
    let mut p: *mut List;
    let mut sam: *mut SFSample;
    p = (*sf).sample;
    while !p.is_null() {
        sam = (*p).data as *mut SFSample;
        if (*sam).sampletype as libc::c_int & 0x8000 as libc::c_int == 0
            && (*sam).end > SDTACHUNK_SIZE
            || (*sam).start > (*sam).end.wrapping_sub(4 as libc::c_int as libc::c_uint)
        {
            fluid_log!(FLUID_WARN,
                      "Sample \'{}\' start/end file positions are invalid, disabling and will not be saved", CStr::from_ptr((*sam).name.as_ptr()).to_str().unwrap());
            (*sam).loopend = 0 as libc::c_int as libc::c_uint;
            (*sam).loopstart = (*sam).loopend;
            (*sam).end = (*sam).loopstart;
            (*sam).start = (*sam).end;
            return 1 as libc::c_int;
        } else {
            if !((*sam).sampletype as libc::c_int & 0x10 as libc::c_int != 0) {
                if (*sam).loopend > (*sam).end
                    || (*sam).loopstart >= (*sam).loopend
                    || (*sam).loopstart <= (*sam).start
                {
                    if (*sam).end.wrapping_sub((*sam).start) >= 20 as libc::c_int as libc::c_uint {
                        (*sam).loopstart =
                            (*sam).start.wrapping_add(8 as libc::c_int as libc::c_uint);
                        (*sam).loopend = (*sam).end.wrapping_sub(8 as libc::c_int as libc::c_uint)
                    } else {
                        (*sam).loopstart =
                            (*sam).start.wrapping_add(1 as libc::c_int as libc::c_uint);
                        (*sam).loopend = (*sam).end.wrapping_sub(1 as libc::c_int as libc::c_uint)
                    }
                }
            }
        }
        (*sam).end = (*sam)
            .end
            .wrapping_sub((*sam).start.wrapping_add(1 as libc::c_int as libc::c_uint));
        (*sam).loopstart = (*sam).loopstart.wrapping_sub((*sam).start);
        (*sam).loopend = (*sam).loopend.wrapping_sub((*sam).start);
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub static mut badgen: [libc::c_ushort; 8] = [
    GEN_UNUSED1 as libc::c_int as libc::c_ushort,
    GEN_UNUSED2 as libc::c_int as libc::c_ushort,
    GEN_UNUSED3 as libc::c_int as libc::c_ushort,
    GEN_UNUSED4 as libc::c_int as libc::c_ushort,
    GEN_RESERVED1 as libc::c_int as libc::c_ushort,
    GEN_RESERVED2 as libc::c_int as libc::c_ushort,
    GEN_RESERVED3 as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub static mut badpgen: [libc::c_ushort; 14] = [
    GEN_START_ADDR_OFS as libc::c_int as libc::c_ushort,
    GEN_END_ADDR_OFS as libc::c_int as libc::c_ushort,
    GEN_START_LOOP_ADDR_OFS as libc::c_int as libc::c_ushort,
    GEN_END_LOOP_ADDR_OFS as libc::c_int as libc::c_ushort,
    GEN_START_ADDR_COARSE_OFS as libc::c_int as libc::c_ushort,
    GEN_END_ADDR_COARSE_OFS as libc::c_int as libc::c_ushort,
    GEN_START_LOOP_ADDR_COARSE_OFS as libc::c_int as libc::c_ushort,
    GEN_KEYNUM as libc::c_int as libc::c_ushort,
    GEN_VELOCITY as libc::c_int as libc::c_ushort,
    GEN_END_LOOP_ADDR_COARSE_OFS as libc::c_int as libc::c_ushort,
    GEN_SAMPLE_MODES as libc::c_int as libc::c_ushort,
    GEN_EXCLUSIVE_CLASS as libc::c_int as libc::c_ushort,
    GEN_OVERRIDE_ROOT_KEY as libc::c_int as libc::c_ushort,
    0 as libc::c_int as libc::c_ushort,
];
#[no_mangle]
pub unsafe extern "C" fn sfont_close(mut sf: *mut SFData, fapi: *mut fluid_fileapi_t) {
    let mut p: *mut List;
    let mut p2: *mut List;
    if !(*sf).sffd.is_null() {
        (*fapi).fclose.expect("non-null function pointer")((*sf).sffd as *mut libc::c_void);
    }
    if !(*sf).fname.is_null() {
        libc::free((*sf).fname as *mut libc::c_void);
    }
    p = (*sf).info;
    while !p.is_null() {
        libc::free((*p).data);
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    delete_fluid_list((*sf).info);
    (*sf).info = 0 as *mut List;
    p = (*sf).preset;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFPreset)).zone;
        while !p2.is_null() {
            sfont_free_zone((*p2).data as *mut SFZone);
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        delete_fluid_list((*((*p).data as *mut SFPreset)).zone);
        libc::free((*p).data);
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    delete_fluid_list((*sf).preset);
    (*sf).preset = 0 as *mut List;
    p = (*sf).inst;
    while !p.is_null() {
        p2 = (*((*p).data as *mut SFInst)).zone;
        while !p2.is_null() {
            sfont_free_zone((*p2).data as *mut SFZone);
            p2 = if !p2.is_null() {
                (*p2).next
            } else {
                0 as *mut List
            }
        }
        delete_fluid_list((*((*p).data as *mut SFInst)).zone);
        libc::free((*p).data);
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    delete_fluid_list((*sf).inst);
    (*sf).inst = 0 as *mut List;
    p = (*sf).sample;
    while !p.is_null() {
        libc::free((*p).data);
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    delete_fluid_list((*sf).sample);
    (*sf).sample = 0 as *mut List;
    libc::free(sf as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sfont_free_zone(zone: *mut SFZone) {
    let mut p: *mut List;
    if zone.is_null() {
        return;
    }
    p = (*zone).gen;
    while !p.is_null() {
        if !(*p).data.is_null() {
            libc::free((*p).data);
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    delete_fluid_list((*zone).gen);
    p = (*zone).mod_0;
    while !p.is_null() {
        if !(*p).data.is_null() {
            libc::free((*p).data);
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    delete_fluid_list((*zone).mod_0);
    libc::free(zone as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn sfont_preset_compare_func(
    a: *mut libc::c_void,
    b: *mut libc::c_void,
) -> libc::c_int {
    let aval: libc::c_int;
    let bval: libc::c_int;
    aval = ((*(a as *mut SFPreset)).bank as libc::c_int) << 16 as libc::c_int
        | (*(a as *mut SFPreset)).prenum as libc::c_int;
    bval = ((*(b as *mut SFPreset)).bank as libc::c_int) << 16 as libc::c_int
        | (*(b as *mut SFPreset)).prenum as libc::c_int;
    return aval - bval;
}
#[no_mangle]
pub unsafe extern "C" fn sfont_zone_delete(
    _sf: *mut SFData,
    zlist: *mut *mut List,
    zone: *mut SFZone,
) {
    *zlist = fluid_list_remove(*zlist, zone as *mut libc::c_void);
    sfont_free_zone(zone);
}
#[no_mangle]
pub unsafe extern "C" fn gen_inlist(
    gen: libc::c_int,
    genlist: *mut List,
) -> *mut List {
    let mut p: *mut List;
    p = genlist;
    while !p.is_null() {
        if (*p).data.is_null() {
            return 0 as *mut List;
        }
        if gen == (*((*p).data as *mut SFGen)).id as libc::c_int {
            break;
        }
        p = if !p.is_null() {
            (*p).next
        } else {
            0 as *mut List
        }
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn gen_valid(gen: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    if gen > GEN_DUMMY as libc::c_int - 1 as libc::c_int {
        return 0 as libc::c_int;
    }
    while badgen[i as usize] as libc::c_int != 0 && badgen[i as usize] as libc::c_int != gen {
        i += 1
    }
    return (badgen[i as usize] as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn gen_validp(gen: libc::c_int) -> libc::c_int {
    let mut i: libc::c_int = 0 as libc::c_int;
    if gen_valid(gen) == 0 {
        return 0 as libc::c_int;
    }
    while badpgen[i as usize] as libc::c_int != 0
        && badpgen[i as usize] as libc::c_int != gen as libc::c_ushort as libc::c_int
    {
        i += 1
    }
    return (badpgen[i as usize] as libc::c_int == 0 as libc::c_int) as libc::c_int;
}
