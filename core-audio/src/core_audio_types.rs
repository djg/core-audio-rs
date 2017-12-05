use ffi;

use ffi_binding::Binding;
use std::{fmt, marker, mem, ops, slice};
use util::*;

//==================================================================================================
// AudioValueRange

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioValueRange {
    pub min: f64,
    pub max: f64,
}

//==================================================================================================
// AudioValueTranslation

pub struct AudioValueTranslation<'t, 'u, T: 't, U: 'u> {
    input_data: &'t T,
    output_data: &'u mut U,
}

impl<'t, 'u, T, U> AudioValueTranslation<'t, 'u, T, U> {
    pub fn new(
        t: &'t T,
        u: &'u mut U,
    ) -> Self {
        AudioValueTranslation {
            input_data: t,
            output_data: u,
        }
    }
}

impl<'t, 'u, T, U> Binding for AudioValueTranslation<'t, 'u, T, U> {
    type Ffi = ffi::AudioValueTranslation;

    fn as_ffi(&self) -> Self::Ffi {
        unsafe {
            let mut translation: Self::Ffi = mem::zeroed();
            translation.mInputData = self.input_data as *const T as *mut _;
            translation.mInputDataSize = mem::size_of::<T>() as _;
            translation.mOutputData = self.output_data as *const U as *mut _;
            translation.mOutputDataSize = mem::size_of::<T>() as _;
            translation
        }
    }

    unsafe fn from_ffi(_ffi: Self::Ffi) -> Self {
        panic!("unimplemented");
    }
}

//==================================================================================================
// AudioBuffer/AudioBufferList

#[derive(Clone, Copy, Debug)]
pub struct AudioBuffer(ffi::AudioBuffer);

impl AudioBuffer {
    /// The number of interleaved channels in the buffer.
    pub fn num_channels(&self) -> usize { self.0.mNumberChannels as _ }
}

impl ops::Deref for AudioBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.0.mData as *const _, self.0.mDataByteSize as _) }
    }
}

impl ops::DerefMut for AudioBuffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.0.mData as *mut _, self.0.mDataByteSize as _) }
    }
}

//==================================================================================================
// AudioBufferList

pub struct AudioBufferList {
    vla: VariableLengthArray<ffi::AudioBufferList, AudioBuffer>,
}

impl AudioBufferList {
    pub fn with_len(len: usize) -> Self {
        AudioBufferList {
            vla: VariableLengthArray::with_len(len),
        }
    }
}

impl Binding for AudioBufferList {
    type Ffi = *mut ffi::AudioBufferList;

    #[inline]
    fn as_ffi(&self) -> Self::Ffi { self.vla.as_ffi() }

    #[inline]
    unsafe fn from_ffi(ffi: Self::Ffi) -> AudioBufferList {
        AudioBufferList {
            vla: VariableLengthArray::from_ffi(ffi),
        }
    }
}

impl ops::Deref for AudioBufferList {
    type Target = [AudioBuffer];

    fn deref(&self) -> &Self::Target {
        unsafe {
            let list = &(*self.as_ffi());
            slice::from_raw_parts(
                &list.mBuffers[0] as *const _ as *const _,
                list.mNumberBuffers as _,
            )
        }
    }
}

impl ops::DerefMut for AudioBufferList {
    fn deref_mut(&mut self) -> &mut [AudioBuffer] {
        unsafe {
            let list = &mut (*self.as_ffi());
            slice::from_raw_parts_mut(
                &mut list.mBuffers[0] as *mut _ as *mut _,
                list.mNumberBuffers as _,
            )
        }
    }
}


