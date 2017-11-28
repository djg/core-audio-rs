use super::*;

use std::default::Default;
use std::mem;
use std::os::raw::c_void;
use std::slice;

//==================================================================================================
// General Error Codes

e! {
    enum OSStatus {
        kAudio_UnimplementedError     = -4,
        kAudio_FileNotFoundError      = -43,
        kAudio_FilePermissionError    = -54,
        kAudio_TooManyFilesOpenError  = -42,
        kAudio_BadFilePathError       = fourcc!(b"!pth") as _,
        kAudio_ParamError             = -50,
        kAudio_MemFullError           = -108,
    }
}
      
// AudioValueRange

s! {
    #[derive(Clone, Copy)]
    struct AudioValueRange
    {
        pub mMinimum: f64,
        pub mMaximum: f64,
    }
}

//==================================================================================================
// AudioValueTranslation

s! {
    #[derive(Clone, Copy)]
    struct AudioValueTranslation {
        pub mInputData: *mut c_void,
        pub mInputDataSize: u32,
        pub mOutputData: *mut c_void,
        pub mOutputDataSize: u32,
    }
}

//==================================================================================================
// AudioBuffer/AudioBufferList

s! {
    #[derive(Clone, Copy)]
    struct AudioBuffer {
        pub mNumberChannels: u32,
        pub mDataByteSize: u32,
        pub mData: *mut c_void,
    }

    struct AudioBufferList {
        pub mNumberBuffers: u32,
        pub mBuffers: [AudioBuffer;1],
    }
}

//==================================================================================================
// Audio Formats

cfg_if! {
    if #[cfg(not(feature = "prefer-fixed-point"))] {
        pub type AudioSampleType = f32;
        pub type AudioUnitSampleType = f32;
    } else {
        pub type AudioSampleType = i16;
        pub type AudioUnitSampleType = i32;
        pub const kAudioUnitSampleFractionBits: u32 = 24;
    }
}

pub type AudioFormatID = u32;
pub type AudioFormatFlags = u32;

s! {
    #[derive(Clone, Copy)]
    struct AudioStreamBasicDescription {
        pub mSampleRate: f64,
        pub mFormatID: AudioFormatID,
        pub mFormatFlags: AudioFormatFlags,
        pub mBytesPerPacket: u32,
        pub mFramesPerPacket: u32,
        pub mBytesPerFrame: u32,
        pub mChannelsPerFrame: u32,
        pub mBitsPerChannel: u32,
            mReserved: u32,
    }
}
pub const kAudioStreamAnyRate: f64 = 0.0;

e! {
    enum AudioFormatID {
        kAudioFormatLinearPCM               = fourcc!(b"lpcm"),
        kAudioFormatAC3                     = fourcc!(b"ac-3"),
        kAudioFormat60958AC3                = fourcc!(b"cac3"),
        kAudioFormatAppleIMA4               = fourcc!(b"ima4"),
        kAudioFormatMPEG4AAC                = fourcc!(b"aac "),
        kAudioFormatMPEG4CELP               = fourcc!(b"celp"),
        kAudioFormatMPEG4HVXC               = fourcc!(b"hvxc"),
        kAudioFormatMPEG4TwinVQ             = fourcc!(b"twvq"),
        kAudioFormatMACE3                   = fourcc!(b"MAC3"),
        kAudioFormatMACE6                   = fourcc!(b"MAC6"),
        kAudioFormatULaw                    = fourcc!(b"ulaw"),
        kAudioFormatALaw                    = fourcc!(b"alaw"),
        kAudioFormatQDesign                 = fourcc!(b"QDMC"),
        kAudioFormatQDesign2                = fourcc!(b"QDM2"),
        kAudioFormatQUALCOMM                = fourcc!(b"Qclp"),
        kAudioFormatMPEGLayer1              = fourcc!(b".mp1"),
        kAudioFormatMPEGLayer2              = fourcc!(b".mp2"),
        kAudioFormatMPEGLayer3              = fourcc!(b".mp3"),
        kAudioFormatTimeCode                = fourcc!(b"time"),
        kAudioFormatMIDIStream              = fourcc!(b"midi"),
        kAudioFormatParameterValueStream    = fourcc!(b"apvs"),
        kAudioFormatAppleLossless           = fourcc!(b"alac"),
        kAudioFormatMPEG4AAC_HE             = fourcc!(b"aach"),
        kAudioFormatMPEG4AAC_LD             = fourcc!(b"aacl"),
        kAudioFormatMPEG4AAC_ELD            = fourcc!(b"aace"),
        kAudioFormatMPEG4AAC_ELD_SBR        = fourcc!(b"aacf"),
        kAudioFormatMPEG4AAC_ELD_V2         = fourcc!(b"aacg"),    
        kAudioFormatMPEG4AAC_HE_V2          = fourcc!(b"aacp"),
        kAudioFormatMPEG4AAC_Spatial        = fourcc!(b"aacs"),
        kAudioFormatAMR                     = fourcc!(b"samr"),
        kAudioFormatAMR_WB                  = fourcc!(b"sawb"),
        kAudioFormatAudible                 = fourcc!(b"AUDB"),
        kAudioFormatiLBC                    = fourcc!(b"ilbc"),
        kAudioFormatDVIIntelIMA             = 0x6D730011,
        kAudioFormatMicrosoftGSM            = 0x6D730031,
        kAudioFormatAES3                    = fourcc!(b"aes3"),
        kAudioFormatEnhancedAC3             = fourcc!(b"ec-3"),
        kAudioFormatFLAC                    = fourcc!(b"flac"),
        kAudioFormatOpus                    = fourcc!(b"opus"),
    }
}

