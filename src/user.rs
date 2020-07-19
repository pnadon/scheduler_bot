use crate::day::*;

/// Represents a single user's schedule.
/// The schedule itself is stored in UTC time as an int (used as a bit vector).
/// eg. 0b010000000000000000000001 represents availability at 0 and 22.
#[derive(Debug)]
pub struct User {
    name: String,
    timezone: i32,
    schedule: [u32; 7],
}

impl User {
    pub fn new(name: String) -> User {
        Self {
            name: name,
            schedule: [0; 7],
            timezone: 0,
        }
    }

    /// Retrieves the user's name.
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Sets the user's name.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Retrieves the user's timezone.
    pub fn timezone(&self) -> i32 {
        self.timezone
    }

    /// Sets the user's timezone.
    pub fn set_timezone(&mut self, timezone: i32) {
        self.timezone = timezone;
    }

    /// Checks if the user is available at that time and day in that timezone.
    pub fn is_available(&self, day: Day, time: u32, timezone: i32) -> bool {
        let (day, time) = global_daytime(day, time, timezone);
        self.schedule[day as usize] & 2_u32.pow(time) > 0
    }

    /// Returns a simple string representation of the user's schedule.
    pub fn disp_schedule(&self, time_as_row: bool, timezone: i32) -> String {
        let local_schedule = local_schedule(self.schedule, timezone);

        // If time_as_row is true, the days will be the columns,
        // otherwise they are the rows.
        match time_as_row {
            true => (0..24)
                .map(|pwr| 2u32.pow(pwr))
                .map(|bit| {
                    local_schedule
                        .iter()
                        .map(move |times| match times & bit {
                            0 => '░',
                            _ => '█',
                        })
                        .collect::<String>()
                        + &'\n'.to_string()
                })
                .collect::<String>(),
            _ => local_schedule
                .iter()
                .map(|times| {
                    (0..24)
                        .map(|pwr| 2u32.pow(pwr))
                        .map(|bit| match times & bit {
                            0 => '░',
                            _ => '█',
                        })
                        .collect::<String>()
                        + &'\n'.to_string()
                })
                .collect::<String>(),
        }
    }

    /// Sets the time on the specified day to available or unavailable.
    pub fn set_time(&mut self, day: Day, time: u32, available: bool) {
        let (day, time) = global_daytime(day, time, self.timezone);
        match available {
            true => self.schedule[day as usize] |= 2u32.pow(time),
            false => self.schedule[day as usize] &= u32::max_value() - 2u32.pow(time),
        };
    }

    /// Sets the range of times (inclusive) on the specified day to available or
    /// unavailable.
    pub fn set_time_range(&mut self, day: Day, start_time: u32, end_time: u32, available: bool) {
        for time in start_time..=end_time {
            self.set_time(day, time, available);
        }
    }

    /// Sets the specified time in the range of days to available or unavailable.
    /// Ranges are inclusive.
    pub fn set_day_range(&mut self, start_day: Day, end_day: Day, time: u32, available: bool) {
        let end_num;

        // If the range is from a later day to an earlier day,
        // we still want to iterate. Eg., from Friday to Tuesday.
        if end_day < start_day {
            end_num = end_day as u32 + 7;
        } else {
            end_num = end_day as u32;
        }

        for day_num in (start_day as u32)..=(end_num) {
            self.set_time(num_to_day(day_num % 7).unwrap(), time, available);
        }
    }

    /// Sets the range of times and range of days to available or unavailable.
    /// Ranges are inclusive.
    pub fn set_day_time_range(
        &mut self,
        start_day: Day,
        end_day: Day,
        start_time: u32,
        end_time: u32,
        available: bool,
    ) {
        let end_num;

        // If the range is from a later day to an earlier day,
        // we still want to iterate. Eg., from Friday to Tuesday.
        if end_day < start_day {
            end_num = end_day as u32 + 7;
        } else {
            end_num = end_day as u32;
        }

        for day_num in (start_day as u32)..=(end_num) {
            self.set_time_range(
                num_to_day(day_num % 7).unwrap(),
                start_time,
                end_time,
                available,
            );
        }
    }
}

/// Converts the local day and time to UTC.
fn global_daytime(day: Day, time: u32, timezone: i32) -> (Day, u32) {
    let new_time = time as i32 - timezone;
    let day_shift: u32;
    if new_time < 0 {
        day_shift = 6;
    } else if new_time >= 24 {
        day_shift = 1;
    } else {
        day_shift = 0;
    }
    return (
        num_to_day(((day as u32) + day_shift) % 7).unwrap(),
        (new_time + 24) as u32 % 24,
    );
}

/// Converts the schedule (stored as UTC time) to match the specified timezone.
fn local_schedule(schedule: [u32; 7], timezone: i32) -> [u32; 7] {
    let mut res = schedule.clone();

    // Depending on if the timezone goes forward or backwards in time,
    // the operations may be reversed.
    // Bit shifts are done to move the bits which are now in another day.
    if timezone > 0 {
        for day in 0..7 {
            res[(day + 1) % 7] = ((schedule[(day + 1) % 7] << timezone) % 2u32.pow(24))
                | schedule[day] >> (24 - timezone) as u32;
        }
    } else if timezone < 0 {
        for day in 0..7 {
            res[(day + 1) % 7] = (schedule[(day + 1) % 7] >> -timezone) % 2u32.pow(24)
                | schedule[day] << (24 + timezone) as u32;
        }
    }
    res
}
