use crate::day::Day;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ScheduleCollection {
  users: HashMap<String, User>,
}

impl ScheduleCollection {
  pub fn new() -> ScheduleCollection {
    ScheduleCollection {
      users: HashMap::new(),
    }
  }

  pub fn available_at(&self, day: Day, time: u32, timezone: i32) -> Vec<String> {
    self
      .users
      .values()
      .filter(|user| user.is_available(day, time, timezone))
      .map(|user| user.name())
      .collect::<Vec<String>>()
  }

  fn available_on_day(&self, day: Day, timezone: i32) -> Vec<Vec<String>> {
    (0..24)
      .map(|time| self.available_at(day, time, timezone))
      .collect::<Vec<Vec<String>>>()
  }

  pub fn insert_user(&mut self, name: String, user: User) {
    self.users.insert(name, user);
  }

  pub fn get_mut_user(&mut self, name: String) -> Option<&mut User> {
    if self.users.contains_key(&name) {
      Some(self.users.entry(name.clone()).or_insert(User::new(name)))
    } else {
      None
    }
  }

  pub fn get_user(&self, name: String) -> Option<&User> {
    self.users.get(&name)
  }
}

#[derive(Debug)]
pub struct User {
  name: String,
  timezone: i32,
  schedule: [u32; 7],
}

fn num_to_day(num: u32) -> Option<Day> {
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
impl Iterator for Day {
  // we will be counting with usize
  type Item = Day;

  // next() is the only required method
  fn next(&mut self) -> Option<Self::Item> {
    match self {
      Day::Sunday => Some(Day::Monday),
      Day::Monday => Some(Day::Tuesday),
      Day::Tuesday => Some(Day::Wednesday),
      Day::Wednesday => Some(Day::Thursday),
      Day::Thursday => Some(Day::Friday),
      Day::Friday => Some(Day::Saturday),
      Day::Saturday => Some(Day::Sunday),
    }
  }
}
impl User {
  pub fn new(name: String) -> User {
    Self {
      name: name,
      schedule: [0; 7],
      timezone: 0,
    }
  }

  pub fn name(&self) -> String {
    self.name.clone()
  }

  pub fn set_name(&mut self, name: String) {
    self.name = name;
  }

  pub fn timezone(&self) -> i32 {
    self.timezone
  }

  pub fn set_timezone(&mut self, timezone: i32) {
    self.timezone = timezone;
  }

  fn schedule(&self) -> [u32; 7] {
    self.schedule
  }

  fn is_available(&self, day: Day, time: u32, timezone: i32) -> bool {
    let (day, time) = global_daytime(day, time, timezone);
    self.schedule[day as usize] & 2_u32.pow(time) > 0
  }

  pub fn disp_schedule(&self, time_as_row: bool, timezone: i32) -> String {
    let local_schedule = local_schedule(self.schedule, timezone);
    match time_as_row {
      true => (0..24)
        .map(|pwr| 2u32.pow(pwr))
        .map(|bit| {
          local_schedule
            .iter()
            .map(move |times| match times & bit {
              0 => 'X',
              _ => 'O',
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
              0 => 'X',
              _ => 'O',
            })
            .collect::<String>()
            + &'\n'.to_string()
        })
        .collect::<String>(),
    }
  }

  fn set_day(&mut self, day: Day, times: [bool; 24]) {
    self.schedule[day as usize] = times.iter().enumerate().fold(0, |acc, x| {
      acc
        + match x.1 {
          true => 2u32.pow(x.0 as u32),
          false => 0,
        }
    });
  }

  pub fn set_time(&mut self, day: Day, time: u32, available: bool) {
    let (day, time) = global_daytime(day, time, self.timezone);
    match available {
      true => self.schedule[day as usize] |= 2u32.pow(time),
      false => self.schedule[day as usize] &= u32::max_value() - 2u32.pow(time),
    };
  }

  pub fn set_time_range(&mut self, day: Day, start_time: u32, end_time: u32, available: bool) {
    for time in start_time..=end_time {
      self.set_time(day, time, available);
    }
  }

  pub fn set_day_range(&mut self, start_day: Day, end_day: Day, time: u32, available: bool) {
    for day_num in (start_day as u32)..=(end_day as u32) {
      self.set_time(num_to_day(day_num).unwrap(), time, available);
    }
  }

  pub fn set_day_time_range(
    &mut self,
    start_day: Day,
    end_day: Day,
    start_time: u32,
    end_time: u32,
    available: bool,
  ) {
    for day_num in (start_day as u32)..=(end_day as u32) {
      self.set_time_range(
        num_to_day(day_num).unwrap(),
        start_time,
        end_time,
        available,
      );
    }
  }

  fn set_day_val(&mut self, day: Day, val: u32) {
    self.schedule[day as usize] = val;
  }
}
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
fn local_schedule(schedule: [u32; 7], timezone: i32) -> [u32; 7] {
  let mut res = schedule.clone();
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
