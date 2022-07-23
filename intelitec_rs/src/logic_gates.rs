//! This is a logic Gates simulator module built to demonstrate writing unit tests and integration tests
//!


pub struct Gate{
    pub name: String,
}

impl Gate {
    pub fn new(name: String) -> Gate {
        Gate { name: name }
    }
    /// Implements a boolean `and` Gate by taking as input two (2) bits and returning a bit as output
    pub fn and(&mut self, x: u8, y: u8) -> u8 {
        match (x,y) {
            (1,1) => 1,
            _ => 0,
        }
    }
    /// Implements a boolean `xor` Gate by taking as input two (2) bits and returning a bit as output
    pub fn xor(&mut self, x: u8, y: u8) -> u8 {
        match (x,y) {
            (1,0) | (0,1) => 1,
            _ => 0,
        }
    }
}

#[cfg(test)]
mod tests{
    use super::*;


    #[test]
    fn test_and(){
        let mut g: Gate = Gate::new("g1".to_string());

        assert_eq!(1, g.and(1,1));
        assert_eq!(0, g.and(0,1));
        assert_eq!(0, g.and(1,0));
        assert_eq!(0, g.and(0,0));

    }

    #[test]
    fn test_xor(){
        let mut g: Gate = Gate::new("g1".to_string());

        assert_eq!(0, g.xor(1,1));
        assert_eq!(1, g.xor(0,1));
        assert_eq!(1, g.xor(1,0));
        assert_eq!(0, g.xor(0,0));
    }
}