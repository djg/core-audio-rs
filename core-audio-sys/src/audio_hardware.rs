use super::*;

use std::default::Default;
use std::mem;
use std::os::raw::c_void;
use std::slice;

//==================================================================================================
// Basic Constants

e! {
    enum AudioObjectID {
        kAudioObjectSystemObject                        = 1,
    }
}

//==================================================================================================
// AudioObject Types

pub type AudioObjectPropertyListenerProc =
    Option<unsafe extern fn(inObjectID: AudioObjectID,
                            inNumberAddresses: u32,
                            inAddresses: *const AudioObjectPropertyAddress,
                            inClientData: *mut c_void) -> OSStatus>;

//==================================================================================================
// AudioObject Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioObjectPropertyCreator                     = fourcc!(b"oplg"),
        kAudioObjectPropertyListenerAdded               = fourcc!(b"lisa"),
        kAudioObjectPropertyListenerRemoved             = fourcc!(b"lisr"),
    }
}

//==================================================================================================
// AudioObject Functions

extern {
    pub fn AudioObjectShow(inObjectID: AudioObjectID);
    pub fn AudioObjectHasProperty(inObjectID: AudioObjectID,
                                  inAddress: *const AudioObjectPropertyAddress) -> Boolean;
    pub fn AudioObjectIsPropertySettable(inObjectID: AudioObjectID,
                                         inAddress: *const AudioObjectPropertyAddress,
                                         outIsSettable: *mut Boolean) -> OSStatus;
    pub fn AudioObjectGetPropertyDataSize(inObjectID: AudioObjectID,
                                          inAddress: *const  AudioObjectPropertyAddress,
                                          inQualifierDataSize: u32,
                                          inQualifierData: *const c_void,
                                          outDataSize: *mut u32) -> OSStatus;
    pub fn AudioObjectGetPropertyData(inObjectID: AudioObjectID,
                                      inAddress: *const AudioObjectPropertyAddress,
                                      inQualifierDataSize: u32,
                                      inQualifierData: *const c_void,
                                      ioDataSize: *mut u32,
                                      outData: *mut c_void) -> OSStatus;
    pub fn AudioObjectSetPropertyData(inObjectID: AudioObjectID,
                                      inAddress: *const AudioObjectPropertyAddress,
                                      inQualifierDataSize: u32,
                                      inQualifierData: *const c_void,
                                      inDataSize: u32,
                                      inData: *const c_void) -> OSStatus;
    pub fn AudioObjectAddPropertyListener(inObjectID: AudioObjectID,
                                          inAddress: *const AudioObjectPropertyAddress,
                                          inListener: AudioObjectPropertyListenerProc,
                                          inClientData: *mut c_void) -> OSStatus;
    pub fn AudioObjectRemovePropertyListener(inObjectID: AudioObjectID,
                                             inAddress: *const AudioObjectPropertyAddress,
                                             inListener:     AudioObjectPropertyListenerProc,
                                             inClientData: *mut c_void) -> OSStatus;
}

//==================================================================================================
// AudioSystemObject Constants

e! {
    enum AudioClassID {
        kAudioSystemObjectClassID                       = fourcc!(b"asys"),
    }
}

e! {
    enum AudioHardwarePowerHint: u32 {
        kAudioHardwarePowerHintNone                     = 0,
        kAudioHardwarePowerHintFavorSavingPower         = 1,
    }
}

