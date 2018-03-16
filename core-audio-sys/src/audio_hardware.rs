use super::*;
use std::default::Default;
use std::mem;
use std::os::raw::c_void;
use std::slice;

//==============================================================================
// Basic Constants

e! {
    enum AudioObjectID {
        kAudioObjectSystemObject                        = 1,
    }
}

//==============================================================================
// AudioObject Types

pub type AudioObjectPropertyListenerProc = Option<
    unsafe extern "C" fn(
        inObjectID: AudioObjectID,
        inNumberAddresses: u32,
        inAddresses: *const AudioObjectPropertyAddress,
        inClientData: *mut c_void,
    ) -> OSStatus,
>;

//==============================================================================
// AudioObject Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioObjectPropertyCreator = 0x6f706c67,
        kAudioObjectPropertyListenerAdded = 0x6c697361,
        kAudioObjectPropertyListenerRemoved = 0x6c697372,
    }
}

//==============================================================================
// AudioObject Functions

extern "C" {
    pub fn AudioObjectShow(inObjectID: AudioObjectID);
    pub fn AudioObjectHasProperty(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
    ) -> Boolean;
    pub fn AudioObjectIsPropertySettable(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        outIsSettable: *mut Boolean,
    ) -> OSStatus;
    pub fn AudioObjectGetPropertyDataSize(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: u32,
        inQualifierData: *const c_void,
        outDataSize: *mut u32,
    ) -> OSStatus;
    pub fn AudioObjectGetPropertyData(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: u32,
        inQualifierData: *const c_void,
        ioDataSize: *mut u32,
        outData: *mut c_void,
    ) -> OSStatus;
    pub fn AudioObjectSetPropertyData(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inQualifierDataSize: u32,
        inQualifierData: *const c_void,
        inDataSize: u32,
        inData: *const c_void,
    ) -> OSStatus;
    pub fn AudioObjectAddPropertyListener(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inListener: AudioObjectPropertyListenerProc,
        inClientData: *mut c_void,
    ) -> OSStatus;
    pub fn AudioObjectRemovePropertyListener(
        inObjectID: AudioObjectID,
        inAddress: *const AudioObjectPropertyAddress,
        inListener: AudioObjectPropertyListenerProc,
        inClientData: *mut c_void,
    ) -> OSStatus;
}

//==============================================================================
// AudioSystemObject Constants

e! {
    enum AudioClassID {
        kAudioSystemObjectClassID = 0x61737973,
    }
}

e! {
    enum AudioHardwarePowerHint: u32 {
        kAudioHardwarePowerHintNone                     = 0,
        kAudioHardwarePowerHintFavorSavingPower         = 1,
    }
}

//==============================================================================
// AudioSystemObject Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioHardwarePropertyDevices = 0x64657623,
        kAudioHardwarePropertyDefaultInputDevice = 0x64496e20,
        kAudioHardwarePropertyDefaultOutputDevice = 0x644f7574,
        kAudioHardwarePropertyDefaultSystemOutputDevice = 0x734f7574,
        kAudioHardwarePropertyTranslateUIDToDevice = 0x75696464,
        kAudioHardwarePropertyMixStereoToMono = 0x73746d6f,
        kAudioHardwarePropertyPlugInList = 0x706c6723,
        kAudioHardwarePropertyTranslateBundleIDToPlugIn = 0x62696470,
        kAudioHardwarePropertyTransportManagerList = 0x746d6723,
        kAudioHardwarePropertyTranslateBundleIDToTransportManager = 0x746d6269,
        kAudioHardwarePropertyBoxList = 0x626f7823,
        kAudioHardwarePropertyTranslateUIDToBox = 0x75696462,
        kAudioHardwarePropertyClockDeviceList = 0x636c6b23,
        kAudioHardwarePropertyTranslateUIDToClockDevice = 0x75696463,
        kAudioHardwarePropertyProcessIsMaster = 0x6d617374,
        kAudioHardwarePropertyIsInitingOrExiting = 0x696e6f74,
        kAudioHardwarePropertyUserIDChanged = 0x65756964,
        kAudioHardwarePropertyProcessIsAudible = 0x706d7574,
        kAudioHardwarePropertySleepingIsAllowed = 0x736c6570,
        kAudioHardwarePropertyUnloadingIsAllowed = 0x756e6c64,
        kAudioHardwarePropertyHogModeIsAllowed = 0x686f6772,
        kAudioHardwarePropertyUserSessionIsActiveOrHeadless = 0x75736572,
        kAudioHardwarePropertyServiceRestarted = 0x73727374,
        kAudioHardwarePropertyPowerHint = 0x706f7768,
    }
}