//==================================================================================================
// Audio Formats

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioFormat(ffi::AudioFormatID);
impl AudioFormat {
    pub const LINEAR_PCM: AudioFormat = AudioFormat(ffi::kAudioFormatLinearPCM);
    pub const AC3: AudioFormat = AudioFormat(ffi::kAudioFormatAC3);
    pub const _60958AC3: AudioFormat = AudioFormat(ffi::kAudioFormat60958AC3);
    pub const APPLE_IMA4: AudioFormat = AudioFormat(ffi::kAudioFormatAppleIMA4);
    pub const MPEG4_AAC: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC);
    pub const MPEG4_CELP: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4CELP);
    pub const MPEG4_HVXC: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4HVXC);
    pub const MPEG4_TWIN_VQ: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4TwinVQ);
    pub const MACE3: AudioFormat = AudioFormat(ffi::kAudioFormatMACE3);
    pub const MACE6: AudioFormat = AudioFormat(ffi::kAudioFormatMACE6);
    pub const U_LAW: AudioFormat = AudioFormat(ffi::kAudioFormatULaw);
    pub const A_LAW: AudioFormat = AudioFormat(ffi::kAudioFormatALaw);
    pub const QDESIGN: AudioFormat = AudioFormat(ffi::kAudioFormatQDesign);
    pub const QDESIGN2: AudioFormat = AudioFormat(ffi::kAudioFormatQDesign2);
    pub const QUALCOMM: AudioFormat = AudioFormat(ffi::kAudioFormatQUALCOMM);
    pub const MPEGLAYER1: AudioFormat = AudioFormat(ffi::kAudioFormatMPEGLayer1);
    pub const MPEGLAYER2: AudioFormat = AudioFormat(ffi::kAudioFormatMPEGLayer2);
    pub const MPEGLAYER3: AudioFormat = AudioFormat(ffi::kAudioFormatMPEGLayer3);
    pub const TIME_CODE: AudioFormat = AudioFormat(ffi::kAudioFormatTimeCode);
    pub const MIDI_STREAM: AudioFormat = AudioFormat(ffi::kAudioFormatMIDIStream);
    pub const PARAMETER_VALUE_STREAM: AudioFormat = AudioFormat(ffi::kAudioFormatParameterValueStream);
    pub const APPLE_LOSSLESS: AudioFormat = AudioFormat(ffi::kAudioFormatAppleLossless);
    pub const MPEG4AAC_HE: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC_HE);
    pub const MPEG4AAC_LD: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC_LD);
    pub const MPEG4AAC_ELD: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC_ELD);
    pub const MPEG4AAC_ELD_SBR: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC_ELD_SBR);
    pub const MPEG4AAC_ELD_V2: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC_ELD_V2);
    pub const MPEG4AAC_HE_V2: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC_HE_V2);
    pub const MPEG4AAC_SPATIAL: AudioFormat = AudioFormat(ffi::kAudioFormatMPEG4AAC_Spatial);
    pub const AMR: AudioFormat = AudioFormat(ffi::kAudioFormatAMR);
    pub const AMR_WB: AudioFormat = AudioFormat(ffi::kAudioFormatAMR_WB);
    pub const AUDIBLE: AudioFormat = AudioFormat(ffi::kAudioFormatAudible);
    pub const ILBC: AudioFormat = AudioFormat(ffi::kAudioFormatiLBC);
    pub const DVI_INTEL_IMA: AudioFormat = AudioFormat(ffi::kAudioFormatDVIIntelIMA);
    pub const MICROSOFT_GSM: AudioFormat = AudioFormat(ffi::kAudioFormatMicrosoftGSM);
    pub const AES3: AudioFormat = AudioFormat(ffi::kAudioFormatAES3);
    pub const ENHANCED_AC3: AudioFormat = AudioFormat(ffi::kAudioFormatEnhancedAC3);
    pub const FLAC: AudioFormat = AudioFormat(ffi::kAudioFormatFLAC);
    pub const OPUS: AudioFormat = AudioFormat(ffi::kAudioFormatOpus);
}

bitflags! {
    pub struct AudioFormatFlags: ffi::AudioFormatFlags {
        const IS_FLOAT             = ffi::kAudioFormatFlagIsFloat;
        const IS_BIG_ENDIAN        = ffi::kAudioFormatFlagIsBigEndian;
        const IS_SIGNED_INTEGER    = ffi::kAudioFormatFlagIsSignedInteger;
        const IS_PACKED            = ffi::kAudioFormatFlagIsPacked;
        const IS_ALIGNED_HIGH      = ffi::kAudioFormatFlagIsAlignedHigh;
        const IS_NON_INTERLEAVED   = ffi::kAudioFormatFlagIsNonInterleaved;
        const IS_NON_MIXABLE       = ffi::kAudioFormatFlagIsNonMixable;
        const ARE_ALL_CLEAR        = ffi::kAudioFormatFlagsAreAllClear;
        const NATIVE_ENDIAN        = ffi::kAudioFormatFlagsNativeEndian;
        const CANONICAL            = ffi::kAudioFormatFlagsCanonical;
        const AUDIO_UNIT_CANONICAL = ffi::kAudioFormatFlagsAudioUnitCanonical;
    }
}

