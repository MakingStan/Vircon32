use log::info;
use crate::constants::CARTRIDGE_CONTROLLER_PREFIX;
use crate::components::vircon_component::VirconComponent;
use crate::local_ports::CartridgeControllerLocalPorts;

pub struct CartridgeController {
    // state of ports
    number_of_textures: i32,
    number_of_sounds: i32,

    // cartridge memory size
    memory_size: i32,

    // Additional data about the connected cartridge
    cartridge_file_name: String,
    cartridge_title: String,
    cartridge_version: i32,
    cartridge_revision: i32,
}

impl VirconComponent for CartridgeController {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        info!("{} Reading local port \"{}\"", CARTRIDGE_CONTROLLER_PREFIX, local_port);

        //check range
        if local_port > CartridgeControllerLocalPorts::NumberOfSounds as i32
        {
            return false;
        }

        //provide value
        if local_port == CartridgeControllerLocalPorts::Connected as i32 {
            if self.memory_size > 0 {
                *result = 1;
            }
            else {
                *result = 0;
            }
        }
        else if local_port == CartridgeControllerLocalPorts::ProgramRomSize as i32 {
            *result = self.memory_size;
        }
        else if local_port == CartridgeControllerLocalPorts::NumberOfTextures as i32{
            *result = self.number_of_textures;
        }
        else {
            *result = self.number_of_sounds;
        }

        return true;
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        info!("{} Writing value \"{}\" to local port \"{}\"", CARTRIDGE_CONTROLLER_PREFIX, value, local_port);
        // all these registers are read-only, so just ignore the request
        return false;
    }

    
}
impl CartridgeController
{
    pub fn new() -> CartridgeController
    {
        info!("{} Creating new CartridgeController...", CARTRIDGE_CONTROLLER_PREFIX);
        return CartridgeController {
            number_of_textures: 0,
            number_of_sounds: 0,

            memory_size: 0,

            cartridge_file_name: String::new(),
            cartridge_title: String::new(),
            cartridge_version: 0,
            cartridge_revision: 0,
        }
    }
}