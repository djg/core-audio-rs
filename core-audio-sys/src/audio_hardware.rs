use super::*;
use std::os::raw::c_void;

//==============================================================================
// Basic Constants

pub const kAudioObjectSystemObject: AudioObjectID = 1;

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

pub const kAudioObjectPropertyCreator: AudioObjectPropertySelector = 1869638759;
pub const kAudioObjectPropertyListenerAdded: AudioObjectPropertySelector = 1818850145;
pub const kAudioObjectPropertyListenerRemoved: AudioObjectPropertySelector = 1818850162;

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

pub const kAudioSystemObjectClassID: AudioClassID = 1634957683;

pub type AudioHardwarePowerHint = u32;
pub const kAudioHardwarePowerHintNone: u32 = 0;
pub const kAudioHardwarePowerHintFavorSavingPower: u32 = 1;

//==============================================================================
// AudioSystemObject Properties

pub const kAudioHardwarePropertyDevices: AudioObjectPropertySelector = 1684370979;
pub const kAudioHardwarePropertyDefaultInputDevice: AudioObjectPropertySelector = 1682533920;
pub const kAudioHardwarePropertyDefaultOutputDevice: AudioObjectPropertySelector = 1682929012;
pub const kAudioHardwarePropertyDefaultSystemOutputDevice: AudioObjectPropertySelector = 1934587252;
pub const kAudioHardwarePropertyTranslateUIDToDevice: AudioObjectPropertySelector = 1969841252;
pub const kAudioHardwarePropertyMixStereoToMono: AudioObjectPropertySelector = 1937010031;
pub const kAudioHardwarePropertyPlugInList: AudioObjectPropertySelector = 1886152483;
pub const kAudioHardwarePropertyTranslateBundleIDToPlugIn: AudioObjectPropertySelector = 1651074160;
pub const kAudioHardwarePropertyTransportManagerList: AudioObjectPropertySelector = 1953326883;
pub const kAudioHardwarePropertyTranslateBundleIDToTransportManager: AudioObjectPropertySelector =
    1953325673;
pub const kAudioHardwarePropertyBoxList: AudioObjectPropertySelector = 1651472419;
pub const kAudioHardwarePropertyTranslateUIDToBox: AudioObjectPropertySelector = 1969841250;
pub const kAudioHardwarePropertyClockDeviceList: AudioObjectPropertySelector = 1668049699;
pub const kAudioHardwarePropertyTranslateUIDToClockDevice: AudioObjectPropertySelector = 1969841251;
pub const kAudioHardwarePropertyProcessIsMaster: AudioObjectPropertySelector = 1835103092;
pub const kAudioHardwarePropertyIsInitingOrExiting: AudioObjectPropertySelector = 1768845172;
pub const kAudioHardwarePropertyUserIDChanged: AudioObjectPropertySelector = 1702193508;
pub const kAudioHardwarePropertyProcessIsAudible: AudioObjectPropertySelector = 1886221684;
pub const kAudioHardwarePropertySleepingIsAllowed: AudioObjectPropertySelector = 1936483696;
pub const kAudioHardwarePropertyUnloadingIsAllowed: AudioObjectPropertySelector = 1970170980;
pub const kAudioHardwarePropertyHogModeIsAllowed: AudioObjectPropertySelector = 1752131442;
pub const kAudioHardwarePropertyUserSessionIsActiveOrHeadless: AudioObjectPropertySelector =
    1970496882;
pub const kAudioHardwarePropertyServiceRestarted: AudioObjectPropertySelector = 1936880500;
pub const kAudioHardwarePropertyPowerHint: AudioObjectPropertySelector = 1886353256;

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

pub const kAudioPlugInCreateAggregateDevice: AudioObjectPropertySelector = 1667327847;
pub const kAudioPlugInDestroyAggregateDevice: AudioObjectPropertySelector = 1684105063;

//==============================================================================
// AudioTransportManager Properties

pub const kAudioTransportManagerCreateEndPointDevice: AudioObjectPropertySelector = 1667523958;
pub const kAudioTransportManagerDestroyEndPointDevice: AudioObjectPropertySelector = 1684301174;

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

#[repr(C)]
#[derive(Debug)]
pub struct AudioHardwareIOProcStreamUsage {
    pub mIOProc: *mut c_void,
    pub mNumberStreams: u32,
    pub mStreamIsOn: [u32; 1],
}

//==============================================================================
// AudioDevice Constants

pub const kAudioDeviceStartTimeIsInputFlag: u32 = (1 << 0);
pub const kAudioDeviceStartTimeDontConsultDeviceFlag: u32 = (1 << 1);
pub const kAudioDeviceStartTimeDontConsultHALFlag: u32 = (1 << 2);

//==============================================================================
// AudioDevice Properties

