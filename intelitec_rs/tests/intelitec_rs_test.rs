mod common;



#[cfg(test)]
mod test_intelitec_rs {
    use crate::common::{setup, teardown};
    use intelitec_rs::logic_gates::{Gate};
    pub type Sum = u8;
    pub type Carry = u8;


    #[test]
    fn test_gcd(){
        setup();
        assert_eq!(8, intelitec_rs::great_common_divisor(24,8));
        teardown();
    }

    pub fn half_adder_input_output() -> Vec<((u8, u8), (Sum, Carry))> {
        vec![
            ((0,0), (0,0)),
            ((0,1), (1,0)),
            ((1,0),(1,0)),
            ((1,1), (0,1)),
        ]
    }

    /// This function implements a half adder using primitive gates
    fn half_adder(x: u8, y: u8) -> (Sum,Carry) {
        let mut g = Gate::new("g".to_string());
        (g.xor(x,y), g.and(x,y))
    }

    #[test]
    fn one_bit_adder(){
        for (input, output) in half_adder_input_output(){
            let (x,y) = input;
            println!("Testing: {}, {} -> {:?}", x, y, output);
            assert_eq!(half_adder(x, y), output);
        }
    }
}