impl AudioFormatFlags {
    pub fn with_lpcm_flags(
        valid_bits_per_channel: u32,
        total_bits_per_channel: u32,
        is_float: bool,
        is_big_endian: bool,
        is_non_interleaved: bool,
    ) -> AudioFormatFlags {
        let mut result = if is_float {
            AudioFormatFlags::IS_FLOAT
        } else {
            AudioFormatFlags::IS_SIGNED_INTEGER
        };
        if is_big_endian {
            result |= AudioFormatFlags::IS_BIG_ENDIAN;
        }
        result |= if valid_bits_per_channel == total_bits_per_channel {
            AudioFormatFlags::IS_PACKED
        } else {
            AudioFormatFlags::IS_ALIGNED_HIGH
        };
        if is_non_interleaved {
            result |= AudioFormatFlags::IS_NON_INTERLEAVED;
        }
        result
    }
}


#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioStreamBasicDescription {
    pub sample_rate: f64,
    pub format_id: AudioFormat,
    pub format_flags: AudioFormatFlags,
    pub bytes_per_packet: u32,
    pub frames_per_packet: u32,
    pub bytes_per_frame: u32,
    pub channels_per_frame: u32,
    pub bits_per_channel: u32,
    _reserved: u32,
}

impl AudioStreamBasicDescription {
    pub fn with_lpcm(
        sample_rate: f64,
        channels_per_frame: u32,
        valid_bits_per_channel: u32,
        total_bits_per_channel: u32,
        is_float: bool,
        is_big_endian: bool,
        is_non_interleaved: bool,
    ) -> Self {
        let flags = AudioFormatFlags::with_lpcm_flags(
            valid_bits_per_channel,
            total_bits_per_channel,
            is_float,
            is_big_endian,
            is_non_interleaved,
        );
        let bytes_per_packet = match is_non_interleaved {
            true => 1,
            _ => channels_per_frame,
        } * (total_bits_per_channel / 8);
        let bytes_per_frame = match is_non_interleaved {
            true => 1,
            _ => channels_per_frame,
        } * (total_bits_per_channel / 8);

        AudioStreamBasicDescription {
            sample_rate: sample_rate,
            format_id: AudioFormat::LINEAR_PCM,
            format_flags: flags,
            bytes_per_packet: bytes_per_packet,
            frames_per_packet: 1,
            bytes_per_frame: bytes_per_frame,
            channels_per_frame: channels_per_frame,
            bits_per_channel: valid_bits_per_channel,
            _reserved: 0,
        }
    }

    pub fn is_native_endian(&self) -> bool {
        self.format_id == AudioFormat::LINEAR_PCM &&
            (self.format_flags & AudioFormatFlags::IS_BIG_ENDIAN) == AudioFormatFlags::NATIVE_ENDIAN
    }
}

//==================================================================================================
// Audio Channel Layout

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioChannelLabel(ffi::AudioChannelLabel);
impl AudioChannelLabel {
    pub const UNKNOWN: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Unknown);
    pub const UNUSED: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Unused);
    pub const USE_COORDINATES: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_UseCoordinates);
}

pub struct StandardChannelLabel {}
impl StandardChannelLabel {
    pub const LEFT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Left);
    pub const RIGHT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Right);
    pub const CENTER: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Center);
    pub const LFESCREEN: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_LFEScreen);
    pub const LEFT_SURROUND: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_LeftSurround);
    pub const RIGHT_SURROUND: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_RightSurround);
    pub const LEFT_CENTER: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_LeftCenter);
    pub const RIGHT_CENTER: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_RightCenter);
    pub const CENTER_SURROUND: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_CenterSurround);
    pub const LEFT_SURROUND_DIRECT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_LeftSurroundDirect);
    pub const RIGHT_SURROUND_DIRECT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_RightSurroundDirect);
    pub const TOP_CENTER_SURROUND: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_TopCenterSurround);
    pub const VERTICAL_HEIGHT_LEFT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_VerticalHeightLeft);
    pub const VERTICAL_HEIGHT_CENTER: AudioChannelLabel =
        AudioChannelLabel(ffi::kAudioChannelLabel_VerticalHeightCenter);
    pub const VERTICAL_HEIGHT_RIGHT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_VerticalHeightRight);

    pub const TOP_BACK_LEFT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_TopBackLeft);
    pub const TOP_BACK_CENTER: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_TopBackCenter);
    pub const TOP_BACK_RIGHT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_TopBackRight);

    pub const REAR_SURROUND_LEFT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_RearSurroundLeft);
    pub const REAR_SURROUND_RIGHT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_RearSurroundRight);
    pub const LEFT_WIDE: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_LeftWide);
    pub const RIGHT_WIDE: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_RightWide);
    pub const LFE2: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_LFE2);
    pub const LEFT_TOTAL: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_LeftTotal);
    pub const RIGHT_TOTAL: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_RightTotal);
    pub const HEARING_IMPAIRED: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HearingImpaired);
    pub const NARRATION: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Narration);
    pub const MONO: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Mono);
    pub const DIALOG_CENTRIC_MIX: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_DialogCentricMix);

    pub const CENTER_SURROUND_DIRECT: AudioChannelLabel =
        AudioChannelLabel(ffi::kAudioChannelLabel_CenterSurroundDirect);

    pub const HAPTIC: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Haptic);
}

