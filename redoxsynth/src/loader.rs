use crate::{ll, result_from_ptr, Result};
use std::{
    ffi::CStr,
    io::SeekFrom,
    mem::{transmute, MaybeUninit},
    os::raw::{c_char, c_int, c_void},
    path::Path,
    ptr::null_mut,
    slice::from_raw_parts_mut,
};

/**
The file reading API

Application can provide its own File API to override default.
For example, this may be useful in cases when soundfonts isn't located in the file system.
*/
pub trait FileApi {
    /// The type for file descriptor
    type File;

    /// Open file with specified name
    fn open(&mut self, filename: &Path) -> Option<Self::File>;

    /// Read binary data from file descriptor
    fn read(file: &mut Self::File, buf: &mut [u8]) -> bool;

    /// Seek current reading position
    fn seek(file: &mut Self::File, pos: SeekFrom) -> bool;

    /// Get current reading position from beginning of file
    fn tell(file: &mut Self::File) -> Option<u64>;
}

/**
The SoundFont loader object
 */
#[repr(transparent)]
pub struct Loader {
    handle: *mut ll::sfont::SoundfontLoader,
}

unsafe impl Send for Loader {}

impl Loader {
    /**
    Create default SoundFont loader
     */
    pub fn new_default() -> Result<Self> {
        result_from_ptr(ll::defsfont::new_fluid_defsfloader()).map(|handle| Self { handle })
    }

    pub(crate) fn into_ptr(self) -> *mut ll::sfont::SoundfontLoader {
        unsafe { transmute(self) }
    }

    /**
    Set the file reading API which will be used by loader
     */
    pub fn set_file_api<F: FileApi>(&self, fileapi: F) {
        let handle = unsafe { &mut *self.handle };

        handle.fileapi = wrap_fileapi(fileapi);
    }

    /**
    Set the file reading API which will be used by loaders by default
     */
    pub fn set_default_file_api<F: FileApi>(fileapi: F) {
        unsafe {
            ll::defsfont::fluid_set_default_fileapi(wrap_fileapi(fileapi));
        }
    }

    /**
    Reset the file reading API which will be used by loaders by default
     */
    pub fn reset_default_file_api() {
        unsafe {
            ll::defsfont::fluid_set_default_fileapi(null_mut());
        }
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        unsafe {
            ll::defsfont::delete_fluid_defsfloader(self.handle);
        }
    }
}

fn wrap_fileapi<F: FileApi>(fapi_rs: F) -> *mut ll::sfont::FileApi {
    let mut fapi_c = MaybeUninit::<ll::sfont::FileApi>::uninit();

    {
        let fapi = unsafe { &mut *fapi_c.as_mut_ptr() };

        fapi.data = Box::into_raw(Box::new(fapi_rs)) as _;
        fapi.free = Some(free_wrapper::<F>);
        fapi.fopen = Some(open_wrapper::<F>);
        fapi.fread = Some(read_wrapper::<F>);
        fapi.fseek = Some(seek_wrapper::<F>);
        fapi.ftell = Some(tell_wrapper::<F>);
        fapi.fclose = Some(close_wrapper::<F>);
    }

    Box::into_raw(Box::new(unsafe { fapi_c.assume_init() }))
}

fn free_wrapper<F: FileApi>(fapi_c: *mut ll::sfont::FileApi) -> c_int {
    let fapi = unsafe { Box::from_raw(fapi_c) };
    let _fapi_rs = unsafe { Box::from_raw(fapi.data as *mut F) };
    ll::OK
}

fn open_wrapper<F: FileApi>(
    fapi_c: *mut ll::sfont::FileApi,
    filename: *const c_char,
) -> *mut c_void {
    let fapi = unsafe { &mut *fapi_c };
    let fapi_rs = unsafe { &mut *(fapi.data as *mut F) };

    let filename = if let Ok(filename) = unsafe { CStr::from_ptr(filename) }.to_str() {
        filename
    } else {
        return null_mut();
    };

    let filename = Path::new(filename);

    if let Some(handle) = fapi_rs.open(&filename) {
        Box::into_raw(Box::new(handle)) as _
    } else {
        null_mut()
    }
}

fn read_wrapper<F: FileApi>(
    buf: *mut c_void,
    count: c_int,
    handle: *mut c_void,
) -> c_int {
    let handle = unsafe { &mut *(handle as *mut F::File) };

    let buffer: &mut [u8] = unsafe { from_raw_parts_mut(buf as _, count as _) };

    if F::read(handle, buffer) {
        ll::OK
    } else {
        ll::FAILED
    }
}

fn seek_wrapper<F: FileApi>(
    handle: *mut c_void,
    offset: isize,
    origin: i32,
) -> i32 {
    let handle = unsafe { &mut *(handle as *mut F::File) };

    use self::SeekFrom::*;

    let pos = match origin as _ {
        ll::SEEK_SET => Start(offset as _),
        ll::SEEK_END => End(offset as _),
        ll::SEEK_CUR => Current(offset as _),
        _ => return ll::FAILED,
    };

    if F::seek(handle, pos) {
        ll::OK
    } else {
        ll::FAILED
    }
}

fn tell_wrapper<F: FileApi>(handle: *mut c_void) -> isize {
    let handle = unsafe { &mut *(handle as *mut F::File) };

    if let Some(pos) = F::tell(handle) {
        pos as _
    } else {
        ll::FAILED as _
    }
}

fn close_wrapper<F: FileApi>(handle: *mut c_void) -> c_int {
    let _handle = unsafe { Box::from_raw(handle as *mut F::File) };

    ll::OK
}

#[cfg(test)]
mod test {
    use crate::{FileApi, Loader, Settings, Synth};
    use std::{
        fs::File,
        io::{Read, Seek, SeekFrom},
        path::Path,
    };

    struct TestFileApi;

    impl FileApi for TestFileApi {
        type File = File;

        fn open(&mut self, filename: &Path) -> Option<Self::File> {
            File::open(filename).ok()
        }

        fn read(file: &mut Self::File, buf: &mut [u8]) -> bool {
            file.read(buf).is_ok()
        }

        fn seek(file: &mut Self::File, pos: SeekFrom) -> bool {
            file.seek(pos).is_ok()
        }

        fn tell(file: &mut Self::File) -> Option<u64> {
            file.seek(SeekFrom::Current(0)).ok()
        }
    }

    #[test]
    fn fileapi() {
        let settings = Settings::new().unwrap();
        let mut synth = Synth::new(settings).unwrap();
        let loader = Loader::new_default().unwrap();

        loader.set_file_api(TestFileApi);
        synth.add_sfloader(loader);

        synth.sfload("../sf_/Boomwhacker.sf3", true).unwrap();
    }
}
