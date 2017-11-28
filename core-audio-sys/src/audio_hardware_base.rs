use super::*;

use std::default::Default;
use std::mem;

//==================================================================================================
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

//==================================================================================================
// Basic Constants

e! {
    enum OSStatus {
        kAudioHardwareNoError                   = 0,
        kAudioHardwareNotRunningError           = 1937010544, // 'stop
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
        kAudioObjectPropertyScopeGlobal         = fourcc!(b"glob"),
        kAudioObjectPropertyScopeInput          = fourcc!(b"inpt"),
        kAudioObjectPropertyScopeOutput         = fourcc!(b"outp"),
        kAudioObjectPropertyScopePlayThrough    = fourcc!(b"ptru"),
        kAudioObjectPropertyElementMaster       = 0,
    }
}

e! {
    enum AudioObjectPropertySelector {
        kAudioObjectPropertySelectorWildcard    = fourcc!(b"****"),
    }
}

e! {
    enum AudioObjectPropertyScope {
        kAudioObjectPropertyScopeWildcard       = fourcc!(b"****"),
    }
}

e! {
    enum AudioObjectPropertyElement {
        kAudioObjectPropertyElementWildcard     = 0xFFFFFFFF,
    }
}

e! {
    enum AudioClassID {
        kAudioObjectClassIDWildcard             = fourcc!(b"****"),
    }
}

//==================================================================================================
// AudioObject Constants

e! {
    enum AudioClassID {
        kAudioObjectClassID = fourcc!(b"aobj"),
    }
}

//==================================================================================================
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
    
//==================================================================================================
// AudioPlugIn Constants

e! {
    enum AudioClassID {
        kAudioPlugInClassID = fourcc!(b"aplg"),
    }
}

//==================================================================================================
// AudioPlugIn Properties

e! {
    enum AudioObjectPropertySelector {
        kAudioPlugInPropertyBundleID                  = fourcc!(b"piid"),
        kAudioPlugInPropertyDeviceList                = fourcc!(b"dev#"),
        kAudioPlugInPropertyTranslateUIDToDevice      = fourcc!(b"uidd"),
        kAudioPlugInPropertyBoxList                   = fourcc!(b"box#"),
        kAudioPlugInPropertyTranslateUIDToBox         = fourcc!(b"uidb"),
        kAudioPlugInPropertyClockDeviceList           = fourcc!(b"clk#"),
        kAudioPlugInPropertyTranslateUIDToClockDevice = fourcc!(b"uidc"),
    }
}

//==================================================================================================
// AudioTransportManager Constants
e! {
    enum AudioClassID  {
        kAudioTransportManagerClassID   = fourcc!(b"trpm"),
    }
}

//==================================================================================================
// AudioTransportManager Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioTransportManagerPropertyEndPointList              = fourcc!(b"end#"),
        kAudioTransportManagerPropertyTranslateUIDToEndPoint    = fourcc!(b"uide"),
        kAudioTransportManagerPropertyTransportType             = fourcc!(b"tran"),
    }
}

//==================================================================================================
// AudioBox Constants

e! {
    enum AudioClassID
    {
        kAudioBoxClassID    = fourcc!(b"abox"),
    }
}

//==================================================================================================
// AudioBox Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioBoxPropertyBoxUID             = fourcc!(b"buid"),
        kAudioBoxPropertyTransportType      = fourcc!(b"tran"),
        kAudioBoxPropertyHasAudio           = fourcc!(b"bhau"),
        kAudioBoxPropertyHasVideo           = fourcc!(b"bhvi"),
        kAudioBoxPropertyHasMIDI            = fourcc!(b"bhmi"),
        kAudioBoxPropertyIsProtected        = fourcc!(b"bpro"),
        kAudioBoxPropertyAcquired           = fourcc!(b"bxon"),
        kAudioBoxPropertyAcquisitionFailed  = fourcc!(b"bxof"),
        kAudioBoxPropertyDeviceList         = fourcc!(b"bdv#"),
        kAudioBoxPropertyClockDeviceList    = fourcc!(b"bcl#"),
    }
}

//==================================================================================================
// AudioDevice Constants

