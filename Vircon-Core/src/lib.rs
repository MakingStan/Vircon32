mod components;
mod buses;

mod cpu;
mod constants;
mod local_ports;
mod vircon_word;

use crate::cpu::Cpu;

pub struct Emulator {
    cpu: Cpu,
}