//==================================================================================================
// AudioSystemObject Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioHardwarePropertyDevices                               = fourcc!(b"dev#"),
        kAudioHardwarePropertyDefaultInputDevice                    = fourcc!(b"dIn "),
        kAudioHardwarePropertyDefaultOutputDevice                   = fourcc!(b"dOut"),
        kAudioHardwarePropertyDefaultSystemOutputDevice             = fourcc!(b"sOut"),
        kAudioHardwarePropertyTranslateUIDToDevice                  = fourcc!(b"uidd"),
        kAudioHardwarePropertyMixStereoToMono                       = fourcc!(b"stmo"),
        kAudioHardwarePropertyPlugInList                            = fourcc!(b"plg#"),
        kAudioHardwarePropertyTranslateBundleIDToPlugIn             = fourcc!(b"bidp"),
        kAudioHardwarePropertyTransportManagerList                  = fourcc!(b"tmg#"),
        kAudioHardwarePropertyTranslateBundleIDToTransportManager   = fourcc!(b"tmbi"),
        kAudioHardwarePropertyBoxList                               = fourcc!(b"box#"),
        kAudioHardwarePropertyTranslateUIDToBox                     = fourcc!(b"uidb"),
        kAudioHardwarePropertyClockDeviceList                       = fourcc!(b"clk#"),
        kAudioHardwarePropertyTranslateUIDToClockDevice             = fourcc!(b"uidc"),
        kAudioHardwarePropertyProcessIsMaster                       = fourcc!(b"mast"),
        kAudioHardwarePropertyIsInitingOrExiting                    = fourcc!(b"inot"),
        kAudioHardwarePropertyUserIDChanged                         = fourcc!(b"euid"),
        kAudioHardwarePropertyProcessIsAudible                      = fourcc!(b"pmut"),
        kAudioHardwarePropertySleepingIsAllowed                     = fourcc!(b"slep"),
        kAudioHardwarePropertyUnloadingIsAllowed                    = fourcc!(b"unld"),
        kAudioHardwarePropertyHogModeIsAllowed                      = fourcc!(b"hogr"),
        kAudioHardwarePropertyUserSessionIsActiveOrHeadless         = fourcc!(b"user"),
        kAudioHardwarePropertyServiceRestarted                      = fourcc!(b"srst"),
        kAudioHardwarePropertyPowerHint                             = fourcc!(b"powh"),
    }
}

//==================================================================================================
// AudioSystemObject Functions

extern {
    pub fn AudioHardwareUnload() -> OSStatus;
    pub fn AudioHardwareCreateAggregateDevice(inDescription: CFDictionaryRef,
                                              outDeviceID: *mut AudioObjectID) -> OSStatus;
    pub fn AudioHardwareDestroyAggregateDevice(inDeviceID: AudioObjectID)  -> OSStatus;
}

//==================================================================================================
// AudioPlugIn Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioPlugInCreateAggregateDevice               = fourcc!(b"cagg"),
        kAudioPlugInDestroyAggregateDevice              = fourcc!(b"dagg"),
    }
}

//==================================================================================================
// AudioTransportManager Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioTransportManagerCreateEndPointDevice      = fourcc!(b"cdev"),
        kAudioTransportManagerDestroyEndPointDevice     = fourcc!(b"ddev"),
    }
}

//==================================================================================================
// AudioDevice Types

pub type AudioDeviceIOProc =
    Option<unsafe extern fn(inDevice: AudioObjectID,
                            inNow: *const AudioTimeStamp,
                            inInputData: *const AudioBufferList,
                            inInputTime: *const AudioTimeStamp,
                            outOutputData: *mut AudioBufferList,
                            inOutputTime: *const AudioTimeStamp,
                            inClientData: *mut c_void) -> OSStatus>;
pub type AudioDeviceIOProcID = AudioDeviceIOProc;

s! {
    struct AudioHardwareIOProcStreamUsage {
        pub mIOProc: *mut c_void,
        pub mNumberStreams: u32,
            mStreamIsOn: [u32;0],
    }
}

impl AudioHardwareIOProcStreamUsage {
    pub fn stream_is_on(&self) -> &[u32] {
        unsafe {
            slice::from_raw_parts(&self.mStreamIsOn as *const _,
                                  self.mNumberStreams as _)
        }
    }
}

//==================================================================================================
// AudioDevice Constants

e! {
    enum u32 {
        kAudioDeviceStartTimeIsInputFlag                = (1 << 0),
        kAudioDeviceStartTimeDontConsultDeviceFlag      = (1 << 1),
        kAudioDeviceStartTimeDontConsultHALFlag         = (1 << 2),
    }
}

