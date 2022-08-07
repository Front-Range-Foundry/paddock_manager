use uuid::{Uuid}; 
use chrono::{offset::{Local}, DateTime, TimeZone};

#[derive(Debug)]

pub struct Paddock {
    pub id: Uuid,
    pub square_footage: u32,
    pub is_powered: bool,
    pub staff_assigned: Vec<Uuid>,
    created: DateTime<Local>,
    last_updated: DateTime<Local>,
    last_veterinary_check: DateTime<Local>,
    last_utility_check: DateTime<Local>,
    last_security_check: DateTime<Local>
}

impl Paddock {
  pub fn new(sqft: u32, is_powered: bool) -> Paddock {
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

  pub fn created(&self) -> DateTime<Local> {
    self.created
  }

  pub fn last_updated(&self) -> DateTime<Local> {
    self.last_updated
  }

  pub fn last_veterinary_check(&self) -> DateTime<Local> {
    self.last_veterinary_check
  }

  pub fn last_utility_check(&self) -> DateTime<Local> {
    self.last_utility_check
  }

  pub fn last_security_check(&self) -> DateTime<Local> {
    self.last_security_check
  }

  fn report_update(&mut self, field: &str) {
    self.last_updated = Local::now();

    match field { 
      "veterinary" => self.last_veterinary_check = self.last_updated,
      "utility" => self.last_utility_check = self.last_updated,
      "security" => self.last_security_check = self.last_updated,
      _ => {}
    }
  }

  pub fn report_security_check(&mut self) -> DateTime<Local> {
    self.report_update("security");
    self.last_security_check
  }

  pub fn report_utility_check(&mut self) -> DateTime<Local> {
    self.report_update("utility");
    self.last_utility_check
  }

  pub fn report_veterinary_check(&mut self) -> DateTime<Local> {
    self.report_update("veterinary");
    self.last_veterinary_check
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

  #[test]
  fn updating_timestamps_with_functions() {
    let mut paddock = Paddock::new(100, true);    
    let new_vet_update = paddock.report_veterinary_check();
    assert_eq!(new_vet_update, paddock.last_veterinary_check());

    let new_sec_update = paddock.report_security_check();
    assert_eq!(new_sec_update, paddock.last_security_check());

    let new_ute_update = paddock.report_utility_check();
    assert_eq!(new_ute_update, paddock.last_utility_check());

    // finally, ensure the update field is commensurate with the most recent edit. 
    assert_eq!(new_ute_update, paddock.last_updated());
  }
}