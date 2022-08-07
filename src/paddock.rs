use uuid::{Uuid}; 
#[derive(Debug)]

pub struct Paddock {
    pub id: Uuid,
    pub square_footage: u32,
    pub is_powered: bool,
    pub staff_assigned: Vec<Uuid>
}

#[cfg(test)]
mod tests {
  use super::*; 

  #[test] 
  fn it_can_be_created() {
    let paddock = Paddock {
      id: Uuid::new_v4(),
      square_footage: 100,
      is_powered: false,
      staff_assigned: Vec::new(),
    };

    assert_eq!(paddock.square_footage, 100);
    assert_eq!(paddock.is_powered, false);
    assert_eq!(paddock.staff_assigned, Vec::new());
  }  
}