e! {
    enum AudioClassID
    {
        kAudioDeviceClassID = fourcc!(b"adev"),
    }
}

e! {
    enum u32
    {
        kAudioDeviceTransportTypeUnknown        = 0,
        kAudioDeviceTransportTypeBuiltIn        = fourcc!(b"bltn"),
        kAudioDeviceTransportTypeAggregate      = fourcc!(b"grup"),
        kAudioDeviceTransportTypeVirtual        = fourcc!(b"virt"),
        kAudioDeviceTransportTypePCI            = fourcc!(b"pci "),
        kAudioDeviceTransportTypeUSB            = fourcc!(b"usb "),
        kAudioDeviceTransportTypeFireWire       = fourcc!(b"1394"),
        kAudioDeviceTransportTypeBluetooth      = fourcc!(b"blue"),
        kAudioDeviceTransportTypeBluetoothLE    = fourcc!(b"blea"),
        kAudioDeviceTransportTypeHDMI           = fourcc!(b"hdmi"),
        kAudioDeviceTransportTypeDisplayPort    = fourcc!(b"dprt"),
        kAudioDeviceTransportTypeAirPlay        = fourcc!(b"airp"),
        kAudioDeviceTransportTypeAVB            = fourcc!(b"eavb"),
        kAudioDeviceTransportTypeThunderbolt    = fourcc!(b"thun"),
    }
}

//==================================================================================================
// AudioDevice Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioDevicePropertyConfigurationApplication        = fourcc!(b"capp"),
        kAudioDevicePropertyDeviceUID                       = fourcc!(b"uid "),
        kAudioDevicePropertyModelUID                        = fourcc!(b"muid"),
        kAudioDevicePropertyTransportType                   = fourcc!(b"tran"),
        kAudioDevicePropertyRelatedDevices                  = fourcc!(b"akin"),
        kAudioDevicePropertyClockDomain                     = fourcc!(b"clkd"),
        kAudioDevicePropertyDeviceIsAlive                   = fourcc!(b"livn"),
        kAudioDevicePropertyDeviceIsRunning                 = fourcc!(b"goin"),
        kAudioDevicePropertyDeviceCanBeDefaultDevice        = fourcc!(b"dflt"),
        kAudioDevicePropertyDeviceCanBeDefaultSystemDevice  = fourcc!(b"sflt"),
        kAudioDevicePropertyLatency                         = fourcc!(b"ltnc"),
        kAudioDevicePropertyStreams                         = fourcc!(b"stm#"),
        kAudioObjectPropertyControlList                     = fourcc!(b"ctrl"),
        kAudioDevicePropertySafetyOffset                    = fourcc!(b"saft"),
        kAudioDevicePropertyNominalSampleRate               = fourcc!(b"nsrt"),
        kAudioDevicePropertyAvailableNominalSampleRates     = fourcc!(b"nsr#"),
        kAudioDevicePropertyIcon                            = fourcc!(b"icon"),
        kAudioDevicePropertyIsHidden                        = fourcc!(b"hidn"),
        kAudioDevicePropertyPreferredChannelsForStereo      = fourcc!(b"dch2"),
        kAudioDevicePropertyPreferredChannelLayout          = fourcc!(b"srnd"),
    }
}

//==================================================================================================
// AudioClockDevice Constants

e! {
    enum AudioObjectPropertySelector
    {
        kAudioClockDeviceClassID    = fourcc!(b"aclk"),
    }
}
    
//==================================================================================================
// AudioClockDevice Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioClockDevicePropertyDeviceUID                   = fourcc!(b"cuid"),
        kAudioClockDevicePropertyTransportType               = fourcc!(b"tran"),
        kAudioClockDevicePropertyClockDomain                 = fourcc!(b"clkd"),
        kAudioClockDevicePropertyDeviceIsAlive               = fourcc!(b"livn"),
        kAudioClockDevicePropertyDeviceIsRunning             = fourcc!(b"goin"),
        kAudioClockDevicePropertyLatency                     = fourcc!(b"ltnc"),
        kAudioClockDevicePropertyControlList                 = fourcc!(b"ctrl"),
        kAudioClockDevicePropertyNominalSampleRate           = fourcc!(b"nsrt"),
        kAudioClockDevicePropertyAvailableNominalSampleRates = fourcc!(b"nsr#"),
    }
}

