use crate::day::Day;
use crate::schedules::ScheduleCollection;

#[derive(Clone, PartialEq, PartialOrd, Debug)]
pub enum ParamVals {
  TimeCollection(Vec<u32>),
  DayCollection(Vec<Day>),
  TimeRange(u32, u32),
  DayRange(Day, Day),
  Name(String),
  TimeZone(i32),
  ViewId(String),
}

#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum ParamType {
  TimeZone,
  Name,
  AddSchedule,
  RemoveSchedule,
  ViewSchedule,
}

pub fn to_params(input: &str) -> (Option<ParamType>, Option<Vec<ParamVals>>) {
  let params: Vec<String> = input
    .split(|chr| chr == ' ' || chr == ',')
    .map(|word| {
      word
        .chars()
        .filter(|chr| chr.is_ascii_alphanumeric())
        .collect::<String>()
    })
    .filter(|word| word.len() > 0)
    .map(|word| word.to_lowercase())
    .collect();

  parse_params(
    params.first().unwrap().as_str(),
    params[1..]
      .iter()
      .map(|word| word.as_str())
      .collect::<Vec<&str>>()
      .to_vec(),
  )
}

fn parse_params(
  param_type_str: &str,
  param_vals_str: Vec<&str>,
) -> (Option<ParamType>, Option<Vec<ParamVals>>) {
  if param_type_str.starts_with("add") {
    (Some(ParamType::AddSchedule), parse_schedule(param_vals_str))
  } else if param_type_str.starts_with("remove") {
    (
      Some(ParamType::RemoveSchedule),
      parse_schedule(param_vals_str),
    )
  } else if param_type_str.starts_with("name") {
    (Some(ParamType::Name), parse_name(param_vals_str))
  } else if param_type_str.starts_with("timezone") {
    (Some(ParamType::TimeZone), parse_timezone(param_vals_str))
  } else if param_type_str.starts_with("view") {
    (
      Some(ParamType::ViewSchedule),
      parse_schedule_id(param_vals_str),
    )
  } else {
    (None, None)
  }
}

fn parse_name(params: Vec<&str>) -> Option<Vec<ParamVals>> {
  if params.len() > 0 {
    Some(vec![ParamVals::Name(params.concat())])
  } else {
    Some(vec![])
  }
}

fn parse_schedule_id(params: Vec<&str>) -> Option<Vec<ParamVals>> {
  if params.len() > 4 {
    Some(vec![ParamVals::ViewId(
      params[..(params.len() - 4)].concat() + "#" + &params[(params.len() - 4)..].concat(),
    )])
  } else {
    Some(vec![])
  }
}

fn parse_timezone(params: Vec<&str>) -> Option<Vec<ParamVals>> {
  if params.len() > 0 {
    if let Ok(num) = params.first().unwrap().parse::<f64>() {
      let time_offset: i32 = num.log10().trunc() as i32;
      if time_offset > -24 && time_offset < 24 {
        return Some(vec![ParamVals::TimeZone(time_offset)]);
      }
    }
  } else {
    return Some(vec![]);
  }
  None
}