//==============================================================================
// AudioSystemObject Functions

extern "C" {
    pub fn AudioHardwareUnload() -> OSStatus;
    pub fn AudioHardwareCreateAggregateDevice(
        inDescription: CFDictionaryRef,
        outDeviceID: *mut AudioObjectID,
    ) -> OSStatus;
    pub fn AudioHardwareDestroyAggregateDevice(inDeviceID: AudioObjectID) -> OSStatus;
}

//==============================================================================
// AudioPlugIn Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioPlugInCreateAggregateDevice = 0x63616767,
        kAudioPlugInDestroyAggregateDevice = 0x64616767,
    }
}

//==============================================================================
// AudioTransportManager Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioTransportManagerCreateEndPointDevice = 0x63646576,
        kAudioTransportManagerDestroyEndPointDevice = 0x64646576,
    }
}

//==============================================================================
// AudioDevice Types

pub type AudioDeviceIOProc = Option<
    unsafe extern "C" fn(
        inDevice: AudioObjectID,
        inNow: *const AudioTimeStamp,
        inInputData: *const AudioBufferList,
        inInputTime: *const AudioTimeStamp,
        outOutputData: *mut AudioBufferList,
        inOutputTime: *const AudioTimeStamp,
        inClientData: *mut c_void,
    ) -> OSStatus,
>;
pub type AudioDeviceIOProcID = AudioDeviceIOProc;

s! {
    #[repr(packed)]
    struct AudioHardwareIOProcStreamUsage {
        pub mIOProc: *mut c_void,
        pub mNumberStreams: u32,
        pub mStreamIsOn: [u32;0],
    }
}

impl AudioHardwareIOProcStreamUsage {
    pub fn stream_is_on(&self) -> &[u32] {
        unsafe { slice::from_raw_parts(&self.mStreamIsOn as *const _, self.mNumberStreams as _) }
    }
}

//==============================================================================
// AudioDevice Constants

e! {
    enum u32 {
        kAudioDeviceStartTimeIsInputFlag                = (1 << 0),
        kAudioDeviceStartTimeDontConsultDeviceFlag      = (1 << 1),
        kAudioDeviceStartTimeDontConsultHALFlag         = (1 << 2),
    }
}

//==============================================================================
// AudioDevice Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioDevicePropertyPlugIn = 0x706c7567,
        kAudioDevicePropertyDeviceHasChanged = 0x64696666,
        kAudioDevicePropertyDeviceIsRunningSomewhere = 0x676f6e65,
        kAudioDeviceProcessorOverload = 0x6f766572,
        kAudioDevicePropertyIOStoppedAbnormally = 0x73747064,
        kAudioDevicePropertyHogMode = 0x6f696e6b,
        kAudioDevicePropertyBufferFrameSize = 0x6673697a,
        kAudioDevicePropertyBufferFrameSizeRange = 0x66737a23,
        kAudioDevicePropertyUsesVariableBufferFrameSizes = 0x7666737a,
        kAudioDevicePropertyIOCycleUsage = 0x6e637963,
        kAudioDevicePropertyStreamConfiguration = 0x736c6179,
        kAudioDevicePropertyIOProcStreamUsage = 0x73757365,
        kAudioDevicePropertyActualSampleRate = 0x61737274,
        kAudioDevicePropertyClockDevice = 0x61706364,
    }
}

