extern "C" {
    pub fn AudioGetCurrentHostTime() -> u64;
    pub fn AudioGetHostClockFrequency() -> f64;
    pub fn AudioGetHostClockMinimumTimeDelta() -> u32;
    pub fn AudioConvertHostTimeToNanos(inHostTime: u64) -> u64;
    pub fn AudioConvertNanosToHostTime(inNanos: u64) -> u64;
}
