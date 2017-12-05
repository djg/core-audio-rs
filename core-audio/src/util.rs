/// Types that can be used as the backing store for a C FFI VLA
use std::{mem, ptr};
use ffi_binding::Binding;

const INLINE_SIZE: usize = 8;

unsafe trait Array {
    type Item;
    fn size() -> usize;
}

macro_rules! impl_array(
    ($($size:expr),+) => {
        $(
            unsafe impl<T> Array for [T; $size] {
                type Item = T;
                fn size() -> usize { $size }
            }
        )+
    }
);

impl_array!(1, 2, 3, 4, 5, 6, 7, 8);

// Rust is free to reorganise the fields in a struct.  `StackVLA` and
// `HeapVLA` exist to enforce the packing requirement for interop with
// C FFI.
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct StackVLA<T> {
    pub len: u32,
    pub array: [T;INLINE_SIZE],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct HeapVLA<T> {
    pub len: u32,
    pub array: [T;0],
}

pub enum VariableLengthArray<R,T> {
    Borrowed(*mut R),
    Inline(StackVLA<T>),
    Heap(*mut HeapVLA<T>),
}

use self::VariableLengthArray::{Borrowed, Inline, Heap};

impl<R,T> VariableLengthArray<R,T> {
    pub fn with_len(len: usize) -> Self {
        if len <= INLINE_SIZE {
            Inline(StackVLA { len: len as _, array: unsafe { mem::zeroed() } })
        } else {
            let n_bytes = mem::size_of::<HeapVLA<T>>() + len as usize * mem::size_of::<T>();
            assert_eq!(n_bytes % mem::size_of::<usize>(), 0);
            let n_usize = n_bytes / mem::size_of::<usize>();
            let mut vec = Vec::<usize>::with_capacity(n_usize);
            let ptr = vec.as_mut_ptr() as *mut HeapVLA<T>;
            unsafe {
                mem::forget(vec);
                ptr::write(ptr, HeapVLA { len: len as _, array:[] });
            }
            Heap(ptr)
        }
    }
}

impl<R,T> Binding for VariableLengthArray<R,T> {
    type Ffi = *mut R;
    
    fn as_ffi(&self) -> Self::Ffi {
        match *self {
            Borrowed(raw) => raw,
            Inline(ref inline) => inline as *const _ as *mut _,
            Heap(ptr) => ptr as *mut _,
        }
    }

    unsafe fn from_ffi(ffi: Self::Ffi) -> Self {
        debug_assert!(!ffi.is_null());
        Borrowed(ffi)
    }
}

impl<R,T> Drop for VariableLengthArray<R,T> {
    fn drop(&mut self) {
        unsafe {
            match *self {
                Borrowed(..) => {},
                ref mut inline @ Inline(..) => {
                    // Inhibit the array destructor.
                    ptr::write(inline, Heap(ptr::null_mut()));
                },
                Heap(ptr) => deallocate(ptr as *mut usize, (*ptr).len as _)
            }
        }
    }
}

unsafe fn deallocate<T>(ptr: *mut T, capacity: usize) {
    drop(Vec::<T>::from_raw_parts(ptr, 0, capacity))
}
