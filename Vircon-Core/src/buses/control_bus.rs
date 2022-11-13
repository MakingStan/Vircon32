use crate::components::cartridge_controller::CartridgeController;
use crate::components::gamepad_controller::GamePadController;
use crate::components::gpu::Gpu;
use crate::components::memory_card_controller::MemoryCardController;
use crate::components::rng::Rng;
use crate::components::spu::Spu;
use crate::components::timer::Timer;

const ControlBusSlaves()
pub struct ControlBus {
    slaves:
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

    pub fn read_port(port: i32)
    {
        // separate device ID and local address
        device_id: i32 = (port >> 8) & 7;
        local_port: i32 = port & 0xFF;
    }

    pub fn write_port(port: u32)
    {

    }
}