//==================================================================================================
// AudioDevice Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioDevicePropertyPlugIn                          = fourcc!(b"plug"),
        kAudioDevicePropertyDeviceHasChanged                = fourcc!(b"diff"),
        kAudioDevicePropertyDeviceIsRunningSomewhere        = fourcc!(b"gone"),
        kAudioDeviceProcessorOverload                       = fourcc!(b"over"),
        kAudioDevicePropertyIOStoppedAbnormally             = fourcc!(b"stpd"),
        kAudioDevicePropertyHogMode                         = fourcc!(b"oink"),
        kAudioDevicePropertyBufferFrameSize                 = fourcc!(b"fsiz"),
        kAudioDevicePropertyBufferFrameSizeRange            = fourcc!(b"fsz#"),
        kAudioDevicePropertyUsesVariableBufferFrameSizes    = fourcc!(b"vfsz"),
        kAudioDevicePropertyIOCycleUsage                    = fourcc!(b"ncyc"),
        kAudioDevicePropertyStreamConfiguration             = fourcc!(b"slay"),
        kAudioDevicePropertyIOProcStreamUsage               = fourcc!(b"suse"),
        kAudioDevicePropertyActualSampleRate                = fourcc!(b"asrt"),
        kAudioDevicePropertyClockDevice                     = fourcc!(b"apcd"),
    }
}

e! {
    enum AudioObjectPropertySelector {
        kAudioDevicePropertyJackIsConnected                                 = fourcc!(b"jack"),
        kAudioDevicePropertyVolumeScalar                                    = fourcc!(b"volm"),
        kAudioDevicePropertyVolumeDecibels                                  = fourcc!(b"vold"),
        kAudioDevicePropertyVolumeRangeDecibels                             = fourcc!(b"vdb#"),
        kAudioDevicePropertyVolumeScalarToDecibels                          = fourcc!(b"v2db"),
        kAudioDevicePropertyVolumeDecibelsToScalar                          = fourcc!(b"db2v"),
        kAudioDevicePropertyStereoPan                                       = fourcc!(b"span"),
        kAudioDevicePropertyStereoPanChannels                               = fourcc!(b"spn#"),
        kAudioDevicePropertyMute                                            = fourcc!(b"mute"),
        kAudioDevicePropertySolo                                            = fourcc!(b"solo"),
        kAudioDevicePropertyPhantomPower                                    = fourcc!(b"phan"),
        kAudioDevicePropertyPhaseInvert                                     = fourcc!(b"phsi"),
        kAudioDevicePropertyClipLight                                       = fourcc!(b"clip"),
        kAudioDevicePropertyTalkback                                        = fourcc!(b"talb"),
        kAudioDevicePropertyListenback                                      = fourcc!(b"lsnb"),
        kAudioDevicePropertyDataSource                                      = fourcc!(b"ssrc"),
        kAudioDevicePropertyDataSources                                     = fourcc!(b"ssc#"),
        kAudioDevicePropertyDataSourceNameForIDCFString                     = fourcc!(b"lscn"),
        kAudioDevicePropertyDataSourceKindForID                             = fourcc!(b"ssck"),
        kAudioDevicePropertyClockSource                                     = fourcc!(b"csrc"),
        kAudioDevicePropertyClockSources                                    = fourcc!(b"csc#"),
        kAudioDevicePropertyClockSourceNameForIDCFString                    = fourcc!(b"lcsn"),
        kAudioDevicePropertyClockSourceKindForID                            = fourcc!(b"csck"),
        kAudioDevicePropertyPlayThru                                        = fourcc!(b"thru"),
        kAudioDevicePropertyPlayThruSolo                                    = fourcc!(b"thrs"),
        kAudioDevicePropertyPlayThruVolumeScalar                            = fourcc!(b"mvsc"),
        kAudioDevicePropertyPlayThruVolumeDecibels                          = fourcc!(b"mvdb"),
        kAudioDevicePropertyPlayThruVolumeRangeDecibels                     = fourcc!(b"mvd#"),
        kAudioDevicePropertyPlayThruVolumeScalarToDecibels                  = fourcc!(b"mv2d"),
        kAudioDevicePropertyPlayThruVolumeDecibelsToScalar                  = fourcc!(b"mv2s"),
        kAudioDevicePropertyPlayThruStereoPan                               = fourcc!(b"mspn"),
        kAudioDevicePropertyPlayThruStereoPanChannels                       = fourcc!(b"msp#"),
        kAudioDevicePropertyPlayThruDestination                             = fourcc!(b"mdds"),
        kAudioDevicePropertyPlayThruDestinations                            = fourcc!(b"mdd#"),
        kAudioDevicePropertyPlayThruDestinationNameForIDCFString            = fourcc!(b"mddc"),
        kAudioDevicePropertyChannelNominalLineLevel                         = fourcc!(b"nlvl"),
        kAudioDevicePropertyChannelNominalLineLevels                        = fourcc!(b"nlv#"),
        kAudioDevicePropertyChannelNominalLineLevelNameForIDCFString        = fourcc!(b"lcnl"),
        kAudioDevicePropertyHighPassFilterSetting                           = fourcc!(b"hipf"),
        kAudioDevicePropertyHighPassFilterSettings                          = fourcc!(b"hip#"),
        kAudioDevicePropertyHighPassFilterSettingNameForIDCFString          = fourcc!(b"hipl"),
        kAudioDevicePropertySubVolumeScalar                                 = fourcc!(b"svlm"),
        kAudioDevicePropertySubVolumeDecibels                               = fourcc!(b"svld"),
        kAudioDevicePropertySubVolumeRangeDecibels                          = fourcc!(b"svd#"),
        kAudioDevicePropertySubVolumeScalarToDecibels                       = fourcc!(b"sv2d"),
        kAudioDevicePropertySubVolumeDecibelsToScalar                       = fourcc!(b"sd2v"),
        kAudioDevicePropertySubMute                                         = fourcc!(b"smut"),
    }
}