//==================================================================================================
// AudioEndPointDevice Constants

e! {
    enum AudioClassID
    {
        kAudioEndPointDeviceClassID = fourcc!(b"edev"),
    }
}

cs! {
    kAudioEndPointDeviceUIDKey            = "uid";
    kAudioEndPointDeviceNameKey           = "name";
    kAudioEndPointDeviceEndPointListKey   = "endpoints";
    kAudioEndPointDeviceMasterEndPointKey = "master";
    kAudioEndPointDeviceIsPrivateKey      = "private";
}

//==================================================================================================
// AudioEndPointDevice Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioEndPointDevicePropertyComposition         = fourcc!(b"acom"),
        kAudioEndPointDevicePropertyEndPointList        = fourcc!(b"agrp"),
        kAudioEndPointDevicePropertyIsPrivate           = fourcc!(b"priv"),
    }
}

//==================================================================================================
// AudioEndPoint Constants

e! {
    enum AudioClassID
    {
        kAudioEndPointClassID   = fourcc!(b"endp"),
    }
}

cs! {
    kAudioEndPointUIDKey            = "uid";
    kAudioEndPointNameKey           = "name";
    kAudioEndPointInputChannelsKey  = "channels-in";
    kAudioEndPointOutputChannelsKey = "channels-out";
}

//==================================================================================================
// AudioStream Types

s! {
    #[derive(Clone, Copy)]
    struct AudioStreamRangedDescription
    {
        pub mFormat: AudioStreamBasicDescription,
        pub mSampleRateRange: AudioValueRange,
    }
}

//==================================================================================================
// AudioStream Constants

e! {
    enum AudioClassID
    {
        kAudioStreamClassID = fourcc!(b"astr"),
    }
}

e! {
    enum u32
    {
        kAudioStreamTerminalTypeUnknown                 = 0,
        kAudioStreamTerminalTypeLine                    = fourcc!(b"line"),
        kAudioStreamTerminalTypeDigitalAudioInterface   = fourcc!(b"spdf"),
        kAudioStreamTerminalTypeSpeaker                 = fourcc!(b"spkr"),
        kAudioStreamTerminalTypeHeadphones              = fourcc!(b"hdph"),
        kAudioStreamTerminalTypeLFESpeaker              = fourcc!(b"lfes"),
        kAudioStreamTerminalTypeReceiverSpeaker         = fourcc!(b"rspk"),
        kAudioStreamTerminalTypeMicrophone              = fourcc!(b"micr"),
        kAudioStreamTerminalTypeHeadsetMicrophone       = fourcc!(b"hmic"),
        kAudioStreamTerminalTypeReceiverMicrophone      = fourcc!(b"rmic"),
        kAudioStreamTerminalTypeTTY                     = fourcc!(b"tty_"),
        kAudioStreamTerminalTypeHDMI                    = fourcc!(b"hdmi"),
        kAudioStreamTerminalTypeDisplayPort             = fourcc!(b"dprt"),
    }
}

//==================================================================================================
// AudioStream Properties

e! {
    enum AudioObjectPropertySelector
    {
        kAudioStreamPropertyIsActive                    = fourcc!(b"sact"),
        kAudioStreamPropertyDirection                   = fourcc!(b"sdir"),
        kAudioStreamPropertyTerminalType                = fourcc!(b"term"),
        kAudioStreamPropertyStartingChannel             = fourcc!(b"schn"),
        kAudioStreamPropertyLatency                     = kAudioDevicePropertyLatency,
        kAudioStreamPropertyVirtualFormat               = fourcc!(b"sfmt"),
        kAudioStreamPropertyAvailableVirtualFormats     = fourcc!(b"sfma"),
        kAudioStreamPropertyPhysicalFormat              = fourcc!(b"pft "),
        kAudioStreamPropertyAvailablePhysicalFormats    = fourcc!(b"pfta"),
    }
}

