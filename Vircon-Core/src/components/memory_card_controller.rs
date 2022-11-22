use log::info;
use crate::constants::MEMORY_CARD_CONTROLLER_PREFIX;
use crate::constants::MEMORY_CARD_SIZE;
use crate::components::vircon_component::VirconComponent;
use crate::vircon_word::VirconWord;

pub struct MemoryCardController {
    memory: [VirconWord; MEMORY_CARD_SIZE as usize] // assuming it's always plugged in
}

impl VirconComponent for MemoryCardController {
    fn read_port(&mut self, local_port: i32, result: &mut VirconWord) -> bool {
        info!("{} Reading local port \"{}\"", MEMORY_CARD_CONTROLLER_PREFIX,  local_port);
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: VirconWord) -> bool {
        info!("{} Writing value \"{}\" to local port \"{}\"", MEMORY_CARD_CONTROLLER_PREFIX, value.as_integer, local_port);
        todo!()
    }
}
impl MemoryCardController {
    pub fn new() -> MemoryCardController
    {
        info!("{} Creating new MemoryCardController...", MEMORY_CARD_CONTROLLER_PREFIX);
        MemoryCardController
        {
            memory: [VirconWord::new(); MEMORY_CARD_SIZE]
        }
    }

    pub fn load_memory(&mut self, data: &[u8])
    {
        info!("{} Loading memory card contents into memory...", MEMORY_CARD_CONTROLLER_PREFIX);
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
        for (i, word) in new_data.iter().enumerate() {
            self.memory[i].as_binary = *word;
        }
    }
}