extern crate core_audio;

use core_audio::{audio_object_iter, audio_system_object, ffi, AudioObject, Result};
use std::error::Error;
use std::fmt::Debug;

fn print<T: Debug>(name: &str, t: Result<T>) {
    match t {
        Ok(t) => println!("{} = {:?}", name, t),
        Err(e) => println!("{} not found: {:?}", name, e.description()),
    }
}

static SELECTORS: &'static [ffi::AudioObjectPropertySelector] = &[
    ffi::kAudioObjectPropertyClass,
    ffi::kAudioObjectPropertyBaseClass,
    ffi::kAudioObjectPropertyOwner,
    ffi::kAudioObjectPropertyName,
    ffi::kAudioObjectPropertyModelName,
    ffi::kAudioObjectPropertyManufacturer,
    ffi::kAudioObjectPropertyElementName,
    ffi::kAudioObjectPropertyElementCategoryName,
    ffi::kAudioObjectPropertyElementNumberName,
    ffi::kAudioObjectPropertyOwnedObjects,
    ffi::kAudioObjectPropertyIdentify,
    ffi::kAudioObjectPropertySerialNumber,
    ffi::kAudioObjectPropertyFirmwareVersion,
    //
    ffi::kAudioHardwarePropertyDevices,
    ffi::kAudioHardwarePropertyDefaultInputDevice,
    ffi::kAudioHardwarePropertyDefaultOutputDevice,
    ffi::kAudioHardwarePropertyDefaultSystemOutputDevice,
    ffi::kAudioHardwarePropertyTranslateUIDToDevice,
    ffi::kAudioHardwarePropertyMixStereoToMono,
    ffi::kAudioHardwarePropertyPlugInList,
    ffi::kAudioHardwarePropertyTranslateBundleIDToPlugIn,
    ffi::kAudioHardwarePropertyTransportManagerList,
    ffi::kAudioHardwarePropertyTranslateBundleIDToTransportManager,
    ffi::kAudioHardwarePropertyBoxList,
    ffi::kAudioHardwarePropertyTranslateUIDToBox,
    ffi::kAudioHardwarePropertyClockDeviceList,
    ffi::kAudioHardwarePropertyTranslateUIDToClockDevice,
    ffi::kAudioHardwarePropertyProcessIsMaster,
    ffi::kAudioHardwarePropertyIsInitingOrExiting,
    ffi::kAudioHardwarePropertyUserIDChanged,
    ffi::kAudioHardwarePropertyProcessIsAudible,
    ffi::kAudioHardwarePropertySleepingIsAllowed,
    ffi::kAudioHardwarePropertyUnloadingIsAllowed,
    ffi::kAudioHardwarePropertyHogModeIsAllowed,
    ffi::kAudioHardwarePropertyUserSessionIsActiveOrHeadless,
    ffi::kAudioHardwarePropertyServiceRestarted,
    ffi::kAudioHardwarePropertyPowerHint,
    //
    ffi::kAudioDevicePropertyConfigurationApplication,
    ffi::kAudioDevicePropertyDeviceUID,
    ffi::kAudioDevicePropertyModelUID,
    ffi::kAudioDevicePropertyTransportType,
    ffi::kAudioDevicePropertyRelatedDevices,
    ffi::kAudioDevicePropertyClockDomain,
    ffi::kAudioDevicePropertyDeviceIsAlive,
    ffi::kAudioDevicePropertyDeviceIsRunning,
    ffi::kAudioDevicePropertyDeviceCanBeDefaultDevice,
    ffi::kAudioDevicePropertyDeviceCanBeDefaultSystemDevice,
    ffi::kAudioDevicePropertyLatency,
    ffi::kAudioDevicePropertyStreams,
    ffi::kAudioObjectPropertyControlList,
    ffi::kAudioDevicePropertySafetyOffset,
    ffi::kAudioDevicePropertyNominalSampleRate,
    ffi::kAudioDevicePropertyAvailableNominalSampleRates,
    ffi::kAudioDevicePropertyIcon,
    ffi::kAudioDevicePropertyIsHidden,
    ffi::kAudioDevicePropertyPreferredChannelsForStereo,
    ffi::kAudioDevicePropertyPreferredChannelLayout,
    //
    ffi::kAudioDevicePropertyPlugIn,
    ffi::kAudioDevicePropertyDeviceHasChanged,
    ffi::kAudioDevicePropertyDeviceIsRunningSomewhere,
    ffi::kAudioDeviceProcessorOverload,
    ffi::kAudioDevicePropertyIOStoppedAbnormally,
    ffi::kAudioDevicePropertyHogMode,
    ffi::kAudioDevicePropertyBufferFrameSize,
    ffi::kAudioDevicePropertyBufferFrameSizeRange,
    ffi::kAudioDevicePropertyUsesVariableBufferFrameSizes,
    ffi::kAudioDevicePropertyIOCycleUsage,
    ffi::kAudioDevicePropertyStreamConfiguration,
    ffi::kAudioDevicePropertyIOProcStreamUsage,
    ffi::kAudioDevicePropertyActualSampleRate,
    ffi::kAudioDevicePropertyClockDevice,
    ffi::kAudioDevicePropertyJackIsConnected,
    ffi::kAudioDevicePropertyVolumeScalar,
    ffi::kAudioDevicePropertyVolumeDecibels,
    ffi::kAudioDevicePropertyVolumeRangeDecibels,
    ffi::kAudioDevicePropertyVolumeScalarToDecibels,
    ffi::kAudioDevicePropertyVolumeDecibelsToScalar,
    ffi::kAudioDevicePropertyStereoPan,
    ffi::kAudioDevicePropertyStereoPanChannels,
    ffi::kAudioDevicePropertyMute,
    ffi::kAudioDevicePropertySolo,
    ffi::kAudioDevicePropertyPhantomPower,
    ffi::kAudioDevicePropertyPhaseInvert,
    ffi::kAudioDevicePropertyClipLight,
    ffi::kAudioDevicePropertyTalkback,
    ffi::kAudioDevicePropertyListenback,
    ffi::kAudioDevicePropertyDataSource,
    ffi::kAudioDevicePropertyDataSources,
    ffi::kAudioDevicePropertyDataSourceNameForIDCFString,
    ffi::kAudioDevicePropertyDataSourceKindForID,
    ffi::kAudioDevicePropertyClockSource,
    ffi::kAudioDevicePropertyClockSources,
    ffi::kAudioDevicePropertyClockSourceNameForIDCFString,
    ffi::kAudioDevicePropertyClockSourceKindForID,
    ffi::kAudioDevicePropertyPlayThru,
    ffi::kAudioDevicePropertyPlayThruSolo,
    ffi::kAudioDevicePropertyPlayThruVolumeScalar,
    ffi::kAudioDevicePropertyPlayThruVolumeDecibels,
    ffi::kAudioDevicePropertyPlayThruVolumeRangeDecibels,
    ffi::kAudioDevicePropertyPlayThruVolumeScalarToDecibels,
    ffi::kAudioDevicePropertyPlayThruVolumeDecibelsToScalar,
    ffi::kAudioDevicePropertyPlayThruStereoPan,
    ffi::kAudioDevicePropertyPlayThruStereoPanChannels,
    ffi::kAudioDevicePropertyPlayThruDestination,
    ffi::kAudioDevicePropertyPlayThruDestinations,
    ffi::kAudioDevicePropertyPlayThruDestinationNameForIDCFString,
    ffi::kAudioDevicePropertyChannelNominalLineLevel,
    ffi::kAudioDevicePropertyChannelNominalLineLevels,
    ffi::kAudioDevicePropertyChannelNominalLineLevelNameForIDCFString,
    ffi::kAudioDevicePropertyHighPassFilterSetting,
    ffi::kAudioDevicePropertyHighPassFilterSettings,
    ffi::kAudioDevicePropertyHighPassFilterSettingNameForIDCFString,
    ffi::kAudioDevicePropertySubVolumeScalar,
    ffi::kAudioDevicePropertySubVolumeDecibels,
    ffi::kAudioDevicePropertySubVolumeRangeDecibels,
    ffi::kAudioDevicePropertySubVolumeScalarToDecibels,
    ffi::kAudioDevicePropertySubVolumeDecibelsToScalar,
    ffi::kAudioDevicePropertySubMute,
];

