use std::os::raw::c_double;

extern "C" {
    pub fn AudioGetCurrentHostTime() -> u64;
    pub fn AudioGetHostClockFrequency() -> c_double;
    pub fn AudioGetHostClockMinimumTimeDelta() -> u32;
    pub fn AudioConvertHostTimeToNanos(inHostTime: u64) -> u64;
    pub fn AudioConvertNanosToHostTime(inNanos: u64) -> u64;
}
