use crate::day::Day;

/// Tokens representing the type of query.
#[derive(Copy, Clone, PartialEq, PartialOrd, Debug)]
pub enum ParamType {
    TimeZone,
    Name,
    AddSchedule,
    RemoveSchedule,
    ViewSchedule,
    Available,
    Meme,
    Help,
}

/// Tokens representing the values passed to the user's query.
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

/// Transforms the raw text of the query into a cleaned list of params.
/// Examples include splitting by spaces and commands, and lowercasing input.
pub fn filter_query(input: &str) -> Vec<String> {
    input
        .split(|chr| chr == ' ' || chr == ',')
        .map(|word| {
            word.chars()
                .filter(|chr| chr.is_ascii_alphanumeric() || chr == &'-')
                .collect::<String>()
        })
        .filter(|word| word.len() > 0)
        .map(|word| word.to_lowercase())
        .collect::<Vec<String>>()
}

/// Parses the list of params into tokens representing their value.
/// The function mostly serves as a router to sub-functions which handle
/// each individual type of query.
pub fn parse_query(params: Vec<String>) -> (Option<ParamType>, Option<Vec<ParamVals>>) {
    if params.len() < 1 {
        return (None, None);
    }
    let param_type_str = params.first().unwrap().as_str();
    let param_vals_str = params[1..]
        .iter()
        .map(|word| word.as_str())
        .collect::<Vec<&str>>()
        .to_vec();

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
    } else if param_type_str.starts_with("available") {
        (Some(ParamType::Available), parse_schedule(param_vals_str))
    } else if param_type_str.starts_with("showtime") {
        (Some(ParamType::Meme), Some(vec![]))
    } else if param_type_str.starts_with("help") {
        (Some(ParamType::Help), Some(vec![]))
    } else {
        (None, None)
    }
}

/// Parses the value of the inputted name.
fn parse_name(params: Vec<&str>) -> Option<Vec<ParamVals>> {
    if params.len() > 0 {
        Some(vec![ParamVals::Name(params.concat())])
    } else {
        Some(vec![])
    }
}

/// Parses the value of the user id.
fn parse_schedule_id(params: Vec<&str>) -> Option<Vec<ParamVals>> {
    if params.len() > 4 {
        Some(vec![ParamVals::ViewId(
            params[..(params.len() - 4)].concat() + "#" + &params[(params.len() - 4)..].concat(),
        )])
    } else {
        Some(vec![])
    }
}

/// Parses the value of the inputted timezone.
fn parse_timezone(params: Vec<&str>) -> Option<Vec<ParamVals>> {
    if params.len() > 0 {
        if let Ok(num) = params.first().unwrap().parse::<f64>() {
            let time_offset: i32 = get_largest_digit(num);
            if time_offset > -24 && time_offset < 24 {
                return Some(vec![ParamVals::TimeZone(time_offset)]);
            }
        }
    } else {
        return Some(vec![]);
    }
    None
}

/// Retrieves the largest digit in an int.
fn get_largest_digit(num: f64) -> i32 {
    num as i32 / 10i32.pow(num.abs().log10().trunc() as u32)
}

/// Parses the values corresponding to a query related to the schedule itself.
/// Handles various cases such as a day and/or time range,
/// as well as multiple specific days and/or times.
fn parse_schedule(params: Vec<&str>) -> Option<Vec<ParamVals>> {
    let mut params_iter = params.iter().peekable();
    let mut res: Vec<ParamVals> = vec![];
    let mut param = params_iter.next();
    if param.is_none() {
        return None;
    }
    if param.unwrap().starts_with("weekday") {
        res.push(ParamVals::DayRange(Day::Mon, Day::Fri));
        param = params_iter.next();
    } else if param.unwrap().starts_with("weekend") {
        res.push(ParamVals::DayRange(Day::Sat, Day::Sun));
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
        res.push(ParamVals::DayRange(Day::Sun, Day::Sat));
    }
    if param.is_none() {
        return Some(res);
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

/// Parses the value of the inputted day.
/// Used in the parse_schedule function.
fn parse_day(word: &str) -> Result<Day, &str> {
    let prefix_string = word.chars().take(3).collect::<String>();
    match &prefix_string[..] {
        "sun" => Ok(Day::Sun),
        "mon" => Ok(Day::Mon),
        "tue" => Ok(Day::Tue),
        "wed" => Ok(Day::Wed),
        "thu" => Ok(Day::Thu),
        "fri" => Ok(Day::Fri),
        "sat" => Ok(Day::Sat),
        _ => Err("Invalid Date"),
    }
}