e! {
    enum AudioFormatFlags {
        kAudioFormatFlagIsFloat                  = (1 << 0),     // 0x1
        kAudioFormatFlagIsBigEndian              = (1 << 1),     // 0x2
        kAudioFormatFlagIsSignedInteger          = (1 << 2),     // 0x4
        kAudioFormatFlagIsPacked                 = (1 << 3),     // 0x8
        kAudioFormatFlagIsAlignedHigh            = (1 << 4),     // 0x10
        kAudioFormatFlagIsNonInterleaved         = (1 << 5),     // 0x20
        kAudioFormatFlagIsNonMixable             = (1 << 6),     // 0x40
        kAudioFormatFlagsAreAllClear             = 0x80000000,
    
        kLinearPCMFormatFlagIsFloat              = kAudioFormatFlagIsFloat,
        kLinearPCMFormatFlagIsBigEndian          = kAudioFormatFlagIsBigEndian,
        kLinearPCMFormatFlagIsSignedInteger      = kAudioFormatFlagIsSignedInteger,
        kLinearPCMFormatFlagIsPacked             = kAudioFormatFlagIsPacked,
        kLinearPCMFormatFlagIsAlignedHigh        = kAudioFormatFlagIsAlignedHigh,
        kLinearPCMFormatFlagIsNonInterleaved     = kAudioFormatFlagIsNonInterleaved,
        kLinearPCMFormatFlagIsNonMixable         = kAudioFormatFlagIsNonMixable,
        kLinearPCMFormatFlagsSampleFractionShift = 7,
        kLinearPCMFormatFlagsSampleFractionMask  = (0x3F << kLinearPCMFormatFlagsSampleFractionShift),
        kLinearPCMFormatFlagsAreAllClear         = kAudioFormatFlagsAreAllClear,
    
        kAppleLosslessFormatFlag_16BitSourceData = 1,
        kAppleLosslessFormatFlag_20BitSourceData = 2,
        kAppleLosslessFormatFlag_24BitSourceData = 3,
        kAppleLosslessFormatFlag_32BitSourceData = 4,
    }
}


cfg_if! {
    if #[cfg(target_endian = "big")] {
        e! {
            enum AudioFormatFlags {
                kAudioFormatFlagsNativeEndian       = kAudioFormatFlagIsBigEndian,
            }
        }
    } else {
        e! {
            enum AudioFormatFlags {
                kAudioFormatFlagsNativeEndian       = 0,
            }
        }
    }
}

cfg_if! {
    if #[cfg(not(feature = "prefer-fixed-point"))] {
        e! {
            enum AudioFormatFlags {
                kAudioFormatFlagsCanonical          = kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked,
                kAudioFormatFlagsAudioUnitCanonical = kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked | kAudioFormatFlagIsNonInterleaved,
            }
        }
    } else {
        e! {
            enum AudioFormatFlags {
                kAudioFormatFlagsCanonical          = kAudioFormatFlagIsSignedInteger | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked,
                kAudioFormatFlagsAudioUnitCanonical = kAudioFormatFlagIsSignedInteger | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked | kAudioFormatFlagIsNonInterleaved | (kAudioUnitSampleFractionBits << kLinearPCMFormatFlagsSampleFractionShift),
#endif
             kAudioFormatFlagsNativeFloatPacked  = kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked,
            }
        }
    }
}

#[inline]
pub fn IsAudioFormatNativeEndian(f: &AudioStreamBasicDescription) -> bool {
    f.mFormatID == kAudioFormatLinearPCM &&
        (f.mFormatFlags & kAudioFormatFlagIsBigEndian) == kAudioFormatFlagsNativeEndian
}

#[inline]
pub fn CalculateLPCMFlags(inValidBitsPerChannel: u32,
                          inTotalBitsPerChannel: u32,
                          inIsFloat: bool,
                          inIsBigEndian: bool,
                          inIsNonInterleaved: bool) -> AudioFormatFlags {
    (match inIsFloat { true => kAudioFormatFlagIsFloat, _ => kAudioFormatFlagIsSignedInteger }) |
    (match inIsBigEndian { true => kAudioFormatFlagIsBigEndian, _ => 0 }) |
    (if inValidBitsPerChannel == inTotalBitsPerChannel {
        kAudioFormatFlagIsPacked
    } else {
        kAudioFormatFlagIsAlignedHigh
    }) |
    (match inIsNonInterleaved { true => kAudioFormatFlagIsNonInterleaved, _ =>  0 })
}

#[inline]
pub fn FillOutASBDForLPCM(outASBD: &mut AudioStreamBasicDescription,
                          inSampleRate: f64,
                          inChannelsPerFrame: u32,
                          inValidBitsPerChannel: u32,
                          inTotalBitsPerChannel: u32,
                          inIsFloat: bool,
                          inIsBigEndian: bool,
                          inIsNonInterleaved: bool)
{
    outASBD.mSampleRate = inSampleRate;
    outASBD.mFormatID = kAudioFormatLinearPCM;
    outASBD.mFormatFlags = CalculateLPCMFlags(inValidBitsPerChannel, inTotalBitsPerChannel, inIsFloat, inIsBigEndian, inIsNonInterleaved);
    outASBD.mBytesPerPacket = match inIsNonInterleaved { true => 1, _ => inChannelsPerFrame } * (inTotalBitsPerChannel / 8);
    outASBD.mFramesPerPacket = 1;
    outASBD.mBytesPerFrame = match inIsNonInterleaved { true => 1, _ => inChannelsPerFrame } * (inTotalBitsPerChannel / 8);
    outASBD.mChannelsPerFrame = inChannelsPerFrame;
    outASBD.mBitsPerChannel = inValidBitsPerChannel;
}