e! {
    enum AudioObjectPropertySelector {
        kAudioDevicePropertyJackIsConnected = 0x6a61636b,
        kAudioDevicePropertyVolumeScalar = 0x766f6c6d,
        kAudioDevicePropertyVolumeDecibels = 0x766f6c64,
        kAudioDevicePropertyVolumeRangeDecibels = 0x76646223,
        kAudioDevicePropertyVolumeScalarToDecibels = 0x76326462,
        kAudioDevicePropertyVolumeDecibelsToScalar = 0x64623276,
        kAudioDevicePropertyStereoPan = 0x7370616e,
        kAudioDevicePropertyStereoPanChannels = 0x73706e23,
        kAudioDevicePropertyMute = 0x6d757465,
        kAudioDevicePropertySolo = 0x736f6c6f,
        kAudioDevicePropertyPhantomPower = 0x7068616e,
        kAudioDevicePropertyPhaseInvert = 0x70687369,
        kAudioDevicePropertyClipLight = 0x636c6970,
        kAudioDevicePropertyTalkback = 0x74616c62,
        kAudioDevicePropertyListenback = 0x6c736e62,
        kAudioDevicePropertyDataSource = 0x73737263,
        kAudioDevicePropertyDataSources = 0x73736323,
        kAudioDevicePropertyDataSourceNameForIDCFString = 0x6c73636e,
        kAudioDevicePropertyDataSourceKindForID = 0x7373636b,
        kAudioDevicePropertyClockSource = 0x63737263,
        kAudioDevicePropertyClockSources = 0x63736323,
        kAudioDevicePropertyClockSourceNameForIDCFString = 0x6c63736e,
        kAudioDevicePropertyClockSourceKindForID = 0x6373636b,
        kAudioDevicePropertyPlayThru = 0x74687275,
        kAudioDevicePropertyPlayThruSolo = 0x74687273,
        kAudioDevicePropertyPlayThruVolumeScalar = 0x6d767363,
        kAudioDevicePropertyPlayThruVolumeDecibels = 0x6d766462,
        kAudioDevicePropertyPlayThruVolumeRangeDecibels = 0x6d766423,
        kAudioDevicePropertyPlayThruVolumeScalarToDecibels = 0x6d763264,
        kAudioDevicePropertyPlayThruVolumeDecibelsToScalar = 0x6d763273,
        kAudioDevicePropertyPlayThruStereoPan = 0x6d73706e,
        kAudioDevicePropertyPlayThruStereoPanChannels = 0x6d737023,
        kAudioDevicePropertyPlayThruDestination = 0x6d646473,
        kAudioDevicePropertyPlayThruDestinations = 0x6d646423,
        kAudioDevicePropertyPlayThruDestinationNameForIDCFString = 0x6d646463,
        kAudioDevicePropertyChannelNominalLineLevel = 0x6e6c766c,
        kAudioDevicePropertyChannelNominalLineLevels = 0x6e6c7623,
        kAudioDevicePropertyChannelNominalLineLevelNameForIDCFString = 0x6c636e6c,
        kAudioDevicePropertyHighPassFilterSetting = 0x68697066,
        kAudioDevicePropertyHighPassFilterSettings = 0x68697023,
        kAudioDevicePropertyHighPassFilterSettingNameForIDCFString = 0x6869706c,
        kAudioDevicePropertySubVolumeScalar = 0x73766c6d,
        kAudioDevicePropertySubVolumeDecibels = 0x73766c64,
        kAudioDevicePropertySubVolumeRangeDecibels = 0x73766423,
        kAudioDevicePropertySubVolumeScalarToDecibels = 0x73763264,
        kAudioDevicePropertySubVolumeDecibelsToScalar = 0x73643276,
        kAudioDevicePropertySubMute = 0x736d7574,
    }
}

//==============================================================================
// AudioDevice Functions

