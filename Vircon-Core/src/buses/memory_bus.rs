use crate::components::memory_card_controller::{MemoryCardController};
use crate::constants::*;

pub struct MemoryBus {
    pub ram: [i32; RAM_SIZE as usize],
    pub memory_card: MemoryCardController, // this emulator will always have a memory card connected
}

impl MemoryBus {
    pub fn new() -> MemoryBus
    {
        return MemoryBus {
            ram: [0; RAM_SIZE],
            memory_card: MemoryCardController::new(),
        }
    }
}