// first order ambisonic channels
pub struct AmbisonicChannelLabel {}
impl AmbisonicChannelLabel {
    pub const W: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Ambisonic_W);
    pub const X: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Ambisonic_X);
    pub const Y: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Ambisonic_Y);
    pub const Z: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Ambisonic_Z);
}

// Mid/Side Recording
pub struct MidSideChannelLabel {}
impl MidSideChannelLabel {
    pub const MID: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_MS_Mid);
    pub const SIDE: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_MS_Side);
}

// X-Y Recording
pub struct XyChannelLabel {}
impl XyChannelLabel {
    pub const X: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_XY_X);
    pub const Y: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_XY_Y);
}

// other
pub struct OtherChannelLabel {}
impl OtherChannelLabel {
    pub const HEADPHONES_LEFT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HeadphonesLeft);
    pub const HEADPHONES_RIGHT: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HeadphonesRight);
    pub const CLICK_TRACK: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_ClickTrack);
    pub const FOREIGN_LANGUAGE: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_ForeignLanguage);
}

// generic discrete channel
pub struct DiscreteChannelLabel {}
impl DiscreteChannelLabel {
    pub const DISCRETE: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete);
    pub const DISCRETE_0: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_0);
    pub const DISCRETE_1: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_1);
    pub const DISCRETE_2: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_2);
    pub const DISCRETE_3: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_3);
    pub const DISCRETE_4: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_4);
    pub const DISCRETE_5: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_5);
    pub const DISCRETE_6: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_6);
    pub const DISCRETE_7: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_7);
    pub const DISCRETE_8: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_8);
    pub const DISCRETE_9: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_9);
    pub const DISCRETE_10: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_10);
    pub const DISCRETE_11: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_11);
    pub const DISCRETE_12: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_12);
    pub const DISCRETE_13: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_13);
    pub const DISCRETE_14: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_14);
    pub const DISCRETE_15: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_15);
    pub const DISCRETE_65535: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_Discrete_65535);
}

pub struct HoaAcnChannelLabel {}
impl HoaAcnChannelLabel {
    pub const HOA_ACN: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN);
    pub const HOA_ACN0: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_0);
    pub const HOA_ACN1: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_1);
    pub const HOA_ACN2: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_2);
    pub const HOA_ACN3: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_3);
    pub const HOA_ACN4: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_4);
    pub const HOA_ACN5: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_5);
    pub const HOA_ACN6: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_6);
    pub const HOA_ACN7: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_7);
    pub const HOA_ACN8: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_8);
    pub const HOA_ACN9: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_9);
    pub const HOA_ACN10: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_10);
    pub const HOA_ACN11: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_11);
    pub const HOA_ACN12: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_12);
    pub const HOA_ACN13: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_13);
    pub const HOA_ACN14: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_14);
    pub const HOA_ACN15: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_15);
    pub const HOA_ACN65024: AudioChannelLabel = AudioChannelLabel(ffi::kAudioChannelLabel_HOA_ACN_65024);
}

