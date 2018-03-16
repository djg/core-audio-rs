#![allow(bad_style, improper_ctypes)]

extern crate core_audio_sys;

use core_audio_sys::*;
use std::os::raw::c_double;

include!(concat!(env!("OUT_DIR"), "/all.rs"));
