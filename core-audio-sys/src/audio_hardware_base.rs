use super::*;

//==============================================================================
// Basic Types

pub type AudioObjectID = u32;
pub type AudioClassID = u32;
pub type AudioObjectPropertySelector = u32;
pub type AudioObjectPropertyScope = u32;
pub type AudioObjectPropertyElement = u32;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioObjectPropertyAddress {
    pub mSelector: AudioObjectPropertySelector,
    pub mScope: AudioObjectPropertyScope,
    pub mElement: AudioObjectPropertyElement,
}

//==============================================================================
// Basic Constants

pub const kAudioHardwareNoError: OSStatus = 0;
pub const kAudioHardwareNotRunningError: OSStatus = 1937010544;
pub const kAudioHardwareUnspecifiedError: OSStatus = 2003329396;
pub const kAudioHardwareUnknownPropertyError: OSStatus = 2003332927;
pub const kAudioHardwareBadPropertySizeError: OSStatus = 561211770;
pub const kAudioHardwareIllegalOperationError: OSStatus = 1852797029;
pub const kAudioHardwareBadObjectError: OSStatus = 560947818;
pub const kAudioHardwareBadDeviceError: OSStatus = 560227702;
pub const kAudioHardwareBadStreamError: OSStatus = 561214578;
pub const kAudioHardwareUnsupportedOperationError: OSStatus = 1970171760;
pub const kAudioDeviceUnsupportedFormatError: OSStatus = 560226676;
pub const kAudioDevicePermissionsError: OSStatus = 560492391;

pub const kAudioObjectUnknown: AudioObjectID = 0;

pub const kAudioObjectPropertyScopeGlobal: AudioObjectPropertyScope = 1735159650;
pub const kAudioObjectPropertyScopeInput: AudioObjectPropertyScope = 1768845428;
pub const kAudioObjectPropertyScopeOutput: AudioObjectPropertyScope = 1869968496;
pub const kAudioObjectPropertyScopePlayThrough: AudioObjectPropertyScope = 1886679669;
pub const kAudioObjectPropertyElementMaster: AudioObjectPropertyScope = 0;

pub const kAudioObjectPropertySelectorWildcard: AudioObjectPropertySelector = 707406378;
pub const kAudioObjectPropertyScopeWildcard: AudioObjectPropertyScope = 707406378;
pub const kAudioObjectPropertyElementWildcard: AudioObjectPropertyElement = 4294967295;
pub const kAudioObjectClassIDWildcard: AudioClassID = 707406378;

//==============================================================================
// AudioObject Constants

pub const kAudioObjectClassID: AudioClassID = 1634689642;

//==============================================================================
// AudioObject Properties
pub const kAudioObjectPropertyBaseClass: AudioObjectPropertySelector = 1650682995;
pub const kAudioObjectPropertyClass: AudioObjectPropertySelector = 1668047219;
pub const kAudioObjectPropertyOwner: AudioObjectPropertySelector = 1937007734;
pub const kAudioObjectPropertyName: AudioObjectPropertySelector = 1819173229;
pub const kAudioObjectPropertyModelName: AudioObjectPropertySelector = 1819111268;
pub const kAudioObjectPropertyManufacturer: AudioObjectPropertySelector = 1819107691;
pub const kAudioObjectPropertyElementName: AudioObjectPropertySelector = 1818454126;
pub const kAudioObjectPropertyElementCategoryName: AudioObjectPropertySelector = 1818452846;
pub const kAudioObjectPropertyElementNumberName: AudioObjectPropertySelector = 1818455662;
pub const kAudioObjectPropertyOwnedObjects: AudioObjectPropertySelector = 1870098020;
pub const kAudioObjectPropertyIdentify: AudioObjectPropertySelector = 1768187246;
pub const kAudioObjectPropertySerialNumber: AudioObjectPropertySelector = 1936618861;
pub const kAudioObjectPropertyFirmwareVersion: AudioObjectPropertySelector = 1719105134;

//==============================================================================
// AudioPlugIn Constants

pub const kAudioPlugInClassID: AudioClassID = 1634757735;

//==============================================================================
// AudioPlugIn Properties

pub const kAudioPlugInPropertyBundleID: AudioObjectPropertySelector = 1885956452;
pub const kAudioPlugInPropertyDeviceList: AudioObjectPropertySelector = 1684370979;
pub const kAudioPlugInPropertyTranslateUIDToDevice: AudioObjectPropertySelector = 1969841252;
pub const kAudioPlugInPropertyBoxList: AudioObjectPropertySelector = 1651472419;
pub const kAudioPlugInPropertyTranslateUIDToBox: AudioObjectPropertySelector = 1969841250;
pub const kAudioPlugInPropertyClockDeviceList: AudioObjectPropertySelector = 1668049699;
pub const kAudioPlugInPropertyTranslateUIDToClockDevice: AudioObjectPropertySelector = 1969841251;

