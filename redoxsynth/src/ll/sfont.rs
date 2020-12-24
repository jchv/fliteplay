use super::synth::Synth;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundfontLoader {
    pub data: *mut libc::c_void,
    pub free: Option<unsafe fn(_: *mut SoundfontLoader) -> i32>,
    pub load: Option<unsafe fn(_: *mut SoundfontLoader, _: *const libc::c_char) -> *mut SoundFont>,
    pub fileapi: *mut FileApi,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct FileApi {
    pub data: *mut libc::c_void,
    pub free: Option<unsafe fn(_: *mut FileApi) -> i32>,
    pub fopen: Option<unsafe fn(_: *mut FileApi, _: *const libc::c_char) -> *mut libc::c_void>,
    pub fread: Option<
        unsafe fn(_: *mut libc::c_void, _: i32, _: *mut libc::c_void) -> i32,
    >,
    pub fseek:
        Option<unsafe fn(_: *mut libc::c_void, _: isize, _: i32) -> i32>,
    pub fclose: Option<unsafe fn(_: *mut libc::c_void) -> i32>,
    pub ftell: Option<unsafe fn(_: *mut libc::c_void) -> isize>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Preset {
    pub data: *mut libc::c_void,
    pub sfont: *mut SoundFont,
    pub free: Option<unsafe fn(_: *mut Preset) -> i32>,
    pub get_name: Option<unsafe fn(_: *mut Preset) -> *mut libc::c_char>,
    pub get_banknum: Option<unsafe fn(_: *mut Preset) -> i32>,
    pub get_num: Option<unsafe fn(_: *mut Preset) -> i32>,
    pub noteon: Option<
        unsafe fn(
            _: *mut Preset,
            _: *mut Synth,
            _: i32,
            _: i32,
            _: i32,
        ) -> i32,
    >,
    pub notify: Option<unsafe fn(_: *mut Preset, _: i32, _: i32) -> i32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct SoundFont {
    pub data: *mut libc::c_void,
    pub id: u32,
    pub free: Option<unsafe fn(_: *mut SoundFont) -> i32>,
    pub get_name: Option<unsafe fn(_: *mut SoundFont) -> *mut libc::c_char>,
    pub get_preset:
        Option<unsafe fn(_: *mut SoundFont, _: u32, _: u32) -> *mut Preset>,
    pub iteration_start: Option<unsafe fn(_: *mut SoundFont) -> ()>,
    pub iteration_next: Option<unsafe fn(_: *mut SoundFont, _: *mut Preset) -> i32>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Sample {
    pub name: [libc::c_char; 21],
    pub start: u32,
    pub end: u32,
    pub loopstart: u32,
    pub loopend: u32,
    pub samplerate: u32,
    pub origpitch: i32,
    pub pitchadj: i32,
    pub sampletype: i32,
    pub valid: i32,
    pub data: *mut i16,
    pub amplitude_that_reaches_noise_floor_is_valid: i32,
    pub amplitude_that_reaches_noise_floor: f64,
    pub refcount: u32,
    pub notify: Option<unsafe fn(_: *mut Sample, _: i32) -> i32>,
    pub userdata: *mut libc::c_void,
}
