use uuid::{Uuid}; 
use chrono::{offset::{Local}, DateTime, TimeZone};

#[derive(Debug)]

pub struct Paddock {
    pub id: Uuid,
    pub square_footage: u32,
    pub is_powered: bool,
    pub staff_assigned: Vec<Uuid>,
    pub created: DateTime<Local>,
    pub last_updated: DateTime<Local>,
    pub last_veterinary_check: DateTime<Local>,
    pub last_utility_check: DateTime<Local>,
    pub last_security_check: DateTime<Local>
}

impl Paddock {
  fn new(sqft: u32, is_powered: bool) -> Paddock {
    let datetime = Local::now();
    let paddock = Paddock {
      id: Uuid::new_v4(),
      square_footage: sqft,
      is_powered, 
      staff_assigned: Vec::new(),
      created: datetime,
      last_updated: datetime,
      last_veterinary_check: datetime,
      last_security_check: datetime,
      last_utility_check: datetime,
    };
    paddock
  }
}

#[cfg(test)]
mod tests {
  use super::*; 

  #[test] 
  fn it_can_be_created() {
    let paddock = Paddock::new(100, true);
    assert_eq!(paddock.square_footage, 100);
    assert_eq!(paddock.is_powered, true);
    assert_eq!(paddock.staff_assigned, Vec::new());
  } 
  
  #[test]
  fn paddock_timestamps() {
    // runs too small to compare with external timestamp
    // so just using a pilot initializing value and checking other timestamps against the pilot.
    let paddock = Paddock::new(100, true);
    let generated_timestamp = paddock.created;
    assert_eq!(paddock.last_updated, generated_timestamp);
    assert_eq!(paddock.last_veterinary_check, generated_timestamp);
    assert_eq!(paddock.last_security_check, generated_timestamp);
    assert_eq!(paddock.last_utility_check, generated_timestamp);
  }
}