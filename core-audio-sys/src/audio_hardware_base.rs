use super::*;
use std::default::Default;
use std::mem;

//==============================================================================
// Basic Types

pub type AudioObjectID = u32;
pub type AudioClassID = u32;
pub type AudioObjectPropertySelector = u32;
pub type AudioObjectPropertyScope = u32;
pub type AudioObjectPropertyElement = u32;

s! {
    #[derive(Clone, Copy)]
    struct AudioObjectPropertyAddress {
        pub mSelector: AudioObjectPropertySelector,
        pub mScope:    AudioObjectPropertyScope,
        pub mElement:  AudioObjectPropertyElement,
    }
}

//==============================================================================
// Basic Constants

e! {
    enum OSStatus {
        kAudioHardwareNoError                   = 0,
        kAudioHardwareNotRunningError           = 1937010544, // 'stop'
        kAudioHardwareUnspecifiedError          = 2003329396, // 'what'
        kAudioHardwareUnknownPropertyError      = 2003332927, // 'who?'
        kAudioHardwareBadPropertySizeError      = 561211770,  // '!siz'
        kAudioHardwareIllegalOperationError     = 1852797029, // 'nope'
        kAudioHardwareBadObjectError            = 560947818,  // '!obj'
        kAudioHardwareBadDeviceError            = 560227702,  // '!dev'
        kAudioHardwareBadStreamError            = 561214578,  // '!str'
        kAudioHardwareUnsupportedOperationError = 1970171760, // 'unop'
        kAudioDeviceUnsupportedFormatError      = 560226676,  // '!dat'
        kAudioDevicePermissionsError            = 560492391,  // '!hog'
    }
}

e! {
    enum AudioObjectID {
        kAudioObjectUnknown         = 0,
    }
}

e! {
    enum AudioObjectPropertyScope {
        kAudioObjectPropertyScopeGlobal = 0x676c6f62,
        kAudioObjectPropertyScopeInput = 0x696e7074,
        kAudioObjectPropertyScopeOutput = 0x6f757470,
        kAudioObjectPropertyScopePlayThrough = 0x70747275,
        kAudioObjectPropertyElementMaster       = 0,
    }
}

e! {
    enum AudioObjectPropertySelector {
        kAudioObjectPropertySelectorWildcard = 0x2a2a2a2a,
    }
}

e! {
    enum AudioObjectPropertyScope {
        kAudioObjectPropertyScopeWildcard = 0x2a2a2a2a,
    }
}

e! {
    enum AudioObjectPropertyElement {
        kAudioObjectPropertyElementWildcard     = 0xFFFFFFFF,
    }
}

e! {
    enum AudioClassID {
        kAudioObjectClassIDWildcard = 0x2a2a2a2a,
    }
}

//==============================================================================
// AudioObject Constants

e! {
    enum AudioClassID {
        kAudioObjectClassID = 0x616f626a,
    }
}

//==============================================================================
// AudioObject Properties
e! {
    enum AudioObjectPropertySelector {
        kAudioObjectPropertyBaseClass           = 0x62636c73, // "bcls"
        kAudioObjectPropertyClass               = 0x636c6173, // "clas"
        kAudioObjectPropertyOwner               = 0x73746476, // "stdv"
        kAudioObjectPropertyName                = 0x6c6e616d, // "lnam"
        kAudioObjectPropertyModelName           = 0x6c6d6f64, // "lmod"
        kAudioObjectPropertyManufacturer        = 0x6c6d616b, // "lmak"
        kAudioObjectPropertyElementName         = 0x6c63686e, // "lchn"
        kAudioObjectPropertyElementCategoryName = 0x6c63636e, // "lccn"
        kAudioObjectPropertyElementNumberName   = 0x6c636e6e, // "lcnn"
        kAudioObjectPropertyOwnedObjects        = 0x6f776e64, // "ownd"
        kAudioObjectPropertyIdentify            = 0x6964656e, // "iden"
        kAudioObjectPropertySerialNumber        = 0x736e756d, // "snum"
        kAudioObjectPropertyFirmwareVersion     = 0x6677766e, // "fwvn"
    }
}

//==============================================================================
// AudioPlugIn Constants

