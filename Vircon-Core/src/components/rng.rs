use std::thread::current;
use crate::components::vircon_component::VirconComponent;
use crate::constants::{RNG_FIRST_PORT, RNG_LOCAL_CURRENT_VALUE_PORT};

pub struct Rng {
    current_value: i32,
}

impl Rng()
{
    pub fn new() -> Rng
    {
        return Rng {
            // do not use a Seed value of 0!
            current_value: 1,
        }
    }

    pub fn reset(&mut self)
    {
        self.current_value = 1;
    }
}

impl VirconComponent for Rng {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        //Check range
        if local_port != RNG_LOCAL_CURRENT_VALUE_PORT {
            return false;
        }

        *result = self.current_value;

        //determine the next value, with the formula
        //of a linear congruential generator
        let mut aux: i32 = self.current_value;
        aux *= 48271;
        self.current_value = 0x7FFFFFFF;

        return true;
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        //Check range
        if local_port != RNG_LOCAL_CURRENT_VALUE_PORT {
            return false;
        }

        // value 0 would produce a sequence of zeroes, so
        // just silently refuse to write that particular value
        if value == 0 {
            return true;
        }

        // write value, disregarding the sign bit
        // (don't just use abs, since negative range
        // is higher and it can overflow our variable)
        let mut converted_value: i32 = value;
        converted_value &= 0x7FFFFFFF;

        self.current_value  = converted_value.abs();

        return true;
    }
}