//==================================================================================================
// AudioControl Constants

e! {
    enum AudioClassID
    {
    kAudioControlClassID    = fourcc!(b"actl"),
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioControlPropertyScope      = fourcc!(b"cscp"),
        kAudioControlPropertyElement    = fourcc!(b"celm"),
    }
}

//==================================================================================================
// AudioSliderControl Constants

e! {
    enum AudioClassID
    {
        kAudioSliderControlClassID  = fourcc!(b"sldr"),
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioSliderControlPropertyValue    = fourcc!(b"sdrv"),
        kAudioSliderControlPropertyRange    = fourcc!(b"sdrr"),
    }
}

//==================================================================================================
// AudioLevelControl Constants

e! {
    enum AudioClassID
    {
        kAudioLevelControlClassID       = fourcc!(b"levl"),
        kAudioVolumeControlClassID      = fourcc!(b"vlme"),
        kAudioLFEVolumeControlClassID   = fourcc!(b"subv"),
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioLevelControlPropertyScalarValue               = fourcc!(b"lcsv"),
        kAudioLevelControlPropertyDecibelValue              = fourcc!(b"lcdv"),
        kAudioLevelControlPropertyDecibelRange              = fourcc!(b"lcdr"),
        kAudioLevelControlPropertyConvertScalarToDecibels   = fourcc!(b"lcsd"),
        kAudioLevelControlPropertyConvertDecibelsToScalar   = fourcc!(b"lcds"),
    }
}

//==================================================================================================
// AudioBooleanControl Constants

e! {
    enum AudioClassID
    {
        kAudioBooleanControlClassID         = fourcc!(b"togl"),
        kAudioMuteControlClassID            = fourcc!(b"mute"),
        kAudioSoloControlClassID            = fourcc!(b"solo"),
        kAudioJackControlClassID            = fourcc!(b"jack"),
        kAudioLFEMuteControlClassID         = fourcc!(b"subm"),
        kAudioPhantomPowerControlClassID    = fourcc!(b"phan"),
        kAudioPhaseInvertControlClassID     = fourcc!(b"phsi"),
        kAudioClipLightControlClassID       = fourcc!(b"clip"),
        kAudioTalkbackControlClassID        = fourcc!(b"talb"),
        kAudioListenbackControlClassID      = fourcc!(b"lsnb"),
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioBooleanControlPropertyValue   = fourcc!(b"bcvl"),
    }
}

//==================================================================================================
// AudioSelectorControl Constants

e! {
    enum AudioClassID
    {
        kAudioSelectorControlClassID        = fourcc!(b"slct"),
        kAudioDataSourceControlClassID      = fourcc!(b"dsrc"),
        kAudioDataDestinationControlClassID = fourcc!(b"dest"),
        kAudioClockSourceControlClassID     = fourcc!(b"clck"),
        kAudioLineLevelControlClassID       = fourcc!(b"nlvl"),
        kAudioHighPassFilterControlClassID  = fourcc!(b"hipf"),
    }
}

e! {
    enum AudioObjectPropertySelector
    {
        kAudioSelectorControlPropertyCurrentItem    = fourcc!(b"scci"),
        kAudioSelectorControlPropertyAvailableItems = fourcc!(b"scai"),
        kAudioSelectorControlPropertyItemName       = fourcc!(b"scin"),
        kAudioSelectorControlPropertyItemKind       = fourcc!(b"clkk"),
    }
}

e! {
    enum u32
    {
        kAudioSelectorControlItemKindSpacer = fourcc!(b"spcr"),
    }
}

e! {
    enum u32
    {
        kAudioClockSourceItemKindInternal   = fourcc!(b"int "),
    }
}

//==================================================================================================
// AudioStereoPanControl Constants
e! {
    enum AudioClassID
    {
        kAudioStereoPanControlClassID   = fourcc!(b"span"),
    }
}

e! { 
    enum AudioObjectPropertySelector
    {
        kAudioStereoPanControlPropertyValue             = fourcc!(b"spcv"),
        kAudioStereoPanControlPropertyPanningChannels   = fourcc!(b"spcc"),
    }
}