s! {
    #[derive(Clone, Copy)]
    struct  AudioStreamPacketDescription {
        pub mStartOffset: i64,
        pub mVariableFramesInPacket: u32,
        pub mDataByteSize: u32,
    }
}

//==================================================================================================
// Audio Time Stamps

e! {
    enum SMPTETimeType: u32 {
        kSMPTETimeType24        = 0,
        kSMPTETimeType25        = 1,
        kSMPTETimeType30Drop    = 2,
        kSMPTETimeType30        = 3,
        kSMPTETimeType2997      = 4,
        kSMPTETimeType2997Drop  = 5,
        kSMPTETimeType60        = 6,
        kSMPTETimeType5994      = 7,
        kSMPTETimeType60Drop    = 8,
        kSMPTETimeType5994Drop  = 9,
        kSMPTETimeType50        = 10,
        kSMPTETimeType2398      = 11,
    }
}

e! {
    enum SMPTETimeFlags: u32 {
        kSMPTETimeUnknown   = 0,
        kSMPTETimeValid     = (1 << 0),
        kSMPTETimeRunning   = (1 << 1),
    }
}


s! {
    #[derive(Clone, Copy)]
    struct SMPTETime {
        pub mSubframes: i16,
        pub mSubframeDivisor: i16,
        pub mCounter: u32,
        pub mType: SMPTETimeType,
        pub mFlags: SMPTETimeFlags,
        pub mHours: i16,
        pub mMinutes: i16,
        pub mSeconds: i16,
        pub mFrames: i16,
    }
}

e! {
    enum AudioTimeStampFlags: u32 {
        kAudioTimeStampNothingValid         = 0,
        kAudioTimeStampSampleTimeValid      = (1 << 0),
        kAudioTimeStampHostTimeValid        = (1 << 1),
        kAudioTimeStampRateScalarValid      = (1 << 2),
        kAudioTimeStampWordClockTimeValid   = (1 << 3),
        kAudioTimeStampSMPTETimeValid       = (1 << 4),
        kAudioTimeStampSampleHostTimeValid  = (kAudioTimeStampSampleTimeValid | kAudioTimeStampHostTimeValid),
    }
}

s! {
    #[derive(Clone, Copy)]
    struct AudioTimeStamp {
        pub mSampleTime: f64,
        pub mHostTime: u64,
        pub mRateScalar: f64,
        pub mWordClockTime: u64,
        pub mSMPTETime: SMPTETime,
        pub mFlags: AudioTimeStampFlags,
        pub mReserved: u32,
    }
}

pub fn FillOutAudioTimeStampWithSampleTime(outATS: &mut AudioTimeStamp, inSampleTime: f64) {
    outATS.mSampleTime = inSampleTime;
    outATS.mHostTime = 0;
    outATS.mRateScalar = 0.0;
    outATS.mWordClockTime = 0;
    outATS.mSMPTETime = Default::default();
    outATS.mFlags = kAudioTimeStampSampleTimeValid;
}

pub fn FillOutAudioTimeStampWithHostTime(outATS: &mut AudioTimeStamp, inHostTime: u64) {
    outATS.mSampleTime = 0.0;
    outATS.mHostTime = inHostTime;
    outATS.mRateScalar = 0.0;
    outATS.mWordClockTime = 0;
    outATS.mSMPTETime = Default::default();
    outATS.mFlags = kAudioTimeStampHostTimeValid;
}

pub fn FillOutAudioTimeStampWithSampleAndHostTime(outATS: &mut AudioTimeStamp,
                                                  inSampleTime: f64,
                                                  inHostTime: u64) {
    outATS.mSampleTime = inSampleTime;
    outATS.mHostTime = inHostTime;
    outATS.mRateScalar = 0.0;
    outATS.mWordClockTime = 0;
    outATS.mSMPTETime = Default::default();
    outATS.mFlags = kAudioTimeStampSampleTimeValid | kAudioTimeStampHostTimeValid;
}

//==================================================================================================
// AudioClassDescription

s! {
    struct AudioClassDescription {
        pub mType:  OSType,
        pub mSubType: OSType,
        pub mManufacturer:  OSType,
    }
}

//==================================================================================================
// Audio Channel Layout

pub type AudioChannelLabel = u32;
pub type AudioChannelLayoutTag = u32;

