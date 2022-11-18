use crate::constants::RAM_SIZE;
use crate::constants::RAM_PREFIX;
use log::*;
use crate::components::vircon_component::VirconComponent;

pub struct Ram {
    ram: [i32; RAM_SIZE as usize]
}


impl Ram
{
    pub fn new() -> Ram
    {
        info!("{} Creating new RAM...", RAM_PREFIX);
        return Ram {
            ram: [0; RAM_SIZE]
        }
    }

    pub fn clear_memory(&mut self)
    {
        info!("{} Clearing RAM memory..", RAM_PREFIX);

        // clear previous memory
        self.ram = [0; RAM_SIZE];
    }

    pub fn write_address(&mut self, local_address: i32, value: i32) -> bool
    {
        info!("{} Writing value \"{}\" at local address \"{}\"", RAM_PREFIX, value, local_address);

        if local_address >= RAM_SIZE
        {
            return false;
        }

        self.ram[local_address] = value;
        return true;
    }

    pub fn read_address(&mut self, local_address: i32, mut result: &mut i32) -> bool
    {
        info!("{} Reading local address \"{}\"", RAM_PREFIX, local_address);

        if local_address >= RAM_SIZE
        {
            return false;
        }

        *result = self.ram[local_address];
        return true;
    }
}
