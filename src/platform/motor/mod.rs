use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    AlreadyRegistered,
    Runtime(moto_rt::Error),
}

impl Error {
    pub const EEXIST: Error = Error::AlreadyRegistered;
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AlreadyRegistered => write!(f, "Ctrl-C handler already registered"),
            Self::Runtime(error) => write!(f, "Motor Ctrl-C runtime error: {error}"),
        }
    }
}

impl std::error::Error for Error {}

/// Native signal identifier used by the crate's portable API.
pub type Signal = i32;

pub enum Registration {
    Active(u64),
    Dormant,
}

pub fn register_handler() -> Result<Registration, Error> {
    match moto_rt::process::ctrl_c_register_handler() {
        Ok(baseline) => Ok(Registration::Active(baseline)),
        Err(moto_rt::Error::NotFound) => Ok(Registration::Dormant),
        Err(moto_rt::Error::AlreadyInUse) => Err(Error::AlreadyRegistered),
        Err(error) => Err(Error::Runtime(error)),
    }
}

pub fn wait(last: u64) -> Result<u64, Error> {
    moto_rt::process::ctrl_c_wait(last).map_err(Error::Runtime)
}
