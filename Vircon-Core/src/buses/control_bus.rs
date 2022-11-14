use crate::components::cartridge_controller::CartridgeController;
use crate::components::gamepad_controller::GamePadController;
use crate::components::gpu::Gpu;
use crate::components::memory_card_controller::MemoryCardController;
use crate::components::rng::Rng;
use crate::components::spu::Spu;
use crate::components::timer::Timer;
use crate::components::vircon_component::VirconComponent;


pub struct ControlBus {
    slaves: [dyn VirconComponent; 8],
}

impl ControlBus {
    pub fn new() -> ControlBus
    {
        let control_bus =  ControlBus {
            slaves: [0; 8],
        };

        slaves[0] = Timer::new();
        slaves[1] = Rng::new();
        slaves[2] = Gpu::new();
        slaves[3] = Spu::new();
        slaves[4] = GamePadController::new();
        slaves[5] = CartridgeController::new();
        slaves[6] = MemoryCardController::new();

        return control_bus;
    }

    pub fn read_port(&mut self, global_port: i32, mut result: &mut i32) -> bool
    {
        // separate device ID and local address
        let device_id: i32 = (global_port >> 8) & 7;
        let local_port: i32 = global_port & 0xFF;

        let succes: bool =  self.slaves[device_id].read_port(local_port, &mut result);

        return succes;
        //Don't handle hardware errors for now
    }

    pub fn write_port(&mut self, global_port: i32, value: i32) -> bool
    {
        // separate device ID and local address
        let device_id: i32 = (global_port >> 8) & 7;
        let local_port: i32 = global_port & 0xFF;


        let success: bool = self.slaves[device_id].write_port(local_port, value);

        return success;

        //Don't handle hardware errors for now
    }
}