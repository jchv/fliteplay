use crate::ll::synth::fluid_synth_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundfontLoader {
    pub data: *mut libc::c_void,
    pub free: Option<unsafe extern "C" fn(_: *mut SoundfontLoader) -> libc::c_int>,
    pub load: Option<
        unsafe extern "C" fn(
            _: *mut SoundfontLoader,
            _: *const libc::c_char,
        ) -> *mut SoundFont,
    >,
    pub fileapi: *mut FileApi,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileApi {
    pub data: *mut libc::c_void,
    pub free: Option<unsafe extern "C" fn(_: *mut FileApi) -> libc::c_int>,
    pub fopen: Option<
        unsafe extern "C" fn(_: *mut FileApi, _: *const libc::c_char) -> *mut libc::c_void,
    >,
    pub fread: Option<
        unsafe extern "C" fn(
            _: *mut libc::c_void,
            _: libc::c_int,
            _: *mut libc::c_void,
        ) -> libc::c_int,
    >,
    pub fseek: Option<
        unsafe extern "C" fn(_: *mut libc::c_void, _: libc::c_long, _: libc::c_int) -> libc::c_int,
    >,
    pub fclose: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_int>,
    pub ftell: Option<unsafe extern "C" fn(_: *mut libc::c_void) -> libc::c_long>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Preset {
    pub data: *mut libc::c_void,
    pub sfont: *mut SoundFont,
    pub free: Option<unsafe extern "C" fn(_: *mut Preset) -> libc::c_int>,
    pub get_name: Option<unsafe extern "C" fn(_: *mut Preset) -> *mut libc::c_char>,
    pub get_banknum: Option<unsafe extern "C" fn(_: *mut Preset) -> libc::c_int>,
    pub get_num: Option<unsafe extern "C" fn(_: *mut Preset) -> libc::c_int>,
    pub noteon: Option<
        unsafe extern "C" fn(
            _: *mut Preset,
            _: *mut fluid_synth_t,
            _: libc::c_int,
            _: libc::c_int,
            _: libc::c_int,
        ) -> libc::c_int,
    >,
    pub notify: Option<
        unsafe extern "C" fn(_: *mut Preset, _: libc::c_int, _: libc::c_int) -> libc::c_int,
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundFont {
    pub data: *mut libc::c_void,
    pub id: libc::c_uint,
    pub free: Option<unsafe extern "C" fn(_: *mut SoundFont) -> libc::c_int>,
    pub get_name: Option<unsafe extern "C" fn(_: *mut SoundFont) -> *mut libc::c_char>,
    pub get_preset: Option<
        unsafe extern "C" fn(
            _: *mut SoundFont,
            _: libc::c_uint,
            _: libc::c_uint,
        ) -> *mut Preset,
    >,
    pub iteration_start: Option<unsafe extern "C" fn(_: *mut SoundFont) -> ()>,
    pub iteration_next:
        Option<unsafe extern "C" fn(_: *mut SoundFont, _: *mut Preset) -> libc::c_int>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sample {
    pub name: [libc::c_char; 21],
    pub start: libc::c_uint,
    pub end: libc::c_uint,
    pub loopstart: libc::c_uint,
    pub loopend: libc::c_uint,
    pub samplerate: libc::c_uint,
    pub origpitch: libc::c_int,
    pub pitchadj: libc::c_int,
    pub sampletype: libc::c_int,
    pub valid: libc::c_int,
    pub data: *mut libc::c_short,
    pub amplitude_that_reaches_noise_floor_is_valid: libc::c_int,
    pub amplitude_that_reaches_noise_floor: f64,
    pub refcount: libc::c_uint,
    pub notify: Option<unsafe extern "C" fn(_: *mut Sample, _: libc::c_int) -> libc::c_int>,
    pub userdata: *mut libc::c_void,
}