e! {
    enum AudioChannelLabel {
        kAudioChannelLabel_Unknown                  = 0xFFFFFFFF,   // unknown or unspecified other use
        kAudioChannelLabel_Unused                   = 0,            // channel is present, but has no intended use or destination
        kAudioChannelLabel_UseCoordinates           = 100,          // channel is described by the mCoordinates fields.
        
        kAudioChannelLabel_Left                     = 1,
        kAudioChannelLabel_Right                    = 2,
        kAudioChannelLabel_Center                   = 3,
        kAudioChannelLabel_LFEScreen                = 4,
        kAudioChannelLabel_LeftSurround             = 5,            // WAVE: "Back Left"
        kAudioChannelLabel_RightSurround            = 6,            // WAVE: "Back Right"
        kAudioChannelLabel_LeftCenter               = 7,
        kAudioChannelLabel_RightCenter              = 8,
        kAudioChannelLabel_CenterSurround           = 9,            // WAVE: "Back Center" or plain "Rear Surround"
        kAudioChannelLabel_LeftSurroundDirect       = 10,           // WAVE: "Side Left"
        kAudioChannelLabel_RightSurroundDirect      = 11,           // WAVE: "Side Right"
        kAudioChannelLabel_TopCenterSurround        = 12,
        kAudioChannelLabel_VerticalHeightLeft       = 13,           // WAVE: "Top Front Left"
        kAudioChannelLabel_VerticalHeightCenter     = 14,           // WAVE: "Top Front Center"
        kAudioChannelLabel_VerticalHeightRight      = 15,           // WAVE: "Top Front Right"
        
        kAudioChannelLabel_TopBackLeft              = 16,
        kAudioChannelLabel_TopBackCenter            = 17,
        kAudioChannelLabel_TopBackRight             = 18,
        
        kAudioChannelLabel_RearSurroundLeft         = 33,
        kAudioChannelLabel_RearSurroundRight        = 34,
        kAudioChannelLabel_LeftWide                 = 35,
        kAudioChannelLabel_RightWide                = 36,
        kAudioChannelLabel_LFE2                     = 37,
        kAudioChannelLabel_LeftTotal                = 38,           // matrix encoded 4 channels
        kAudioChannelLabel_RightTotal               = 39,           // matrix encoded 4 channels
        kAudioChannelLabel_HearingImpaired          = 40,
        kAudioChannelLabel_Narration                = 41,
        kAudioChannelLabel_Mono                     = 42,
        kAudioChannelLabel_DialogCentricMix         = 43,
        
        kAudioChannelLabel_CenterSurroundDirect     = 44,           // back center, non diffuse
    
        kAudioChannelLabel_Haptic                   = 45,

        // first order ambisonic channels
        kAudioChannelLabel_Ambisonic_W              = 200,
        kAudioChannelLabel_Ambisonic_X              = 201,
        kAudioChannelLabel_Ambisonic_Y              = 202,
        kAudioChannelLabel_Ambisonic_Z              = 203,
        
        // Mid/Side Recording
        kAudioChannelLabel_MS_Mid                   = 204,
        kAudioChannelLabel_MS_Side                  = 205,

        // X-Y Recording
        kAudioChannelLabel_XY_X                     = 206,
        kAudioChannelLabel_XY_Y                     = 207,

        // other
        kAudioChannelLabel_HeadphonesLeft           = 301,
        kAudioChannelLabel_HeadphonesRight          = 302,
        kAudioChannelLabel_ClickTrack               = 304,
        kAudioChannelLabel_ForeignLanguage          = 305,
        
        // generic discrete channel
        kAudioChannelLabel_Discrete                 = 400,

        // numbered discrete channel
        kAudioChannelLabel_Discrete_0               = (1<<16) | 0,
        kAudioChannelLabel_Discrete_1               = (1<<16) | 1,
        kAudioChannelLabel_Discrete_2               = (1<<16) | 2,
        kAudioChannelLabel_Discrete_3               = (1<<16) | 3,
        kAudioChannelLabel_Discrete_4               = (1<<16) | 4,
        kAudioChannelLabel_Discrete_5               = (1<<16) | 5,
        kAudioChannelLabel_Discrete_6               = (1<<16) | 6,
        kAudioChannelLabel_Discrete_7               = (1<<16) | 7,
        kAudioChannelLabel_Discrete_8               = (1<<16) | 8,
        kAudioChannelLabel_Discrete_9               = (1<<16) | 9,
        kAudioChannelLabel_Discrete_10              = (1<<16) | 10,
        kAudioChannelLabel_Discrete_11              = (1<<16) | 11,
        kAudioChannelLabel_Discrete_12              = (1<<16) | 12,
        kAudioChannelLabel_Discrete_13              = (1<<16) | 13,
        kAudioChannelLabel_Discrete_14              = (1<<16) | 14,
        kAudioChannelLabel_Discrete_15              = (1<<16) | 15,
        kAudioChannelLabel_Discrete_65535           = (1<<16) | 65535,
    
        // generic HOA ACN channel
        kAudioChannelLabel_HOA_ACN                  = 500,
    
        // numbered HOA ACN channels
        kAudioChannelLabel_HOA_ACN_0                = (2 << 16) | 0,
        kAudioChannelLabel_HOA_ACN_1                = (2 << 16) | 1,
        kAudioChannelLabel_HOA_ACN_2                = (2 << 16) | 2,
        kAudioChannelLabel_HOA_ACN_3                = (2 << 16) | 3,
        kAudioChannelLabel_HOA_ACN_4                = (2 << 16) | 4,
        kAudioChannelLabel_HOA_ACN_5                = (2 << 16) | 5,
        kAudioChannelLabel_HOA_ACN_6                = (2 << 16) | 6,
        kAudioChannelLabel_HOA_ACN_7                = (2 << 16) | 7,
        kAudioChannelLabel_HOA_ACN_8                = (2 << 16) | 8,
        kAudioChannelLabel_HOA_ACN_9                = (2 << 16) | 9,
        kAudioChannelLabel_HOA_ACN_10               = (2 << 16) | 10,
        kAudioChannelLabel_HOA_ACN_11               = (2 << 16) | 11,
        kAudioChannelLabel_HOA_ACN_12               = (2 << 16) | 12,
        kAudioChannelLabel_HOA_ACN_13               = (2 << 16) | 13,
        kAudioChannelLabel_HOA_ACN_14               = (2 << 16) | 14,
        kAudioChannelLabel_HOA_ACN_15               = (2 << 16) | 15,
        kAudioChannelLabel_HOA_ACN_65024            = (2 << 16) | 65024,   // 254th order uses 65025 channels
    }
}