fn introspect(audio_object: &AudioObject, scope: ffi::AudioObjectPropertyScope) {
    let element = ffi::kAudioObjectPropertyElementMaster;

    for sel in SELECTORS {
        let sel = *sel;

        let addr = ffi::AudioObjectPropertyAddress {
            mSelector: sel,
            mScope: scope,
            mElement: element,
        };

        if !audio_object.has_property(&addr) {
            // println!("{}{}{}{} (0x{:08x}), not found...",
            //          (sel >> 24) as u8 as char,
            //          (sel >> 16) as u8 as char,
            //          (sel >> 8) as u8 as char,
            //          sel as u8 as char,
            //          sel);
            continue;
        }

        let is_settable = audio_object.is_property_settable(&addr).unwrap();

        println!(
            "{}{}{}{} (0x{:08x}), {}",
            (sel >> 24) as u8 as char,
            (sel >> 16) as u8 as char,
            (sel >> 8) as u8 as char,
            sel as u8 as char,
            sel,
            is_settable
        );

        match sel as u32 {
            ffi::kAudioObjectPropertyBaseClass => print("base class", audio_object.base_class()),
            ffi::kAudioObjectPropertyClass => print("class", audio_object.class()),
            ffi::kAudioObjectPropertyOwner => print("owner", audio_object.owner()),
            ffi::kAudioObjectPropertyName => print("name", audio_object.name()),
            ffi::kAudioObjectPropertyManufacturer => {
                print("manufacturer", audio_object.manufacturer())
            }
            _ => {}
        }
    }
}

