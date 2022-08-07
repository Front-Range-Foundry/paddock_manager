use crate::paddock::Paddock;
#[derive(Debug)]

pub struct PaddockManager { 
    paddocks: Vec<Paddock>,
}

#[cfg(test)]
mod tests {
  use super::*; 
    #[test]
    fn it_can_be_created() {
        let paddock_manager = PaddockManager {
            paddocks: Vec::new(),
        };
        assert_eq!(paddock_manager.paddocks.len(), 0);
    }
}