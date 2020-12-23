use crate::{ll, Error, Result, Status, Synth};
use std::ffi::CStr;

impl Synth {
    /**
    Get a textual representation of the last error
     */
    pub(super) fn error(&self) -> String {
        let error = unsafe { ll::synth::fluid_synth_error(self.handle) };
        let error = unsafe { CStr::from_ptr(error) };
        error.to_str().unwrap().into()
    }

    pub(super) fn neg_err(&self, ret: i32) -> Result<i32> {
        if ret < 0 {
            Err(Error::Fluid(self.error()))
        } else {
            Ok(ret)
        }
    }

    pub(super) fn zero_ok(&self, ret: i32) -> Status {
        if ret == 0 {
            Ok(())
        } else {
            Err(Error::Fluid(self.error()))
        }
    }
}
