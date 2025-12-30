use std::fmt::Display;

#[derive(Debug, PartialEq)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}
impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let target_minutes = minutes.rem_euclid(60);
        let target_hours = (hours + minutes.div_euclid(60)).rem_euclid(24);

        Clock {
            hours: target_hours,
            minutes: target_minutes,
        }
    }

    pub fn add_minutes(self, minutes: i32) -> Self {
        let target_minutes = (self.minutes + minutes).rem_euclid(60);
        let target_hours = (self.hours + (self.minutes + minutes).div_euclid(60)).rem_euclid(24);

        Clock {
            hours: target_hours,
            minutes: target_minutes,
        }
    }
}