e! {
    enum AudioClassID {
        kAudioPlugInClassID = 0x61706c67,
    }
}

//==============================================================================
// AudioPlugIn Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioPlugInPropertyBundleID = 0x70696964,
        kAudioPlugInPropertyDeviceList = 0x64657623,
        kAudioPlugInPropertyTranslateUIDToDevice = 0x75696464,
        kAudioPlugInPropertyBoxList = 0x626f7823,
        kAudioPlugInPropertyTranslateUIDToBox = 0x75696462,
        kAudioPlugInPropertyClockDeviceList = 0x636c6b23,
        kAudioPlugInPropertyTranslateUIDToClockDevice = 0x75696463,
    }
}

//==============================================================================
// AudioTransportManager Constants
e! {
    enum AudioClassID  {
        kAudioTransportManagerClassID = 0x7472706d,
    }
}

//==============================================================================
// AudioTransportManager Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioTransportManagerPropertyEndPointList = 0x656e6423,
        kAudioTransportManagerPropertyTranslateUIDToEndPoint = 0x75696465,
        kAudioTransportManagerPropertyTransportType = 0x7472616e,
    }
}

//==============================================================================
// AudioBox Constants

e! {
    enum AudioClassID
    {
        kAudioBoxClassID = 0x61626f78,
    }
}

//==============================================================================
// AudioBox Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioBoxPropertyBoxUID = 0x62756964,
        kAudioBoxPropertyTransportType = 0x7472616e,
        kAudioBoxPropertyHasAudio = 0x62686175,
        kAudioBoxPropertyHasVideo = 0x62687669,
        kAudioBoxPropertyHasMIDI = 0x62686d69,
        kAudioBoxPropertyIsProtected = 0x6270726f,
        kAudioBoxPropertyAcquired = 0x62786f6e,
        kAudioBoxPropertyAcquisitionFailed = 0x62786f66,
        kAudioBoxPropertyDeviceList = 0x62647623,
        kAudioBoxPropertyClockDeviceList = 0x62636c23,
    }
}

//==============================================================================
// AudioDevice Constants

e! {
    enum AudioClassID
    {
        kAudioDeviceClassID = 0x61646576,
    }
}

e! {
    enum u32
    {
        kAudioDeviceTransportTypeUnknown = 0,
        kAudioDeviceTransportTypeBuiltIn = 0x626c746e,
        kAudioDeviceTransportTypeAggregate = 0x67727570,
        kAudioDeviceTransportTypeVirtual = 0x76697274,
        kAudioDeviceTransportTypePCI = 0x70636920,
        kAudioDeviceTransportTypeUSB = 0x75736220,
        kAudioDeviceTransportTypeFireWire = 0x31333934,
        kAudioDeviceTransportTypeBluetooth = 0x626c7565,
        kAudioDeviceTransportTypeBluetoothLE = 0x626c6561,
        kAudioDeviceTransportTypeHDMI = 0x68646d69,
        kAudioDeviceTransportTypeDisplayPort = 0x64707274,
        kAudioDeviceTransportTypeAirPlay = 0x61697270,
        kAudioDeviceTransportTypeAVB = 0x65617662,
        kAudioDeviceTransportTypeThunderbolt = 0x7468756e,
    }
}

//==============================================================================
// AudioDevice Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioDevicePropertyConfigurationApplication = 0x63617070,
        kAudioDevicePropertyDeviceUID = 0x75696420,
        kAudioDevicePropertyModelUID = 0x6d756964,
        kAudioDevicePropertyTransportType = 0x7472616e,
        kAudioDevicePropertyRelatedDevices = 0x616b696e,
        kAudioDevicePropertyClockDomain = 0x636c6b64,
        kAudioDevicePropertyDeviceIsAlive = 0x6c69766e,
        kAudioDevicePropertyDeviceIsRunning = 0x676f696e,
        kAudioDevicePropertyDeviceCanBeDefaultDevice = 0x64666c74,
        kAudioDevicePropertyDeviceCanBeDefaultSystemDevice = 0x73666c74,
        kAudioDevicePropertyLatency = 0x6c746e63,
        kAudioDevicePropertyStreams = 0x73746d23,
        kAudioObjectPropertyControlList = 0x6374726c,
        kAudioDevicePropertySafetyOffset = 0x73616674,
        kAudioDevicePropertyNominalSampleRate = 0x6e737274,
        kAudioDevicePropertyAvailableNominalSampleRates = 0x6e737223,
        kAudioDevicePropertyIcon = 0x69636f6e,
        kAudioDevicePropertyIsHidden = 0x6869646e,
        kAudioDevicePropertyPreferredChannelsForStereo = 0x64636832,
        kAudioDevicePropertyPreferredChannelLayout = 0x73726e64,
    }
}

