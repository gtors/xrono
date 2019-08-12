pub type TimeInt = u64;

// Time units that can be easyly converted to seconds.
pub enum PreciseTime {
    Picoseconds(TimeInt),
    Nanoseconds(TimeInt),
    Microseconds(TimeInt),
    Milliseconds(TimeInt),
    Seconds(TimeInt),
    Minutes(TimeInt),
    Hours(TimeInt),
    Days(TimeInt),
    Weeks(TimeInt),
}

// Time units that can be converted to seconds only relative to some date.
pub enum RelativeTime {
    Months(TimeInt),
    Quartals(TimeInt),
    Halfs(TimeInt),
    Years(TimeInt),
    Centuries(TimeInt),
    Millenniums(TimeInt),
}

pub enum Unit {
    PreciseTime(PreciseTime),
    RelativeTime(RelativeTime),
    Duration(Duration),
}