e! {
    enum AudioChannelBitmap: u32 {
        kAudioChannelBit_Left                       = (1<<0),
        kAudioChannelBit_Right                      = (1<<1),
        kAudioChannelBit_Center                     = (1<<2),
        kAudioChannelBit_LFEScreen                  = (1<<3),
        kAudioChannelBit_LeftSurround               = (1<<4),      // WAVE: "Back Left"
        kAudioChannelBit_RightSurround              = (1<<5),      // WAVE: "Back Right"
        kAudioChannelBit_LeftCenter                 = (1<<6),
        kAudioChannelBit_RightCenter                = (1<<7),
        kAudioChannelBit_CenterSurround             = (1<<8),      // WAVE: "Back Center"
        kAudioChannelBit_LeftSurroundDirect         = (1<<9),      // WAVE: "Side Left"
        kAudioChannelBit_RightSurroundDirect        = (1<<10),     // WAVE: "Side Right"
        kAudioChannelBit_TopCenterSurround          = (1<<11),
        kAudioChannelBit_VerticalHeightLeft         = (1<<12),     // WAVE: "Top Front Left"
        kAudioChannelBit_VerticalHeightCenter       = (1<<13),     // WAVE: "Top Front Center"
        kAudioChannelBit_VerticalHeightRight        = (1<<14),     // WAVE: "Top Front Right"
        kAudioChannelBit_TopBackLeft                = (1<<15),
        kAudioChannelBit_TopBackCenter              = (1<<16),
        kAudioChannelBit_TopBackRight               = (1<<17),
    }
}

e! {
    enum AudioChannelFlags: u32 {
        kAudioChannelFlags_AllOff                   = 0,
        kAudioChannelFlags_RectangularCoordinates   = (1<<0),
        kAudioChannelFlags_SphericalCoordinates     = (1<<1),
        kAudioChannelFlags_Meters                   = (1<<2),
    }
}

e! {
    enum AudioChannelCoordinateIndex: u32 {
        kAudioChannelCoordinates_LeftRight  = 0,
        kAudioChannelCoordinates_BackFront  = 1,
        kAudioChannelCoordinates_DownUp     = 2,
        kAudioChannelCoordinates_Azimuth    = 0,
        kAudioChannelCoordinates_Elevation  = 1,
        kAudioChannelCoordinates_Distance   = 2,
    }
}