fn introspect_dev<'a, S>(dev: &'a AudioObject, desc: S)
where
    S: Into<Option<&'a str>>,
{
    let desc = match desc.into() {
        Some(desc) => desc,
        None => "",
    };

    println!("\n\n{} {:?} Global Properties...", desc, dev);
    introspect(&dev, ffi::kAudioObjectPropertyScopeGlobal);
    println!("\n{} {:?} Input Properties...", desc, dev);
    introspect(&dev, ffi::kAudioObjectPropertyScopeInput);
    println!("\n{} {:?} Output Properties...", desc, dev);
    introspect(&dev, ffi::kAudioObjectPropertyScopeOutput);
    println!("\n{} {:?} Play Through Properties...", desc, dev);
    introspect(&dev, ffi::kAudioObjectPropertyScopePlayThrough);

    println!("\n\nDefault Output {:?} Owned Objects...", dev);
    for obj in audio_object_iter(&dev.owned_objects().unwrap()) {
        println!("Object {:?} Global Properties...", obj);
        introspect(&obj, ffi::kAudioObjectPropertyScopeGlobal);
        // println!("\nObject {:?} Input Properties...", obj);
        // introspect(&obj, ffi::kAudioObjectPropertyScopeInput);
        // println!("\nObject {:?} Output Properties...", obj);
        // introspect(&obj, ffi::kAudioObjectPropertyScopeOutput);
        // println!("\nObject {:?} Play Through Properties...", obj);
        // introspect(&obj, ffi::kAudioObjectPropertyScopePlayThrough);
        println!("");
    }
}

fn main() {
    let aso = audio_system_object();
    println!("Audio System Object ({:?}) Global Properties...", aso);
    introspect(&aso, ffi::kAudioObjectPropertyScopeGlobal);

    let dev = audio_system_object().default_output_device().unwrap();
    introspect_dev(&dev, "Default Output");

    let dev = audio_system_object().default_input_device().unwrap();
    introspect_dev(&dev, "Default Input");
}
