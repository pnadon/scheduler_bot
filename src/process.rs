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
                        "```\n".to_string()
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
                    Ok(Some(schedule.available_to_string(
                        day_vec[0],
                        time_vec[0],
                        usr.timezone(),
                    )))
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
                        schedule.available_day_to_string(day_vec[0], usr.timezone()),
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
        Ok(Some(usr.name().to_string()))
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
            "```\n".to_string() + &usr.disp_schedule(true, usr.timezone()) + "```",
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
