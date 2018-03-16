use {AudioChannelBitmap, AudioChannelDescription, AudioChannelLayoutTag};
use ffi;
use foreign_types::ForeignTypeRef;
use std::{mem, ops, ptr, slice};

#[repr(C)]
struct ACLHeap {
    pub channel_layout_tag: AudioChannelLayoutTag,
    pub channel_bitmap: AudioChannelBitmap,
    pub number_channel_descriptions: u32,
    pub channel_descriptions: [AudioChannelDescription; 0],
}

fn new_heap(len: usize) -> *mut ffi::AudioChannelLayout {
    let n_bytes = mem::size_of::<ACLHeap>() + len * mem::size_of::<AudioChannelDescription>();
    assert_eq!(n_bytes % mem::size_of::<u32>(), 0);
    let n_u32 = n_bytes / mem::size_of::<u32>();
    let mut vec = Vec::<u32>::with_capacity(n_u32);
    let ptr = vec.as_mut_ptr() as *mut ACLHeap;
    unsafe {
        let mut header: ACLHeap = mem::zeroed();
        header.number_channel_descriptions = len as _;
        mem::forget(vec);
        ptr::write(ptr, header);
    }
    ptr as _
}

unsafe fn delete_heap(ptr: *mut ffi::AudioChannelLayout) {
    let len = (*ptr).mNumberChannelDescriptions as usize;
    let n_bytes = mem::size_of::<ACLHeap>() + len * mem::size_of::<AudioChannelDescription>();
    let n_u32 = n_bytes / mem::size_of::<u32>();
    drop(Vec::<u32>::from_raw_parts(ptr as _, 0, n_u32))
}

foreign_type! {
    type CType = ffi::AudioChannelLayout;
    fn drop = delete_heap;
    pub struct AudioChannelLayout;
    pub struct AudioChannelLayoutRef;
}

impl AudioChannelLayout {
    pub fn with_len(len: usize) -> Self {
        AudioChannelLayout(new_heap(len))
    }
}

impl ops::Deref for AudioChannelLayoutRef {
    type Target = [AudioChannelDescription];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let acl: &ffi::AudioChannelLayout = &*self.as_ptr();
            slice::from_raw_parts(
                &acl.mChannelDescriptions[0] as *const _ as *const _,
                acl.mNumberChannelDescriptions as _,
            )
        }
    }
}

impl ops::DerefMut for AudioChannelLayoutRef {
    fn deref_mut(&mut self) -> &mut [AudioChannelDescription] {
        unsafe {
            let acl: &ffi::AudioChannelLayout = &*self.as_ptr();
            slice::from_raw_parts_mut(
                &acl.mChannelDescriptions[0] as *const _ as *mut _,
                acl.mNumberChannelDescriptions as _,
            )
        }
    }
}
