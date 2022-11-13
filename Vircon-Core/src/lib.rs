mod components;
mod buses;

mod cpu;
mod constants;

use crate::cpu::Cpu;

pub struct Emulator {
    cpu: Cpu,
}
