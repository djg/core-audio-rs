use error::Error;
use ffi::OSStatus;

pub fn cvt_r(ret: OSStatus) -> Result<(), Error> {
    match ret {
        0 => Ok(()),
        e => Err(Error::from_osstatus(e)),
    }
}
