use ffi::OSStatus;
use error::Error;

macro_rules! try_call {
    ($($p:ident)::* ($($e:expr),* $(,)*)) => ({
        match ::call::try(call!($($p)::*($($e),*))) { 
            Ok(o) => o,
            Err(e) => return Err(e).into()
        }
    })
}

pub fn try(ret: OSStatus) -> Result<(), Error> {
    match ret {
        0 => Ok(()),
        e => Err(Error::from_osstatus(e)),
    }
}
