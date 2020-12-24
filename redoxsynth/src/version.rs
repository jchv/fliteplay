use crate::ll;
use std::fmt::{Display, Formatter, Result as FmtResult};

/**
The library version info
 */
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub micro: u32,
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        self.major.fmt(f)?;
        '.'.fmt(f)?;
        self.minor.fmt(f)?;
        '.'.fmt(f)?;
        self.micro.fmt(f)
    }
}

impl Version {
    pub fn new(major: u32, minor: u32, micro: u32) -> Self {
        Self {
            major,
            minor,
            micro,
        }
    }

    pub fn get() -> Version {
        let mut major = 0;
        let mut minor = 0;
        let mut micro = 0;

        ll::synth::fluid_version(&mut major, &mut minor, &mut micro);

        Version::new(
            major as _,
            minor as _,
            micro as _,
        )
    }
}

#[cfg(test)]
mod test {
    use crate::Version;

    #[test]
    fn version() {
        let ver = Version::get();

        assert_eq!(ver, Version::new(1, 2, 0));
        assert_eq!(ver.to_string(), "1.2.0");
    }
}
