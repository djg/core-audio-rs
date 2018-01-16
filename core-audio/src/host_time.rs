use ffi;
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::time::Duration;

const NSEC_PER_SEC: u64 = 1_000_000_000;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct HostTime {
    t: u64,
}

impl HostTime {
    pub fn now() -> Self { HostTime { t: unsafe { ffi::AudioGetCurrentHostTime() }, } }

    pub fn clock_frequency() -> f64 { unsafe { ffi::AudioGetHostClockFrequency() } }

    pub fn clock_minimum_time_delta() -> u32 {
        unsafe { ffi::AudioGetHostClockMinimumTimeDelta() }
    }

    fn sub_hosttime(&self,
                    other: &HostTime)
                    -> Duration
    {
        let diff = self.t.checked_sub(other.t)
                       .expect("second HostTime is later than self");
        let nanos = unsafe { ffi::AudioConvertHostTimeToNanos(diff) };
        Duration::new(nanos / NSEC_PER_SEC, (nanos % NSEC_PER_SEC) as u32)
    }

    fn add_duration(&self,
                    other: &Duration)
                    -> HostTime
    {
        HostTime { t: self.t.checked_add(dur2hosttime(other))
                          .expect("overflow when adding Duration to HostTime"), }
    }

    fn sub_duration(&self,
                    other: &Duration)
                    -> HostTime
    {
        HostTime { t: self.t.checked_sub(dur2hosttime(other))
                          .expect("overflow when subtracting Duration from HostTime"), }
    }

    pub fn duration_since(&self,
                          earlier: HostTime)
                          -> Duration
    {
        self.sub_hosttime(&earlier)
    }

    pub fn elapsed(&self) -> Duration { HostTime::now() - *self }
}

impl Add<Duration> for HostTime {
    type Output = HostTime;

    fn add(self,
           other: Duration)
           -> HostTime
    {
        self.add_duration(&other)
    }
}

impl Sub<Duration> for HostTime {
    type Output = HostTime;

    fn sub(self,
           other: Duration)
           -> HostTime
    {
        self.sub_duration(&other)
    }
}

impl AddAssign<Duration> for HostTime {
    fn add_assign(&mut self,
                  other: Duration)
    {
        *self = *self + other;
    }
}

impl SubAssign<Duration> for HostTime {
    fn sub_assign(&mut self,
                  other: Duration)
    {
        *self = *self - other;
    }
}

impl Sub<HostTime> for HostTime {
    type Output = Duration;

    fn sub(self,
           other: HostTime)
           -> Duration
    {
        self.duration_since(other)
    }
}

fn dur2hosttime(dur: &Duration) -> u64 {
    let nanos = dur.as_secs().checked_mul(NSEC_PER_SEC)
                   .and_then(|nanos| nanos.checked_add(u64::from(dur.subsec_nanos())))
                   .expect("overflow converting duration to nanoseconds");
    unsafe { ffi::AudioConvertNanosToHostTime(nanos) }
}
