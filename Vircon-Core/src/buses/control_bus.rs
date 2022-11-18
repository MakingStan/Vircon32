use log::info;
use crate::components::cartridge_controller::CartridgeController;
use crate::components::gamepad_controller::GamePadController;
use crate::components::gpu::Gpu;
use crate::components::memory_card_controller::MemoryCardController;
use crate::components::rng::Rng;
use crate::components::spu::Spu;
use crate::components::timer::Timer;
use crate::components::vircon_component::VirconComponent;

use crate::constants::CONTROL_BUS_PREFIX;

pub struct ControlBus {
    gpu: Gpu,
    spu: Spu,
    rng: Rng,
    cartridge_controller: CartridgeController,
    timer: Timer,
    gamepad_controller: GamePadController,
    memory_card_controller: MemoryCardController,
}

impl ControlBus {
    pub fn new() -> ControlBus
    {
        info!("{} Creating new ControlBus...", CONTROL_BUS_PREFIX);

        return ControlBus {
            gpu: Gpu::new(),
            spu: Spu::new(),
            rng: Rng::new(),
            cartridge_controller: CartridgeController::new(),
            timer: Timer::new(),
            gamepad_controller: GamePadController::new(),
            memory_card_controller: MemoryCardController::new()
        }
    }

    pub fn read_port(&mut self, global_port: i32, mut result: &mut i32) -> bool
    {
        info!("{} Reading global port \"{}\"", global_port);

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
        info!("{} Writing value \"{}\" to global port \"{}\"", CONTROL_BUS_PREFIX, value, global_port);

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
                return Box::new(&self.timer);
            }
            1 => {
                return Box::new(&self.rng);
            }
            2 => {
                return Box::new(&self.gpu);
            }
            3 => {
                return Box::new(&self.spu);
            }
            4 => {
                return Box::new(&self.gamepad_controller);
            }
            5 => {
                return Box::new(&self.cartridge_controller);
            }
            6 => {
                return Box::new(&self.memory_card_controller);
            }
            _ => {
                //should not exist
                panic!("Device does not exist.");
            }
        }
    }
}