bitflags! {
    pub struct AudioChannelBitmap: u32 {
        const LEFT = ffi::kAudioChannelBit_Left;
        const RIGHT = ffi::kAudioChannelBit_Right;
        const CENTER = ffi::kAudioChannelBit_Center;
        const LFESCREEN = ffi::kAudioChannelBit_LFEScreen;
        const LEFT_SURROUND = ffi::kAudioChannelBit_LeftSurround;
        const RIGHT_SURROUND = ffi::kAudioChannelBit_RightSurround;
        const LEFT_CENTER = ffi::kAudioChannelBit_LeftCenter;
        const RIGHT_CENTER = ffi::kAudioChannelBit_RightCenter;
        const CENTER_SURROUND = ffi::kAudioChannelBit_CenterSurround;
        const LEFT_SURROUND_DIRECT = ffi::kAudioChannelBit_LeftSurroundDirect;
        const RIGHT_SURROUND_DIRECT = ffi::kAudioChannelBit_RightSurroundDirect;
        const TOP_CENTER_SURROUND = ffi::kAudioChannelBit_TopCenterSurround;
        const VERTICAL_HEIGHT_LEFT = ffi::kAudioChannelBit_VerticalHeightLeft;
        const VERTICAL_HEIGHT_CENTER = ffi::kAudioChannelBit_VerticalHeightCenter;
        const VERTICAL_HEIGHT_RIGHT = ffi::kAudioChannelBit_VerticalHeightRight;
        const TOP_BACK_LEFT = ffi::kAudioChannelBit_TopBackLeft;
        const TOP_BACK_CENTER = ffi::kAudioChannelBit_TopBackCenter;
        const TOP_BACK_RIGHT = ffi::kAudioChannelBit_TopBackRight;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioChannelLayoutTag(u32);
impl AudioChannelLayoutTag {
    pub const USE_CHANNEL_DESCRIPTIONS: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_UseChannelDescriptions);
    pub const USE_CHANNEL_BITMAP: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_UseChannelBitmap);
    pub const DISCRETE_IN_ORDER: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DiscreteInOrder);
    pub const UNKNOWN: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Unknown);

    pub fn channels(&self) -> usize { (self.0 & 0xFFFF) as _ }

    pub fn discrete_in_order(channels: u16) -> Self {
        AudioChannelLayoutTag(Self::DISCRETE_IN_ORDER.0 | channels as u32)
    }
    pub fn unknown(channels: u16) -> Self { AudioChannelLayoutTag(Self::UNKNOWN.0 | channels as u32) }
}

pub struct StandardChannelLayoutTag {}
impl StandardChannelLayoutTag {
    pub const MONO: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Mono);
    pub const STEREO: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Stereo);
    pub const STEREO_HEADPHONES: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_StereoHeadphones);
    pub const MATRIX_STEREO: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MatrixStereo);
    pub const MID_SIDE: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MidSide);
    pub const XY: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_XY);
    pub const BINAURAL: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Binaural);
    pub const AMBISONIC_B_FORMAT: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Ambisonic_B_Format);
    pub const QUADRAPHONIC: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Quadraphonic);
    pub const PENTAGONAL: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Pentagonal);
    pub const HEXAGONAL: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Hexagonal);
    pub const OCTAGONAL: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Octagonal);
    pub const CUBE: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Cube);
}

//  MPEG defined layouts
pub struct MpegChannelLayoutTag {}
impl MpegChannelLayoutTag {
    pub const _1_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_1_0);
    pub const _2_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_2_0);
    pub const _3_0_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_3_0_A);
    pub const _3_0_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_3_0_B);
    pub const _4_0_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_4_0_A);
    pub const _4_0_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_4_0_B);
    pub const _5_0_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_0_A);
    pub const _5_0_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_0_B);
    pub const _5_0_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_0_C);
    pub const _5_0_D: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_0_D);
    pub const _5_1_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_1_A);
    pub const _5_1_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_1_B);
    pub const _5_1_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_1_C);
    pub const _5_1_D: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_5_1_D);
    pub const _6_1_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_6_1_A);
    pub const _7_1_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_7_1_A);
    pub const _7_1_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_7_1_B);
    pub const _7_1_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_MPEG_7_1_C);
    pub const EMAGIC_DEFAULT_7_1: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_Emagic_Default_7_1);
    pub const SMPTE_DTV: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_SMPTE_DTV);
}

//  ITU defined layouts
pub struct ItuChannelLayoutTag {}
impl ItuChannelLayoutTag {
    pub const _1_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_1_0);
    pub const _2_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_2_0);

    pub const _2_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_2_1);
    pub const _2_2: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_2_2);
    pub const _3_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_3_0);
    pub const _3_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_3_1);

    pub const _3_2: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_3_2);
    pub const _3_2_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_3_2_1);
    pub const _3_4_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_ITU_3_4_1);
}