//==================================================================================================
// AudioDevice Functions

extern {
    pub fn AudioDeviceCreateIOProcID(inDevice: AudioObjectID,
                                     inProc: AudioDeviceIOProc,
                                     inClientData: *mut c_void,
                                     outIOProcID: *mut AudioDeviceIOProcID) -> OSStatus;
    pub fn AudioDeviceDestroyIOProcID(inDevice: AudioObjectID,
                                      inIOProcID: AudioDeviceIOProcID) -> OSStatus;
    pub fn AudioDeviceStart(inDevice: AudioObjectID,
                            inProcID: AudioDeviceIOProcID) -> OSStatus;
    pub fn AudioDeviceStartAtTime(inDevice: AudioObjectID,
                                  inProcID: AudioDeviceIOProcID,
                                  ioRequestedStartTime: *mut AudioTimeStamp,
                                  inFlags: u32) -> OSStatus;
    pub fn AudioDeviceStop(inDevice: AudioObjectID,
                           inProcID: AudioDeviceIOProcID) -> OSStatus;
    pub fn AudioDeviceGetCurrentTime(inDevice: AudioObjectID,
                                     outTime: *mut AudioTimeStamp) -> OSStatus;
    pub fn AudioDeviceTranslateTime(inDevice: AudioObjectID,
                                    inTime: *const AudioTimeStamp,
                                    outTime: *mut AudioTimeStamp) -> OSStatus;
    pub fn AudioDeviceGetNearestStartTime(inDevice:AudioObjectID,
                                          ioRequestedStartTime: *mut AudioTimeStamp,
                                          inFlags: u32) -> OSStatus;
}

//==================================================================================================
// AudioAggregateDevice Constants

e! {
    enum AudioClassID {
        kAudioAggregateDeviceClassID                    = fourcc!(b"aagg"),
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

//==================================================================================================
// AudioAggregateDevice Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioAggregateDevicePropertyFullSubDeviceList  = fourcc!(b"grup"),
        kAudioAggregateDevicePropertyActiveSubDeviceList = fourcc!(b"agrp"),
        kAudioAggregateDevicePropertyComposition        = fourcc!(b"acom"),
        kAudioAggregateDevicePropertyMasterSubDevice    = fourcc!(b"amst"),
        kAudioAggregateDevicePropertyClockDevice        = fourcc!(b"apcd"),
    }
}

//==================================================================================================
// AudioSubDevice Constants

e! {
    enum AudioClassID {
        kAudioSubDeviceClassID                          = fourcc!(b"asub"),
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

//==================================================================================================
// AudioSubDevice Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioSubDevicePropertyExtraLatency             = fourcc!(b"xltc"),
        kAudioSubDevicePropertyDriftCompensation        = fourcc!(b"drft"),
        kAudioSubDevicePropertyDriftCompensationQuality = fourcc!(b"drfq"),
    }
}
