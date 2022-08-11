pub mod paddock;
pub mod paddock_manager;
pub mod perimeter;

pub mod sup {
    pub fn init() {
        println!("Hello, world!");
    }
}
#[cfg(test)]
mod tests {
    #[test]
    fn it_returns_nothing() {
       let result = super::sup::init();
       assert_eq!(result, ());
    }
}