extern "C" {
    pub fn AudioDeviceCreateIOProcID(
        inDevice: AudioObjectID,
        inProc: AudioDeviceIOProc,
        inClientData: *mut c_void,
        outIOProcID: *mut AudioDeviceIOProcID,
    ) -> OSStatus;
    pub fn AudioDeviceDestroyIOProcID(
        inDevice: AudioObjectID,
        inIOProcID: AudioDeviceIOProcID,
    ) -> OSStatus;
    pub fn AudioDeviceStart(inDevice: AudioObjectID, inProcID: AudioDeviceIOProcID) -> OSStatus;
    pub fn AudioDeviceStartAtTime(
        inDevice: AudioObjectID,
        inProcID: AudioDeviceIOProcID,
        ioRequestedStartTime: *mut AudioTimeStamp,
        inFlags: u32,
    ) -> OSStatus;
    pub fn AudioDeviceStop(inDevice: AudioObjectID, inProcID: AudioDeviceIOProcID) -> OSStatus;
    pub fn AudioDeviceGetCurrentTime(
        inDevice: AudioObjectID,
        outTime: *mut AudioTimeStamp,
    ) -> OSStatus;
    pub fn AudioDeviceTranslateTime(
        inDevice: AudioObjectID,
        inTime: *const AudioTimeStamp,
        outTime: *mut AudioTimeStamp,
    ) -> OSStatus;
    pub fn AudioDeviceGetNearestStartTime(
        inDevice: AudioObjectID,
        ioRequestedStartTime: *mut AudioTimeStamp,
        inFlags: u32,
    ) -> OSStatus;
}

//==============================================================================
// AudioAggregateDevice Constants

e! {
    enum AudioClassID {
        kAudioAggregateDeviceClassID = 0x61616767,
    }
}

cs! {
    kAudioAggregateDeviceUIDKey                         = "uid";
    kAudioAggregateDeviceNameKey                        = "name";
    kAudioAggregateDeviceSubDeviceListKey               = "subdevices";
    kAudioAggregateDeviceMasterSubDeviceKey             = "master";
    kAudioAggregateDeviceClockDeviceKey                 = "clock";
    kAudioAggregateDeviceIsPrivateKey                   = "private";
    kAudioAggregateDeviceIsStackedKey                   = "stacked";
}

//==============================================================================
// AudioAggregateDevice Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioAggregateDevicePropertyFullSubDeviceList = 0x67727570,
        kAudioAggregateDevicePropertyActiveSubDeviceList = 0x61677270,
        kAudioAggregateDevicePropertyComposition = 0x61636f6d,
        kAudioAggregateDevicePropertyMasterSubDevice = 0x616d7374,
        kAudioAggregateDevicePropertyClockDevice = 0x61706364,
    }
}

//==============================================================================
// AudioSubDevice Constants

e! {
    enum AudioClassID {
        kAudioSubDeviceClassID = 0x61737562,
    }
}

e! {
    enum u32 {
        kAudioSubDeviceDriftCompensationMinQuality      = 0,
        kAudioSubDeviceDriftCompensationLowQuality      = 0x20,
        kAudioSubDeviceDriftCompensationMediumQuality   = 0x40,
        kAudioSubDeviceDriftCompensationHighQuality     = 0x60,
        kAudioSubDeviceDriftCompensationMaxQuality      = 0x7F,
    }
}

cs! {
    kAudioSubDeviceUIDKey                               = "uid";
    kAudioSubDeviceNameKey                              = "name";
    kAudioSubDeviceInputChannelsKey                     = "channels-in";
    kAudioSubDeviceOutputChannelsKey                    = "channels-out";
    kAudioSubDeviceExtraInputLatencyKey                 = "latency-in";
    kAudioSubDeviceExtraOutputLatencyKey                = "latency-out";
    kAudioSubDeviceDriftCompensationKey                 = "drift";
    kAudioSubDeviceDriftCompensationQualityKey          = "drift quality";
}

//==============================================================================
// AudioSubDevice Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioSubDevicePropertyExtraLatency = 0x786c7463,
        kAudioSubDevicePropertyDriftCompensation = 0x64726674,
        kAudioSubDevicePropertyDriftCompensationQuality = 0x64726671,
    }
}
