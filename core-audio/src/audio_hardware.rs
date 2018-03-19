use super::*;
use call;
use core_foundation::array::CFArray;
use core_foundation::dictionary::CFDictionary;
use core_foundation::string::CFString;
use ffi;
use libc::pid_t;
use std::{fmt, iter, mem, ops, ptr, slice};
use std::os::raw::c_void;

macro_rules! addr {
    ($sel:ident) => {
        ffi::AudioObjectPropertyAddress {
            mSelector: ffi::$sel,
            mScope: ffi::kAudioObjectPropertyScopeGlobal,
            mElement: ffi::kAudioObjectPropertyElementMaster
        }
    };
    ($sel:ident, $scope:expr, $elem:expr) => {
        ffi::AudioObjectPropertyAddress {
            mSelector: ffi::$sel,
            mScope: $scope,
            mElement: $elem
        }
    };
    ($sel:ident, $scope:ident, $elem:ident) => {
        ffi::AudioObjectPropertyAddress {
            mSelector: ffi::$sel,
            mScope: ffi::$scope,
            mElement: ffi::$elem
        }
    };
}

macro_rules! audio_object {
    (struct $name:ident : $class:ident {}) => {
        #[derive(Clone, Copy, PartialEq, Eq)]
        pub struct $name(ffi::AudioObjectID);

        impl Unknown for $name {
            type Type = $name;
            const UNKNOWN: $name = $name(ffi::kAudioObjectUnknown);
        }

        impl ClassID for $name {
            const CLASS_ID: AudioClassID = AudioClassID(ffi::$class);
        }

        impl fmt::Debug for $name {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.debug_struct(stringify!($name))
                    .field("id", &self.0)
                    .finish()
            }
        }

    }
}

macro_rules! audio_object_is_a {
    ($class:ty, $base_class:ty) => {
        impl ops::Deref for $class {
            type Target = $base_class;
            fn deref(&self) -> &Self::Target {
                unsafe { mem::transmute(self) }
            }
        }
    }
}

macro_rules! getters {
    ($name:ident() => $sel:ident -> int_bool ; $($rest:tt)*) => {
        pub fn $name(&self,
                     scope: AudioObjectPropertyScope,
                     elem: AudioObjectPropertyElement) -> Result<bool> {
            let addr = addr!($sel, scope as _, elem);
            ao::get_property_data::<u32>(self, &addr).map(|v| v != 0)
        }
        getters! {
            $( $rest )*
        }
    };
    ($name:ident() => $sel:ident -> $ret:ty ; $($rest:tt)*) => {
        pub fn $name(&self,
                     scope: AudioObjectPropertyScope,
                     elem: AudioObjectPropertyElement) -> Result<$ret> {
            let addr = addr!($sel, scope as _, elem);
            ao::get_property_data::<$ret>(self, &addr)
        }
        getters! {
            $( $rest )*
        }
    };
    ($name:ident => $sel:ident -> int_bool ; $($rest:tt)*) => {
        pub fn $name(&self) -> Result<bool> {
            let addr = addr!($sel);
            ao::get_property_data::<u32>(self, &addr).map(|v| v != 0)
        }
        getters! {
            $( $rest )*
        }
    };
    ($name:ident => $sel:ident -> [$ret:ty] ; $($rest:tt)*) => {
        pub fn $name(&self) -> Result<Vec<$ret>> {
            let addr = addr!($sel);
            ao::get_property_array::<$ret>(self, &addr)
        }
        getters! {
            $( $rest )*
        }
    };
    ($name:ident => $sel:ident($qual:ty) -> $ret:ty ; $($rest:tt)*) => {
        pub fn $name(&self, qual: &$qual) -> Result<$ret> {
            let addr = addr!($sel);
            ao::get_property_data_with_qualifier::<$ret, $qual>(self, &addr, qual)
        }
        getters! {
            $( $rest )*
        }
    };
    ($name:ident => $sel:ident -> $ret:ty ; $($rest:tt)*) => {
        pub fn $name(&self) -> Result<$ret> {
            let addr = addr!($sel);
            ao::get_property_data::<$ret>(self, &addr)
        }
        getters! {
            $( $rest )*
        }
    };
    () => {}
}

macro_rules! setters {
    ($name:ident => $sel:ident(int_bool); $($rest:tt)*) => {
        pub fn $name(&mut self, data: bool) -> Result<()> {
            let addr = addr!($sel);
            let data: u32 = data as u32;
            ao::set_property_data(self, &addr, &data)
        }
    };
    ($name:ident => $sel:ident($p:ty); $($rest:tt)*) => {
        pub fn $name(&mut self, data: &$p) -> Result<()> {
            let addr = addr!($sel);
            ao::set_property_data(self, &addr, data)
        }
    }
}

//==============================================================================

mod ao {
    use super::{AudioObject, ObjectID, Result};
    use call;
    use ffi::{self, AudioObjectPropertyAddress};
    use std::{mem, ptr};

    // Introspection
    pub fn has_property(id: &AudioObject, addr: &AudioObjectPropertyAddress) -> bool {
        unsafe { ffi::AudioObjectHasProperty(id.id(), addr) != 0 }
    }

    pub fn is_property_settable(
        id: &AudioObject,
        addr: &AudioObjectPropertyAddress,
    ) -> Result<bool> {
        let mut result: ffi::Boolean = 0;
        unsafe {
            call::cvt_r(ffi::AudioObjectIsPropertySettable(
                id.id(),
                addr,
                &mut result,
            ))?;
        }
        Ok(result == 1)
    }

    // Property Access
    pub fn get_property_data_size(
        id: &AudioObject,
        addr: &AudioObjectPropertyAddress,
    ) -> Result<u32> {
        let mut data_size: u32 = 0;
        unsafe {
            call::cvt_r(ffi::AudioObjectGetPropertyDataSize(
                id.id(),
                addr,
                0,
                ptr::null(),
                &mut data_size,
            ))?;
        }
        Ok(data_size)
    }

