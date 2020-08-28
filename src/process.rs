//===----------------------------------------------------------------------===//
// process.rs
//
// This source file is part of the scheduler_bot project
//
// Copyright (c) 2020 Philippe Nadon
// Licensed under Apache License v2.0
//===----------------------------------------------------------------------===//
use crate::parse::{ParamType, ParamVals};
use crate::schedules::ScheduleCollection;

/// Processes the extracted tokens from the user's query.
/// Mostly serves to route tokens to their corresponding process
/// based on the type of parameter and the number of values passed.
pub fn process(
    schedule: &mut ScheduleCollection,
    user_name: &str,
    p_type: ParamType,
    vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
    println!(">Processing: {:?}  {:?}", p_type, vals);
    match (p_type, vals.len()) {
        (ParamType::TimeZone, 1) => process_set_timezone(schedule, user_name, vals),
        (ParamType::Name, 1) => process_set_name(schedule, user_name, vals),
        (ParamType::RemoveSchedule, 2) | (ParamType::AddSchedule, 2) => {
            process_set_schedule(schedule, user_name, p_type, vals)
        }
        (ParamType::ViewSchedule, 1) => process_view_user_schedule(schedule, user_name, vals),
        (ParamType::Available, 2) => process_available_day_time(schedule, user_name, vals),
        (ParamType::Available, 1) => process_available_day(schedule, user_name, vals),
        (ParamType::TimeZone, 0) => process_view_timezone(schedule, user_name),
        (ParamType::Name, 0) => process_view_name(schedule, user_name),
        (ParamType::ViewSchedule, 0) => process_view_schedule(schedule, user_name),
        (ParamType::Meme, 0) => process_post_meme(),
        (ParamType::Help, 0) => process_view_help(),
        (_, _) => Err("Incorrect param type and/or param value"),
    }
}

/// Sets the user's timezone.
fn process_set_timezone(
    schedule: &mut ScheduleCollection,
    user_name: &str,
    vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
    if let Some(usr) = schedule.mut_user(user_name) {
        match &vals[0] {
            ParamVals::TimeZone(timezone) => {
                usr.set_timezone(*timezone);
                Ok(None)
            }
            _ => Err("Incorrect timezone params"),
        }
    } else {
        Err("Could not find user")
    }
}

/// Sets the user's name.
fn process_set_name(
    schedule: &mut ScheduleCollection,
    user_name: &str,
    vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
    if let Some(usr) = schedule.mut_user(user_name) {
        match &vals[0] {
            ParamVals::Name(name) => {
                usr.set_name(name.to_string());
                Ok(None)
            }
            _ => Err("Incorrect name params"),
        }
    } else {
        Err("Could not find user")
    }
}

/// Sets the user's schedule.
fn process_set_schedule(
    schedule: &mut ScheduleCollection,
    user_name: &str,
    p_type: ParamType,
    vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
    if let Some(usr) = schedule.mut_user(user_name) {
        let available = p_type == ParamType::AddSchedule;
        match (&vals[0], &vals[1]) {
            (ParamVals::DayCollection(day_vec), ParamVals::TimeCollection(time_vec)) => {
                day_vec.iter().for_each(|day| {
                    time_vec
                        .iter()
                        .for_each(|time| usr.set_time(*day, *time, available))
                });
                Ok(None)
            }
            (ParamVals::DayCollection(day_vec), ParamVals::TimeRange(start_time, end_time)) => {
                day_vec
                    .iter()
                    .for_each(|day| usr.set_time_range(*day, *start_time, *end_time, available));
                Ok(None)
            }
            (ParamVals::DayRange(start_day, end_day), ParamVals::TimeCollection(time_vec)) => {
                time_vec
                    .iter()
                    .for_each(|time| usr.set_day_range(*start_day, *end_day, *time, available));
                Ok(None)
            }
            (
                ParamVals::DayRange(start_day, end_day),
                ParamVals::TimeRange(start_time, end_time),
            ) => {
                usr.set_day_time_range(*start_day, *end_day, *start_time, *end_time, available);
                Ok(None)
            }
            _ => Err("Incorrect params"),
        }
    } else {
        Err("Could not find user")
    }
}

