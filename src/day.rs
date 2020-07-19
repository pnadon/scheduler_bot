use std::fmt;

/// Represents the days of the week.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum Day {
    Sunday,
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
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
        0 => Some(Day::Sunday),
        1 => Some(Day::Monday),
        2 => Some(Day::Tuesday),
        3 => Some(Day::Wednesday),
        4 => Some(Day::Thursday),
        5 => Some(Day::Friday),
        6 => Some(Day::Saturday),
        _ => None,
    }
}