    pub fn get_property_data_size_with_qualifier<Q>(
        id: &AudioObject,
        addr: &AudioObjectPropertyAddress,
        qual: &Q,
    ) -> Result<u32> {
        let mut data_size: u32 = 0;
        unsafe {
            call::cvt_r(ffi::AudioObjectGetPropertyDataSize(
                id.id(),
                addr,
                mem::size_of::<Q>() as u32,
                qual as *const _ as *const _,
                &mut data_size,
            ))?;
        }
        Ok(data_size)
    }

    pub fn get_property_data<T>(id: &AudioObject, addr: &AudioObjectPropertyAddress) -> Result<T> {
        debug_assert_eq!(
            mem::size_of::<T>(),
            try!(get_property_data_size(id, addr)) as _
        );
        let mut data: T = unsafe { mem::uninitialized() };
        let mut data_size = mem::size_of::<T>() as u32;
        unsafe {
            call::cvt_r(ffi::AudioObjectGetPropertyData(
                id.id(),
                addr,
                0,
                ptr::null(),
                &mut data_size as *mut _,
                &mut data as *mut _ as *mut _,
            ))?;
        }
        Ok(data)
    }

    pub fn get_property_array<T>(
        id: &AudioObject,
        addr: &AudioObjectPropertyAddress,
    ) -> Result<Vec<T>>
    where
        T: Sized,
    {
        let mut data_size = get_property_data_size(id, addr)?;
        let mut data = Vec::<T>::with_capacity(data_size as _);
        unsafe {
            data.set_len(data_size as _);
            call::cvt_r(ffi::AudioObjectGetPropertyData(
                id.id(),
                addr,
                0,
                ptr::null(),
                &mut data_size as *mut _,
                data.as_mut_ptr() as *mut _,
            ))?;
        }
        Ok(data)
    }

    pub fn get_property_data_with_qualifier<T, Q>(
        id: &AudioObject,
        addr: &AudioObjectPropertyAddress,
        qual: &Q,
    ) -> Result<T>
    where
        T: Sized,
        Q: Sized,
    {
        let mut data: T = unsafe { mem::uninitialized() };
        let mut data_size = mem::size_of::<T>() as u32;
        unsafe {
            call::cvt_r(ffi::AudioObjectGetPropertyData(
                id.id(),
                addr,
                mem::size_of::<Q>() as u32,
                qual as *const _ as *const _,
                &mut data_size as *mut _,
                &mut data as *mut _ as *mut _,
            ))?;
        }
        Ok(data)
    }

    // Property Setting
    pub fn set_property_data<T>(
        id: &AudioObject,
        addr: &AudioObjectPropertyAddress,
        data: &T,
    ) -> Result<()>
    where
        T: Sized,
    {
        unsafe {
            call::cvt_r(ffi::AudioObjectSetPropertyData(
                id.id(),
                addr,
                0,
                ptr::null(),
                mem::size_of::<T>() as u32,
                data as *const _ as *const _,
            ))?;
        }
        Ok(())
    }

    pub fn set_property_data_with_qualifier<T, Q>(
        id: &AudioObject,
        addr: &AudioObjectPropertyAddress,
        qual: &Q,
        data: &T,
    ) -> Result<()>
    where
        T: Sized,
        Q: Sized,
    {
        unsafe {
            call::cvt_r(ffi::AudioObjectSetPropertyData(
                id.id(),
                addr,
                mem::size_of::<Q>() as u32,
                qual as *const _ as *const _,
                mem::size_of::<T>() as u32,
                data as *const _ as *const _,
            ))?;
        }
        Ok(())
    }

    // Property Listeners
/*
    pub fn add_property_listener<F>(id: AudioObjectID,
                                    addr: &AudioObjectPropertyAddress,
                                    f: F) -> Result<ListenerHandle>
        where F: FnMut(ffi::AudioObjectID, &[ffi::AudioObjectPropertyAddress]) -> Result<()> + Send + 'static
    {
        let cb_thunk = Box::new(PropertyListenerThunk::new(f));
        let cb_thunk_ptr = Box::into_raw(cb_thunk) as *mut c_void;
        
        unsafe { try_call!(ffi::AudioObjectAddPropertyListener(id, addr,
                                                               Some(_property_listener_shim as _),
                                                               cb_thunk_ptr)); }
        Ok(ListenerHandle(*addr, cb_thunk_ptr))
    }

    pub fn remove_property_listener(id: ffi::AudioObjectID, handle: ListenerHandle) -> Result<()> {
        let addr = handle.0;
        let cb_thunk_ptr = handle.1;
        unsafe {
            try_call!(ffi::AudioObjectRemovePropertyListener(id, &addr,
                                                             Some(_property_listener_shim as _),
                                                             cb_thunk_ptr));
            drop(Box::from_raw(cb_thunk_ptr as *mut PropertyListenerThunk));
        }
        Ok(())
    }
*/
}

//==============================================================================

pub type AudioObjectPropertySelector = ffi::AudioObjectPropertySelector;
#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AudioObjectPropertyScope {
    Global = ffi::kAudioObjectPropertyScopeGlobal,
    Input = ffi::kAudioObjectPropertyScopeInput,
    Output = ffi::kAudioObjectPropertyScopeOutput,
    PlayThrought = ffi::kAudioObjectPropertyScopePlayThrough,
}
pub type AudioObjectPropertyElement = ffi::AudioObjectPropertyElement;

audio_object! {
    struct AudioObject: kAudioObjectClassID {}
}

impl ObjectID for AudioObject {
    fn id(&self) -> ffi::AudioObjectID {
        self.0
    }
}

impl AudioObject {
    pub fn is(&self, class: AudioClassID) -> bool {
        match self.class() {
            Ok(my_class) => my_class == class,
            _ => false,
        }
    }

    pub fn downcast_ref<T: ClassID>(&self) -> Option<&T> {
        if self.is(T::CLASS_ID) {
            unsafe { Some(&*(self as *const _ as *const T)) }
        } else {
            None
        }
    }

    // Introspection
    #[inline]
    pub fn has_property(&self, addr: &ffi::AudioObjectPropertyAddress) -> bool {
        ao::has_property(self, addr)
    }

