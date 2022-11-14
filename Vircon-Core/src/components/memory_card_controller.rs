use log::info;
use crate::constants::*;
use crate::components::vircon_component::VirconComponent;

pub struct MemoryCardController {
    memory: [u32; MEMORY_CARD_SIZE as usize] // assuming it's always plugged in
}

impl VirconComponent for MemoryCardController {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        todo!()
    }
}
impl MemoryCardController {
    pub fn new() -> MemoryCardController
    {
        MemoryCardController
        {
            memory: [0; MEMORY_CARD_SIZE]
        }
    }

    pub fn load_memory(&mut self, data: &[u8])
    {
        info!("Loading memory card contents into memory...");
        let new_data: Vec<u32> = Vec::new();

        //convert u8 array to u32 array
        for i in 0..data.len()/4
        {
            //combine all of the u8's to one u32

            /*
                let's say byte 1 is: 1111_1101
                byte 1 will be interpeted as a u32 so it will become: 0000_0000_0000_0000_0000_0000_1111_1101
                byte 1 will be left shifted by 12 so it will become: 1111_1101_0000_0000_0000_0000_0000_0000

                that process with be repreated by byte 2 but it will be left shifted by 8
                let's say byte 2 is 1111_1111

                that in total will become:
                1111_1101_1111_1111_0000_0000_0000_0000

                etc...
             */
            new_data[i] =
                (data[i*4] as u32) << 12 |
                (data[i*4+1] as u32) << 8 |
                (data[i*4+2] as u32) << 4 |
                data[i*4+3] as u32;
        }

        //copy the data into memory
        self.memory[0..new_data.len()].copy_from_slice(new_data.as_slice());
    }



    pub fn read_port(local_port: i32)
    {

    }
}