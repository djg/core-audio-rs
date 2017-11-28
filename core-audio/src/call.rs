use ffi;
use error::Error;

macro_rules! call {
    (ffi::$p:ident ($($e:expr),*)) => (
        ffi::$p($(::call::convert(&$e)),*)
    )
}

macro_rules! try_call {
    (ffi::$p:ident ($($e:expr),*)) => ({
        match ::call::try(ffi::$p($(::call::convert(&$e)),*)) { 
            Ok(o) => o,
            Err(e) => return Err(e).into()
        }
    })
}

pub fn try(ret: ffi::OSStatus) -> Result<(), Error> {
    match ret {
        0 => Ok(()),
        e => Err(Error::from_osstatus(e))
    }
}

#[doc(hidden)]
pub trait IsNull {
    fn is_ptr_null(&self) -> bool;
}

impl<T> IsNull for *const T {
    fn is_ptr_null(&self) -> bool {
        self.is_null()
    }
}

impl<T> IsNull for *mut T {
    fn is_ptr_null(&self) -> bool {
        self.is_null()
    }
}

#[doc(hidden)]
pub trait Binding: Sized {
    type Ffi;

    fn as_ffi(&self) -> Self::Ffi;

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self;
    unsafe fn from_ffi_opt<T>(ffi: T) -> Option<Self>
        where T: Copy + IsNull,
              Self: Binding<Ffi=T>
    {
        if ffi.is_ptr_null() {
            None
        } else {
            Some(Binding::from_ffi(ffi))
        }
    }
}

#[doc(hidden)]
pub trait Convert<T> {
    fn convert(&self) -> T;
}

pub fn convert<T, U>(u: &U) -> T
    where U: Convert<T>
{
    u.convert()
}

mod impls {
    use call::{Convert};
    
    impl<T> Convert<T> for T
        where T: Copy
    {
        fn convert(&self) -> T {
            *self
        }
    }

    impl<'a, T> Convert<*const T> for &'a T {
        fn convert(&self) -> *const T {
            &**self as *const _
        }
    }

    impl<'a, T> Convert<*const T> for *mut T {
        fn convert(&self) -> *const T {
            *self as *const _
        }
    }

    impl<'a, T> Convert<*mut T> for &'a mut T {
        fn convert(&self) -> *mut T {
            &**self as *const _ as *mut _
        }
    }
}