    #[inline]
    pub fn is_property_settable(&self, addr: &ffi::AudioObjectPropertyAddress) -> Result<bool> {
        ao::is_property_settable(self, addr)
    }

    getters! {
        base_class => kAudioObjectPropertyBaseClass -> AudioClassID;
        class => kAudioObjectPropertyClass -> AudioClassID;
        owner => kAudioObjectPropertyOwner -> AudioObject;
        name => kAudioObjectPropertyName -> CFString;
        manufacturer => kAudioObjectPropertyManufacturer -> CFString;
        element_name => kAudioObjectPropertyElementName -> CFString;
        owned_objects => kAudioObjectPropertyOwnedObjects -> [AudioObject];
    }

    setters! {
        set_element_name => kAudioObjectPropertyElementName(CFString);
    }
}

pub trait ObjectID {
    fn id(&self) -> ffi::AudioObjectID;
}

pub trait ClassID {
    const CLASS_ID: AudioClassID;
}

pub trait Unknown {
    type Type;
    const UNKNOWN: Self::Type;
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct AudioClassID(ffi::AudioClassID);

impl fmt::Debug for AudioClassID {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(
            fmt,
            "{:08x} ('{}{}{}{}')",
            self.0,
            (self.0 >> 24) as u8 as char,
            ((self.0 >> 16) & 0xFF) as u8 as char,
            ((self.0 >> 8) & 0xFF) as u8 as char,
            (self.0 & 0xFF) as u8 as char
        )
    }
}

//==============================================================================
// Iterator
type AudioObjectIter<'a> = iter::TakeWhile<slice::Iter<'a, AudioObject>, fn(&&AudioObject) -> bool>;
pub fn audio_object_iter(v: &[AudioObject]) -> AudioObjectIter {
    v.iter().take_while(|ao| **ao != AudioObject::UNKNOWN)
}

//==============================================================================
// Audio Object

pub struct ListenerHandle(ffi::AudioObjectPropertyAddress, *mut c_void);

unsafe extern "C" fn _property_listener_shim(
    id: ffi::AudioObjectID,
    addr_count: u32,
    addr: *const ffi::AudioObjectPropertyAddress,
    client_data: *mut c_void,
) -> ffi::OSStatus {
    debug_assert!(!client_data.is_null());
    let addrs = slice::from_raw_parts(addr, addr_count as _);
    let thunk: &mut PropertyListenerThunk = &mut *(client_data as *mut _);
    match (thunk.cb)(id, addrs) {
        Ok(_) => ffi::kAudioHardwareNoError,
        Err(e) => e.raw_osstatus(),
    }
}

pub type PropertyListenerFn =
    FnMut(ffi::AudioObjectID, &[ffi::AudioObjectPropertyAddress]) -> Result<()>;

struct PropertyListenerThunk {
    cb: Box<PropertyListenerFn>,
}

impl PropertyListenerThunk {
    fn new<F>(f: F) -> Self
    where
        F: FnMut(ffi::AudioObjectID, &[ffi::AudioObjectPropertyAddress]) -> Result<()>
            + Send
            + 'static,
    {
        PropertyListenerThunk { cb: Box::new(f) }
    }
}