pub const kAudioDevicePropertyPlugIn: AudioObjectPropertySelector = 1886156135;
pub const kAudioDevicePropertyDeviceHasChanged: AudioObjectPropertySelector = 1684629094;
pub const kAudioDevicePropertyDeviceIsRunningSomewhere: AudioObjectPropertySelector = 1735356005;
pub const kAudioDeviceProcessorOverload: AudioObjectPropertySelector = 1870030194;
pub const kAudioDevicePropertyIOStoppedAbnormally: AudioObjectPropertySelector = 1937010788;
pub const kAudioDevicePropertyHogMode: AudioObjectPropertySelector = 1869180523;
pub const kAudioDevicePropertyBufferFrameSize: AudioObjectPropertySelector = 1718839674;
pub const kAudioDevicePropertyBufferFrameSizeRange: AudioObjectPropertySelector = 1718843939;
pub const kAudioDevicePropertyUsesVariableBufferFrameSizes: AudioObjectPropertySelector =
    1986425722;
pub const kAudioDevicePropertyIOCycleUsage: AudioObjectPropertySelector = 1852012899;
pub const kAudioDevicePropertyStreamConfiguration: AudioObjectPropertySelector = 1936482681;
pub const kAudioDevicePropertyIOProcStreamUsage: AudioObjectPropertySelector = 1937077093;
pub const kAudioDevicePropertyActualSampleRate: AudioObjectPropertySelector = 1634955892;
pub const kAudioDevicePropertyClockDevice: AudioObjectPropertySelector = 1634755428;

pub const kAudioDevicePropertyJackIsConnected: AudioObjectPropertySelector = 1784767339;
pub const kAudioDevicePropertyVolumeScalar: AudioObjectPropertySelector = 1987013741;
pub const kAudioDevicePropertyVolumeDecibels: AudioObjectPropertySelector = 1987013732;
pub const kAudioDevicePropertyVolumeRangeDecibels: AudioObjectPropertySelector = 1986290211;
pub const kAudioDevicePropertyVolumeScalarToDecibels: AudioObjectPropertySelector = 1983013986;
pub const kAudioDevicePropertyVolumeDecibelsToScalar: AudioObjectPropertySelector = 1684157046;
pub const kAudioDevicePropertyStereoPan: AudioObjectPropertySelector = 1936744814;
pub const kAudioDevicePropertyStereoPanChannels: AudioObjectPropertySelector = 1936748067;
pub const kAudioDevicePropertyMute: AudioObjectPropertySelector = 1836414053;
pub const kAudioDevicePropertySolo: AudioObjectPropertySelector = 1936682095;
pub const kAudioDevicePropertyPhantomPower: AudioObjectPropertySelector = 1885888878;
pub const kAudioDevicePropertyPhaseInvert: AudioObjectPropertySelector = 1885893481;
pub const kAudioDevicePropertyClipLight: AudioObjectPropertySelector = 1668049264;
pub const kAudioDevicePropertyTalkback: AudioObjectPropertySelector = 1952541794;
pub const kAudioDevicePropertyListenback: AudioObjectPropertySelector = 1819504226;
pub const kAudioDevicePropertyDataSource: AudioObjectPropertySelector = 1936945763;
pub const kAudioDevicePropertyDataSources: AudioObjectPropertySelector = 1936941859;
pub const kAudioDevicePropertyDataSourceNameForIDCFString: AudioObjectPropertySelector = 1819501422;
pub const kAudioDevicePropertyDataSourceKindForID: AudioObjectPropertySelector = 1936941931;
pub const kAudioDevicePropertyClockSource: AudioObjectPropertySelector = 1668510307;
pub const kAudioDevicePropertyClockSources: AudioObjectPropertySelector = 1668506403;
pub const kAudioDevicePropertyClockSourceNameForIDCFString: AudioObjectPropertySelector =
    1818456942;
pub const kAudioDevicePropertyClockSourceKindForID: AudioObjectPropertySelector = 1668506475;
pub const kAudioDevicePropertyPlayThru: AudioObjectPropertySelector = 1953002101;
pub const kAudioDevicePropertyPlayThruSolo: AudioObjectPropertySelector = 1953002099;
pub const kAudioDevicePropertyPlayThruVolumeScalar: AudioObjectPropertySelector = 1836479331;
pub const kAudioDevicePropertyPlayThruVolumeDecibels: AudioObjectPropertySelector = 1836475490;
pub const kAudioDevicePropertyPlayThruVolumeRangeDecibels: AudioObjectPropertySelector = 1836475427;
pub const kAudioDevicePropertyPlayThruVolumeScalarToDecibels: AudioObjectPropertySelector =
    1836462692;
pub const kAudioDevicePropertyPlayThruVolumeDecibelsToScalar: AudioObjectPropertySelector =
    1836462707;
