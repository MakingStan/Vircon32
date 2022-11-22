use crate::constants::RAM_SIZE;
use crate::constants::RAM_PREFIX;
use log::*;
use crate::components::vircon_component::VirconComponent;
use crate::vircon_word::VirconWord;

pub struct Ram {
    ram: [VirconWord; RAM_SIZE as usize]
}


impl Ram
{
    pub fn new() -> Ram
    {
        info!("{} Creating new RAM...", RAM_PREFIX);
        return Ram {
            ram: [VirconWord::new(); RAM_SIZE]
        }
    }

    pub fn clear_memory(&mut self)
    {
        info!("{} Clearing RAM memory..", RAM_PREFIX);

        // clear previous memory
        self.ram = [VirconWord::new(); RAM_SIZE];
    }

    pub fn write_address(&mut self, local_address: i32, value: VirconWord) -> bool
    {
        info!("{} Writing value \"{}\" at local address \"{}\"", RAM_PREFIX, value.as_integer, local_address);

        if local_address >= RAM_SIZE
        {
            return false;
        }

        self.ram[local_address] = value;
        return true;
    }

    pub fn read_address(&mut self, local_address: i32, mut result: &mut VirconWord) -> bool
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