//==============================================================================
// AudioTransportManager Constants
pub const kAudioTransportManagerClassID: AudioClassID = 1953656941;

//==============================================================================
// AudioTransportManager Properties

pub const kAudioTransportManagerPropertyEndPointList: AudioObjectPropertySelector = 1701733411;
pub const kAudioTransportManagerPropertyTranslateUIDToEndPoint: AudioObjectPropertySelector =
    1969841253;
pub const kAudioTransportManagerPropertyTransportType: AudioObjectPropertySelector = 1953653102;

//==============================================================================
// AudioBox Constants

pub const kAudioBoxClassID: AudioClassID = 1633841016;

//==============================================================================
// AudioBox Properties

pub const kAudioBoxPropertyBoxUID: AudioObjectPropertySelector = 1651861860;
pub const kAudioBoxPropertyTransportType: AudioObjectPropertySelector = 1953653102;
pub const kAudioBoxPropertyHasAudio: AudioObjectPropertySelector = 1651007861;
pub const kAudioBoxPropertyHasVideo: AudioObjectPropertySelector = 1651013225;
pub const kAudioBoxPropertyHasMIDI: AudioObjectPropertySelector = 1651010921;
pub const kAudioBoxPropertyIsProtected: AudioObjectPropertySelector = 1651536495;
pub const kAudioBoxPropertyAcquired: AudioObjectPropertySelector = 1652060014;
pub const kAudioBoxPropertyAcquisitionFailed: AudioObjectPropertySelector = 1652060006;
pub const kAudioBoxPropertyDeviceList: AudioObjectPropertySelector = 1650751011;
pub const kAudioBoxPropertyClockDeviceList: AudioObjectPropertySelector = 1650682915;

//==============================================================================
// AudioDevice Constants

pub const kAudioDeviceClassID: AudioClassID = 1633969526;

pub const kAudioDeviceTransportTypeUnknown: u32 = 0;
pub const kAudioDeviceTransportTypeBuiltIn: u32 = 1651274862;
pub const kAudioDeviceTransportTypeAggregate: u32 = 1735554416;
pub const kAudioDeviceTransportTypeVirtual: u32 = 1986622068;
pub const kAudioDeviceTransportTypePCI: u32 = 1885563168;
pub const kAudioDeviceTransportTypeUSB: u32 = 1970496032;
pub const kAudioDeviceTransportTypeFireWire: u32 = 825440564;
pub const kAudioDeviceTransportTypeBluetooth: u32 = 1651275109;
pub const kAudioDeviceTransportTypeBluetoothLE: u32 = 1651271009;
pub const kAudioDeviceTransportTypeHDMI: u32 = 1751412073;
pub const kAudioDeviceTransportTypeDisplayPort: u32 = 1685090932;
pub const kAudioDeviceTransportTypeAirPlay: u32 = 1634300528;
pub const kAudioDeviceTransportTypeAVB: u32 = 1700886114;
pub const kAudioDeviceTransportTypeThunderbolt: u32 = 1953002862;

//==============================================================================
// AudioDevice Properties

pub const kAudioDevicePropertyConfigurationApplication: AudioObjectPropertySelector = 1667330160;
pub const kAudioDevicePropertyDeviceUID: AudioObjectPropertySelector = 1969841184;
pub const kAudioDevicePropertyModelUID: AudioObjectPropertySelector = 1836411236;
pub const kAudioDevicePropertyTransportType: AudioObjectPropertySelector = 1953653102;
pub const kAudioDevicePropertyRelatedDevices: AudioObjectPropertySelector = 1634429294;
pub const kAudioDevicePropertyClockDomain: AudioObjectPropertySelector = 1668049764;
pub const kAudioDevicePropertyDeviceIsAlive: AudioObjectPropertySelector = 1818850926;
pub const kAudioDevicePropertyDeviceIsRunning: AudioObjectPropertySelector = 1735354734;
pub const kAudioDevicePropertyDeviceCanBeDefaultDevice: AudioObjectPropertySelector = 1684434036;
pub const kAudioDevicePropertyDeviceCanBeDefaultSystemDevice: AudioObjectPropertySelector =
    1936092276;