e! {
    enum AudioChannelLayoutTag {
        //  General layouts
        kAudioChannelLayoutTag_UseChannelDescriptions   = (0<<16) | 0,     // use the array of AudioChannelDescriptions to define the mapping.
        kAudioChannelLayoutTag_UseChannelBitmap         = (1<<16) | 0,     // use the bitmap to define the mapping.
        
        kAudioChannelLayoutTag_Mono                     = (100<<16) | 1,   // a standard mono stream
        kAudioChannelLayoutTag_Stereo                   = (101<<16) | 2,   // a standard stereo stream (L R) - implied playback
        kAudioChannelLayoutTag_StereoHeadphones         = (102<<16) | 2,   // a standard stereo stream (L R) - implied headphone playback
        kAudioChannelLayoutTag_MatrixStereo             = (103<<16) | 2,   // a matrix encoded stereo stream (Lt, Rt)
        kAudioChannelLayoutTag_MidSide                  = (104<<16) | 2,   // mid/side recording
        kAudioChannelLayoutTag_XY                       = (105<<16) | 2,   // coincident mic pair (often 2 figure 8's)
        kAudioChannelLayoutTag_Binaural                 = (106<<16) | 2,   // binaural stereo (left, right)
        kAudioChannelLayoutTag_Ambisonic_B_Format       = (107<<16) | 4,   // W, X, Y, Z
        
        kAudioChannelLayoutTag_Quadraphonic             = (108<<16) | 4,   // L R Ls Rs  -- 90 degree speaker separation
        kAudioChannelLayoutTag_Pentagonal               = (109<<16) | 5,   // L R Ls Rs C  -- 72 degree speaker separation
        kAudioChannelLayoutTag_Hexagonal                = (110<<16) | 6,   // L R Ls Rs C Cs  -- 60 degree speaker separation
        kAudioChannelLayoutTag_Octagonal                = (111<<16) | 8,   // L R Ls Rs C Cs Lw Rw  -- 45 degree speaker separation
        kAudioChannelLayoutTag_Cube                     = (112<<16) | 8,   // left, right, rear left, rear right
        // top left, top right, top rear left, top rear right
        
        //  MPEG defined layouts
        kAudioChannelLayoutTag_MPEG_1_0                 = kAudioChannelLayoutTag_Mono,         //  C
        kAudioChannelLayoutTag_MPEG_2_0                 = kAudioChannelLayoutTag_Stereo,       //  L R
        kAudioChannelLayoutTag_MPEG_3_0_A               = (113<<16) | 3,                       //  L R C
        kAudioChannelLayoutTag_MPEG_3_0_B               = (114<<16) | 3,                       //  C L R
        kAudioChannelLayoutTag_MPEG_4_0_A               = (115<<16) | 4,                       //  L R C Cs
        kAudioChannelLayoutTag_MPEG_4_0_B               = (116<<16) | 4,                       //  C L R Cs
        kAudioChannelLayoutTag_MPEG_5_0_A               = (117<<16) | 5,                       //  L R C Ls Rs
        kAudioChannelLayoutTag_MPEG_5_0_B               = (118<<16) | 5,                       //  L R Ls Rs C
        kAudioChannelLayoutTag_MPEG_5_0_C               = (119<<16) | 5,                       //  L C R Ls Rs
        kAudioChannelLayoutTag_MPEG_5_0_D               = (120<<16) | 5,                       //  C L R Ls Rs
        kAudioChannelLayoutTag_MPEG_5_1_A               = (121<<16) | 6,                       //  L R C LFE Ls Rs
        kAudioChannelLayoutTag_MPEG_5_1_B               = (122<<16) | 6,                       //  L R Ls Rs C LFE
        kAudioChannelLayoutTag_MPEG_5_1_C               = (123<<16) | 6,                       //  L C R Ls Rs LFE
        kAudioChannelLayoutTag_MPEG_5_1_D               = (124<<16) | 6,                       //  C L R Ls Rs LFE
        kAudioChannelLayoutTag_MPEG_6_1_A               = (125<<16) | 7,                       //  L R C LFE Ls Rs Cs
        kAudioChannelLayoutTag_MPEG_7_1_A               = (126<<16) | 8,                       //  L R C LFE Ls Rs Lc Rc
        kAudioChannelLayoutTag_MPEG_7_1_B               = (127<<16) | 8,                       //  C Lc Rc L R Ls Rs LFE    (doc: IS-13818-7 MPEG2-AAC Table 3.1)
        kAudioChannelLayoutTag_MPEG_7_1_C               = (128<<16) | 8,                       //  L R C LFE Ls Rs Rls Rrs
        kAudioChannelLayoutTag_Emagic_Default_7_1       = (129<<16) | 8,                       //  L R Ls Rs C LFE Lc Rc
        kAudioChannelLayoutTag_SMPTE_DTV                = (130<<16) | 8,                       //  L R C LFE Ls Rs Lt Rt
        //      (kAudioChannelLayoutTag_ITU_5_1 plus a matrix encoded stereo mix)
        
        //  ITU defined layouts
        kAudioChannelLayoutTag_ITU_1_0                  = kAudioChannelLayoutTag_Mono,         //  C
        kAudioChannelLayoutTag_ITU_2_0                  = kAudioChannelLayoutTag_Stereo,       //  L R
        
        kAudioChannelLayoutTag_ITU_2_1                  = (131<<16) | 3,                       //  L R Cs
        kAudioChannelLayoutTag_ITU_2_2                  = (132<<16) | 4,                       //  L R Ls Rs
        kAudioChannelLayoutTag_ITU_3_0                  = kAudioChannelLayoutTag_MPEG_3_0_A,   //  L R C
        kAudioChannelLayoutTag_ITU_3_1                  = kAudioChannelLayoutTag_MPEG_4_0_A,   //  L R C Cs
        
        kAudioChannelLayoutTag_ITU_3_2                  = kAudioChannelLayoutTag_MPEG_5_0_A,   //  L R C Ls Rs
        kAudioChannelLayoutTag_ITU_3_2_1                = kAudioChannelLayoutTag_MPEG_5_1_A,   //  L R C LFE Ls Rs
        kAudioChannelLayoutTag_ITU_3_4_1                = kAudioChannelLayoutTag_MPEG_7_1_C,   //  L R C LFE Ls Rs Rls Rrs
        
        // DVD defined layouts
        kAudioChannelLayoutTag_DVD_0                    = kAudioChannelLayoutTag_Mono,         // C (mono)
        kAudioChannelLayoutTag_DVD_1                    = kAudioChannelLayoutTag_Stereo,       // L R
        kAudioChannelLayoutTag_DVD_2                    = kAudioChannelLayoutTag_ITU_2_1,      // L R Cs
        kAudioChannelLayoutTag_DVD_3                    = kAudioChannelLayoutTag_ITU_2_2,      // L R Ls Rs
        kAudioChannelLayoutTag_DVD_4                    = (133<<16) | 3,                       // L R LFE
        kAudioChannelLayoutTag_DVD_5                    = (134<<16) | 4,                       // L R LFE Cs
        kAudioChannelLayoutTag_DVD_6                    = (135<<16) | 5,                       // L R LFE Ls Rs
        kAudioChannelLayoutTag_DVD_7                    = kAudioChannelLayoutTag_MPEG_3_0_A,   // L R C
        kAudioChannelLayoutTag_DVD_8                    = kAudioChannelLayoutTag_MPEG_4_0_A,   // L R C Cs
        kAudioChannelLayoutTag_DVD_9                    = kAudioChannelLayoutTag_MPEG_5_0_A,   // L R C Ls Rs
        kAudioChannelLayoutTag_DVD_10                   = (136<<16) | 4,                       // L R C LFE
        kAudioChannelLayoutTag_DVD_11                   = (137<<16) | 5,                       // L R C LFE Cs
        kAudioChannelLayoutTag_DVD_12                   = kAudioChannelLayoutTag_MPEG_5_1_A,   // L R C LFE Ls Rs
        // 13 through 17 are duplicates of 8 through 12.
        kAudioChannelLayoutTag_DVD_13                   = kAudioChannelLayoutTag_DVD_8,        // L R C Cs
        kAudioChannelLayoutTag_DVD_14                   = kAudioChannelLayoutTag_DVD_9,        // L R C Ls Rs
        kAudioChannelLayoutTag_DVD_15                   = kAudioChannelLayoutTag_DVD_10,       // L R C LFE
        kAudioChannelLayoutTag_DVD_16                   = kAudioChannelLayoutTag_DVD_11,       // L R C LFE Cs
        kAudioChannelLayoutTag_DVD_17                   = kAudioChannelLayoutTag_DVD_12,       // L R C LFE Ls Rs
        kAudioChannelLayoutTag_DVD_18                   = (138<<16) | 5,                       // L R Ls Rs LFE
        kAudioChannelLayoutTag_DVD_19                   = kAudioChannelLayoutTag_MPEG_5_0_B,   // L R Ls Rs C
        kAudioChannelLayoutTag_DVD_20                   = kAudioChannelLayoutTag_MPEG_5_1_B,   // L R Ls Rs C LFE
        
        // These layouts are recommended for AudioUnit usage
        // These are the symmetrical layouts
        kAudioChannelLayoutTag_AudioUnit_4              = kAudioChannelLayoutTag_Quadraphonic,
        kAudioChannelLayoutTag_AudioUnit_5              = kAudioChannelLayoutTag_Pentagonal,
        kAudioChannelLayoutTag_AudioUnit_6              = kAudioChannelLayoutTag_Hexagonal,
        kAudioChannelLayoutTag_AudioUnit_8              = kAudioChannelLayoutTag_Octagonal,
        // These are the surround-based layouts
        kAudioChannelLayoutTag_AudioUnit_5_0            = kAudioChannelLayoutTag_MPEG_5_0_B,   // L R Ls Rs C
        kAudioChannelLayoutTag_AudioUnit_6_0            = (139<<16) | 6,                       // L R Ls Rs C Cs
        kAudioChannelLayoutTag_AudioUnit_7_0            = (140<<16) | 7,                       // L R Ls Rs C Rls Rrs
        kAudioChannelLayoutTag_AudioUnit_7_0_Front      = (148<<16) | 7,                       // L R Ls Rs C Lc Rc
        kAudioChannelLayoutTag_AudioUnit_5_1            = kAudioChannelLayoutTag_MPEG_5_1_A,   // L R C LFE Ls Rs
        kAudioChannelLayoutTag_AudioUnit_6_1            = kAudioChannelLayoutTag_MPEG_6_1_A,   // L R C LFE Ls Rs Cs
        kAudioChannelLayoutTag_AudioUnit_7_1            = kAudioChannelLayoutTag_MPEG_7_1_C,   // L R C LFE Ls Rs Rls Rrs
        kAudioChannelLayoutTag_AudioUnit_7_1_Front      = kAudioChannelLayoutTag_MPEG_7_1_A,   // L R C LFE Ls Rs Lc Rc
        
        kAudioChannelLayoutTag_AAC_3_0                  = kAudioChannelLayoutTag_MPEG_3_0_B,   // C L R
        kAudioChannelLayoutTag_AAC_Quadraphonic         = kAudioChannelLayoutTag_Quadraphonic, // L R Ls Rs
        kAudioChannelLayoutTag_AAC_4_0                  = kAudioChannelLayoutTag_MPEG_4_0_B,   // C L R Cs
        kAudioChannelLayoutTag_AAC_5_0                  = kAudioChannelLayoutTag_MPEG_5_0_D,   // C L R Ls Rs
        kAudioChannelLayoutTag_AAC_5_1                  = kAudioChannelLayoutTag_MPEG_5_1_D,   // C L R Ls Rs Lfe
        kAudioChannelLayoutTag_AAC_6_0                  = (141<<16) | 6,                       // C L R Ls Rs Cs
        kAudioChannelLayoutTag_AAC_6_1                  = (142<<16) | 7,                       // C L R Ls Rs Cs Lfe
        kAudioChannelLayoutTag_AAC_7_0                  = (143<<16) | 7,                       // C L R Ls Rs Rls Rrs
        kAudioChannelLayoutTag_AAC_7_1                  = kAudioChannelLayoutTag_MPEG_7_1_B,   // C Lc Rc L R Ls Rs Lfe
        kAudioChannelLayoutTag_AAC_7_1_B                = (183<<16) | 8,                       // C L R Ls Rs Rls Rrs LFE
        kAudioChannelLayoutTag_AAC_7_1_C                = (184<<16) | 8,                       // C L R Ls Rs LFE Vhl Vhr
        kAudioChannelLayoutTag_AAC_Octagonal            = (144<<16) | 8,                       // C L R Ls Rs Rls Rrs Cs
        
        kAudioChannelLayoutTag_TMH_10_2_std             = (145<<16) | 16,                      // L R C Vhc Lsd Rsd Ls Rs Vhl Vhr Lw Rw Csd Cs LFE1 LFE2
        kAudioChannelLayoutTag_TMH_10_2_full            = (146<<16) | 21,                      // TMH_10_2_std plus: Lc Rc HI VI Haptic
        
        kAudioChannelLayoutTag_AC3_1_0_1                = (149<<16) | 2,                       // C LFE
        kAudioChannelLayoutTag_AC3_3_0                  = (150<<16) | 3,                       // L C R
        kAudioChannelLayoutTag_AC3_3_1                  = (151<<16) | 4,                       // L C R Cs
        kAudioChannelLayoutTag_AC3_3_0_1                = (152<<16) | 4,                       // L C R LFE
        kAudioChannelLayoutTag_AC3_2_1_1                = (153<<16) | 4,                       // L R Cs LFE
        kAudioChannelLayoutTag_AC3_3_1_1                = (154<<16) | 5,                       // L C R Cs LFE
        
        kAudioChannelLayoutTag_EAC_6_0_A                = (155<<16) | 6,                       // L C R Ls Rs Cs
        kAudioChannelLayoutTag_EAC_7_0_A                = (156<<16) | 7,                       // L C R Ls Rs Rls Rrs
        
        kAudioChannelLayoutTag_EAC3_6_1_A               = (157<<16) | 7,                       // L C R Ls Rs LFE Cs
        kAudioChannelLayoutTag_EAC3_6_1_B               = (158<<16) | 7,                       // L C R Ls Rs LFE Ts
        kAudioChannelLayoutTag_EAC3_6_1_C               = (159<<16) | 7,                       // L C R Ls Rs LFE Vhc
        kAudioChannelLayoutTag_EAC3_7_1_A               = (160<<16) | 8,                       // L C R Ls Rs LFE Rls Rrs
        kAudioChannelLayoutTag_EAC3_7_1_B               = (161<<16) | 8,                       // L C R Ls Rs LFE Lc Rc
        kAudioChannelLayoutTag_EAC3_7_1_C               = (162<<16) | 8,                       // L C R Ls Rs LFE Lsd Rsd
        kAudioChannelLayoutTag_EAC3_7_1_D               = (163<<16) | 8,                       // L C R Ls Rs LFE Lw Rw
        kAudioChannelLayoutTag_EAC3_7_1_E               = (164<<16) | 8,                       // L C R Ls Rs LFE Vhl Vhr
        
        kAudioChannelLayoutTag_EAC3_7_1_F               = (165<<16) | 8,                        // L C R Ls Rs LFE Cs Ts
        kAudioChannelLayoutTag_EAC3_7_1_G               = (166<<16) | 8,                        // L C R Ls Rs LFE Cs Vhc
        kAudioChannelLayoutTag_EAC3_7_1_H               = (167<<16) | 8,                        // L C R Ls Rs LFE Ts Vhc
        
        kAudioChannelLayoutTag_DTS_3_1                  = (168<<16) | 4,                        // C L R LFE
        kAudioChannelLayoutTag_DTS_4_1                  = (169<<16) | 5,                        // C L R Cs LFE
        kAudioChannelLayoutTag_DTS_6_0_A                = (170<<16) | 6,                        // Lc Rc L R Ls Rs
        kAudioChannelLayoutTag_DTS_6_0_B                = (171<<16) | 6,                        // C L R Rls Rrs Ts
        kAudioChannelLayoutTag_DTS_6_0_C                = (172<<16) | 6,                        // C Cs L R Rls Rrs
        kAudioChannelLayoutTag_DTS_6_1_A                = (173<<16) | 7,                        // Lc Rc L R Ls Rs LFE
        kAudioChannelLayoutTag_DTS_6_1_B                = (174<<16) | 7,                        // C L R Rls Rrs Ts LFE
        kAudioChannelLayoutTag_DTS_6_1_C                = (175<<16) | 7,                        // C Cs L R Rls Rrs LFE
        kAudioChannelLayoutTag_DTS_7_0                  = (176<<16) | 7,                        // Lc C Rc L R Ls Rs
        kAudioChannelLayoutTag_DTS_7_1                  = (177<<16) | 8,                        // Lc C Rc L R Ls Rs LFE    
        kAudioChannelLayoutTag_DTS_8_0_A                = (178<<16) | 8,                        // Lc Rc L R Ls Rs Rls Rrs
        kAudioChannelLayoutTag_DTS_8_0_B                = (179<<16) | 8,                        // Lc C Rc L R Ls Cs Rs
        kAudioChannelLayoutTag_DTS_8_1_A                = (180<<16) | 9,                        // Lc Rc L R Ls Rs Rls Rrs LFE
        kAudioChannelLayoutTag_DTS_8_1_B                = (181<<16) | 9,                        // Lc C Rc L R Ls Cs Rs LFE
        kAudioChannelLayoutTag_DTS_6_1_D                = (182<<16) | 7,                        // C L R Ls Rs LFE Cs
        
        kAudioChannelLayoutTag_HOA_ACN_SN3D             = (190<<16) | 0,                        // Higher Order Ambisonics, Ambisonics Channel Number, SN3D normalization
        // needs to be ORed with the actual number of channels (not the HOA order)
        kAudioChannelLayoutTag_HOA_ACN_N3D              = (191<<16) | 0,                        // Higher Order Ambisonics, Ambisonics Channel Number, N3D normalization
        // needs to be ORed with the actual number of channels (not the HOA order)
        
        kAudioChannelLayoutTag_DiscreteInOrder          = (147<<16) | 0,                        // needs to be ORed with the actual number of channels
        kAudioChannelLayoutTag_Unknown                  = 0xFFFF0000,                            // needs to be ORed with the actual number of channels  
    }
}

s! {
    #[derive(Clone, Copy)]
    struct AudioChannelDescription {
        pub mChannelLabel: AudioChannelLabel,
        pub mChannelFlags: AudioChannelFlags,
        pub mCoordinates: [f32;3],
    }

    struct AudioChannelLayout {
        pub mChannelLayoutTag: AudioChannelLayoutTag,
        pub mChannelBitmap: AudioChannelBitmap,
        pub mNumberChannelDescriptions: u32,
            mChannelDescriptions: [AudioChannelDescription;0],
    }
}

impl AudioChannelLayout {
    pub fn channel_descriptions(&self) -> &[AudioChannelDescription] {
        unsafe {
            slice::from_raw_parts(&self.mChannelDescriptions as *const _,
                                  self.mNumberChannelDescriptions as _)
        }
    }
}

pub fn AudioChannelLayoutTag_GetNumberOfChannels(inLayoutTag: AudioChannelLayoutTag) -> u32 {
    inLayoutTag & 0x0000FFFF
}
