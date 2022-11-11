mod components;
mod buses;

mod cpu;

use crate::cpu::Cpu;

pub struct Emulator {
    cpu: Cpu,
}
