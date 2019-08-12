use std::cmp::Ordering;
use crate::units::{PreciseTime};
use crate::constants::*;

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Duration {
    secs: u64,
    picos: u64,
}

impl Duration {

    /// Creates a duration from the specified number of whole seconds and
    /// additional picoseconds.
    fn new(secs: u64, picos: u64) {
        Self { secs, picos }
    }

    /// Create a duration with the specified number of weeks.
    fn from_weeks(n: u64) -> Self {
        Self {
            secs: n * SECS_PER_WEEK,
            picos: 0
        }
    }

    /// Create a duration with the specified number of days.
    fn from_days(n: u64) -> Self {
        Self {
            secs: n * SECS_PER_DAY,
            picos: 0
        }
    }

    /// Create a duration with the specified number of hours.
    fn from_hours(n: u64) -> Self {
        Self {
            secs: n * SECS_PER_HOUR,
            picos: 0
        }
    }

    /// Create a duration with the specified number of minutes.
    fn from_minutes(n: u64) -> Self {
        Self {
            secs: n * SECS_PER_MINUTE,
            picos: 0
        }
    }

    /// Create a duration with the specified number of seconds.
    fn from_secs(n: u64) -> Self {
        Self {
            secs: n,
            picos: 0
        }
    }

    /// Create a duration with the specified number of milliseconds.
    fn from_millis(n: u128) -> Self {
        Self {
            secs: n / MILLIS_PER_SEC,
            picos: (n % MILLIS_PER_SEC) * PICOS_PER_MILLI
        }
    }

    /// Create a duration with the specified number of microseconds.
    fn from_micros(n: u128) -> Self {
        Self {
            secs: n / MICROS_PER_SEC,
            picos: (n % MICROS_PER_SEC) * PICOS_PER_MICRO
        }
    }

    /// Create a duration with the specified number of nanoseconds.
    fn from_nanos(n: u128) -> Self {
        Self {
            secs: n / NANOS_PER_SEC,
            picos: (n % NANOS_PER_SEC) * PICOS_PER_NANO
        }
    }

    /// Create a duration with the specified number of picoseconds.
    fn from_picos(n: u128) -> Self {
        Self {
            secs: n / PICOS_PER_SEC,
            picos: n % PICOS_PER_SEC
        }
    }

    /// Gets the length of this duration in weeks assuming that there are the
    /// standard number of seconds in a week without respect of Daylight Savings
    fn as_weeks(&self) -> u64 {
        self.secs / SECS_PER_WEEK
    }

    /// Gets the length of this duration in days assuming that there are the
    /// standard number of seconds in a day without respect of Daylight Savings
    fn as_days(&self) -> u64 {
        self.secs / SECS_PER_DAY
    }

    /// Gets the length of this duration in hours.
    fn as_hours(&self) -> u64 {
        self.secs / SECS_PER_HOUR
    }

    /// Gets the length of this duration in minutes.
    fn as_minutes(&self) -> u64 {
        self.secs / SECS_PER_MINUTE
    }

    /// Gets the length of this duration in seconds.
    fn as_secs(&self) -> u64 {
        self.secs
    }

    /// Gets the length of this duration in milliseconds.
    fn as_millis(&self) -> u128 {
        (self.secs as u128 * MILLIS_PER_SEC) + (self.picos / PICOS_PER_MILLI)
    }

    /// Gets the length of this duration in microseconds.
    fn as_micros(&self) -> u128 {
        (self.secs as u128 * MICROS_PER_SEC) + (self.picos / PICOS_PER_MICRO)
    }
    /// Gets the length of this duration in nanoseconds.
    fn as_nanos(&self) -> u128 {
        (self.secs as u128 * NANOS_PER_SEC) + (self.picos / PICOS_PER_NANO)
    }

    /// Gets the length of this duration in picoseconds.
    fn as_picos(&self) -> u128 {
        self.secs as u128 * PICOS_PER_SEC + self.picos
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
    fn from(t: PreciseTime) -> Self {
        use crate::units::TimeUnit::*;
        match t {
            Picoseconds(n) => Self::from_picos(n),
            Nanoseconds(n) => Self::from_nanos(n),
            Microseconds(n) => Self::from_micros(n),
            Milliseconds(n) => Self::from_millis(n),
            Seconds(n) => Self::from_secs(n),
            Minutes(n) => Self::from_minutes(n),
            Hours(n) => Self::from_hours(n),
            Days(n) => Self::from_days(n),
            Weeks(n) => Self::from_weeks(n)
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

impl Into<std::time::Duration> for Duration {
    fn into(&self) -> std::time::Duration {
        std::time::Duration::new(self.secs, (self.picos / PICOS_PER_NANO) as u32)
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let mut picos = self.picos + other.picos;
        let mut picos_secs = 0;

        if picos => PICOS_PER_SEC {
            picos_secs = picos / PICOS_PER_SEC;
            picos = picos % PICOS_PER_SEC;
        }

        Self {
            secs: self.secs + other.secs + picos_secs,
            picos: picos
        }
    }
}

impl Add<PreciseTime> for Duration {
    type Output = Self;

    fn add(self, t: PreciseTime) -> Self {
        self + Self::from(t)
    }
}
