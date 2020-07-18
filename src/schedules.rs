use crate::day::*;
use std::collections::HashMap;

#[derive(Debug)]
pub struct ScheduleCollection {
  users: HashMap<u64, User>,
  name_id_map: HashMap<String, u64>,
}

impl ScheduleCollection {
  pub fn new() -> ScheduleCollection {
    ScheduleCollection {
      users: HashMap::new(),
      name_id_map: HashMap::new(),
    }
  }

  pub fn get_id(&self, name: &str) -> Option<&u64> {
    self.name_id_map.get(name)
  }

  pub fn add_name_id(&mut self, name: &str, id: u64) -> Result<String, String> {
    if !self.name_id_map.contains_key(name) {
      self.name_id_map.insert(name.to_string(), id);
      Ok("inserted new user".to_string())
    } else {
      Err("User name already mapped".to_string())
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

  pub fn available_day_to_string(&self, day: Day, timezone: i32) -> String {
    (0..24)
      .map(|time| self.available_to_string(day, time, timezone))
      .collect::<String>()
  }

  pub fn available_to_string(&self, day: Day, time: u32, timezone: i32) -> String {
    let names = self.available_at(day, time, timezone);

    match names.len() {
      0 => "".to_string(),
      _ => {
        day.to_string() + " at " 
        + &time.to_string() + ": "
        + &self.available_at(day, time, timezone)
          .iter().map(move |name| name.to_string() + ", ")
            .collect::<String>()
        + "\n"
      }
    }
  }

  pub fn id_exists(&self, name: u64) -> bool {
    self.users.contains_key(&name)
  }

  pub fn name_exists(&self, name: &str) -> bool {
    self.name_id_map.contains_key(name)
  }

  pub fn insert_user(&mut self, id: u64, user: User) {
    self.users.insert(id, user);
  }

  pub fn mut_user_by_id(&mut self, id: u64) -> Option<&mut User> {
    if self.users.contains_key(&id) {
      Some(self.users.entry(id).or_insert(User::new(" ".to_string())))
    } else {
      None
    }
  }

  pub fn user_by_id(&self, id: u64) -> Option<&User> {
    self.users.get(&id)
  }

  pub fn mut_user_by_name(&mut self, name: &str) -> Option<&mut User> {
    if self.name_id_map.contains_key(name) {
      Some(self.users.entry(*self.get_id(name).unwrap()).or_insert(User::new(" ".to_string())))
    } else {
      None
    }
  }

  pub fn user_by_name(&self, name: &str) -> Option<&User> {
    if self.name_id_map.contains_key(name) {
      self.users.get(self.get_id(name).unwrap())
    } else {
      None
    }
  }
}

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
    let end_num;
    if end_day < start_day {
      end_num = end_day as u32 + 7;
    } else {
      end_num = end_day as u32;
    }
    for day_num in (start_day as u32)..=(end_num) {
      self.set_time(num_to_day(day_num % 7).unwrap(), time, available);
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