//==============================================================================
// AudioClockDevice Constants

e! {
    enum AudioObjectPropertySelector
    {
        kAudioClockDeviceClassID = 0x61636c6b,
    }
}

//==============================================================================
// AudioClockDevice Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioClockDevicePropertyDeviceUID = 0x63756964,
        kAudioClockDevicePropertyTransportType = 0x7472616e,
        kAudioClockDevicePropertyClockDomain = 0x636c6b64,
        kAudioClockDevicePropertyDeviceIsAlive = 0x6c69766e,
        kAudioClockDevicePropertyDeviceIsRunning = 0x676f696e,
        kAudioClockDevicePropertyLatency = 0x6c746e63,
        kAudioClockDevicePropertyControlList = 0x6374726c,
        kAudioClockDevicePropertyNominalSampleRate = 0x6e737274,
        kAudioClockDevicePropertyAvailableNominalSampleRates = 0x6e737223,
    }
}

//==============================================================================
// AudioEndPointDevice Constants

e! {
    enum AudioClassID
    {
        kAudioEndPointDeviceClassID = 0x65646576,
    }
}

cs! {
    kAudioEndPointDeviceUIDKey            = "uid";
    kAudioEndPointDeviceNameKey           = "name";
    kAudioEndPointDeviceEndPointListKey   = "endpoints";
    kAudioEndPointDeviceMasterEndPointKey = "master";
    kAudioEndPointDeviceIsPrivateKey      = "private";
}

//==============================================================================
// AudioEndPointDevice Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioEndPointDevicePropertyComposition = 0x61636f6d,
        kAudioEndPointDevicePropertyEndPointList = 0x61677270,
        kAudioEndPointDevicePropertyIsPrivate = 0x70726976,
    }
}

//==============================================================================
// AudioEndPoint Constants

e! {
    enum AudioClassID
    {
        kAudioEndPointClassID = 0x656e6470,
    }
}

cs! {
    kAudioEndPointUIDKey            = "uid";
    kAudioEndPointNameKey           = "name";
    kAudioEndPointInputChannelsKey  = "channels-in";
    kAudioEndPointOutputChannelsKey = "channels-out";
}

//==============================================================================
// AudioStream Types

s! {
    #[derive(Clone, Copy)]
    struct AudioStreamRangedDescription
    {
        pub mFormat: AudioStreamBasicDescription,
        pub mSampleRateRange: AudioValueRange,
    }
}

//==============================================================================
// AudioStream Constants

e! {
    enum AudioClassID
    {
        kAudioStreamClassID = 0x61737472,
    }
}

e! {
    enum u32
    {
        kAudioStreamTerminalTypeUnknown                 = 0,
        kAudioStreamTerminalTypeLine = 0x6c696e65,
        kAudioStreamTerminalTypeDigitalAudioInterface = 0x73706466,
        kAudioStreamTerminalTypeSpeaker = 0x73706b72,
        kAudioStreamTerminalTypeHeadphones = 0x68647068,
        kAudioStreamTerminalTypeLFESpeaker = 0x6c666573,
        kAudioStreamTerminalTypeReceiverSpeaker = 0x7273706b,
        kAudioStreamTerminalTypeMicrophone = 0x6d696372,
        kAudioStreamTerminalTypeHeadsetMicrophone = 0x686d6963,
        kAudioStreamTerminalTypeReceiverMicrophone = 0x726d6963,
        kAudioStreamTerminalTypeTTY = 0x7474795f,
        kAudioStreamTerminalTypeHDMI = 0x68646d69,
        kAudioStreamTerminalTypeDisplayPort = 0x64707274,
    }
}

