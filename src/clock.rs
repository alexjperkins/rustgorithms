use std::cmp::Ordering;
use std::fmt;

#[derive(Debug, Default)]
pub struct Clock {
    hour: i32,
    minute: i32,
}

impl PartialEq for Clock {
    fn eq(&self, other: &Self) -> bool {
        if self.hour != other.hour {
            return false
        }
        if self.minute != other.minute {
            return false
        }

        return true
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

impl Clock {
    // Implement new to allow for negative hours & minutes.
    pub fn new(hour: i32, minute: i32) -> Self {
        let h = {
            if hour > 0 {
                hour % 24
            } else {
                24 - (hour.abs() % 24)
            }
        };

        let m = {
            if minute > 0 {
                minute % 60
            } else {
                60 - (minute.abs() % 60)
            }
        };

        Self {hour: h, minute: m}
    }

    // Implement add_minutes & allow for negative minutes.
    pub fn add_minutes(&self, minutes: i32) -> Self {
        let minute_boundary = 60;

        match (self.minute + minutes).abs().partial_cmp(&minute_boundary) {
            Some(o) => {
                match o {
                    Ordering::Equal => Self{hour: self.mod_hour(minutes), minute: 0},
                    Ordering::Less => Self{hour: self.hour, minute: self.minute + minutes},
                    Ordering::Greater => Self{
                        hour: self.mod_hour(minutes),
                        minute: self.mod_minute(minutes)
                    },
                }
            }
            _ => panic!("Failed to partially cmp"),
        }
    }

    fn mod_minute(&self, minutes: i32) -> i32 {
        if minutes > 0 {
            (self.minute + minutes) % 60
        } else {
            60 - ((self.minute + minutes).abs() % 60)
        }
    }

    fn mod_hour(&self,  minutes: i32) -> i32 {
        let minutes_carry_over: i32 = {
            if self.minute + (minutes.abs() % 60) > 59 {
                if minutes >= 0 {
                    1
                } else {
                    -1
                }
            } else {
                0
            }
        };

        if minutes > 0 {
            self.hour + ((minutes / 60) + minutes_carry_over) % 24
        } else {
            (self.hour + ((minutes / 60) + minutes_carry_over)).abs() % 24
        }
    }
}
