use crate::components::memory_card_controller::MemoryCard;
use crate::memory_card_controller::*;

const RAM_SIZE: usize = 419_430_4;

pub struct MemoryBus {
    ram: [u32; RAM_SIZE],
    memory_card: MemoryCard, // this emulator will always have a memory card connected
}