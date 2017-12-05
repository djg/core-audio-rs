extern crate core_audio;

use core_audio::{AudioDevice, audio_object_iterator, audio_system_object, Result};
use std::error::Error;
use std::fmt::Debug;

fn print<T: Debug>(name: &str, t: Result<T>) {
    match t {
        Ok(t) => println!("{} = {:?}", name, t),
        Err(e) => println!("{} not found: {:?}", name, e.description())
    }
}

fn main() {
    let devices = audio_system_object().devices().unwrap();
    for dod in audio_object_iterator(devices) {
        if let Some(dod) = dod.downcast_ref::<AudioDevice>() {
            println!("\nDefault Output Device properties...");
            print("base_class", dod.base_class());
            print("class", dod.class());
            print("owner", dod.owner());
            print("name", dod.name());
            print("model_name", dod.model_name());
            print("manufacturer", dod.manufacturer());
            print("element_name", dod.element_name());
            print("element_category_name", dod.element_category_name());
            print("element_number_name", dod.element_number_name());
            print("owned_objects", dod.owned_objects());
            print("identify", dod.identify());
            print("serial_number", dod.serial_number());
            print("firmware_version", dod.firmware_version());
            
            print("plug_in", dod.plug_in());
            print("device_is_running_somewhere", dod.device_is_running_somewhere());
            print("hog_mode", dod.hog_mode());
            print("buffer_frame_size", dod.buffer_frame_size());
            print("buffer_frame_size_range", dod.buffer_frame_size_range());
            print("uses_variable_buffer_frame_sizes", dod.uses_variable_buffer_frame_sizes());
            print("io_cycle_usage", dod.io_cycle_usage());
            print("stream_configuration", dod.stream_configuration());
            print("io_proc_stream_usage", dod.io_proc_stream_usage());
            print("actual_sample_rate", dod.actual_sample_rate());
            print("clock_device", dod.clock_device());
        }

    }
}