pub const kAudioDevicePropertyLatency: AudioObjectPropertySelector = 1819569763;
pub const kAudioDevicePropertyStreams: AudioObjectPropertySelector = 1937009955;
pub const kAudioObjectPropertyControlList: AudioObjectPropertySelector = 1668575852;
pub const kAudioDevicePropertySafetyOffset: AudioObjectPropertySelector = 1935763060;
pub const kAudioDevicePropertyNominalSampleRate: AudioObjectPropertySelector = 1853059700;
pub const kAudioDevicePropertyAvailableNominalSampleRates: AudioObjectPropertySelector = 1853059619;
pub const kAudioDevicePropertyIcon: AudioObjectPropertySelector = 1768124270;
pub const kAudioDevicePropertyIsHidden: AudioObjectPropertySelector = 1751737454;
pub const kAudioDevicePropertyPreferredChannelsForStereo: AudioObjectPropertySelector = 1684236338;
pub const kAudioDevicePropertyPreferredChannelLayout: AudioObjectPropertySelector = 1936879204;

//==============================================================================
// AudioClockDevice Constants

pub const kAudioClockDeviceClassID: AudioObjectPropertySelector = 1633905771;

//==============================================================================
// AudioClockDevice Properties

pub const kAudioClockDevicePropertyDeviceUID: AudioObjectPropertySelector = 1668639076;
pub const kAudioClockDevicePropertyTransportType: AudioObjectPropertySelector = 1953653102;
pub const kAudioClockDevicePropertyClockDomain: AudioObjectPropertySelector = 1668049764;
pub const kAudioClockDevicePropertyDeviceIsAlive: AudioObjectPropertySelector = 1818850926;
pub const kAudioClockDevicePropertyDeviceIsRunning: AudioObjectPropertySelector = 1735354734;
pub const kAudioClockDevicePropertyLatency: AudioObjectPropertySelector = 1819569763;
pub const kAudioClockDevicePropertyControlList: AudioObjectPropertySelector = 1668575852;
pub const kAudioClockDevicePropertyNominalSampleRate: AudioObjectPropertySelector = 1853059700;
pub const kAudioClockDevicePropertyAvailableNominalSampleRates: AudioObjectPropertySelector =
    1853059619;

//==============================================================================
// AudioEndPointDevice Constants

pub const kAudioEndPointDeviceClassID: AudioClassID = 1701078390;

pub const kAudioEndPointDeviceUIDKey: &'static str = "uid";
pub const kAudioEndPointDeviceNameKey: &'static str = "name";
pub const kAudioEndPointDeviceEndPointListKey: &'static str = "endpoints";
pub const kAudioEndPointDeviceMasterEndPointKey: &'static str = "master";
pub const kAudioEndPointDeviceIsPrivateKey: &'static str = "private";

//==============================================================================
// AudioEndPointDevice Properties

pub const kAudioEndPointDevicePropertyComposition: AudioObjectPropertySelector = 1633906541;
pub const kAudioEndPointDevicePropertyEndPointList: AudioObjectPropertySelector = 1634169456;
pub const kAudioEndPointDevicePropertyIsPrivate: AudioObjectPropertySelector = 1886546294;

//==============================================================================
// AudioEndPoint Constants

pub const kAudioEndPointClassID: AudioClassID = 1701733488;

pub const kAudioEndPointUIDKey: &'static str = "uid";
pub const kAudioEndPointNameKey: &'static str = "name";
pub const kAudioEndPointInputChannelsKey: &'static str = "channels-in";
pub const kAudioEndPointOutputChannelsKey: &'static str = "channels-out";

//==============================================================================
// AudioStream Types

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioStreamRangedDescription {
    pub mFormat: AudioStreamBasicDescription,
    pub mSampleRateRange: AudioValueRange,
}

//==============================================================================
// AudioStream Constants

pub const kAudioStreamClassID: AudioClassID = 1634956402;

pub const kAudioStreamTerminalTypeUnknown: u32 = 0;
pub const kAudioStreamTerminalTypeLine: u32 = 1818848869;
pub const kAudioStreamTerminalTypeDigitalAudioInterface: u32 = 1936745574;
pub const kAudioStreamTerminalTypeSpeaker: u32 = 1936747378;
pub const kAudioStreamTerminalTypeHeadphones: u32 = 1751412840;
pub const kAudioStreamTerminalTypeLFESpeaker: u32 = 1818649971;
pub const kAudioStreamTerminalTypeReceiverSpeaker: u32 = 1920168043;
pub const kAudioStreamTerminalTypeMicrophone: u32 = 1835623282;
pub const kAudioStreamTerminalTypeHeadsetMicrophone: u32 = 1752000867;
pub const kAudioStreamTerminalTypeReceiverMicrophone: u32 = 1919773027;
pub const kAudioStreamTerminalTypeTTY: u32 = 1953790303;
pub const kAudioStreamTerminalTypeHDMI: u32 = 1751412073;
pub const kAudioStreamTerminalTypeDisplayPort: u32 = 1685090932;

//==============================================================================
// AudioStream Properties

