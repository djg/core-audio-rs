use super::*;
use std::mem;
use std::os::raw::{c_double, c_float, c_void};

//==============================================================================
// General Error Codes
pub const kAudio_UnimplementedError: OSStatus = -4;
pub const kAudio_FileNotFoundError: OSStatus = -43;
pub const kAudio_FilePermissionError: OSStatus = -54;
pub const kAudio_TooManyFilesOpenError: OSStatus = -42;
pub const kAudio_BadFilePathError: OSStatus = 561017960;
pub const kAudio_ParamError: OSStatus = -50;
pub const kAudio_MemFullError: OSStatus = -108;

// AudioValueRange

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioValueRange {
    pub mMinimum: c_double,
    pub mMaximum: c_double,
}

//==============================================================================
// AudioValueTranslation

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioValueTranslation {
    pub mInputData: *mut c_void,
    pub mInputDataSize: u32,
    pub mOutputData: *mut c_void,
    pub mOutputDataSize: u32,
}

//==============================================================================
// AudioBuffer/AudioBufferList

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioBuffer {
    pub mNumberChannels: u32,
    pub mDataByteSize: u32,
    pub mData: *mut c_void,
}

impl Default for AudioBuffer {
    fn default() -> AudioBuffer {
        unsafe { mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct AudioBufferList {
    pub mNumberBuffers: u32,
    pub mBuffers: [AudioBuffer; 1],
}

//==============================================================================
// Audio Formats

#[cfg(all(feature = "deprecated", not(feature = "prefer-fixed-point")))]
pub type AudioSampleType = f32;
#[cfg(all(feature = "deprecated", not(feature = "prefer-fixed-point")))]
pub type AudioUnitSampleType = f32;

#[cfg(all(feature = "deprecated", feature = "prefer-fixed-point"))]
pub type AudioSampleType = i16;
#[cfg(all(feature = "deprecated", feature = "prefer-fixed-point"))]
pub type AudioUnitSampleType = i32;

#[cfg(feature = "prefer-fixed-point")]
pub const kAudioUnitSampleFractionBits: u32 = 24;

pub type AudioFormatID = u32;
pub type AudioFormatFlags = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioStreamBasicDescription {
    pub mSampleRate: c_double,
    pub mFormatID: AudioFormatID,
    pub mFormatFlags: AudioFormatFlags,
    pub mBytesPerPacket: u32,
    pub mFramesPerPacket: u32,
    pub mBytesPerFrame: u32,
    pub mChannelsPerFrame: u32,
    pub mBitsPerChannel: u32,
    mReserved: u32,
}

pub const kAudioStreamAnyRate: c_double = 0.0;

pub const kAudioFormatLinearPCM: AudioFormatID = 1819304813;
pub const kAudioFormatAC3: AudioFormatID = 1633889587;
pub const kAudioFormat60958AC3: AudioFormatID = 1667326771;
pub const kAudioFormatAppleIMA4: AudioFormatID = 1768775988;
pub const kAudioFormatMPEG4AAC: AudioFormatID = 1633772320;
pub const kAudioFormatMPEG4CELP: AudioFormatID = 1667591280;
pub const kAudioFormatMPEG4HVXC: AudioFormatID = 1752594531;
pub const kAudioFormatMPEG4TwinVQ: AudioFormatID = 1953986161;
pub const kAudioFormatMACE3: AudioFormatID = 1296122675;
pub const kAudioFormatMACE6: AudioFormatID = 1296122678;
pub const kAudioFormatULaw: AudioFormatID = 1970037111;
pub const kAudioFormatALaw: AudioFormatID = 1634492791;
pub const kAudioFormatQDesign: AudioFormatID = 1363430723;
pub const kAudioFormatQDesign2: AudioFormatID = 1363430706;
pub const kAudioFormatQUALCOMM: AudioFormatID = 1365470320;
pub const kAudioFormatMPEGLayer1: AudioFormatID = 778924081;
pub const kAudioFormatMPEGLayer2: AudioFormatID = 778924082;
pub const kAudioFormatMPEGLayer3: AudioFormatID = 778924083;
pub const kAudioFormatTimeCode: AudioFormatID = 1953066341;
pub const kAudioFormatMIDIStream: AudioFormatID = 1835623529;
pub const kAudioFormatParameterValueStream: AudioFormatID = 1634760307;
pub const kAudioFormatAppleLossless: AudioFormatID = 1634492771;
pub const kAudioFormatMPEG4AAC_HE: AudioFormatID = 1633772392;
pub const kAudioFormatMPEG4AAC_LD: AudioFormatID = 1633772396;
pub const kAudioFormatMPEG4AAC_ELD: AudioFormatID = 1633772389;
pub const kAudioFormatMPEG4AAC_ELD_SBR: AudioFormatID = 1633772390;
pub const kAudioFormatMPEG4AAC_ELD_V2: AudioFormatID = 1633772391;
pub const kAudioFormatMPEG4AAC_HE_V2: AudioFormatID = 1633772400;
pub const kAudioFormatMPEG4AAC_Spatial: AudioFormatID = 1633772403;
pub const kAudioFormatAMR: AudioFormatID = 1935764850;
pub const kAudioFormatAMR_WB: AudioFormatID = 1935767394;
pub const kAudioFormatAudible: AudioFormatID = 1096107074;
pub const kAudioFormatiLBC: AudioFormatID = 1768710755;
pub const kAudioFormatDVIIntelIMA: AudioFormatID = 1836253201;
pub const kAudioFormatMicrosoftGSM: AudioFormatID = 1836253233;
pub const kAudioFormatAES3: AudioFormatID = 1634038579;
pub const kAudioFormatEnhancedAC3: AudioFormatID = 1700998451;
pub const kAudioFormatFLAC: AudioFormatID = 1718378851;
pub const kAudioFormatOpus: AudioFormatID = 1869641075;

pub const kAudioFormatFlagIsFloat: AudioFormatFlags = (1 << 0);
pub const kAudioFormatFlagIsBigEndian: AudioFormatFlags = (1 << 1);
pub const kAudioFormatFlagIsSignedInteger: AudioFormatFlags = (1 << 2);
pub const kAudioFormatFlagIsPacked: AudioFormatFlags = (1 << 3);
pub const kAudioFormatFlagIsAlignedHigh: AudioFormatFlags = (1 << 4);
pub const kAudioFormatFlagIsNonInterleaved: AudioFormatFlags = (1 << 5);
pub const kAudioFormatFlagIsNonMixable: AudioFormatFlags = (1 << 6);
pub const kAudioFormatFlagsAreAllClear: AudioFormatFlags = 2147483648;

pub const kLinearPCMFormatFlagIsFloat: AudioFormatFlags = kAudioFormatFlagIsFloat;
pub const kLinearPCMFormatFlagIsBigEndian: AudioFormatFlags = kAudioFormatFlagIsBigEndian;
pub const kLinearPCMFormatFlagIsSignedInteger: AudioFormatFlags = kAudioFormatFlagIsSignedInteger;
pub const kLinearPCMFormatFlagIsPacked: AudioFormatFlags = kAudioFormatFlagIsPacked;
pub const kLinearPCMFormatFlagIsAlignedHigh: AudioFormatFlags = kAudioFormatFlagIsAlignedHigh;
pub const kLinearPCMFormatFlagIsNonInterleaved: AudioFormatFlags = kAudioFormatFlagIsNonInterleaved;
pub const kLinearPCMFormatFlagIsNonMixable: AudioFormatFlags = kAudioFormatFlagIsNonMixable;
pub const kLinearPCMFormatFlagsSampleFractionShift: AudioFormatFlags = 7;
pub const kLinearPCMFormatFlagsSampleFractionMask: AudioFormatFlags =
    (63 << kLinearPCMFormatFlagsSampleFractionShift);
pub const kLinearPCMFormatFlagsAreAllClear: AudioFormatFlags = kAudioFormatFlagsAreAllClear;

pub const kAppleLosslessFormatFlag_16BitSourceData: AudioFormatFlags = 1;
pub const kAppleLosslessFormatFlag_20BitSourceData: AudioFormatFlags = 2;
pub const kAppleLosslessFormatFlag_24BitSourceData: AudioFormatFlags = 3;
pub const kAppleLosslessFormatFlag_32BitSourceData: AudioFormatFlags = 4;

#[cfg(target_endian = "big")]
pub const kAudioFormatFlagsNativeEndian: AudioFormatFlags = kAudioFormatFlagIsBigEndian;
#[cfg(not(target_endian = "big"))]
pub const kAudioFormatFlagsNativeEndian: AudioFormatFlags = 0;

#[cfg(all(feature = "deprecated", not(feature = "prefer-fixed-point")))]
pub const kAudioFormatFlagsCanonical: AudioFormatFlags =
    kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;
#[cfg(all(feature = "deprecated", not(feature = "prefer-fixed-point")))]
pub const kAudioFormatFlagsAudioUnitCanonical: AudioFormatFlags =
    kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked
        | kAudioFormatFlagIsNonInterleaved;

#[cfg(all(feature = "deprecated", feature = "prefer-fixed-point"))]
pub const kAudioFormatFlagsCanonical: AudioFormatFlags =
    kAudioFormatFlagIsSignedInteger | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;
#[cfg(all(feature = "deprecated", feature = "prefer-fixed-point"))]
pub const kAudioFormatFlagsAudioUnitCanonical: AudioFormatFlags =
    kAudioFormatFlagIsSignedInteger | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked
        | kAudioFormatFlagIsNonInterleaved
        | (kAudioUnitSampleFractionBits << kLinearPCMFormatFlagsSampleFractionShift);
#[cfg(feature = "prefer-fixed-point")]
pub const kAudioFormatFlagsNativeFloatPacked: AudioFormatFlags =
    kAudioFormatFlagIsFloat | kAudioFormatFlagsNativeEndian | kAudioFormatFlagIsPacked;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioStreamPacketDescription {
    pub mStartOffset: i64,
    pub mVariableFramesInPacket: u32,
    pub mDataByteSize: u32,
}

//==============================================================================
// Audio Time Stamps

pub type SMPTETimeType = u32;
pub const kSMPTETimeType24: u32 = 0;
pub const kSMPTETimeType25: u32 = 1;
pub const kSMPTETimeType30Drop: u32 = 2;
pub const kSMPTETimeType30: u32 = 3;
pub const kSMPTETimeType2997: u32 = 4;
pub const kSMPTETimeType2997Drop: u32 = 5;
pub const kSMPTETimeType60: u32 = 6;
pub const kSMPTETimeType5994: u32 = 7;
pub const kSMPTETimeType60Drop: u32 = 8;
pub const kSMPTETimeType5994Drop: u32 = 9;
pub const kSMPTETimeType50: u32 = 10;
pub const kSMPTETimeType2398: u32 = 11;

pub type SMPTETimeFlags = u32;
pub const kSMPTETimeUnknown: u32 = 0;
pub const kSMPTETimeValid: u32 = (1 << 0);
pub const kSMPTETimeRunning: u32 = (1 << 1);

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct SMPTETime {
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

pub type AudioTimeStampFlags = u32;
pub const kAudioTimeStampNothingValid: u32 = 0;
pub const kAudioTimeStampSampleTimeValid: u32 = (1 << 0);
pub const kAudioTimeStampHostTimeValid: u32 = (1 << 1);
pub const kAudioTimeStampRateScalarValid: u32 = (1 << 2);
pub const kAudioTimeStampWordClockTimeValid: u32 = (1 << 3);
pub const kAudioTimeStampSMPTETimeValid: u32 = (1 << 4);
pub const kAudioTimeStampSampleHostTimeValid: u32 =
    (kAudioTimeStampSampleTimeValid | kAudioTimeStampHostTimeValid);

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioTimeStamp {
    pub mSampleTime: c_double,
    pub mHostTime: u64,
    pub mRateScalar: c_double,
    pub mWordClockTime: u64,
    pub mSMPTETime: SMPTETime,
    pub mFlags: AudioTimeStampFlags,
    pub mReserved: u32,
}

impl Default for AudioTimeStamp {
    fn default() -> AudioTimeStamp {
        unsafe { mem::zeroed() }
    }
}

//==============================================================================
// AudioClassDescription

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioClassDescription {
    pub mType: OSType,
    pub mSubType: OSType,
    pub mManufacturer: OSType,
}

//==============================================================================
// Audio Channel Layout

pub type AudioChannelLabel = u32;
pub const kAudioChannelLabel_Unknown: AudioChannelLabel = 4294967295;
pub const kAudioChannelLabel_Unused: AudioChannelLabel = 0;
pub const kAudioChannelLabel_UseCoordinates: AudioChannelLabel = 100;
pub const kAudioChannelLabel_Left: AudioChannelLabel = 1;
pub const kAudioChannelLabel_Right: AudioChannelLabel = 2;
pub const kAudioChannelLabel_Center: AudioChannelLabel = 3;
pub const kAudioChannelLabel_LFEScreen: AudioChannelLabel = 4;
pub const kAudioChannelLabel_LeftSurround: AudioChannelLabel = 5;
pub const kAudioChannelLabel_RightSurround: AudioChannelLabel = 6;
pub const kAudioChannelLabel_LeftCenter: AudioChannelLabel = 7;
pub const kAudioChannelLabel_RightCenter: AudioChannelLabel = 8;
pub const kAudioChannelLabel_CenterSurround: AudioChannelLabel = 9;
pub const kAudioChannelLabel_LeftSurroundDirect: AudioChannelLabel = 10;
pub const kAudioChannelLabel_RightSurroundDirect: AudioChannelLabel = 11;
pub const kAudioChannelLabel_TopCenterSurround: AudioChannelLabel = 12;
pub const kAudioChannelLabel_VerticalHeightLeft: AudioChannelLabel = 13;
pub const kAudioChannelLabel_VerticalHeightCenter: AudioChannelLabel = 14;
pub const kAudioChannelLabel_VerticalHeightRight: AudioChannelLabel = 15;
pub const kAudioChannelLabel_TopBackLeft: AudioChannelLabel = 16;
pub const kAudioChannelLabel_TopBackCenter: AudioChannelLabel = 17;
pub const kAudioChannelLabel_TopBackRight: AudioChannelLabel = 18;
pub const kAudioChannelLabel_RearSurroundLeft: AudioChannelLabel = 33;
pub const kAudioChannelLabel_RearSurroundRight: AudioChannelLabel = 34;
pub const kAudioChannelLabel_LeftWide: AudioChannelLabel = 35;
pub const kAudioChannelLabel_RightWide: AudioChannelLabel = 36;
pub const kAudioChannelLabel_LFE2: AudioChannelLabel = 37;
pub const kAudioChannelLabel_LeftTotal: AudioChannelLabel = 38;
pub const kAudioChannelLabel_RightTotal: AudioChannelLabel = 39;
pub const kAudioChannelLabel_HearingImpaired: AudioChannelLabel = 40;
pub const kAudioChannelLabel_Narration: AudioChannelLabel = 41;
pub const kAudioChannelLabel_Mono: AudioChannelLabel = 42;
pub const kAudioChannelLabel_DialogCentricMix: AudioChannelLabel = 43;
pub const kAudioChannelLabel_CenterSurroundDirect: AudioChannelLabel = 44;
pub const kAudioChannelLabel_Haptic: AudioChannelLabel = 45;
pub const kAudioChannelLabel_Ambisonic_W: AudioChannelLabel = 200;
pub const kAudioChannelLabel_Ambisonic_X: AudioChannelLabel = 201;
pub const kAudioChannelLabel_Ambisonic_Y: AudioChannelLabel = 202;
pub const kAudioChannelLabel_Ambisonic_Z: AudioChannelLabel = 203;
pub const kAudioChannelLabel_MS_Mid: AudioChannelLabel = 204;
pub const kAudioChannelLabel_MS_Side: AudioChannelLabel = 205;
pub const kAudioChannelLabel_XY_X: AudioChannelLabel = 206;
pub const kAudioChannelLabel_XY_Y: AudioChannelLabel = 207;
pub const kAudioChannelLabel_HeadphonesLeft: AudioChannelLabel = 301;
pub const kAudioChannelLabel_HeadphonesRight: AudioChannelLabel = 302;
pub const kAudioChannelLabel_ClickTrack: AudioChannelLabel = 304;
pub const kAudioChannelLabel_ForeignLanguage: AudioChannelLabel = 305;
pub const kAudioChannelLabel_Discrete: AudioChannelLabel = 400;
pub const kAudioChannelLabel_Discrete_0: AudioChannelLabel = (1 << 16) | 0;
pub const kAudioChannelLabel_Discrete_1: AudioChannelLabel = (1 << 16) | 1;
pub const kAudioChannelLabel_Discrete_2: AudioChannelLabel = (1 << 16) | 2;
pub const kAudioChannelLabel_Discrete_3: AudioChannelLabel = (1 << 16) | 3;
pub const kAudioChannelLabel_Discrete_4: AudioChannelLabel = (1 << 16) | 4;
pub const kAudioChannelLabel_Discrete_5: AudioChannelLabel = (1 << 16) | 5;
pub const kAudioChannelLabel_Discrete_6: AudioChannelLabel = (1 << 16) | 6;
pub const kAudioChannelLabel_Discrete_7: AudioChannelLabel = (1 << 16) | 7;
pub const kAudioChannelLabel_Discrete_8: AudioChannelLabel = (1 << 16) | 8;
pub const kAudioChannelLabel_Discrete_9: AudioChannelLabel = (1 << 16) | 9;
pub const kAudioChannelLabel_Discrete_10: AudioChannelLabel = (1 << 16) | 10;
pub const kAudioChannelLabel_Discrete_11: AudioChannelLabel = (1 << 16) | 11;
pub const kAudioChannelLabel_Discrete_12: AudioChannelLabel = (1 << 16) | 12;
pub const kAudioChannelLabel_Discrete_13: AudioChannelLabel = (1 << 16) | 13;
pub const kAudioChannelLabel_Discrete_14: AudioChannelLabel = (1 << 16) | 14;
pub const kAudioChannelLabel_Discrete_15: AudioChannelLabel = (1 << 16) | 15;
pub const kAudioChannelLabel_Discrete_65535: AudioChannelLabel = (1 << 16) | 65535;
pub const kAudioChannelLabel_HOA_ACN: AudioChannelLabel = 500;
pub const kAudioChannelLabel_HOA_ACN_0: AudioChannelLabel = (2 << 16) | 0;
pub const kAudioChannelLabel_HOA_ACN_1: AudioChannelLabel = (2 << 16) | 1;
pub const kAudioChannelLabel_HOA_ACN_2: AudioChannelLabel = (2 << 16) | 2;
pub const kAudioChannelLabel_HOA_ACN_3: AudioChannelLabel = (2 << 16) | 3;
pub const kAudioChannelLabel_HOA_ACN_4: AudioChannelLabel = (2 << 16) | 4;
pub const kAudioChannelLabel_HOA_ACN_5: AudioChannelLabel = (2 << 16) | 5;
pub const kAudioChannelLabel_HOA_ACN_6: AudioChannelLabel = (2 << 16) | 6;
pub const kAudioChannelLabel_HOA_ACN_7: AudioChannelLabel = (2 << 16) | 7;
pub const kAudioChannelLabel_HOA_ACN_8: AudioChannelLabel = (2 << 16) | 8;
pub const kAudioChannelLabel_HOA_ACN_9: AudioChannelLabel = (2 << 16) | 9;
pub const kAudioChannelLabel_HOA_ACN_10: AudioChannelLabel = (2 << 16) | 10;
pub const kAudioChannelLabel_HOA_ACN_11: AudioChannelLabel = (2 << 16) | 11;
pub const kAudioChannelLabel_HOA_ACN_12: AudioChannelLabel = (2 << 16) | 12;
pub const kAudioChannelLabel_HOA_ACN_13: AudioChannelLabel = (2 << 16) | 13;
pub const kAudioChannelLabel_HOA_ACN_14: AudioChannelLabel = (2 << 16) | 14;
pub const kAudioChannelLabel_HOA_ACN_15: AudioChannelLabel = (2 << 16) | 15;
pub const kAudioChannelLabel_HOA_ACN_65024: AudioChannelLabel = (2 << 16) | 65024;

pub type AudioChannelBitmap = u32;
pub const kAudioChannelBit_Left: u32 = (1 << 0);
pub const kAudioChannelBit_Right: u32 = (1 << 1);
pub const kAudioChannelBit_Center: u32 = (1 << 2);
pub const kAudioChannelBit_LFEScreen: u32 = (1 << 3);
pub const kAudioChannelBit_LeftSurround: u32 = (1 << 4);
pub const kAudioChannelBit_RightSurround: u32 = (1 << 5);
pub const kAudioChannelBit_LeftCenter: u32 = (1 << 6);
pub const kAudioChannelBit_RightCenter: u32 = (1 << 7);
pub const kAudioChannelBit_CenterSurround: u32 = (1 << 8);
pub const kAudioChannelBit_LeftSurroundDirect: u32 = (1 << 9);
pub const kAudioChannelBit_RightSurroundDirect: u32 = (1 << 10);
pub const kAudioChannelBit_TopCenterSurround: u32 = (1 << 11);
pub const kAudioChannelBit_VerticalHeightLeft: u32 = (1 << 12);
pub const kAudioChannelBit_VerticalHeightCenter: u32 = (1 << 13);
pub const kAudioChannelBit_VerticalHeightRight: u32 = (1 << 14);
pub const kAudioChannelBit_TopBackLeft: u32 = (1 << 15);
pub const kAudioChannelBit_TopBackCenter: u32 = (1 << 16);
pub const kAudioChannelBit_TopBackRight: u32 = (1 << 17);

pub type AudioChannelFlags = u32;
pub const kAudioChannelFlags_AllOff: u32 = 0;
pub const kAudioChannelFlags_RectangularCoordinates: u32 = (1 << 0);
pub const kAudioChannelFlags_SphericalCoordinates: u32 = (1 << 1);
pub const kAudioChannelFlags_Meters: u32 = (1 << 2);

pub type AudioChannelCoordinateIndex = u32;
pub const kAudioChannelCoordinates_LeftRight: u32 = 0;
pub const kAudioChannelCoordinates_BackFront: u32 = 1;
pub const kAudioChannelCoordinates_DownUp: u32 = 2;
pub const kAudioChannelCoordinates_Azimuth: u32 = 0;
pub const kAudioChannelCoordinates_Elevation: u32 = 1;
pub const kAudioChannelCoordinates_Distance: u32 = 2;

pub type AudioChannelLayoutTag = u32;

pub const kAudioChannelLayoutTag_UseChannelDescriptions: AudioChannelLayoutTag = (0 << 16) | 0;
pub const kAudioChannelLayoutTag_UseChannelBitmap: AudioChannelLayoutTag = (1 << 16) | 0;
pub const kAudioChannelLayoutTag_Mono: AudioChannelLayoutTag = (100 << 16) | 1;
pub const kAudioChannelLayoutTag_Stereo: AudioChannelLayoutTag = (101 << 16) | 2;
pub const kAudioChannelLayoutTag_StereoHeadphones: AudioChannelLayoutTag = (102 << 16) | 2;
pub const kAudioChannelLayoutTag_MatrixStereo: AudioChannelLayoutTag = (103 << 16) | 2;
pub const kAudioChannelLayoutTag_MidSide: AudioChannelLayoutTag = (104 << 16) | 2;
pub const kAudioChannelLayoutTag_XY: AudioChannelLayoutTag = (105 << 16) | 2;
pub const kAudioChannelLayoutTag_Binaural: AudioChannelLayoutTag = (106 << 16) | 2;
pub const kAudioChannelLayoutTag_Ambisonic_B_Format: AudioChannelLayoutTag = (107 << 16) | 4;
pub const kAudioChannelLayoutTag_Quadraphonic: AudioChannelLayoutTag = (108 << 16) | 4;
pub const kAudioChannelLayoutTag_Pentagonal: AudioChannelLayoutTag = (109 << 16) | 5;
pub const kAudioChannelLayoutTag_Hexagonal: AudioChannelLayoutTag = (110 << 16) | 6;
pub const kAudioChannelLayoutTag_Octagonal: AudioChannelLayoutTag = (111 << 16) | 8;
pub const kAudioChannelLayoutTag_Cube: AudioChannelLayoutTag = (112 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_1_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_MPEG_2_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_MPEG_3_0_A: AudioChannelLayoutTag = (113 << 16) | 3;
pub const kAudioChannelLayoutTag_MPEG_3_0_B: AudioChannelLayoutTag = (114 << 16) | 3;
pub const kAudioChannelLayoutTag_MPEG_4_0_A: AudioChannelLayoutTag = (115 << 16) | 4;
pub const kAudioChannelLayoutTag_MPEG_4_0_B: AudioChannelLayoutTag = (116 << 16) | 4;
pub const kAudioChannelLayoutTag_MPEG_5_0_A: AudioChannelLayoutTag = (117 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_B: AudioChannelLayoutTag = (118 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_C: AudioChannelLayoutTag = (119 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_0_D: AudioChannelLayoutTag = (120 << 16) | 5;
pub const kAudioChannelLayoutTag_MPEG_5_1_A: AudioChannelLayoutTag = (121 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_B: AudioChannelLayoutTag = (122 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_C: AudioChannelLayoutTag = (123 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_5_1_D: AudioChannelLayoutTag = (124 << 16) | 6;
pub const kAudioChannelLayoutTag_MPEG_6_1_A: AudioChannelLayoutTag = (125 << 16) | 7;
pub const kAudioChannelLayoutTag_MPEG_7_1_A: AudioChannelLayoutTag = (126 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_7_1_B: AudioChannelLayoutTag = (127 << 16) | 8;
pub const kAudioChannelLayoutTag_MPEG_7_1_C: AudioChannelLayoutTag = (128 << 16) | 8;
pub const kAudioChannelLayoutTag_Emagic_Default_7_1: AudioChannelLayoutTag = (129 << 16) | 8;
pub const kAudioChannelLayoutTag_SMPTE_DTV: AudioChannelLayoutTag = (130 << 16) | 8;
pub const kAudioChannelLayoutTag_ITU_1_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_ITU_2_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_ITU_2_1: AudioChannelLayoutTag = (131 << 16) | 3;
pub const kAudioChannelLayoutTag_ITU_2_2: AudioChannelLayoutTag = (132 << 16) | 4;
pub const kAudioChannelLayoutTag_ITU_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_ITU_3_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_ITU_3_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_ITU_3_2_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_ITU_3_4_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_DVD_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_Mono;
pub const kAudioChannelLayoutTag_DVD_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_Stereo;
pub const kAudioChannelLayoutTag_DVD_2: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_1;
pub const kAudioChannelLayoutTag_DVD_3: AudioChannelLayoutTag = kAudioChannelLayoutTag_ITU_2_2;
pub const kAudioChannelLayoutTag_DVD_4: AudioChannelLayoutTag = (133 << 16) | 3;
pub const kAudioChannelLayoutTag_DVD_5: AudioChannelLayoutTag = (134 << 16) | 4;
pub const kAudioChannelLayoutTag_DVD_6: AudioChannelLayoutTag = (135 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_7: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_A;
pub const kAudioChannelLayoutTag_DVD_8: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_A;
pub const kAudioChannelLayoutTag_DVD_9: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_A;
pub const kAudioChannelLayoutTag_DVD_10: AudioChannelLayoutTag = (136 << 16) | 4;
pub const kAudioChannelLayoutTag_DVD_11: AudioChannelLayoutTag = (137 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_12: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_DVD_13: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_8;
pub const kAudioChannelLayoutTag_DVD_14: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_9;
pub const kAudioChannelLayoutTag_DVD_15: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_10;
pub const kAudioChannelLayoutTag_DVD_16: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_11;
pub const kAudioChannelLayoutTag_DVD_17: AudioChannelLayoutTag = kAudioChannelLayoutTag_DVD_12;
pub const kAudioChannelLayoutTag_DVD_18: AudioChannelLayoutTag = (138 << 16) | 5;
pub const kAudioChannelLayoutTag_DVD_19: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_B;
pub const kAudioChannelLayoutTag_DVD_20: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_B;
pub const kAudioChannelLayoutTag_AudioUnit_4: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Quadraphonic;
pub const kAudioChannelLayoutTag_AudioUnit_5: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Pentagonal;
pub const kAudioChannelLayoutTag_AudioUnit_6: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Hexagonal;
pub const kAudioChannelLayoutTag_AudioUnit_8: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Octagonal;
pub const kAudioChannelLayoutTag_AudioUnit_5_0: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_0_B;
pub const kAudioChannelLayoutTag_AudioUnit_6_0: AudioChannelLayoutTag = (139 << 16) | 6;
pub const kAudioChannelLayoutTag_AudioUnit_7_0: AudioChannelLayoutTag = (140 << 16) | 7;
pub const kAudioChannelLayoutTag_AudioUnit_7_0_Front: AudioChannelLayoutTag = (148 << 16) | 7;
pub const kAudioChannelLayoutTag_AudioUnit_5_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_5_1_A;
pub const kAudioChannelLayoutTag_AudioUnit_6_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_6_1_A;
pub const kAudioChannelLayoutTag_AudioUnit_7_1: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_7_1_C;
pub const kAudioChannelLayoutTag_AudioUnit_7_1_Front: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_MPEG_7_1_A;
pub const kAudioChannelLayoutTag_AAC_3_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_3_0_B;
pub const kAudioChannelLayoutTag_AAC_Quadraphonic: AudioChannelLayoutTag =
    kAudioChannelLayoutTag_Quadraphonic;
pub const kAudioChannelLayoutTag_AAC_4_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_4_0_B;
pub const kAudioChannelLayoutTag_AAC_5_0: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_0_D;
pub const kAudioChannelLayoutTag_AAC_5_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_5_1_D;
pub const kAudioChannelLayoutTag_AAC_6_0: AudioChannelLayoutTag = (141 << 16) | 6;
pub const kAudioChannelLayoutTag_AAC_6_1: AudioChannelLayoutTag = (142 << 16) | 7;
pub const kAudioChannelLayoutTag_AAC_7_0: AudioChannelLayoutTag = (143 << 16) | 7;
pub const kAudioChannelLayoutTag_AAC_7_1: AudioChannelLayoutTag = kAudioChannelLayoutTag_MPEG_7_1_B;
pub const kAudioChannelLayoutTag_AAC_7_1_B: AudioChannelLayoutTag = (183 << 16) | 8;
pub const kAudioChannelLayoutTag_AAC_7_1_C: AudioChannelLayoutTag = (184 << 16) | 8;
pub const kAudioChannelLayoutTag_AAC_Octagonal: AudioChannelLayoutTag = (144 << 16) | 8;
pub const kAudioChannelLayoutTag_TMH_10_2_std: AudioChannelLayoutTag = (145 << 16) | 16;
pub const kAudioChannelLayoutTag_TMH_10_2_full: AudioChannelLayoutTag = (146 << 16) | 21;
pub const kAudioChannelLayoutTag_AC3_1_0_1: AudioChannelLayoutTag = (149 << 16) | 2;
pub const kAudioChannelLayoutTag_AC3_3_0: AudioChannelLayoutTag = (150 << 16) | 3;
pub const kAudioChannelLayoutTag_AC3_3_1: AudioChannelLayoutTag = (151 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_3_0_1: AudioChannelLayoutTag = (152 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_2_1_1: AudioChannelLayoutTag = (153 << 16) | 4;
pub const kAudioChannelLayoutTag_AC3_3_1_1: AudioChannelLayoutTag = (154 << 16) | 5;
pub const kAudioChannelLayoutTag_EAC_6_0_A: AudioChannelLayoutTag = (155 << 16) | 6;
pub const kAudioChannelLayoutTag_EAC_7_0_A: AudioChannelLayoutTag = (156 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_A: AudioChannelLayoutTag = (157 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_B: AudioChannelLayoutTag = (158 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_6_1_C: AudioChannelLayoutTag = (159 << 16) | 7;
pub const kAudioChannelLayoutTag_EAC3_7_1_A: AudioChannelLayoutTag = (160 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_B: AudioChannelLayoutTag = (161 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_C: AudioChannelLayoutTag = (162 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_D: AudioChannelLayoutTag = (163 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_E: AudioChannelLayoutTag = (164 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_F: AudioChannelLayoutTag = (165 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_G: AudioChannelLayoutTag = (166 << 16) | 8;
pub const kAudioChannelLayoutTag_EAC3_7_1_H: AudioChannelLayoutTag = (167 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_3_1: AudioChannelLayoutTag = (168 << 16) | 4;
pub const kAudioChannelLayoutTag_DTS_4_1: AudioChannelLayoutTag = (169 << 16) | 5;
pub const kAudioChannelLayoutTag_DTS_6_0_A: AudioChannelLayoutTag = (170 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_0_B: AudioChannelLayoutTag = (171 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_0_C: AudioChannelLayoutTag = (172 << 16) | 6;
pub const kAudioChannelLayoutTag_DTS_6_1_A: AudioChannelLayoutTag = (173 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_6_1_B: AudioChannelLayoutTag = (174 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_6_1_C: AudioChannelLayoutTag = (175 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_7_0: AudioChannelLayoutTag = (176 << 16) | 7;
pub const kAudioChannelLayoutTag_DTS_7_1: AudioChannelLayoutTag = (177 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_0_A: AudioChannelLayoutTag = (178 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_0_B: AudioChannelLayoutTag = (179 << 16) | 8;
pub const kAudioChannelLayoutTag_DTS_8_1_A: AudioChannelLayoutTag = (180 << 16) | 9;
pub const kAudioChannelLayoutTag_DTS_8_1_B: AudioChannelLayoutTag = (181 << 16) | 9;
pub const kAudioChannelLayoutTag_DTS_6_1_D: AudioChannelLayoutTag = (182 << 16) | 7;
pub const kAudioChannelLayoutTag_HOA_ACN_SN3D: AudioChannelLayoutTag = (190 << 16) | 0;
pub const kAudioChannelLayoutTag_HOA_ACN_N3D: AudioChannelLayoutTag = (191 << 16) | 0;
pub const kAudioChannelLayoutTag_DiscreteInOrder: AudioChannelLayoutTag = (147 << 16) | 0;
pub const kAudioChannelLayoutTag_Unknown: AudioChannelLayoutTag = 4294901760;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioChannelDescription {
    pub mChannelLabel: AudioChannelLabel,
    pub mChannelFlags: AudioChannelFlags,
    pub mCoordinates: [c_float; 3],
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioChannelLayout {
    pub mChannelLayoutTag: AudioChannelLayoutTag,
    pub mChannelBitmap: AudioChannelBitmap,
    pub mNumberChannelDescriptions: u32,
    pub mChannelDescriptions: [AudioChannelDescription; 1],
}
