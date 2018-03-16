use ffi;
use std::{error, fmt};

pub struct Error(ffi::OSStatus);

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ErrorKind {
    NotRunning,
    Unspecified,
    UnknownProperty,
    BadPropertySize,
    IllegalOperation,
    BadObject,
    BadDevice,
    BadStream,
    UnsupportedOperation,
    UnsupportedFormat,
    Permissions,
    Other,
}

impl ErrorKind {
    fn as_str(&self) -> &'static str {
        match *self {
            ErrorKind::NotRunning =>
                "The function call requires that the hardware be running but it isn't.",
            ErrorKind::Unspecified =>
                "The function call failed while doing something that doesn't provide any error messages.",
            ErrorKind::UnknownProperty =>
                "The AudioObject doesn't know about the property at the given address.",
            ErrorKind::BadPropertySize =>
                "An improperly sized buffer was provided when accessing the data of a property.",
            ErrorKind::IllegalOperation =>
                "The requested operation couldn't be completed.",
            ErrorKind::BadObject =>
                "The AudioObjectID passed to the function doesn't map to a valid AudioObject.",
            ErrorKind::BadDevice =>
                "The AudioObjectID passed to the function doesn't map to a valid AudioDevice.",
            ErrorKind::BadStream =>
                "The AudioObjectID passed to the function doesn't map to a valid AudioStream.",
            ErrorKind::UnsupportedOperation =>
                "The AudioObject doesn't support the requested operation.",
            ErrorKind::UnsupportedFormat =>
                "The AudioStream doesn't support the requested format.",
            ErrorKind::Permissions =>
                "The requested operation can't be completed because the process doesn't have permission.",
            ErrorKind::Other =>
                "Unspecified error.",
        }
    }
}

impl Into<ffi::OSStatus> for ErrorKind {
    fn into(self) -> ffi::OSStatus {
        match self {
            ErrorKind::NotRunning => ffi::kAudioHardwareNotRunningError,
            ErrorKind::Unspecified => ffi::kAudioHardwareUnspecifiedError,
            ErrorKind::UnknownProperty => ffi::kAudioHardwareUnknownPropertyError,
            ErrorKind::BadPropertySize => ffi::kAudioHardwareBadPropertySizeError,
            ErrorKind::IllegalOperation => ffi::kAudioHardwareIllegalOperationError,
            ErrorKind::BadObject => ffi::kAudioHardwareBadObjectError,
            ErrorKind::BadDevice => ffi::kAudioHardwareBadDeviceError,
            ErrorKind::BadStream => ffi::kAudioHardwareBadStreamError,
            ErrorKind::UnsupportedOperation => ffi::kAudioHardwareUnsupportedOperationError,
            ErrorKind::UnsupportedFormat => ffi::kAudioDeviceUnsupportedFormatError,
            ErrorKind::Permissions => ffi::kAudioDevicePermissionsError,
            ErrorKind::Other => panic!(),
        }
    }
}

impl From<ErrorKind> for Error {
    #[inline]
    fn from(kind: ErrorKind) -> Error {
        Error(kind.into())
    }
}

impl Error {
    pub fn from_osstatus(err: ffi::OSStatus) -> Self {
        Error(err)
    }

    pub fn raw_osstatus(&self) -> ffi::OSStatus {
        self.0
    }

    pub fn kind(&self) -> ErrorKind {
        decode_error_kind(self.0)
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("OSStatus")
            .field("err", &self.0)
            .field("message", &self.kind())
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        let detail = self.kind().as_str();
        write!(fmt, "{} (os error {})", detail, self.0)
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        self.kind().as_str()
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

fn decode_error_kind(err: ffi::OSStatus) -> ErrorKind {
    match err {
        ffi::kAudioHardwareNotRunningError => ErrorKind::NotRunning,
        ffi::kAudioHardwareUnspecifiedError => ErrorKind::Unspecified,
        ffi::kAudioHardwareUnknownPropertyError => ErrorKind::UnknownProperty,
        ffi::kAudioHardwareBadPropertySizeError => ErrorKind::BadPropertySize,
        ffi::kAudioHardwareIllegalOperationError => ErrorKind::IllegalOperation,
        ffi::kAudioHardwareBadObjectError => ErrorKind::BadObject,
        ffi::kAudioHardwareBadDeviceError => ErrorKind::BadDevice,
        ffi::kAudioHardwareBadStreamError => ErrorKind::BadStream,
        ffi::kAudioHardwareUnsupportedOperationError => ErrorKind::UnsupportedOperation,
        ffi::kAudioDeviceUnsupportedFormatError => ErrorKind::UnsupportedFormat,
        ffi::kAudioDevicePermissionsError => ErrorKind::Permissions,

        _ => ErrorKind::Other,
    }
}
