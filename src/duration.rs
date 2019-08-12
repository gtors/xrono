use std::cmp::Ordering;
use crate::units::{PreciseTime, TimeInt};
use crate::constants::*;

/// ISO 8601 time duration with picosecond precision.
#[derive(Debug,Clone,Copy,PartialEq,Eq)]
pub struct Duration {
    secs: TimeInt,
    picos: TimeInt,
}

impl Duration {

    /// Create a duration with the specified number of weeks
    /// without respect of Daylight Savings.
    fn from_weeks(n: TimeInt) -> Self {
        Self {
            secs: n * SECS_PER_WEEK,
            picos: 0,
        }
    }

    /// Create a duration with the specified number of days
    /// without respect of Daylight Savings.
    fn from_days(n: TimeInt) -> Self {
        Self {
            secs: n * SECS_PER_DAY,
            picos: 0,
        }
    }

    /// Create a duration with the specified number of hours.
    fn from_hours(n: TimeInt) -> Self {
        Self {
            secs: n * SECS_PER_HOUR,
            picos: 0,
        }
    }

    /// Create a duration with the specified number of minutes.
    fn from_minutes(n: TimeInt) -> Self {
        Self {
            secs: n * SECS_PER_MINUTE,
            picos: 0,
        }
    }

    /// Create a duration with the specified number of seconds.
    fn from_secs(n: TimeInt) -> Self {
        Self {
            secs: n,
            picos: 0,
        }
    }

    /// Create a duration with the specified number of milliseconds.
    fn from_millis(n: TimeInt) -> Self {
        Self {
            secs: n / MILLIS_PER_SEC,
            picos: (n % MILLIS_PER_SEC) * PICOS_PER_MILLI,
        }
    }

    /// Create a duration with the specified number of microseconds.
    fn from_micros(n: TimeInt) -> Self {
        Self {
            secs: n / MICROS_PER_SEC,
            picos: (n % MICROS_PER_SEC) * PICOS_PER_MICRO,
        }
    }

    /// Create a duration with the specified number of nanoseconds.
    fn from_nanos(n: TimeInt) -> Self {
        Self {
            secs: n / NANOS_PER_SEC,
            picos: (n % NANOS_PER_SEC) * PICOS_PER_NANO,
        }
    }

    /// Create a duration with the specified number of picoseconds.
    fn from_picos(n: TimeInt) -> Self {
        Self {
            secs: n / PICOS_PER_SEC,
            picos: n % PICOS_PER_SEC,
        }
    }

    /// Gets the length of this duration in weeks assuming that there are the
    /// standard number of seconds in a week without respect of Daylight Savings
    fn as_weeks(&self) -> TimeInt {
        self.secs / SECS_PER_WEEK
    }

    /// Gets the length of this duration in days assuming that there are the
    /// standard number of seconds in a day without respect of Daylight Savings
    fn as_days(&self) -> TimeInt {
        self.secs / SECS_PER_DAY
    }

    /// Gets the length of this duration in hours.
    fn as_hours(&self) -> TimeInt {
        self.secs / SECS_PER_HOUR
    }

    /// Gets the length of this duration in minutes.
    fn as_minutes(&self) -> TimeInt {
        self.secs / SECS_PER_MINUTE
    }

    /// Gets the length of this duration in seconds.
    fn as_secs(&self) -> TimeInt {
        self.secs
    }

    /// Gets the length of this duration in milliseconds.
    fn as_millis(&self) -> TimeInt {
        (self.secs * MILLIS_PER_SEC) + (self.picos / PICOS_PER_MILLI)
    }

    /// Gets the length of this duration in microseconds.
    fn as_micros(&self) -> TimeInt {
        (self.secs * MICROS_PER_SEC) + (self.picos / PICOS_PER_MICRO)
    }
    /// Gets the length of this duration in nanoseconds.
    fn as_nanos(&self) -> TimeInt {
        (self.secs * NANOS_PER_SEC) + (self.picos / PICOS_PER_NANO)
    }

    /// Gets the length of this duration in picoseconds.
    fn as_picos(&self) -> TimeInt {
        self.secs * PICOS_PER_SEC + self.picos
    }

}

impl Ord for Duration {
    fn cmp(&self, other: &Self) -> Ordering {
        self.secs.cmp(other.secs).then(self.picos.cmp(other.picos))
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
            picos: dur.subsec_nanos() * PICOS_PER_NANO,
        }
    }
}

impl Add for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let picos = self.picos + other.picos;
        let (picos, picos_secs) = calc_picos_secs(picos);

        Self {
            secs: self.secs + other.secs + picos_secs,
            picos: picos,
        }
    }
}

impl Add<PreciseTime> for Duration {
    type Output = Self;

    fn add(self, t: PreciseTime) -> Self {
        self + Self::from(t)
    }
}

impl Sub for Duration {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        let picos = self.picos - other.picos;
        let (picos, picos_secs) = calc_picos_secs(picos);

        Self {
            secs: self.secs - other.secs - picos_secs,
            picos: picos,
        }
    }
}

impl Sub<PreciseTime> for Duration {
    type Output = Self;

    fn add(self, t: PreciseTime) -> Self {
        self - Self::from(t)
    }
}

impl <T: Into<TimeInt>> Mul<T> for Duration {
    type Output = Self;

    fn mul(self, rhs: T) -> Self {
        let scale: TimeInt = rhs.into();
        let picos = self.picos * scale;
        let secs = self.secs * scale;
        let (picos, picos_secs) = calc_picos_secs(picos);

        Self {
            secs: secs + picos_secs,
            picos: picos
        }
    }
}

impl Neg for Duration {
    type Output = Self;

    fn neg(self) -> Self {
        Self { secs: -self.secs, picos: -self.picos }
    }
}

#[inline]
fn calc_picos_secs(picos: TimeInt) -> (TimeInt, TimeInt) {
    if picos.abs() => PICOS_PER_SEC {
        picos_secs = picos / PICOS_PER_SEC;
        picos = picos % PICOS_PER_SEC;
        (picos, picos_secs)
    } else {
        (picos, 0)
    }
}
