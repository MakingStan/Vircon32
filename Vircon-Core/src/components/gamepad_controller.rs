pub struct GamePadController {
    gamepad1: bool,
    gamepad2: bool,
    gamepad3: bool,
    gamepad4: bool,

}

impl GamePadController {
    pub fn new() -> GamePadController
    {
        return GamePadController
        {
            gamepad1: false,
            gamepad2: false,
            gamepad3: false,
            gamepad4: false
        }
    }
}