pub const kAudioDevicePropertyPlayThruStereoPan: AudioObjectPropertySelector = 1836281966;
pub const kAudioDevicePropertyPlayThruStereoPanChannels: AudioObjectPropertySelector = 1836281891;
pub const kAudioDevicePropertyPlayThruDestination: AudioObjectPropertySelector = 1835295859;
pub const kAudioDevicePropertyPlayThruDestinations: AudioObjectPropertySelector = 1835295779;
pub const kAudioDevicePropertyPlayThruDestinationNameForIDCFString: AudioObjectPropertySelector =
    1835295843;
pub const kAudioDevicePropertyChannelNominalLineLevel: AudioObjectPropertySelector = 1852601964;
pub const kAudioDevicePropertyChannelNominalLineLevels: AudioObjectPropertySelector = 1852601891;
pub const kAudioDevicePropertyChannelNominalLineLevelNameForIDCFString:
        AudioObjectPropertySelector = 1818455660;
pub const kAudioDevicePropertyHighPassFilterSetting: AudioObjectPropertySelector = 1751740518;
pub const kAudioDevicePropertyHighPassFilterSettings: AudioObjectPropertySelector = 1751740451;
pub const kAudioDevicePropertyHighPassFilterSettingNameForIDCFString: AudioObjectPropertySelector =
    1751740524;
pub const kAudioDevicePropertySubVolumeScalar: AudioObjectPropertySelector = 1937140845;
pub const kAudioDevicePropertySubVolumeDecibels: AudioObjectPropertySelector = 1937140836;
pub const kAudioDevicePropertySubVolumeRangeDecibels: AudioObjectPropertySelector = 1937138723;
pub const kAudioDevicePropertySubVolumeScalarToDecibels: AudioObjectPropertySelector = 1937125988;
pub const kAudioDevicePropertySubVolumeDecibelsToScalar: AudioObjectPropertySelector = 1935946358;
pub const kAudioDevicePropertySubMute: AudioObjectPropertySelector = 1936553332;

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

pub const kAudioAggregateDeviceClassID: AudioClassID = 1633773415;

pub const kAudioAggregateDeviceUIDKey: &'static str = "uid";
pub const kAudioAggregateDeviceNameKey: &'static str = "name";
pub const kAudioAggregateDeviceSubDeviceListKey: &'static str = "subdevices";
pub const kAudioAggregateDeviceMasterSubDeviceKey: &'static str = "master";
pub const kAudioAggregateDeviceClockDeviceKey: &'static str = "clock";
pub const kAudioAggregateDeviceIsPrivateKey: &'static str = "private";
pub const kAudioAggregateDeviceIsStackedKey: &'static str = "stacked";

//==============================================================================
// AudioAggregateDevice Properties

pub const kAudioAggregateDevicePropertyFullSubDeviceList: AudioObjectPropertySelector = 1735554416;
pub const kAudioAggregateDevicePropertyActiveSubDeviceList: AudioObjectPropertySelector =
    1634169456;
pub const kAudioAggregateDevicePropertyComposition: AudioObjectPropertySelector = 1633906541;
pub const kAudioAggregateDevicePropertyMasterSubDevice: AudioObjectPropertySelector = 1634562932;
pub const kAudioAggregateDevicePropertyClockDevice: AudioObjectPropertySelector = 1634755428;

//==============================================================================
// AudioSubDevice Constants

pub const kAudioSubDeviceClassID: AudioClassID = 1634956642;

pub const kAudioSubDeviceDriftCompensationMinQuality: u32 = 0;
pub const kAudioSubDeviceDriftCompensationLowQuality: u32 = 32;
pub const kAudioSubDeviceDriftCompensationMediumQuality: u32 = 64;
pub const kAudioSubDeviceDriftCompensationHighQuality: u32 = 96;
pub const kAudioSubDeviceDriftCompensationMaxQuality: u32 = 127;

pub const kAudioSubDeviceUIDKey: &'static str = "uid";
pub const kAudioSubDeviceNameKey: &'static str = "name";
pub const kAudioSubDeviceInputChannelsKey: &'static str = "channels-in";
pub const kAudioSubDeviceOutputChannelsKey: &'static str = "channels-out";
pub const kAudioSubDeviceExtraInputLatencyKey: &'static str = "latency-in";
pub const kAudioSubDeviceExtraOutputLatencyKey: &'static str = "latency-out";
pub const kAudioSubDeviceDriftCompensationKey: &'static str = "drift";
pub const kAudioSubDeviceDriftCompensationQualityKey: &'static str = "drift quality";

//==============================================================================
// AudioSubDevice Properties

pub const kAudioSubDevicePropertyExtraLatency: AudioObjectPropertySelector = 2020373603;
pub const kAudioSubDevicePropertyDriftCompensation: AudioObjectPropertySelector = 1685218932;
pub const kAudioSubDevicePropertyDriftCompensationQuality: AudioObjectPropertySelector = 1685218929;
