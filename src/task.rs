use std::time::{SystemTime, UNIX_EPOCH};

#[derive(PartialEq, PartialOrd)]
pub struct Task {
  pub priority: u8,
  pub timestamp: u64,
  pub desc: String
}

impl Task {
  fn new(desc: String, priority: u8) -> Self {
    Task {
      priority,
      timestamp: SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs(),
      desc
    }
  }
}

