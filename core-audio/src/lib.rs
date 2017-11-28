#[macro_use]
extern crate bitflags;
pub extern crate core_audio_sys as ffi;
extern crate core_foundation;
extern crate libc;

#[macro_use]
mod call;

mod error;
mod core_audio_types;
mod audio_hardware;
mod host_time;

pub type Result<T> = ::std::result::Result<T, error::Error>;

pub use core_audio_types::*;
pub use audio_hardware::*;
pub use host_time::*;

bitflags! {
    pub struct AudioChannelBitmap: ffi::AudioChannelBitmap {
        const LEFT                  = ffi::kAudioChannelBit_Left;
        const RIGHT                 = ffi::kAudioChannelBit_Right;
        const CENTER                = ffi::kAudioChannelBit_Center;
        const LFE_SCREEN            = ffi::kAudioChannelBit_LFEScreen;
        const LEFT_SURROUND         = ffi::kAudioChannelBit_LeftSurround;
        const RIGHT_SURROUND        = ffi::kAudioChannelBit_RightSurround;
        const LEFT_CENTER           = ffi::kAudioChannelBit_LeftCenter;
        const RIGHT_CENTER          = ffi::kAudioChannelBit_RightCenter;
        const CENTER_SURROUND       = ffi::kAudioChannelBit_CenterSurround;
        const LEFT_SURROUND_DIRECT  = ffi::kAudioChannelBit_LeftSurroundDirect;
        const RIGHT_SURROUND_DIRECT = ffi::kAudioChannelBit_RightSurroundDirect;
        const TOP_CENTER_SURROUND   = ffi::kAudioChannelBit_TopCenterSurround;
        const VERTICAL_HEIGHT_LEFT  = ffi::kAudioChannelBit_VerticalHeightLeft;
        const VERTICAL_HEIGHT_CENTER = ffi::kAudioChannelBit_VerticalHeightCenter;
        const VERTICAL_HEIGHT_RIGHT = ffi::kAudioChannelBit_VerticalHeightRight;
        const TOP_BACK_LEFT         = ffi::kAudioChannelBit_TopBackLeft;
        const TOP_BACK_CENTER       = ffi::kAudioChannelBit_TopBackCenter;
        const TOP_BACK_RIGHT        = ffi::kAudioChannelBit_TopBackRight;
    }
}

bitflags! {
    pub struct AudioChannelFlags: ffi::AudioChannelFlags {
        const ALL_OFF = ffi::kAudioChannelFlags_AllOff;
        const RECTANGULAR_COORDINATES = ffi::kAudioChannelFlags_RectangularCoordinates;
        const SPHERICAL_COORDINATES = ffi::kAudioChannelFlags_SphericalCoordinates;
        const METERS = ffi::kAudioChannelFlags_Meters;
    }
}

