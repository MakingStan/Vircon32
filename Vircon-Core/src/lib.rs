mod memory_bus;
mod control_bus;
mod components;

use crate::memory_bus::*;
use crate::control_bus::*;


struct Emulator {
    memory_bus: MemoryBus,
    control_bus: ControlBus
}