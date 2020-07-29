use std::fmt;

/// Represents the days of the week.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Day {
    Sun,
    Mon,
    Tue,
    Wed,
    Thu,
    Fri,
    Sat,
}

/// Allows the day to be displayed by the bot.
impl fmt::Display for Day {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// Used for converting a u32 into the corresponding day of the week.
pub fn num_to_day(num: u32) -> Option<Day> {
    match num {
        0 => Some(Day::Sun),
        1 => Some(Day::Mon),
        2 => Some(Day::Tue),
        3 => Some(Day::Wed),
        4 => Some(Day::Thu),
        5 => Some(Day::Fri),
        6 => Some(Day::Sat),
        _ => None,
    }
}
