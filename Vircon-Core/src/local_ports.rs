//RNG local ports
pub enum RngLocalPorts {
    CurrentValue = 0,
}

//Timer local ports
pub enum TimerLocalPorts {
    CurrentDate = 0,
    CurrentTime,
    FrameCounter,
    CycleCounter
}

// Cartridge controller local ports
pub enum CartridgeControllerLocalPorts {
    Connected,
    ProgramRomSize,
    NumberOfTextures,
    NumberOfSounds,
}


// Gamepad controller local ports
pub enum GamepadControllerLocalPorts {
    SelectedGamepad = 0,

    GamepadConnected,
    GamepadLeft,
    GamepadRight,
    GamepadUp,
    GamepadDown,
    GamepadButtonStart,
    GamepadButtonA,
    GamepadButtonB,
    GamepadButtonX,
    GamepadButtonY,
    GamepadButtonL,
    GamepadButtonR
}