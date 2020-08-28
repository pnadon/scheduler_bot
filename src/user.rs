//===----------------------------------------------------------------------===//
// user.rs
//
// This source file is part of the scheduler_bot project
//
// Copyright (c) 2020 Philippe Nadon
// Licensed under Apache License v2.0
//===----------------------------------------------------------------------===//
use crate::day::*;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;

/// Represents a single user's schedule.
/// The schedule itself is stored in UTC time as an int (used as a bit vector).
/// eg. 0b010000000000000000000001 represents availability at 0 and 22.
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    name: String,
    timezone: i32,
    schedule: [u32; 7],
}

impl User {
    pub fn new(name: String) -> User {
        Self {
            name,
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
        // Schedule is shifted in 2 steps to maintain <24 hr changes
        // first shifts schedule to UTC
        // second shifts to new timezone
        self.schedule = shift_schedule(self.schedule, self.timezone);
        self.schedule = shift_schedule(self.schedule, -timezone);
        self.timezone = timezone;
    }

    /// Checks if the user is available at that time and day in that timezone.
    pub fn is_available(&self, day: Day, time: u32, timezone: i32) -> bool {
        let (day, time) = global_daytime(day, time, timezone);
        self.schedule[day as usize] & (1 << time) > 0
    }

    /// Returns a simple string representation of the user's schedule.
    pub fn disp_schedule(&self, time_as_row: bool, timezone: i32) -> String {
        let shift_schedule = shift_schedule(self.schedule, timezone);

        // If time_as_row is true, the days will be the columns,
        // otherwise they are the rows.
        match time_as_row {
            true => (0..24)
                .map(|bit| {
                    format!("{:0>2}", bit.to_string())
                        + ": "
                        + &(shift_schedule
                            .iter()
                            .map(move |times| match times & (1 << bit) {
                                0 => "░ ",
                                _ => "█ ",
                            })
                            .collect::<String>())
                        + "\n"
                })
                .collect::<String>(),
            _ => {
                "     012345678901234567890123\n".to_string()
                    + &(shift_schedule
                        .iter()
                        .enumerate()
                        .map(|times| {
                            num_to_day(times.0 as u32).unwrap().to_string()
                                + ": "
                                + &((0..24)
                                    .map(|bit| match times.1 & (1 << bit) {
                                        0 => '░',
                                        _ => '█',
                                    })
                                    .collect::<String>())
                                + "\n"
                        })
                        .collect::<String>())
            }
        }
    }

    /// Sets the time on the specified day to available or unavailable.
    pub fn set_time(&mut self, day: Day, time: u32, available: bool) {
        let (day, time) = global_daytime(day, time, self.timezone);
        match available {
            true => self.schedule[day as usize] |= 1 << time,
            false => self.schedule[day as usize] &= u32::max_value() - (1 << time),
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
        // we still want to iterate. Eg., from Fri to Tue.
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
        // we still want to iterate. Eg., from Fri to Tue.
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

    #[allow(dead_code)]
    pub fn get_raw_schedule(&self) -> [u32; 7] {
        self.schedule
    }

    #[allow(dead_code)]
    pub fn set_raw_schedule(&mut self, schedule: [u32; 7]) {
        self.schedule = schedule;
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

    (
        num_to_day(((day as u32) + day_shift) % 7).unwrap(),
        (new_time + 24) as u32 % 24,
    )
}

/// Converts the schedule (stored as UTC time) to match the specified timezone.
fn shift_schedule(schedule: [u32; 7], timezone: i32) -> [u32; 7] {
    let mut res = schedule;

    // Depending on if the timezone goes forward or backwards in time,
    // the operations may be reversed.
    // Bit shifts are done to move the bits which are now in another day.
    match timezone.cmp(&0) {
        Ordering::Greater => {
            for day in 0..7 {
                res[(day + 1) % 7] = ((schedule[(day + 1) % 7] << timezone) % (1 << 24))
                    | schedule[day] >> (24 - timezone) as u32;
            }
        }
        Ordering::Equal => (),
        Ordering::Less => {
            for day in 0..7 {
                let shift = -timezone as u32;
                res[(day + 6) % 7] = schedule[(day + 6) % 7] >> shift
                    | ((schedule[day] << (24 - shift)) % (1 << 24));
            }
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_timezone() {
        let ans = shift_schedule(
            [
                (1 << 3) + (1 << 4),
                (1 << 3) + (1 << 4) + (1 << 5) + (1 << 6),
                0,
                0,
                0,
                0,
                (1 << 3) + (1 << 4),
            ],
            -5,
        );
        for day in ans.iter() {
            println!("{:024b}", day);
        }
        assert_eq!(
            [
                (1 << 22) + (1 << 23),
                3,
                0,
                0,
                0,
                (1 << 22) + (1 << 23),
                (1 << 22) + (1 << 23)
            ],
            ans
        );
    }

    #[test]
    fn test_set_timezone() {
        let mut usr = User::new("bob".to_string());
        usr.set_timezone(2);
        usr.set_raw_schedule([1, 1 << 23, 0, 1, 0, 1 << 23, 1 << 5]);
        println!("{}", usr.disp_schedule(true, 2));
        usr.set_timezone(-1);
        println!("{}", usr.disp_schedule(true, -1));
        assert_eq!(usr.get_raw_schedule()[0], 8);
        assert_eq!(usr.get_raw_schedule()[1], 0);
        assert_eq!(usr.get_raw_schedule()[6], (1 << 8) + 4);
    }
}
