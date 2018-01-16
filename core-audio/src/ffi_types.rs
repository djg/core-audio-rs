/// A macro to define a wrapper around a POD FFI type that lives on
/// the stack.
#[macro_export]
macro_rules! ffi_type_stack {
    ($(#[$impl_attr:meta])*
     type CType = $ctype:ty;
     $(#[$owned_attr:meta])*
     pub struct $owned:ident;
     $(#[$borrowed_attr:meta])*
     pub struct $borrowed:ident;
    ) => {
        $(#[$owned_attr])*
        pub struct $owned($ctype);

        $(#[$impl_attr])*
        impl ::foreign_types::ForeignType for $owned {
            type CType = $ctype;
            type Ref = $borrowed;

            unsafe fn from_ptr(ptr: *mut $ctype) -> $owned {
                $owned(*ptr)
            }

            fn as_ptr(&self) -> *mut Self::CType {
                &self.0 as *const _ as *mut _
            }
        }

        impl Clone for $owned {
            fn clone(&self) -> $owned {
                $owned(self.0.clone())
            }
        }

        impl ::std::borrow::ToOwned for $borrowed {
            type Owned = $owned;
            fn to_owned(&self) -> $owned {
                unsafe {
                    ::foreign_types::ForeignType::from_ptr(self.as_ptr())
                }
            }
        }

        impl ::std::ops::Deref for $owned {
            type Target = $borrowed;

            #[inline]
            fn deref(&self) -> &$borrowed {
                let ptr = &self as *const _ as *mut _;
                unsafe { ::foreign_types::ForeignTypeRef::from_ptr(ptr) }
            }
        }

        impl ::std::ops::DerefMut for $owned {
            #[inline]
            fn deref_mut(&mut self) -> &mut $borrowed {
                let ptr = &self as *const _ as *mut _;
                unsafe { ::foreign_types::ForeignTypeRef::from_ptr_mut(ptr) }
            }
        }

        impl ::std::borrow::Borrow<$borrowed> for $owned {
            #[inline]
            fn borrow(&self) -> &$borrowed {
                &**self
            }
        }

        impl ::std::convert::AsRef<$borrowed> for $owned {
            #[inline]
            fn as_ref(&self) -> &$borrowed {
                &**self
            }
        }

        $(#[$borrowed_attr])*
        pub struct $borrowed(::foreign_types::Opaque);

        $(#[$impl_attr])*
        impl ::foreign_types::ForeignTypeRef for $borrowed {
            type CType = $ctype;
        }
    }
}
