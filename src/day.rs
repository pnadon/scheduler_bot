use std::fmt;

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

impl fmt::Display for Day {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{:?}", self)
      // or, alternatively:
      // fmt::Debug::fmt(self, f)
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