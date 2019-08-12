use std::cmp::Ordering;
use crate::units::{PreciseTime};
use crate::constants::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Duration {
    secs: u64,
    picos: u64,
}

impl Duration {
    fn new(secs: u64, nanos: u32) {
        Self { secs, nanos }
    }
}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.secs == other.secs {
            self.picos.cmp(other.picos)
        } else {
            self.secs.cmp(other.secs)
        }
    }
}

impl PartialOrd for Duration {
    fn cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<PreciseTime> for Duration {
    fn from(tu: PreciseTime) -> Self {
        use crate::units::TimeUnit::*;
        match tu {
            Picoseconds(n) => Self {
                secs: n / PICOS_PER_SEC,
                picos: n % PICOS_PER_SEC
            }
            Nanoseconds(n) => Self {
                secs: n / NANOS_PER_SEC,
                picos: (n % NANOS_PER_SEC) * PICOS_PER_NANO
            },
            Microseconds(n) => Self {
                secs: n / MICROS_PER_SEC,
                picos: (n % MICROS_PER_SEC) * PICOS_PER_MICRO
            },
            Milliseconds(n) => Self {
                secs: n / MILLIS_PER_SEC,
                picos: (n % MILLIS_PER_SEC) * PICOS_PER_MILLI
            },
            Seconds(n) => Self {
                secs: n,
                picos: 0
            },
            Minutes(n) => Self {
                secs: n * SECS_PER_MINUTE,
                picos: 0
            },
            Hours(n) => Self {
                secs: n * SECS_PER_HOUR,
                picos: 0
            },
            Days(n) => Self {
                secs: n * SECS_PER_DAY,
                picos: 0
            },
            Weeks(n) => Self {
                secs: n * SECS_PER_WEEK,
                picos: 0
            },
        }
    }
}

impl From<std::time::Duration> for Duration {
    fn from(dur: std::time::Duration) -> Self {
        Self {
            secs: dur.as_secs(),
            picos: dur.subsec_nanos() * PICOS_PER_NANO
        }
    }
}