/// Lookup another user's schedule.
fn process_view_user_schedule(
    schedule: &mut ScheduleCollection,
    user_name: &str,
    vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
    match &vals[0] {
        ParamVals::ViewId(id) => {
            if let Some(usr) = schedule.user(user_name) {
                if let Some(lookup_usr) = schedule.user(id) {
                    Ok(Some(
                        "```\nTimezone:".to_string()
                            + &lookup_usr.timezone().to_string()
                            + "\n"
                            + &lookup_usr.disp_schedule(true, usr.timezone())
                            + "```",
                    ))
                } else {
                    Err("Could not lookup other user")
                }
            } else {
                Err("User does not exist")
            }
        }
        _ => Err("Incorrect params"),
    }
}

/// Check who is available at that day and time.
fn process_available_day_time(
    schedule: &mut ScheduleCollection,
    user_name: &str,
    vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
    match (&vals[0], &vals[1]) {
        (ParamVals::DayCollection(day_vec), ParamVals::TimeCollection(time_vec)) => {
            if let Some(usr) = schedule.user(user_name) {
                if day_vec.len() == 1 && time_vec.len() == 1 {
                    Ok(Some(
                        "Timezone:".to_string()
                            + &usr.timezone().to_string()
                            + "\n"
                            + &schedule.available_to_string(
                                day_vec[0],
                                time_vec[0],
                                usr.timezone(),
                            ),
                    ))
                } else {
                    Err("Too many dates")
                }
            } else {
                Err("User does not exist")
            }
        }
        _ => Err("Incorrect params"),
    }
}

/// Check who is available during that day.
fn process_available_day(
    schedule: &mut ScheduleCollection,
    user_name: &str,
    vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
    match &vals[0] {
        ParamVals::DayCollection(day_vec) => {
            if let Some(usr) = schedule.user(user_name) {
                if day_vec.len() == 1 {
                    Ok(Some(
                        "Timezone:".to_string()
                            + &usr.timezone().to_string()
                            + "\n"
                            + &schedule.available_day_to_string(day_vec[0], usr.timezone()),
                    ))
                } else {
                    Err("Too many dates")
                }
            } else {
                Err("User does not exist")
            }
        }
        _ => Err("Incorrect params"),
    }
}

/// View the user's timezone.
fn process_view_timezone(
    schedule: &mut ScheduleCollection,
    user_name: &str,
) -> Result<Option<String>, &'static str> {
    if let Some(usr) = schedule.user(user_name) {
        Ok(Some(usr.timezone().to_string()))
    } else {
        Err("Could not find user")
    }
}

/// View the user's name.
fn process_view_name(
    schedule: &mut ScheduleCollection,
    user_name: &str,
) -> Result<Option<String>, &'static str> {
    if let Some(usr) = schedule.user(user_name) {
        Ok(Some(usr.name()))
    } else {
        Err("Could not find user")
    }
}

/// View the user's schedule.
fn process_view_schedule(
    schedule: &mut ScheduleCollection,
    user_name: &str,
) -> Result<Option<String>, &'static str> {
    if let Some(usr) = schedule.user(user_name) {
        Ok(Some(
            "```\n".to_string()
                + &"Timezone:".to_string()
                + &usr.timezone().to_string()
                + "\n"
                + &usr.disp_schedule(true, usr.timezone())
                + "```",
        ))
    } else {
        Err("Could not find user")
    }
}

/// Post a meme.
fn process_post_meme() -> Result<Option<String>, &'static str> {
    Ok(Some(
        "https://i.postimg.cc/hvJh0k40/showtime.png".to_string() + "\nIt's showtime",
    ))
}

