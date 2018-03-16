extern crate ctest;

//use std::env;

fn main() {
    // for (k, v) in env::vars_os() {
    //     println!("{:?}={:?}", k, v);
    // }

    let mut cfg = ctest::TestGenerator::new();
    cfg.header("/System/Library/Frameworks/CoreAudio.framework/Headers/CoreAudio.h");
    cfg.skip_signededness(|s| match s {
        s if s.starts_with("CF") && s.ends_with("Ref") => true,
        s if s.ends_with("Proc") => true,
        "AudioDeviceIOProcID" => true,
        _ => false,
    });
    cfg.generate("../core-audio-sys/src/lib.rs", "all.rs");
}