// DVD defined layouts
pub struct DvdChannelLayoutTag {}
impl DvdChannelLayoutTag {
    pub const _0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_0);
    pub const _1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_1);
    pub const _2: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_2);
    pub const _3: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_3);
    pub const _4: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_4);
    pub const _5: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_5);
    pub const _6: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_6);
    pub const _7: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_7);
    pub const _8: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_8);
    pub const _9: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_9);
    pub const _10: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_10);
    pub const _11: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_11);
    pub const _12: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_12);
    pub const _13: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_13);
    pub const _14: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_14);
    pub const _15: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_15);
    pub const _16: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_16);
    pub const _17: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_17);
    pub const _18: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_18);
    pub const _19: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_19);
    pub const _20: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DVD_20);
}

// These layouts are recommended for AudioUnit usage
// These are the symmetrical layouts
pub struct AudioUnitChannelLayoutTag {}
impl AudioUnitChannelLayoutTag {
    pub const _4: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_4);
    pub const _5: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_5);
    pub const _6: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_6);
    pub const _8: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_8);
    pub const _5_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_5_0);
    pub const _6_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_6_0);
    pub const _7_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_7_0);
    pub const _7_0_FRONT: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_7_0_Front);
    pub const _5_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_5_1);
    pub const _6_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_6_1);
    pub const _7_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_7_1);
    pub const _7_1_FRONT: AudioChannelLayoutTag =
        AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AudioUnit_7_1_Front);
}

pub struct AacChannelLayoutTag {}
impl AacChannelLayoutTag {
    pub const _3_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_3_0);
    pub const QUADRAPHONIC: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_Quadraphonic);
    pub const _4_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_4_0);
    pub const _5_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_5_0);
    pub const _5_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_5_1);
    pub const _6_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_6_0);
    pub const _6_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_6_1);
    pub const _7_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_7_0);
    pub const _7_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_7_1);
    pub const _7_1_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_7_1_B);
    pub const _7_1_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_7_1_C);
    pub const OCTAGONAL: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AAC_Octagonal);
}

pub struct TmhChannelLayoutTag {}
impl TmhChannelLayoutTag {
    pub const _10_2_STD: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_TMH_10_2_std);
    pub const _10_2_FULL: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_TMH_10_2_full);
}

pub struct Ac3ChannelLayoutTag {}
impl Ac3ChannelLayoutTag {
    pub const _1_0_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AC3_1_0_1);
    pub const _3_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AC3_3_0);
    pub const _3_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AC3_3_1);
    pub const _3_0_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AC3_3_0_1);
    pub const _2_1_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AC3_2_1_1);
    pub const _3_1_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_AC3_3_1_1);
}

pub struct Eac3ChannelLayoutTag {}
impl Eac3ChannelLayoutTag {
    pub const _6_0_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC_6_0_A);
    pub const _7_0_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC_7_0_A);
    pub const _6_1_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_6_1_A);
    pub const _6_1_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_6_1_B);
    pub const _6_1_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_6_1_C);
    pub const _7_1_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_A);
    pub const _7_1_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_B);
    pub const _7_1_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_C);
    pub const _7_1_D: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_D);
    pub const _7_1_E: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_E);
    pub const _7_1_F: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_F);
    pub const _7_1_G: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_G);
    pub const _7_1_H: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_EAC3_7_1_H);
}

pub struct DtsChannelLayoutTag {}
impl DtsChannelLayoutTag {
    pub const _3_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_3_1);
    pub const _4_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_4_1);
    pub const _6_0_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_6_0_A);
    pub const _6_0_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_6_0_B);
    pub const _6_0_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_6_0_C);
    pub const _6_1_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_6_1_A);
    pub const _6_1_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_6_1_B);
    pub const _6_1_C: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_6_1_C);
    pub const _7_0: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_7_0);
    pub const _7_1: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_7_1);
    pub const _8_0_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_8_0_A);
    pub const _8_0_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_8_0_B);
    pub const _8_1_A: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_8_1_A);
    pub const _8_1_B: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_8_1_B);
    pub const _6_1_D: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_DTS_6_1_D);
}

pub struct HoaChannelLayoutTag {}
impl HoaChannelLayoutTag {
    pub const ACN_SN3D: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_HOA_ACN_SN3D);
    pub const ACN_N3D: AudioChannelLayoutTag = AudioChannelLayoutTag(ffi::kAudioChannelLayoutTag_HOA_ACN_N3D);
}