//==============================================================================
// AudioStream

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioStreamDirection {
    Output = 0,
    Input = 1,
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct AudioStreamRangedDescription {
    pub format: AudioStreamBasicDescription,
    pub sample_rate_range: AudioValueRange,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioStreamTerminalType {
    Unknown = ffi::kAudioStreamTerminalTypeUnknown,
    Line = ffi::kAudioStreamTerminalTypeLine,
    DigitalAudioInterface = ffi::kAudioStreamTerminalTypeDigitalAudioInterface,
    Speaker = ffi::kAudioStreamTerminalTypeSpeaker,
    Headphones = ffi::kAudioStreamTerminalTypeHeadphones,
    LfeSpeaker = ffi::kAudioStreamTerminalTypeLFESpeaker,
    ReceiverSpeaker = ffi::kAudioStreamTerminalTypeReceiverSpeaker,
    Microphone = ffi::kAudioStreamTerminalTypeMicrophone,
    HeadsetMicrophone = ffi::kAudioStreamTerminalTypeHeadsetMicrophone,
    ReceiverMicrophone = ffi::kAudioStreamTerminalTypeReceiverMicrophone,
    Tty = ffi::kAudioStreamTerminalTypeTTY,
    Hdmi = ffi::kAudioStreamTerminalTypeHDMI,
    DisplayPort = ffi::kAudioStreamTerminalTypeDisplayPort,
}

audio_object! {
    struct AudioStream: kAudioStreamClassID {}
}
audio_object_is_a!(AudioStream, AudioObject);

impl AudioStream {
    getters! {
        is_active => kAudioStreamPropertyIsActive -> int_bool;
        direction => kAudioStreamPropertyDirection -> AudioStreamDirection;
        terminal_type => kAudioStreamPropertyTerminalType -> AudioStreamTerminalType;
        starting => kAudioStreamPropertyStartingChannel -> u32;
        latency => kAudioStreamPropertyLatency -> u32;
        virtual_format => kAudioStreamPropertyVirtualFormat -> AudioStreamBasicDescription;
        available_virtual_formats =>
            kAudioStreamPropertyAvailableVirtualFormats -> [AudioStreamRangedDescription];
        physical_format => kAudioStreamPropertyPhysicalFormat -> AudioStreamBasicDescription;
        available_physical_formats =>
            kAudioStreamPropertyAvailablePhysicalFormats -> [AudioStreamRangedDescription];
    }
}

//==============================================================================
// AudioPlugIn
audio_object! {
    struct AudioPlugIn: kAudioPlugInClassID {}
}
audio_object_is_a!(AudioPlugIn, AudioObject);

impl AudioPlugIn {
    getters! {
        bundle_id => kAudioPlugInPropertyBundleID -> CFString;
        device_list => kAudioPlugInPropertyDeviceList -> [AudioDevice];
        translate_uid_to_device =>
            kAudioPlugInPropertyTranslateUIDToDevice(CFString) -> AudioDevice;
        //box_list => kAudioPlugInPropertyBoxList -> [AudioBox];
        //translate_uid_to_box => kAudioPlugInPropertyTranslateUIDToBox(CFString) -> AudioBox;
        //clock_device_list => kAudioPlugInPropertyClockDeviceList -> [AudioClockDevice];
        //translate_uid_to_clock_device =>
        //    kAudioPlugInPropertyTranslateUIDToClockDevice(CFString) -> AudioCloceDevice;
        create_aggregate_device => kAudioPlugInCreateAggregateDevice(CFDictionary) -> AudioDevice;
    }

    pub fn destroy_aggregate_device(&self, aggregate_device: AudioDevice) -> Result<()> {
        // This method for destroying an aggregate device is a little
        // strange. You pass the device id *into* a get call.
        let mut data = aggregate_device;
        let addr = addr!(kAudioPlugInDestroyAggregateDevice);
        let mut data_size = mem::size_of::<AudioDevice>() as u32;
        debug_assert_eq!(
            data_size,
            try!(ao::get_property_data_size(self, &addr)) as _
        );
        unsafe {
            call::cvt_r(ffi::AudioObjectGetPropertyData(
                self.id(),
                &addr,
                0,
                ptr::null(),
                &mut data_size as *mut _,
                &mut data as *mut _ as *mut _,
            ))?;
        }
        Ok(())
    }
}

//==============================================================================
// AudioSystemObject

#[repr(u32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AudioHardwarePowerHint {
    None = ffi::kAudioHardwarePowerHintNone,
    FavorSavingPower = ffi::kAudioHardwarePowerHintFavorSavingPower,
}

audio_object! {
    struct AudioSystemObject: kAudioSystemObjectClassID {}
}
audio_object_is_a!(AudioSystemObject, AudioObject);

impl AudioSystemObject {
    getters! {
        devices => kAudioHardwarePropertyDevices -> [AudioObject];
        default_input_device => kAudioHardwarePropertyDefaultInputDevice -> AudioObject;
        default_output_device => kAudioHardwarePropertyDefaultOutputDevice -> AudioObject;
        default_system_output_device =>
            kAudioHardwarePropertyDefaultSystemOutputDevice -> AudioObject;
        translate_uid_to_device =>
            kAudioHardwarePropertyTranslateUIDToDevice(CFString) -> AudioObject;
        mix_stereo_to_mono => kAudioHardwarePropertyMixStereoToMono -> u32;
        plug_in_list => kAudioHardwarePropertyPlugInList -> [AudioObject];
        translate_bundle_id_to_plug_in =>
            kAudioHardwarePropertyTranslateBundleIDToPlugIn(CFString) -> AudioObject;
        transport_manager_list => kAudioHardwarePropertyTransportManagerList -> [AudioObject];
        translate_bundle_id_to_transport_manager =>
            kAudioHardwarePropertyTranslateBundleIDToTransportManager(CFString) -> AudioObject;
        box_list => kAudioHardwarePropertyBoxList -> [AudioObject];
        translate_uid_to_box => kAudioHardwarePropertyTranslateUIDToBox(CFString) -> AudioObject;
        clock_device_list => kAudioHardwarePropertyClockDeviceList -> [AudioObject];
        translate_uid_to_clock_device =>
            kAudioHardwarePropertyTranslateUIDToClockDevice(CFString) -> AudioObject;
        process_is_master => kAudioHardwarePropertyProcessIsMaster -> int_bool;
        is_initing_or_exiting => kAudioHardwarePropertyIsInitingOrExiting -> int_bool;
        // @constant       kAudioHardwarePropertyUserIDChanged
        // This property exists so that clients can tell the HAL when
        // they are changing the effective user ID of the process. The
        // way it works is that a client will set the value of this
        // property and the HAL will flush all its cached per- user
        // preferences such as the default devices. The value of this
        // property is a UInt32, but its value has no currently
        // defined meaning and clients may pass any value when setting
        // it to trigger the cache flush.
        process_is_audible => kAudioHardwarePropertyProcessIsAudible -> int_bool;
        sleeping_is_allowed => kAudioHardwarePropertySleepingIsAllowed -> int_bool;
        unloading_is_allowed => kAudioHardwarePropertyUnloadingIsAllowed -> int_bool;
        hog_mode_is_allowed => kAudioHardwarePropertyHogModeIsAllowed -> int_bool;
        is_active_or_headless => kAudioHardwarePropertyUserSessionIsActiveOrHeadless -> int_bool;
        power_hint => kAudioHardwarePropertyPowerHint -> AudioHardwarePowerHint;
    }

    setters! {
        set_sleeping_is_allowed => kAudioHardwarePropertySleepingIsAllowed(int_bool);
        set_unloading_is_allowed => kAudioHardwarePropertyUnloadingIsAllowed(int_bool);
        set_hog_mode_is_allowed => kAudioHardwarePropertyHogModeIsAllowed(int_bool);
        set_power_hint => kAudioHardwarePropertyPowerHint(AudioHardwarePowerHint);
    }
}

pub fn audio_system_object() -> AudioSystemObject {
    AudioSystemObject(ffi::kAudioObjectSystemObject)
}

//==================================================================================================
// AudioDevice
pub type AudioDeviceIOProc = FnMut(
    AudioObject,
    &AudioTimeStamp,      // inNow
    &AudioBufferList,     // inInputData
    &AudioTimeStamp,      // inInputTime
    &mut AudioBufferList, // outOutputData
    &AudioTimeStamp,
); // inOutputTime

pub type AudioDeviceIOProcID = ffi::AudioDeviceIOProcID;

audio_object! {
    struct AudioDevice: kAudioDeviceClassID {}
}
audio_object_is_a!(AudioDevice, AudioObject);

/// The `AudioDevice` class is a subclass of the `AudioObjectClass`. The class has four
/// scopes, `kAudioObjectPropertyScopeGlobal`, `kAudioObjectPropertyScopeInput`,
/// `kAudioObjectPropertyScopeOutput`, and `kAudioObjectPropertyScopePlayThrough`. The
/// class has a master element and an element for each channel in each stream
/// numbered according to the starting channel number of each stream.
impl AudioDevice {
    pub const CLASS_ID: AudioClassID = AudioClassID(ffi::kAudioDeviceClassID);

    // AudioDevice properties
    pub const CONFIGURATION_APPLICATION: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyConfigurationApplication;
    pub const DEVICE_UID: AudioObjectPropertySelector = ffi::kAudioDevicePropertyDeviceUID;
    pub const MODEL_UID: AudioObjectPropertySelector = ffi::kAudioDevicePropertyModelUID;
    pub const TRANSPORT_TYPE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyTransportType;
    pub const RELATED_DEVICES: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyRelatedDevices;
    pub const CLOCK_DOMAIN: AudioObjectPropertySelector = ffi::kAudioDevicePropertyClockDomain;
    pub const DEVICE_IS_ALIVE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyDeviceIsAlive;
    pub const DEVICE_IS_RUNNING: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyDeviceIsRunning;
    pub const DEVICE_CAN_BE_DEFAULT_DEVICE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyDeviceCanBeDefaultDevice;
    pub const DEVICE_CAN_BE_DEFAULT_SYSTEM_DEVICE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyDeviceCanBeDefaultSystemDevice;
    pub const LATENCY: AudioObjectPropertySelector = ffi::kAudioDevicePropertyLatency;
    pub const STREAMS: AudioObjectPropertySelector = ffi::kAudioDevicePropertyStreams;
    pub const CONTROL_LIST: AudioObjectPropertySelector = ffi::kAudioObjectPropertyControlList;
    pub const SAFETY_OFFSET: AudioObjectPropertySelector = ffi::kAudioDevicePropertySafetyOffset;
    pub const NOMINAL_SAMPLE_RATE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyNominalSampleRate;
    pub const AVAILABLE_NOMINAL_SAMPLE_RATES: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyAvailableNominalSampleRates;
    pub const ICON: AudioObjectPropertySelector = ffi::kAudioDevicePropertyIcon;
    pub const IS_HIDDEN: AudioObjectPropertySelector = ffi::kAudioDevicePropertyIsHidden;
    pub const PREFERRED_CHANNELS_FOR_STEREO: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPreferredChannelsForStereo;
    pub const PREFERRED_CHANNEL_LAYOUT: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPreferredChannelLayout;

    pub const PLUG_IN: AudioObjectPropertySelector = ffi::kAudioDevicePropertyPlugIn;
    pub const DEVICE_HAS_CHANGED: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyDeviceHasChanged;
    pub const DEVICE_IS_RUNNING_SOMEWHERE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyDeviceIsRunningSomewhere;
    pub const R_OVERLOAD: AudioObjectPropertySelector = ffi::kAudioDeviceProcessorOverload;
    pub const IOSTOPPED_ABNORMALLY: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyIOStoppedAbnormally;
    pub const HOG_MODE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyHogMode;
    pub const BUFFER_FRAME_SIZE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyBufferFrameSize;
    pub const BUFFER_FRAME_SIZE_RANGE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyBufferFrameSizeRange;
    pub const USES_VARIABLE_BUFFER_FRAME_SIZES: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyUsesVariableBufferFrameSizes;
    pub const IOCYCLE_USAGE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyIOCycleUsage;
    pub const STREAM_CONFIGURATION: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyStreamConfiguration;
    pub const IOPROC_STREAM_USAGE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyIOProcStreamUsage;
    pub const ACTUAL_SAMPLE_RATE: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyActualSampleRate;
    pub const CLOCK_DEVICE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyClockDevice;

    pub const JACK_IS_CONNECTED: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyJackIsConnected;
    pub const VOLUME_SCALAR: AudioObjectPropertySelector = ffi::kAudioDevicePropertyVolumeScalar;
    pub const VOLUME_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyVolumeDecibels;
    pub const VOLUME_RANGE_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyVolumeRangeDecibels;
    pub const VOLUME_SCALAR_TO_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyVolumeScalarToDecibels;
    pub const VOLUME_DECIBELS_TO_SCALAR: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyVolumeDecibelsToScalar;
    pub const STEREO_PAN: AudioObjectPropertySelector = ffi::kAudioDevicePropertyStereoPan;
    pub const STEREO_PAN_CHANNELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyStereoPanChannels;
    pub const MUTE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyMute;
    pub const SOLO: AudioObjectPropertySelector = ffi::kAudioDevicePropertySolo;
    pub const PHANTOM_POWER: AudioObjectPropertySelector = ffi::kAudioDevicePropertyPhantomPower;
    pub const PHASE_INVERT: AudioObjectPropertySelector = ffi::kAudioDevicePropertyPhaseInvert;
    pub const CLIP_LIGHT: AudioObjectPropertySelector = ffi::kAudioDevicePropertyClipLight;
    pub const TALKBACK: AudioObjectPropertySelector = ffi::kAudioDevicePropertyTalkback;
    pub const LISTENBACK: AudioObjectPropertySelector = ffi::kAudioDevicePropertyListenback;
    pub const DATA_SOURCE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyDataSource;
    pub const DATA_SOURCES: AudioObjectPropertySelector = ffi::kAudioDevicePropertyDataSources;
    pub const DATA_SOURCE_NAME_FOR_IDCFSTRING: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyDataSourceNameForIDCFString;
    pub const DATA_SOURCE_KIND_FOR_ID: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyDataSourceKindForID;
    pub const CLOCK_SOURCE: AudioObjectPropertySelector = ffi::kAudioDevicePropertyClockSource;
    pub const CLOCK_SOURCES: AudioObjectPropertySelector = ffi::kAudioDevicePropertyClockSources;
    pub const CLOCK_SOURCE_NAME_FOR_IDCFSTRING: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyClockSourceNameForIDCFString;
    pub const CLOCK_SOURCE_KIND_FOR_ID: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyClockSourceKindForID;
    pub const PLAY_THRU: AudioObjectPropertySelector = ffi::kAudioDevicePropertyPlayThru;
    pub const PLAY_THRU_SOLO: AudioObjectPropertySelector = ffi::kAudioDevicePropertyPlayThruSolo;
    pub const PLAY_THRU_VOLUME_SCALAR: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruVolumeScalar;
    pub const PLAY_THRU_VOLUME_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruVolumeDecibels;
    pub const PLAY_THRU_VOLUME_RANGE_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruVolumeRangeDecibels;
    pub const PLAY_THRU_VOLUME_SCALAR_TO_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruVolumeScalarToDecibels;
    pub const PLAY_THRU_VOLUME_DECIBELS_TO_SCALAR: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruVolumeDecibelsToScalar;
    pub const PLAY_THRU_STEREO_PAN: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruStereoPan;
    pub const PLAY_THRU_STEREO_PAN_CHANNELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruStereoPanChannels;
    pub const PLAY_THRU_DESTINATION: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruDestination;
    pub const PLAY_THRU_DESTINATIONS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruDestinations;
    pub const PLAY_THRU_DESTINATION_NAME_FOR_IDCFSTRING: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyPlayThruDestinationNameForIDCFString;
    pub const CHANNEL_NOMINAL_LINE_LEVEL: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyChannelNominalLineLevel;
    pub const CHANNEL_NOMINAL_LINE_LEVELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyChannelNominalLineLevels;
    pub const CHANNEL_NOMINAL_LINE_LEVEL_NAME_FOR_IDCFSTRING: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyChannelNominalLineLevelNameForIDCFString;
    pub const HIGH_PASS_FILTER_SETTING: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyHighPassFilterSetting;
    pub const HIGH_PASS_FILTER_SETTINGS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyHighPassFilterSettings;
    pub const HIGH_PASS_FILTER_SETTING_NAME_FOR_IDCFSTRING: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertyHighPassFilterSettingNameForIDCFString;
    pub const SUB_VOLUME_SCALAR: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertySubVolumeScalar;
    pub const SUB_VOLUME_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertySubVolumeDecibels;
    pub const SUB_VOLUME_RANGE_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertySubVolumeRangeDecibels;
    pub const SUB_VOLUME_SCALAR_TO_DECIBELS: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertySubVolumeScalarToDecibels;
    pub const SUB_VOLUME_DECIBELS_TO_SCALAR: AudioObjectPropertySelector =
        ffi::kAudioDevicePropertySubVolumeDecibelsToScalar;
    pub const SUB_MUTE: AudioObjectPropertySelector = ffi::kAudioDevicePropertySubMute;

    getters! {
        base_class => kAudioObjectPropertyBaseClass -> AudioClassID;
        class => kAudioObjectPropertyClass -> AudioClassID;
        owner => kAudioObjectPropertyOwner -> AudioObject;
        name => kAudioObjectPropertyName -> CFString;
        model_name => kAudioObjectPropertyModelName -> CFString;
        manufacturer => kAudioObjectPropertyManufacturer -> CFString;
        element_name => kAudioObjectPropertyElementName -> CFString;
        element_category_name => kAudioObjectPropertyElementCategoryName -> CFString;
        element_number_name => kAudioObjectPropertyElementNumberName -> CFString;
        owned_objects => kAudioObjectPropertyOwnedObjects -> [AudioObject];
        identify => kAudioObjectPropertyIdentify -> u32;
        serial_number => kAudioObjectPropertySerialNumber -> CFString;
        firmware_version => kAudioObjectPropertyFirmwareVersion -> CFString;

        configuration_app => kAudioDevicePropertyConfigurationApplication -> CFString;
        device_uid => kAudioDevicePropertyDeviceUID -> CFString;
        model_uid => kAudioDevicePropertyModelUID -> CFString;
        transport_type => kAudioDevicePropertyTransportType -> u32;
        related_devices => kAudioDevicePropertyRelatedDevices -> [AudioDevice];
        clock_domain => kAudioDevicePropertyClockDomain -> u32;
        is_alive => kAudioDevicePropertyDeviceIsAlive -> int_bool;
        is_running => kAudioDevicePropertyDeviceIsRunning -> int_bool;
        can_be_default_device() => kAudioDevicePropertyDeviceCanBeDefaultDevice -> int_bool;
        can_be_default_system_device() =>
            kAudioDevicePropertyDeviceCanBeDefaultSystemDevice -> int_bool;
        latency() => kAudioDevicePropertyLatency -> u32;
        streams => kAudioDevicePropertyStreams -> [AudioStream];
        //control_list => kAudioObjectPropertyControlList -> [AudioControl];
        nominal_sample_rate => kAudioDevicePropertyNominalSampleRate -> f64;
        available_nominal_sample_rates =>
            kAudioDevicePropertyAvailableNominalSampleRates -> [AudioValueRange];
        //icon => kAudioDevicePropertyIcon -> CFUrl;
        is_hidden => kAudioDevicePropertyIsHidden -> int_bool;
        preferred_channels_for_stereo() =>
            kAudioDevicePropertyPreferredChannelsForStereo -> (u32, u32);
        preferred_channel_layout() =>
            kAudioDevicePropertyPreferredChannelLayout -> AudioChannelLayout;

        plug_in => kAudioDevicePropertyPlugIn -> ffi::OSStatus;
        // @constant       kAudioDevicePropertyDeviceHasChanged
        // The type of this property is a UInt32, but its value has no
        // meaning. This property exists so that clients can listen to
        // it and be told when the configuration of the AudioDevice
        // has changed in ways that cannot otherwise be conveyed
        // through other notifications. In response to this
        // notification, clients should re-evaluate everything they
        // need to know about the device, particularly the layout and
        // values of the controls.
        device_is_running_somewhere => kAudioDevicePropertyDeviceIsRunningSomewhere -> int_bool;

        // @constant       kAudioDeviceProcessorOverload
        // A UInt32 where the value has no meaning. This property
        // exists so that clients can be notified when the AudioDevice
        // detects that an IO cycle has run past its deadline. Note
        // that the notification for this property is usually sent
        // from the AudioDevice's IO thread.

        // @constant       kAudioDevicePropertyIOStoppedAbnormally

        // A UInt32 where the value has no meaning. This property
        // exists so that clients can be notified when IO on the
        // device has stopped outside of the normal mechanisms. This
        // typically comes up when IO is stopped after
        // AudioDeviceStart has returned successfully but prior to the
        // notification for kAudioDevicePropertyIsRunning being sent.
        hog_mode => kAudioDevicePropertyHogMode -> pid_t;
        buffer_frame_size => kAudioDevicePropertyBufferFrameSize -> u32;
        buffer_frame_size_range => kAudioDevicePropertyBufferFrameSizeRange -> AudioValueRange;
        uses_variable_buffer_frame_sizes => kAudioDevicePropertyUsesVariableBufferFrameSizes -> u32;
        io_cycle_usage => kAudioDevicePropertyIOCycleUsage -> f32;
        stream_configuration => kAudioDevicePropertyStreamConfiguration -> ffi::AudioBufferList;
        io_proc_stream_usage =>
            kAudioDevicePropertyIOProcStreamUsage -> ffi::AudioHardwareIOProcStreamUsage;
        actual_sample_rate => kAudioDevicePropertyActualSampleRate -> f64;
        clock_device => kAudioDevicePropertyClockDevice -> CFString;
    }

    setters! {
        set_nominal_sample_rate => kAudioDevicePropertyNominalSampleRate(f64);
        set_preferred_channels_for_stereo =>
            kAudioDevicePropertyPreferredChannelsForStereo((u32,32));
        set_preferred_channel_layout =>
            kAudioDevicePropertyPreferredChannelLayout(AudioChannelLayout);

        set_hog_mode =>  kAudioDevicePropertyHogMode(pid_t);
        // A pid_t indicating the process that currently owns exclusive
        // access to the AudioDevice or a value of -1 indicating that
        // the device is currently available to all processes. If the
        // AudioDevice is in a non-mixable mode, the HAL will
        // automatically take hog mode on behalf of the first process
        // to start an IOProc.  Note that when setting this property,
        // the value passed in is ignored. If another process owns
        // exclusive access, that remains unchanged. If the current
        // process owns exclusive access, it is released and made
        // available to all processes again. If no process has
        // exclusive access (meaning the current value is -1), this
        // process gains ownership of exclusive access.  On return,
        // the pid_t pointed to by inPropertyData will contain the new
        // value of the property.

    // @constant       kAudioDevicePropertyIOCycleUsage
    //                     A Float32 whose range is from 0 to 1. This value indicates how much of the
    //                     client portion of the IO cycle the process will use. The client portion of
    //                     the IO cycle is the portion of the cycle in which the device calls the
    //                     IOProcs so this property does not the apply to the duration of the entire
    //                     cycle.
    // @constant       kAudioDevicePropertyIOProcStreamUsage
    //                     An AudioHardwareIOProcStreamUsage structure which details the stream usage
    //                     of a given IO proc. If a stream is marked as not being used, the given
    //                     IOProc will see a corresponding NULL buffer pointer in the AudioBufferList
    //                     passed to its IO proc. Note that the number of streams detailed in the
    //                     AudioHardwareIOProcStreamUsage must include all the streams of that
    //                     direction on the device. Also, when getting the value of the property, one
    //                     must fill out the mIOProc field of the AudioHardwareIOProcStreamUsage with
    //                     the address of the of the IOProc whose stream usage is to be retrieved.
    // @constant       kAudioDevicePropertyClockDevice
    //                     A CFString that contains the UID for the AudioClockDevice that is currently
    //                     serving as the master time base of the device. The caller is responsible
    //                     for releasing the returned CFObject.
    }

    /// Creates an AudioDeviceIOProcID from an AudioDeviceIOProc and a client data  pointer.
    ///
    /// AudioDeviceIOProcIDs allow for the client to register the same function pointer with a device multiple times.
    pub fn create_io_proc_id<F>(&self, _f: F) -> Result<AudioDeviceIOProcID>
    where
        F: FnMut(
            AudioObject,
            &AudioTimeStamp,
            &AudioBufferList,
            &AudioTimeStamp,
            &mut AudioBufferList,
            &AudioTimeStamp,
        )
            + Send
            + 'static,
    {
        let result: AudioDeviceIOProcID = unsafe { mem::uninitialized() };
        // try_unsafe_call!(ffi::AudioDeviceCreateIOProcID(self.0,
        //                                                AudioDeviceIOProc inProc,
        //                                                inClientData: *mut c_void,
        //                                                &mut result));
        Ok(result)
    }

    /// Destroys an AudioDeviceIOProcID.
    pub fn destroy_io_proc_id(&self, io_proc_id: AudioDeviceIOProcID) -> Result<()> {
        unsafe {
            call::cvt_r(ffi::AudioDeviceDestroyIOProcID(self.id(), io_proc_id))?;
        }
        Ok(())
    }

    /// Starts IO for the given AudioDeviceIOProcID.
    pub fn start(&mut self, proc_id: AudioDeviceIOProcID) -> Result<()> {
        unsafe {
            call::cvt_r(ffi::AudioDeviceStart(self.id(), proc_id))?;
        }
        Ok(())
    }

    /// Starts IO for the given AudioDeviceIOProcID and aligns the IO
    /// cycle of the AudioDevice with the given time.
    pub fn start_at_time(
        &self,
        proc_id: AudioDeviceIOProcID,
        requested_start_time: &AudioTimeStampRef,
        flags: u32,
    ) -> Result<AudioTimeStamp> {
        let start_time = requested_start_time.to_owned();
        unsafe {
            call::cvt_r(ffi::AudioDeviceStartAtTime(
                self.id(),
                proc_id,
                start_time.as_ptr(),
                flags,
            ))?;
        }
        Ok(start_time)
    }

    /// Stops IO for the given AudioDeviceIOProcID.
    pub fn stop(&self, proc_id: AudioDeviceIOProcID) -> Result<()> {
        unsafe {
            call::cvt_r(ffi::AudioDeviceStop(self.id(), proc_id))?;
        }
        Ok(())
    }

    /// Retrieves the current time from an AudioDevice. Note that the
    /// device has to be running.
    pub fn get_current_time(&self) -> Result<AudioTimeStamp> {
        let result: AudioTimeStamp = Default::default();
        unsafe {
            call::cvt_r(ffi::AudioDeviceGetCurrentTime(self.id(), result.as_ptr()))?;
        }
        Ok(result)
    }

    /// Translates the time in the AudioDevice's time base from one
    /// representation to another. Note that the device has to be
    /// running
    pub fn translate_time(&self, time: &AudioTimeStamp) -> Result<AudioTimeStamp> {
        let result: AudioTimeStamp = Default::default();
        unsafe {
            call::cvt_r(ffi::AudioDeviceTranslateTime(
                self.id(),
                time.as_ptr(),
                result.as_ptr(),
            ))?;
        }
        Ok(result)
    }

    /// Query an AudioDevice to get a time equal to or later than the
    /// given time that is the best time to start IO.
    ///
    /// The time that is returned is dictated by the constraints of the device and the
    /// system. For instance, the driver of a device that provides both audio and video
    /// data may only allow start times that coincide with the edge of a video frame.
    /// Also, if the device already has one or more active IOProcs, the start time will
    /// be shifted to the beginning of the next IO cycle so as not to cause
    /// discontinuities in the existing IOProcs. Another reason the start time may shift
    /// is to allow for aligning the buffer accesses in an optimal fashion. Note that
    /// the device must be running to use this function.
    pub fn get_nearest_start_time(
        &self,
        requested_start_time: &mut AudioTimeStamp,
        flags: u32,
    ) -> Result<AudioTimeStamp> {
        let start_time = requested_start_time.to_owned();
        unsafe {
            call::cvt_r(ffi::AudioDeviceGetNearestStartTime(
                self.id(),
                start_time.as_ptr(),
                flags,
            ))?;
        }
        Ok(start_time)
    }
}

//==============================================================================
// AudioAggregateDevice

audio_object! {
    struct AudioAggregateDevice: kAudioAggregateDeviceClassID {}
}
audio_object_is_a!(AudioAggregateDevice, AudioDevice);

impl AudioAggregateDevice {
    pub const UID_KEY: &'static str = "uid";
    pub const NAME_KEY: &'static str = "name";
    pub const SUB_DEVICE_LIST_KEY: &'static str = "subdevices";
    pub const MASTER_SUB_DEVICE_KEY: &'static str = "master";
    pub const CLOCK_DEVICE_KEY: &'static str = "clock";
    pub const IS_PRIVATE_KEY: &'static str = "private";
    pub const IS_STACKED_KEY: &'static str = "stacked";

    getters! {
        full_sub_device_list => kAudioAggregateDevicePropertyFullSubDeviceList -> CFArray;
        active_sub_device_list => kAudioAggregateDevicePropertyActiveSubDeviceList -> [AudioObject];
        composition => kAudioAggregateDevicePropertyComposition -> CFDictionary;
        master_sub_device => kAudioAggregateDevicePropertyMasterSubDevice -> CFString;
        clock_device => kAudioAggregateDevicePropertyClockDevice -> CFString;
    }

    setters! {
        set_full_sub_device_list => kAudioAggregateDevicePropertyFullSubDeviceList(CFArray);
        set_master_sub_device => kAudioAggregateDevicePropertyMasterSubDevice(CFString);
        set_clock_device => kAudioAggregateDevicePropertyClockDevice(CFString);
    }
}

//==================================================================================================
// AudioSubDevice

#[repr(u32)]
pub enum AudioSubDeviceDriftCompensation {
    MinQuality = ffi::kAudioSubDeviceDriftCompensationMinQuality,
    LowQuality = ffi::kAudioSubDeviceDriftCompensationLowQuality,
    MediumQuality = ffi::kAudioSubDeviceDriftCompensationMediumQuality,
    HighQuality = ffi::kAudioSubDeviceDriftCompensationHighQuality,
    MaxQuality = ffi::kAudioSubDeviceDriftCompensationMaxQuality,
}

audio_object! {
    struct AudioSubDevice: kAudioSubDeviceClassID {}
}
audio_object_is_a!(AudioSubDevice, AudioDevice);

impl AudioSubDevice {
    pub const UID_KEY: &'static str = ffi::kAudioSubDeviceUIDKey;
    pub const NAME_KEY: &'static str = ffi::kAudioSubDeviceNameKey;
    pub const INPUT_CHANNELS_KEY: &'static str = ffi::kAudioSubDeviceInputChannelsKey;
    pub const OUTPUT_CHANNELS_KEY: &'static str = ffi::kAudioSubDeviceOutputChannelsKey;
    pub const EXTRA_INPUT_LATENCY_KEY: &'static str = ffi::kAudioSubDeviceExtraInputLatencyKey;
    pub const EXTRA_OUTPUT_LATENCY_KEY: &'static str = ffi::kAudioSubDeviceExtraOutputLatencyKey;
    pub const DRIFT_COMPENSATION_KEY: &'static str = ffi::kAudioSubDeviceDriftCompensationKey;
    pub const DRIFT_COMPENSATION_QUALITY_KEY: &'static str =
        ffi::kAudioSubDeviceDriftCompensationQualityKey;

    getters! {
        extra_latency => kAudioSubDevicePropertyExtraLatency -> f64;
        drift_compensation => kAudioSubDevicePropertyDriftCompensation -> int_bool;
        drift_compensation_quality =>
            kAudioSubDevicePropertyDriftCompensationQuality -> AudioSubDeviceDriftCompensation;
    }

    setters! {
        set_extra_latency => kAudioSubDevicePropertyExtraLatency(f64);
        set_drift_compensation => kAudioSubDevicePropertyDriftCompensation(int_bool);
        set_compensation_quality =>
            kAudioSubDevicePropertyDriftCompensationQuality(AudioSubDeviceDriftCompensation);
    }
}
