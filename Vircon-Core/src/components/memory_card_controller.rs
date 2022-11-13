use crate::constants::*;

pub struct MemoryCardController {
    memory: [u32; MEMORY_CARD_SIZE as usize] // assuming it's always plugged in
}

impl MemoryCardController {
    pub fn new() -> MemoryCardController
    {
        MemoryCardController
        {
            memory: [0; MEMORY_CARD_SIZE]
        }
    }
}