use log::info;
use crate::constants::MAXIMUM_GAMEPADS;
use crate::constants::GAMEPAD_CONTROLLER_PREFIX;
use crate::components::vircon_component::VirconComponent;


struct GamePad
{
    connected: i32,
    left: i32,
    right: i32,
    up: i32,
    down: i32,
    button_start: i32,
    button_a: i32,
    button_b: i32,
    button_x: i32,
    button_y: i32,
    button_l: i32,
    button_r: i32,

}


impl GamePad {
    pub fn new() -> GamePad
    {
        return GamePad {
            connected: 0,
            left: 0,
            right: 0,
            up: 0,
            down: 0,
            button_start: 0,
            button_a: 0,
            button_b: 0,
            button_x: 0,
            button_y: 0,
            button_l: 0,
            button_r: 0
        }
    }
}

pub struct GamePadController {
    selected_gamepad: i32,
    realtime_gaempad_states: [GamePad; MAXIMUM_GAMEPADS as usize], // 0 is not connected and 1 is connected
    provided_gamepad_states: [GamePad; MAXIMUM_GAMEPADS as usize]
}

impl VirconComponent for GamePadController {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        info!("{} Reading local port \"{}\"", GAMEPAD_CONTROLLER_PREFIX, local_port);
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        info!("{} Writing value \"{}\" to local port \"{}\"", GAMEPAD_CONTROLLER_PREFIX, value, local_port);
        todo!()
    }
}

impl GamePadController {
    pub fn reset(&mut self)
    {
        info!("{} Resetting GamePadController...", GAMEPAD_CONTROLLER_PREFIX);
        // Set the first gamepad as selected
        self.selected_gamepad = 0;
    }
    pub fn new() -> GamePadController
    {
        info!("{} Creating new GamePadController...", GAMEPAD_CONTROLLER_PREFIX);
        let mut gamepad_controller = GamePadController {
            selected_gamepad: 0,
            realtime_gaempad_states: [GamePad::new(); MAXIMUM_GAMEPADS],
            provided_gamepad_states: [GamePad::new(); MAXIMUM_GAMEPADS]
        };

        gamepad_controller.reset();
        return gamepad_controller;
    }

    pub fn reset_gamepad(&mut self, gamepad_port: i32)
    {
        if gamepad_port >= MAXIMUM_GAMEPADS
        {
            return;
        }

        gamepad_pressed: i32 = self.realtime_gaempad_states[gamepad_port].left;
    }
}