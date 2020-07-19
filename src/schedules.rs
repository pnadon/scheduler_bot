use crate::day::*;
use std::collections::HashMap;
use crate::user::User;

/// Contains a collection of user's schedules,
/// as well as a mapping from their current name to their unique id.
/// This is so that a user may refer to another user by name,
/// while the system internally uses the user's unique id, which is static.
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

  /// Returns the unique id for the user, given their current name.
  pub fn get_id(&self, name: &str) -> Option<&u64> {
    self.name_id_map.get(name)
  }

  /// Adds a name-id pair, to keep track of which names belong to which user.
  pub fn add_name_id(&mut self, name: &str, id: u64) -> Result<String, String> {
    if !self.name_id_map.contains_key(name) {
      self.name_id_map.insert(name.to_string(), id);
      Ok("inserted new user".to_string())
    } else {
      Err("User name already mapped".to_string())
    }
  }

  /// Checks all current schedules, and returns a list of every user
  /// available at that time, accounting for the timezone of the user
  /// who sent the message.
  pub fn available_at(&self, day: Day, time: u32, timezone: i32) -> Vec<String> {
    self
      .users
      .values()
      .filter(|user| user.is_available(day, time, timezone))
      .map(|user| user.name())
      .collect::<Vec<String>>()
  }

  /// Returns a concatenation of all the times people are available on a day.
  /// Takes the timezone of the author of the message into account.
  pub fn available_day_to_string(&self, day: Day, timezone: i32) -> String {
    (0..24)
      .map(|time| self.available_to_string(day, time, timezone))
      .collect::<String>()
  }

  /// Returns a string of the names of all users available at that time.
  /// Takes into account the timezone of the author of the message.
  pub fn available_to_string(&self, day: Day, time: u32, timezone: i32) -> String {
    let names = self.available_at(day, time, timezone);

    match names.len() {
      0 => "".to_string(),
      _ => {
        day.to_string()
          + " at "
          + &time.to_string()
          + ": "
          + &self
            .available_at(day, time, timezone)
            .iter()
            .map(move |name| name.to_string() + ", ")
            .collect::<String>()
          + "\n"
      }
    }
  }

  /// Checks if the id corresponds to an existing user in the collection.
  pub fn id_exists(&self, name: u64) -> bool {
    self.users.contains_key(&name)
  }

  /// Inserts a new user into the collection of schedules.
  pub fn insert_user(&mut self, id: u64, user: User) {
    self.users.insert(id, user);
  }

  /// Retrieves a mutable reference to a user
  pub fn mut_user(&mut self, name: &str) -> Option<&mut User> {
    if self.name_id_map.contains_key(name) {
      Some(
        self
          .users
          .entry(*self.get_id(name).unwrap())
          .or_insert(User::new(" ".to_string())),
      )
    } else {
      None
    }
  }

  /// Retrieves an immutable reference to a user.
  pub fn user(&self, name: &str) -> Option<&User> {
    if self.name_id_map.contains_key(name) {
      self.users.get(self.get_id(name).unwrap())
    } else {
      None
    }
  }
}
