mod components;
mod buses;

mod cpu;
mod constants;
mod local_ports;

use crate::cpu::Cpu;

pub struct Emulator {
    cpu: Cpu,
}
