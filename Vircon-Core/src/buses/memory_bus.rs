use crate::components::memory_card_controller::{MemoryCardController};

const RAM_SIZE: usize = 419_430_4;

pub struct MemoryBus {
    pub ram: [u32; RAM_SIZE],
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