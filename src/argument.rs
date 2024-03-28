use std::ffi::{OsStr, OsString};
use std::path::PathBuf;

/// An individual argument
#[allow(clippy::exhaustive_enums)]
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Argument {
    /// Literal argument to use
    PassThrough(OsString),
    /// File to read more arguments from
    Path(PathBuf),
}

impl Argument {
    /// Parse an individual argument
    ///
    /// See [PREFIX][crate::PREFIX]
    pub fn parse(arg: impl Into<OsString>, prefix: char) -> Self {
        let arg = arg.into();
        let s = os_str_bytes::RawOsStr::new(&arg);
        if let Some(path) = s.strip_prefix(prefix) {
            Self::Path(path.to_os_str().into_owned().into())
        } else {
            Self::PassThrough(arg)
        }
    }

    /// Parse an individual argument
    ///
    /// See [PREFIX][crate::PREFIX]
    pub fn parse_ref(arg: impl AsRef<OsStr>, prefix: char) -> Self {
        let arg = arg.as_ref();
        let s = os_str_bytes::RawOsStr::new(arg);
        if let Some(path) = s.strip_prefix(prefix) {
            Self::Path(path.to_os_str().into_owned().into())
        } else {
            Self::PassThrough(arg.to_owned())
        }
    }
}