//==============================================================================
// AudioStream Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioStreamPropertyIsActive = 0x73616374,
        kAudioStreamPropertyDirection = 0x73646972,
        kAudioStreamPropertyTerminalType = 0x7465726d,
        kAudioStreamPropertyStartingChannel = 0x7363686e,
        kAudioStreamPropertyLatency                     = kAudioDevicePropertyLatency,
        kAudioStreamPropertyVirtualFormat = 0x73666d74,
        kAudioStreamPropertyAvailableVirtualFormats = 0x73666d61,
        kAudioStreamPropertyPhysicalFormat = 0x70667420,
        kAudioStreamPropertyAvailablePhysicalFormats = 0x70667461,
    }
}

//==============================================================================
// AudioControl Constants

e! {
    enum AudioClassID
    {
        kAudioControlClassID = 0x6163746c,
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioControlPropertyScope = 0x63736370,
        kAudioControlPropertyElement = 0x63656c6d,
    }
}

//==============================================================================
// AudioSliderControl Constants

e! {
    enum AudioClassID
    {
        kAudioSliderControlClassID = 0x736c6472,
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioSliderControlPropertyValue = 0x73647276,
        kAudioSliderControlPropertyRange = 0x73647272,
    }
}

//==============================================================================
// AudioLevelControl Constants

e! {
    enum AudioClassID
    {
        kAudioLevelControlClassID = 0x6c65766c,
        kAudioVolumeControlClassID = 0x766c6d65,
        kAudioLFEVolumeControlClassID = 0x73756276,
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioLevelControlPropertyScalarValue = 0x6c637376,
        kAudioLevelControlPropertyDecibelValue = 0x6c636476,
        kAudioLevelControlPropertyDecibelRange = 0x6c636472,
        kAudioLevelControlPropertyConvertScalarToDecibels = 0x6c637364,
        kAudioLevelControlPropertyConvertDecibelsToScalar = 0x6c636473,
    }
}

//==============================================================================
// AudioBooleanControl Constants

e! {
    enum AudioClassID
    {
        kAudioBooleanControlClassID = 0x746f676c,
        kAudioMuteControlClassID = 0x6d757465,
        kAudioSoloControlClassID = 0x736f6c6f,
        kAudioJackControlClassID = 0x6a61636b,
        kAudioLFEMuteControlClassID = 0x7375626d,
        kAudioPhantomPowerControlClassID = 0x7068616e,
        kAudioPhaseInvertControlClassID = 0x70687369,
        kAudioClipLightControlClassID = 0x636c6970,
        kAudioTalkbackControlClassID = 0x74616c62,
        kAudioListenbackControlClassID = 0x6c736e62,
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioBooleanControlPropertyValue = 0x6263766c,
    }
}

//==============================================================================
// AudioSelectorControl Constants

e! {
    enum AudioClassID
    {
        kAudioSelectorControlClassID = 0x736c6374,
        kAudioDataSourceControlClassID = 0x64737263,
        kAudioDataDestinationControlClassID = 0x64657374,
        kAudioClockSourceControlClassID = 0x636c636b,
        kAudioLineLevelControlClassID = 0x6e6c766c,
        kAudioHighPassFilterControlClassID = 0x68697066,
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioSelectorControlPropertyCurrentItem = 0x73636369,
        kAudioSelectorControlPropertyAvailableItems = 0x73636169,
        kAudioSelectorControlPropertyItemName = 0x7363696e,
        kAudioSelectorControlPropertyItemKind = 0x636c6b6b,
    }
}

e! {
    enum u32
    {
        kAudioSelectorControlItemKindSpacer = 0x73706372,
    }
}

e! {
    enum u32
    {
        kAudioClockSourceItemKindInternal = 0x696e7420,
    }
}

//==============================================================================
// AudioStereoPanControl Constants
e! {
    enum AudioClassID
    {
        kAudioStereoPanControlClassID = 0x7370616e,
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioStereoPanControlPropertyValue = 0x73706376,
        kAudioStereoPanControlPropertyPanningChannels = 0x73706363,
    }
}
