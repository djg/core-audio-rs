use ffi;

use call::Binding;
use std::{fmt, marker, mem, ops, slice};

//==================================================================================================
// AudioValueTranslation

pub struct AudioValueTranslation<'t, 'u, T: 't, U: 'u> {
    input_data: &'t T,
    output_data: &'u mut U,
}

impl<'t, 'u, T, U> AudioValueTranslation<'t, 'u, T, U> {
    pub fn new(t: &'t T, u: &'u mut U) -> Self {
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
            translation.mOutputData = self.output_data as *const U  as *mut _;
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
    pub fn num_channels(&self) -> usize {
        self.0.mNumberChannels as _
    }
}

impl ops::Deref for AudioBuffer {
    type Target = [u8];

    fn deref(&self) -> &Self::Target {
        unsafe { slice::from_raw_parts(self.0.mData as *const _,
                                       self.0.mDataByteSize as _) }
    }
}

impl ops::DerefMut for AudioBuffer {
    fn deref_mut(&mut self) -> &mut [u8] {
        unsafe { slice::from_raw_parts_mut(self.0.mData as *mut _,
                                           self.0.mDataByteSize as  _) }
    }
}

pub struct AudioBufferList(ffi::AudioBufferList);

impl ops::Deref for AudioBufferList {
    type Target = [AudioBuffer];

    fn deref(&self) -> &Self::Target {
        unsafe {
            slice::from_raw_parts(&self.0.mBuffers as *const _ as *const _,
                                  self.0.mNumberBuffers as _)
        }
    }
}

impl ops::DerefMut for AudioBufferList {
    fn deref_mut(&mut self) -> &mut [AudioBuffer] {
        unsafe {
            slice::from_raw_parts_mut(&mut self.0.mBuffers as *mut _ as *mut _,
                                      self.0.mNumberBuffers as _)
        }
    }
}

//==================================================================================================
// Audio Time Stamps

#[repr(u32)]
pub enum SMPTETimeType {
    _24       = ffi::kSMPTETimeType24,
    _25       = ffi::kSMPTETimeType25,
    _30Drop   = ffi::kSMPTETimeType30Drop,
    _30       = ffi::kSMPTETimeType30,
    _2997     = ffi::kSMPTETimeType2997,
    _2997Drop = ffi::kSMPTETimeType2997Drop,
    _60       = ffi::kSMPTETimeType60,
    _5994     = ffi::kSMPTETimeType5994,
    _60Drop   = ffi::kSMPTETimeType60Drop,
    _5994Drop = ffi::kSMPTETimeType5994Drop,
    _50       = ffi::kSMPTETimeType50,
    _2398     = ffi::kSMPTETimeType2398,
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
    pub fn subframes(&self) -> i16 {
        self.0.mSubframes
    }

    pub fn subframe_divisor(&self) -> i16 {
        self.0.mSubframeDivisor
    }

    pub fn counter(&self) -> u32 {
        self.0.mCounter
    }

    pub fn kind(&self) -> SMPTETimeType {
        unsafe { mem::transmute(self.0.mType) }
    }

    pub fn flags(&self) -> SMPTETimeFlags {
        SMPTETimeFlags::from_bits_truncate(self.0.mFlags)
    }

    pub fn hours(&self) -> i16 {
        self.0.mHours
    }

    pub fn minutes(&self) -> i16 {
        self.0.mMinutes
    }

    pub fn seconds(&self) -> i16 {
        self.0.mSeconds
    }

    pub fn frames(&self) -> i16 {
        self.0.mFrames
    }
}

pub struct AudioTimeStamp<'a> {
    inner: AudioTimeStampInner,
    _marker: marker::PhantomData<&'a mut ffi::AudioTimeStamp>
}

enum AudioTimeStampInner {
    Borrowed(*mut ffi::AudioTimeStamp),
    Owned(ffi::AudioTimeStamp),
}

impl<'a> AudioTimeStamp<'a> {
    pub fn with_sample_time(sample_time: f64) -> AudioTimeStamp<'static> {
        let mut result = unsafe { mem::uninitialized() };
        ffi::FillOutAudioTimeStampWithSampleTime(&mut result, sample_time);
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(result),
            _marker: marker::PhantomData
        }
    }

    pub fn with_host_time(host_time: u64) -> AudioTimeStamp<'static> {
        let mut result = unsafe { mem::uninitialized() };
        ffi::FillOutAudioTimeStampWithHostTime(&mut result, host_time);
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(result),
            _marker: marker::PhantomData
        }
    }

    pub fn with_sample_and_host_time(sample_time: f64, host_time: u64) -> AudioTimeStamp<'static> {
        let mut result  = unsafe { mem::uninitialized() };
        ffi::FillOutAudioTimeStampWithSampleAndHostTime(&mut result,
                                                        sample_time,
                                                        host_time);
        AudioTimeStamp {
            inner: AudioTimeStampInner::Owned(result),
            _marker: marker::PhantomData
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
}

impl<'a> Binding for AudioTimeStamp<'a> {
    type Ffi = *mut ffi::AudioTimeStamp;

    fn as_ffi(&self) -> Self::Ffi {
        match self.inner {
            AudioTimeStampInner::Borrowed(ffi) => ffi,
            AudioTimeStampInner::Owned(ref ffi) => ffi as *const _ as Self::Ffi,
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
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
            _marker: marker::PhantomData
        }
    }
}
