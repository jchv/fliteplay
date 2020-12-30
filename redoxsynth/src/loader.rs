use crate::{Result, fileapi::FileSystem, ll, result_from_ptr};
use std::mem::transmute;

/**
The SoundFont loader object
 */
#[repr(transparent)]
pub struct Loader {
    handle: *mut ll::soundfont::SoundFontLoader,
}

unsafe impl Send for Loader {}

impl Loader {
    /**
    Create default SoundFont loader
     */
    pub fn new_default() -> Result<Self> {
        result_from_ptr(ll::sfloader::new_fluid_defsfloader()).map(|handle| Self { handle })
    }

    pub(crate) fn into_ptr(self) -> *mut ll::soundfont::SoundFontLoader {
        unsafe { transmute(self) }
    }

    /**
    Set the file reading API which will be used by loader
     */
    pub fn set_file_api(&self, filesystem: Box<dyn FileSystem>) {
        let handle = unsafe { &mut *self.handle };
        handle.filesystem = filesystem;
    }
}

impl Drop for Loader {
    fn drop(&mut self) {
        unsafe {
            ll::sfloader::delete_fluid_defsfloader(self.handle);
        }
    }
}