fn parse_schedule(params: Vec<&str>) -> Option<Vec<ParamVals>> {
  let mut params_iter = params.iter().peekable();
  let mut res: Vec<ParamVals> = vec![];
  let mut param = params_iter.next();
  if param.is_none() {
    return None;
  }
  if param.unwrap().starts_with("weekday") {
    res.push(ParamVals::DayRange(Day::Monday, Day::Friday));
    param = params_iter.next();
  } else if param.unwrap().starts_with("weekend") {
    res.push(ParamVals::DayRange(Day::Saturday, Day::Sunday));
    param = params_iter.next();
  } else if param.unwrap().starts_with("from")
    && params_iter.peek().is_some()
    && parse_day(params_iter.peek().unwrap()).is_ok()
  {
    match (params_iter.next(), params_iter.next(), params_iter.next()) {
      (Some(fst_input), Some(&"to"), Some(snd_input)) => {
        if let Ok(fst_day) = parse_day(fst_input) {
          if let Ok(snd_day) = parse_day(snd_input) {
            res.push(ParamVals::DayRange(fst_day, snd_day))
          } else {
            return None;
          }
        } else {
          return None;
        }
      }
      (_, _, _) => return None,
    }
    param = params_iter.next();
  } else if let Ok(fst_day) = parse_day(param.unwrap()) {
    let mut days = vec![fst_day];
    while params_iter.peek().is_some() && parse_day(params_iter.peek().unwrap()).is_ok() {
      days.push(parse_day(params_iter.next().unwrap()).unwrap());
    }
    res.push(ParamVals::DayCollection(days));
    param = params_iter.next();
  } else {
    res.push(ParamVals::DayRange(Day::Sunday, Day::Saturday));
  }
  if param.is_none() {
    return None;
  }
  if param.unwrap().starts_with("from") {
    match (params_iter.next(), params_iter.next(), params_iter.next()) {
      (Some(fst_input), Some(&"to"), Some(snd_input)) => {
        if let Ok(fst_num) = fst_input.parse::<u32>() {
          if let Ok(snd_num) = snd_input.parse::<u32>() {
            if fst_num < 24 && snd_num < 24 {
              res.push(ParamVals::TimeRange(fst_num, snd_num))
            } else {
              return None;
            }
          } else {
            return None;
          }
        } else {
          return None;
        }
      }
      (_, _, _) => return None,
    }
  } else if let Ok(fst_time) = param.unwrap().parse::<u32>() {
    let mut times = vec![fst_time];
    while params_iter.peek().is_some() && params_iter.peek().unwrap().parse::<u32>().is_ok() {
      times.push(params_iter.next().unwrap().parse::<u32>().unwrap());
    }
    res.push(ParamVals::TimeCollection(times));
  } else {
    return None;
  }
  Some(res)
}

fn parse_day(word: &str) -> Result<Day, &str> {
  let prefix_string = word.chars().take(3).collect::<String>();
  match &prefix_string[..] {
    "sun" => Ok(Day::Sunday),
    "mon" => Ok(Day::Monday),
    "tue" => Ok(Day::Tuesday),
    "wed" => Ok(Day::Wednesday),
    "thu" => Ok(Day::Thursday),
    "fri" => Ok(Day::Friday),
    "sat" => Ok(Day::Saturday),
    _ => Err("Invalid Date"),
  }
}

pub fn process(
  schedule: &mut ScheduleCollection,
  usr_id: String,
  p_type: ParamType,
  vals: Vec<ParamVals>,
) -> Result<Option<String>, &'static str> {
  match (p_type, vals.len()) {
    (ParamType::TimeZone, 1) => {
      if let Some(usr) = schedule.get_mut_user(usr_id) {
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
    (ParamType::Name, 1) => {
      if let Some(usr) = schedule.get_mut_user(usr_id) {
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
    (ParamType::RemoveSchedule, 2) | (ParamType::AddSchedule, 2) => {
      if let Some(usr) = schedule.get_mut_user(usr_id) {
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
    (ParamType::ViewSchedule, 1) => match &vals[0] {
      ParamVals::ViewId(id) => {
        if let Some(usr) = schedule.get_user(usr_id) {
          if let Some(lookup_usr) = schedule.get_user(id.to_string()) {
            Ok(Some(lookup_usr.disp_schedule(true, usr.timezone())))
          } else {
            Err("Could not lookup other user")
          }
        } else {
          Err("User does not exist")
        }
      }
      _ => Err("Incorrect params"),
    },
    (ParamType::TimeZone, 0) => {
      if let Some(usr) = schedule.get_user(usr_id) {
        Ok(Some(usr.timezone().to_string()))
      } else {
        Err("Could not find user")
      }
    }
    (ParamType::Name, 0) => {
      if let Some(usr) = schedule.get_user(usr_id) {
        Ok(Some(usr.name().to_string()))
      } else {
        Err("Could not find user")
      }
    }
    (ParamType::ViewSchedule, 0) => {
      if let Some(usr) = schedule.get_user(usr_id) {
        Ok(Some(usr.disp_schedule(true, usr.timezone())))
      } else {
        Err("Could not find user")
      }
    }
    (_, _) => Err("Incorrect param type and/or param value"),
  }
}

