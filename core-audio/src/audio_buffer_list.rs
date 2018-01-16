use ::*;
use foreign_types::ForeignTypeRef;
use std::{mem, ops, ptr, slice};

#[repr(C)]
struct ABLHeap {
    pub number_buffers: u32,
    pub buffers: [AudioBufferList; 0],
}

fn new_heap(len: usize) -> *mut ffi::AudioBufferList {
    let n_bytes = mem::size_of::<ABLHeap>() + len * mem::size_of::<AudioBuffer>();
    assert_eq!(n_bytes % mem::size_of::<u32>(), 0);
    let n_u32 = n_bytes / mem::size_of::<u32>();
    let mut vec = Vec::<u32>::with_capacity(n_u32);
    let ptr = vec.as_mut_ptr() as *mut ABLHeap;
    unsafe {
        let mut header: ABLHeap = mem::zeroed();
        header.number_buffers = len as _;
        mem::forget(vec);
        ptr::write(ptr, header);
    }
    ptr as _
}

unsafe fn delete_heap(ptr: *mut ffi::AudioBufferList) {
    let len = (*ptr).mNumberBuffers as usize;
    let n_bytes = mem::size_of::<ABLHeap>() + len * mem::size_of::<AudioBuffer>();
    let n_u32 = n_bytes / mem::size_of::<u32>();
    drop(Vec::<u32>::from_raw_parts(ptr as _, 0, n_u32))
}

foreign_type!{
    type CType = ffi::AudioBufferList;
    fn drop = delete_heap;
    pub struct AudioBufferList;
    pub struct AudioBufferListRef;
}

impl AudioBufferList {
    pub fn with_len(len: usize) -> Self { AudioBufferList(new_heap(len)) }
}

impl AudioBufferListRef {
    pub fn buffer_count(&self) -> usize {
        unsafe {
            let acl: &ffi::AudioBufferList = &*self.as_ptr();
            acl.mNumberBuffers as _
        }
    }
}

impl ops::Deref for AudioBufferListRef {
    type Target = [AudioBuffer];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let abl = &(*self.as_ptr());
            slice::from_raw_parts(&abl.mBuffers[0] as *const _ as *const _,
                                  abl.mNumberBuffers as _)
        }
    }
}

impl ops::DerefMut for AudioBufferListRef {
    fn deref_mut(&mut self) -> &mut [AudioBuffer] {
        unsafe {
            let abl = &mut (*self.as_ptr());
            slice::from_raw_parts_mut(&mut abl.mBuffers[0] as *mut _ as *mut _,
                                      abl.mNumberBuffers as _)
        }
    }
}
