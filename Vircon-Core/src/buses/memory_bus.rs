use crate::components::memory_card_controller::{MemoryCardController};
use crate::components::ram::Ram;
use crate::components::vircon_component::VirconComponent;
use crate::constants::*;

pub struct MemoryBus {
    pub ram: Ram,
    pub memory_card: MemoryCardController, // this emulator will always have a memory card connected
}

impl MemoryBus {
    pub fn new() -> MemoryBus
    {
        return MemoryBus {
            ram: Ram::new(),
            memory_card: MemoryCardController::new(),
        }
    }

    pub fn read_port(&mut self, global_port: i32, mut result: &mut i32) -> bool
    {
        // separate device ID and local address
        let device_id: i32 = (global_port >> 8) & 7;
        let local_port: i32 = global_port & 0xFF;

        let mut device = self.device_id_to_slave(device_id);
        let succes: bool =  device.read_port(local_port, &mut result);

        return succes;
        //Don't handle hardware errors for now
    }

    pub fn write_port(&mut self, global_port: i32, value: i32) -> bool
    {
        // separate device ID and local address
        let device_id: i32 = (global_port >> 8) & 7;
        let local_port: i32 = global_port & 0xFF;

        let mut device = self.device_id_to_slave(device_id);
        let success: bool = device.write_port(local_port, value);

        return success;

        //Don't handle hardware errors for now
    }

    fn device_id_to_slave(&mut self, device_id: i32) -> Box<&dyn VirconComponent>
    {
        match device_id {
            0 => {
                return Box::new(&self.ram);
            }
            3 => {
                return Box::new(&self.memory_card);
            }
            _ => {
                //should not exist
                panic!("Device does not exist.");
            }
        }
    }
}