bitflags! {
    pub struct AudioChannelFlags: ffi::AudioChannelFlags {
        const ALL_OFF = ffi::kAudioChannelFlags_AllOff;
        const RECTANGULAR_COORDINATES = ffi::kAudioChannelFlags_RectangularCoordinates;
        const SPHERICAL_COORDINATES = ffi::kAudioChannelFlags_SphericalCoordinates;
        const METERS = ffi::kAudioChannelFlags_Meters;
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioChannelRectangularCoordinates {
    pub left_right: f32,
    pub back_front: f32,
    pub down_up: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AudioChannelSphericalCoordinates {
    pub azimuth: f32,
    pub elevation: f32,
    pub distance: f32,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioChannelDescription {
    pub channel_label: AudioChannelLabel,
    pub channel_flags: AudioChannelFlags,
    coordinates: [f32; 3],
}

impl AudioChannelDescription {
    pub fn rectangular_coordinate(&self) -> Option<&AudioChannelRectangularCoordinates> {
        if self.channel_flags.contains(
            AudioChannelFlags::RECTANGULAR_COORDINATES,
        )
        {
            Some(unsafe { mem::transmute(&self.coordinates) })
        } else {
            None
        }
    }

    pub fn spherical_coordinate(&self) -> Option<&AudioChannelSphericalCoordinates> {
        if self.channel_flags.contains(
            AudioChannelFlags::SPHERICAL_COORDINATES,
        )
        {
            Some(unsafe { mem::transmute(&self.coordinates) })
        } else {
            None
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct AudioChannelLayout {
    pub channel_layout_tag: AudioChannelLayoutTag,
    pub channel_bitmap: AudioChannelBitmap,
    pub number_channel_descriptions: u32,
    channel_descriptions: [AudioChannelDescription; 1],
}

impl AudioChannelLayout {
    pub fn channel_descriptions(&self) -> &[AudioChannelDescription] {
        unsafe {
            slice::from_raw_parts(
                &self.channel_descriptions as *const _,
                self.number_channel_descriptions as _,
            )
        }
    }
}

//==================================================================================================
// Audio Time Stamps

#[repr(u32)]
pub enum SMPTETimeType {
    _24 = ffi::kSMPTETimeType24,
    _25 = ffi::kSMPTETimeType25,
    _30Drop = ffi::kSMPTETimeType30Drop,
    _30 = ffi::kSMPTETimeType30,
    _2997 = ffi::kSMPTETimeType2997,
    _2997Drop = ffi::kSMPTETimeType2997Drop,
    _60 = ffi::kSMPTETimeType60,
    _5994 = ffi::kSMPTETimeType5994,
    _60Drop = ffi::kSMPTETimeType60Drop,
    _5994Drop = ffi::kSMPTETimeType5994Drop,
    _50 = ffi::kSMPTETimeType50,
    _2398 = ffi::kSMPTETimeType2398,
}

bitflags! {
    pub struct SMPTETimeFlags: ffi::SMPTETimeFlags {
        const UNKNOWN = ffi::kSMPTETimeUnknown;
        const VALID   = ffi::kSMPTETimeValid;
        const RUNNING = ffi::kSMPTETimeRunning;
    }
}


#[derive(Clone, Copy, Debug)]
pub struct SMPTETime(ffi::SMPTETime);
impl SMPTETime {
    pub fn subframes(&self) -> i16 { self.0.mSubframes }

    pub fn subframe_divisor(&self) -> i16 { self.0.mSubframeDivisor }

    pub fn counter(&self) -> u32 { self.0.mCounter }

    pub fn kind(&self) -> SMPTETimeType { unsafe { mem::transmute(self.0.mType) } }

    pub fn flags(&self) -> SMPTETimeFlags { SMPTETimeFlags::from_bits_truncate(self.0.mFlags) }

    pub fn hours(&self) -> i16 { self.0.mHours }

    pub fn minutes(&self) -> i16 { self.0.mMinutes }

    pub fn seconds(&self) -> i16 { self.0.mSeconds }

    pub fn frames(&self) -> i16 { self.0.mFrames }
}

pub struct AudioTimeStamp<'a> {
    inner: AudioTimeStampInner,
    _marker: marker::PhantomData<&'a mut ffi::AudioTimeStamp>,
}

enum AudioTimeStampInner {
    Borrowed(*const ffi::AudioTimeStamp),
    Owned(ffi::AudioTimeStamp),
}

impl<'a> AudioTimeStamp<'a> {
    pub fn with_sample_time(sample_time: f64) -> AudioTimeStamp<'static> {
        let mut result = unsafe { mem::uninitialized() };
        ffi::FillOutAudioTimeStampWithSampleTime(&mut result, sample_time);
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(result),
            _marker: marker::PhantomData,
        }
    }

    pub fn with_host_time(host_time: u64) -> AudioTimeStamp<'static> {
        let mut result = unsafe { mem::uninitialized() };
        ffi::FillOutAudioTimeStampWithHostTime(&mut result, host_time);
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(result),
            _marker: marker::PhantomData,
        }
    }

    pub fn with_sample_and_host_time(
        sample_time: f64,
        host_time: u64,
    ) -> AudioTimeStamp<'static> {
        let mut result = unsafe { mem::uninitialized() };
        ffi::FillOutAudioTimeStampWithSampleAndHostTime(&mut result, sample_time, host_time);
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(result),
            _marker: marker::PhantomData,
        }
    }

    pub fn sample_time(&self) -> Option<f64> {
        unsafe {
            if (*self.as_ffi()).mFlags & ffi::kAudioTimeStampSampleTimeValid != 0 {
                Some((*self.as_ffi()).mSampleTime)
            } else {
                None
            }
        }
    }

    pub fn host_time(&self) -> Option<u64> {
        unsafe {
            if (*self.as_ffi()).mFlags & ffi::kAudioTimeStampHostTimeValid != 0 {
                Some((*self.as_ffi()).mHostTime)
            } else {
                None
            }
        }
    }

    pub fn rate_scalar(&self) -> Option<f64> {
        unsafe {
            if (*self.as_ffi()).mFlags & ffi::kAudioTimeStampRateScalarValid != 0 {
                Some((*self.as_ffi()).mRateScalar)
            } else {
                None
            }
        }
    }

    pub fn word_clock_time(&self) -> Option<u64> {
        unsafe {
            if (*self.as_ffi()).mFlags & ffi::kAudioTimeStampWordClockTimeValid != 0 {
                Some((*self.as_ffi()).mWordClockTime)
            } else {
                None
            }
        }
    }

    pub fn smpte_time(&self) -> Option<SMPTETime> {
        unsafe {
            if (*self.as_ffi()).mFlags & ffi::kAudioTimeStampSMPTETimeValid != 0 {
                Some(SMPTETime((*self.as_ffi()).mSMPTETime))
            } else {
                None
            }
        }
    }

    pub fn to_owned(&self) -> AudioTimeStamp<'static> {
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(unsafe { *self.as_ffi() }),
            _marker: marker::PhantomData,
        }
    }

    pub fn as_ffi_mut(&mut self) -> *mut ffi::AudioTimeStamp {
        match self.inner {
            AudioTimeStampInner::Borrowed(..) => {
                panic!("Can modify a borrowed AudioTimeStamp. Try calling to_owned().")
            },
            AudioTimeStampInner::Owned(ref mut ffi) => ffi,
        }
    }
}