pub const kAudioStreamPropertyIsActive: AudioObjectPropertySelector = 1935762292;
pub const kAudioStreamPropertyDirection: AudioObjectPropertySelector = 1935960434;
pub const kAudioStreamPropertyTerminalType: AudioObjectPropertySelector = 1952805485;
pub const kAudioStreamPropertyStartingChannel: AudioObjectPropertySelector = 1935894638;
pub const kAudioStreamPropertyLatency: AudioObjectPropertySelector = kAudioDevicePropertyLatency;
pub const kAudioStreamPropertyVirtualFormat: AudioObjectPropertySelector = 1936092532;
pub const kAudioStreamPropertyAvailableVirtualFormats: AudioObjectPropertySelector = 1936092513;
pub const kAudioStreamPropertyPhysicalFormat: AudioObjectPropertySelector = 1885762592;
pub const kAudioStreamPropertyAvailablePhysicalFormats: AudioObjectPropertySelector = 1885762657;

//==============================================================================
// AudioControl Constants

pub const kAudioControlClassID: AudioClassID = 1633907820;

pub const kAudioControlPropertyScope: AudioObjectPropertySelector = 1668506480;
pub const kAudioControlPropertyElement: AudioObjectPropertySelector = 1667591277;

//==============================================================================
// AudioSliderControl Constants

pub const kAudioSliderControlClassID: AudioClassID = 1936483442;

pub const kAudioSliderControlPropertyValue: AudioObjectPropertySelector = 1935962742;
pub const kAudioSliderControlPropertyRange: AudioObjectPropertySelector = 1935962738;

//==============================================================================
// AudioLevelControl Constants

pub const kAudioLevelControlClassID: AudioClassID = 1818588780;
pub const kAudioVolumeControlClassID: AudioClassID = 1986817381;
pub const kAudioLFEVolumeControlClassID: AudioClassID = 1937072758;

pub const kAudioLevelControlPropertyScalarValue: AudioObjectPropertySelector = 1818456950;
pub const kAudioLevelControlPropertyDecibelValue: AudioObjectPropertySelector = 1818453110;
pub const kAudioLevelControlPropertyDecibelRange: AudioObjectPropertySelector = 1818453106;
pub const kAudioLevelControlPropertyConvertScalarToDecibels: AudioObjectPropertySelector =
    1818456932;
pub const kAudioLevelControlPropertyConvertDecibelsToScalar: AudioObjectPropertySelector =
    1818453107;

//==============================================================================
// AudioBooleanControl Constants

pub const kAudioBooleanControlClassID: AudioClassID = 1953458028;
pub const kAudioMuteControlClassID: AudioClassID = 1836414053;
pub const kAudioSoloControlClassID: AudioClassID = 1936682095;
pub const kAudioJackControlClassID: AudioClassID = 1784767339;
pub const kAudioLFEMuteControlClassID: AudioClassID = 1937072749;
pub const kAudioPhantomPowerControlClassID: AudioClassID = 1885888878;
pub const kAudioPhaseInvertControlClassID: AudioClassID = 1885893481;
pub const kAudioClipLightControlClassID: AudioClassID = 1668049264;
pub const kAudioTalkbackControlClassID: AudioClassID = 1952541794;
pub const kAudioListenbackControlClassID: AudioClassID = 1819504226;

pub const kAudioBooleanControlPropertyValue: AudioObjectPropertySelector = 1650685548;

//==============================================================================
// AudioSelectorControl Constants

pub const kAudioSelectorControlClassID: AudioClassID = 1936483188;
pub const kAudioDataSourceControlClassID: AudioClassID = 1685287523;
pub const kAudioDataDestinationControlClassID: AudioClassID = 1684370292;
pub const kAudioClockSourceControlClassID: AudioClassID = 1668047723;
pub const kAudioLineLevelControlClassID: AudioClassID = 1852601964;
pub const kAudioHighPassFilterControlClassID: AudioClassID = 1751740518;

pub const kAudioSelectorControlPropertyCurrentItem: AudioObjectPropertySelector = 1935893353;
pub const kAudioSelectorControlPropertyAvailableItems: AudioObjectPropertySelector = 1935892841;
pub const kAudioSelectorControlPropertyItemName: AudioObjectPropertySelector = 1935894894;
pub const kAudioSelectorControlPropertyItemKind: AudioObjectPropertySelector = 1668049771;

pub const kAudioSelectorControlItemKindSpacer: u32 = 1936745330;

pub const kAudioClockSourceItemKindInternal: u32 = 1768846368;

//==============================================================================
// AudioStereoPanControl Constants

pub const kAudioStereoPanControlClassID: AudioClassID = 1936744814;

pub const kAudioStereoPanControlPropertyValue: AudioObjectPropertySelector = 1936745334;
pub const kAudioStereoPanControlPropertyPanningChannels: AudioObjectPropertySelector = 1936745315;
