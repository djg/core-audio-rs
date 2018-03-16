extern crate core_audio;

use core_audio::{audio_system_object, Result};
use std::error::Error;
use std::fmt::Debug;

fn print<T: Debug>(name: &str, t: Result<T>) {
    match t {
        Ok(t) => println!("{} = {:?}", name, t),
        Err(e) => println!("{} not found: {:?}", name, e.description()),
    }
}

fn main() {
    let mut aso = audio_system_object();

    println!("\nAudioSystemObject properties...");
    print("class", Ok(aso.class()));
    print("devices", aso.devices());
    print("default input device", aso.default_input_device());
    print("default_output_device", aso.default_output_device());
    print(
        "default system output device",
        aso.default_system_output_device(),
    );
    print("mix_stereo_to_mono", aso.mix_stereo_to_mono());
    print("plug_in_list", aso.plug_in_list());
    print("transport_manager_list", aso.transport_manager_list());
    // @constant       kAudioHardwarePropertyTranslateBundleIDToTransportManager
    print("box_list", aso.box_list());
    // @constant       kAudioHardwarePropertyTranslateUIDToBox
    print("clock_device_list", aso.clock_device_list());
    // @constant       kAudioHardwarePropertyTranslateUIDToClockDevice
    print("process_is_master", aso.process_is_master());
    print("is_initing_or_exiting", aso.is_initing_or_exiting());
    // @constant       kAudioHardwarePropertyUserIDChanged
    print("process_is_audible", aso.process_is_audible());
    print("sleeping_is_allowed", aso.sleeping_is_allowed());
    print("unloading_is_allowed", aso.unloading_is_allowed());
    print("hog_mode_is_allowed", aso.hog_mode_is_allowed());
    print("is_active_or_headless", aso.is_active_or_headless());
    print("power_hint", aso.power_hint());

    aso.set_sleeping_is_allowed(true);
    print("sleeping_is_allowed", aso.sleeping_is_allowed());
}