/// Displays the help info
pub fn process_view_help() -> Result<Option<String>, &'static str> {
    Ok(Some(format!(
        "
Help:\n
\n
Types of inputs to commands:\n
- time can be any from 0 to 23 (inclusive)\n
- Day can by any from sun to sat (inclusive)\n
- you can also use 'weekends' or 'weekdays' where Day(s) applies.\n
- timezone can be from -23 to 23\n
- user is a discord tag, excluding the '#', case-insensitive, eg. 3ntity2051\n
- name is anything, although it will be converted to alphanumeric lowercase\n
\n
Notation:\n
- <...> represents values (eg. <time> can be 0, 2, 18...)\n
- <add or remove> means you can use either add or remove.\n
\n
{pref}<add or remove>\n
- add adds certain days and times as available\n
- remove makes certain days and times as unavailable\n
    {pref}<add or remove> from <Day> to <Day> from <time> to <time>\n
    {pref}<add or remove> <Day(s)> from <time> to <time>\n
    {pref}<add or remove> from <Day> to <Day> <time(s)>\n
    {pref}<add or remove> <Day(s)> <time(s)>\n
    - eg. {pref}add from mon to thu from 1 to 5\n
    - eg. {pref}remove mon wed fri from 4 to 7\n
    - eg. {pref}add from weekdays 1 5 18\n
\n
{pref}name <name>\n
- set your name, eg. {pref}name philio\n
{pref}name\n
- view your name\n
\n
{pref}timezone <timezone>\n
- set your timezone, eg. {pref}timezone -7\n
{pref}timezone\n
- view your timezone\n
\n
{pref}view <user>\n
- view the user's schedule, eg. {pref}view 3ntity2051\n
{pref}view\n
- view your own schedule\n
\n
{pref}available <Day> <time>\n
- see who is available on that day and time, eg. {pref}available mon 15\n
{pref}available <Day>\n
- see who is available on that day, eg. {pref}available fri\n
\n
{pref}showtime\n
- try it yourself!\n
{pref}help\n
- this message\n
",
        pref = "?"
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::day::Day;

    #[test]
    fn test_schedules() {
        let mut schedule = ScheduleCollection::new();
        schedule.insert_user(123, "bob");
        schedule.add_name_id("bob", 123).unwrap();
        process_set_timezone(&mut schedule, "bob", vec![ParamVals::TimeZone(-5)]).unwrap();
        process_set_schedule(
            &mut schedule,
            "bob",
            ParamType::AddSchedule,
            vec![
                ParamVals::DayRange(Day::Sat, Day::Sun),
                ParamVals::TimeRange(22, 23),
            ],
        )
        .unwrap();
        process_set_schedule(
            &mut schedule,
            "bob",
            ParamType::AddSchedule,
            vec![
                ParamVals::DayCollection(vec![Day::Fri]),
                ParamVals::TimeRange(22, 23),
            ],
        )
        .unwrap();
        process_set_schedule(
            &mut schedule,
            "bob",
            ParamType::AddSchedule,
            vec![
                ParamVals::DayCollection(vec![Day::Mon]),
                ParamVals::TimeRange(0, 1),
            ],
        )
        .unwrap();
        println!(
            "Schedule:\n{}",
            schedule.user("bob").unwrap().disp_schedule(false, -5)
        );
        let usr_schedule = schedule.user("bob").unwrap().get_raw_schedule();
        println!(
            "Raw schedule:\n{:024b}\n{:024b}\n{:024b}\n{:024b}\n{:024b}\n{:024b}\n{:024b}",
            usr_schedule[0],
            usr_schedule[1],
            usr_schedule[2],
            usr_schedule[3],
            usr_schedule[4],
            usr_schedule[5],
            usr_schedule[6]
        );
        assert_eq!((1 << 3) + (1 << 4), usr_schedule[6]);
    }
}
