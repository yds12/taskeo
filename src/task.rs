use std::time::{SystemTime, UNIX_EPOCH};
use std::fmt::{Display, Formatter, Result};
use serde::{Serialize, Deserialize};
use ansi_term::Colour::{Red, Green, Blue, Yellow};

#[derive(PartialEq, PartialOrd, Eq, Ord, Serialize, Deserialize)]
pub struct Task {
  pub priority: u8,
  pub timestamp: u64,
  pub desc: String
}

impl Display for Task {
  fn fmt(&self, f: &mut Formatter) -> Result {
    let painted = match self.priority {
      1 => Red.paint(&self.desc),
      2 => Yellow.paint(&self.desc),
      3 => Green.paint(&self.desc),
      _ => Blue.paint(&self.desc)
    };

    write!(f, "{}", painted)
  }
}

impl Task {
  pub fn new(desc: String, priority: u8) -> Self {
    Task {
      priority,
      timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
      desc
    }
  }
}

