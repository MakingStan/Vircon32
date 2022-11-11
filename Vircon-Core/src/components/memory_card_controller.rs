const MEMORY_CARD_SIZE: usize = 262_144;

pub struct MemoryCardController {
    memory: [u32; MEMORY_CARD_SIZE] // assuming it's always plugged in
}