impl<'a> Binding for AudioTimeStamp<'a> {
    type Ffi = *const ffi::AudioTimeStamp;

    fn as_ffi(&self) -> Self::Ffi {
        match self.inner {
            AudioTimeStampInner::Borrowed(ffi) => ffi,
            AudioTimeStampInner::Owned(ref ffi) => ffi,
        }
    }

    unsafe fn from_ffi(ffi: Self::Ffi) -> AudioTimeStamp<'a> {
        AudioTimeStamp {
            inner: AudioTimeStampInner::Borrowed(ffi),
            _marker: marker::PhantomData,
        }
    }
}

impl<'a> fmt::Debug for AudioTimeStamp<'a> {
    fn fmt(
        &self,
        f: &mut fmt::Formatter,
    ) -> fmt::Result {
        let mut ds = f.debug_struct("AudioTimeStamp");
        if let Some(sample_time) = self.sample_time() {
            ds.field("sample_time", &sample_time);
        }
        if let Some(host_time) = self.host_time() {
            ds.field("host_time", &host_time);
        }
        if let Some(rate_scalar) = self.rate_scalar() {
            ds.field("rate_scalar", &rate_scalar);
        }
        if let Some(word_clock_time) = self.word_clock_time() {
            ds.field("word_clock_time", &word_clock_time);
        }
        if let Some(smpte_time) = self.smpte_time() {
            ds.field("smpte_time", &smpte_time);
        }
        ds.finish()
    }
}

impl Default for AudioTimeStamp<'static> {
    fn default() -> Self {
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(Default::default()),
            _marker: marker::PhantomData,
        }
    }
}
