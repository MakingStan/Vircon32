use crate::components::vircon_component::VirconComponent;
use crate::local_ports::CartridgeControllerLocalPorts;

pub struct CartridgeController {
    // state of ports
    number_of_textures: i32,
    number_of_sounds: i32,

    // Additional data about the connected cartridge
    cartridge_file_name: String,
    cartridge_title: String,
    cartridge_version: i32,
    cartridge_revision: i32,
}

impl VirconComponent for CartridgeController {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        //check range
        if local_port > CartridgeControllerLocalPorts::NumberOfSounds 
        {
            return false;
        }

        //provide value
        if local_port == CartridgeControllerLocalPorts::Connected as i32 {
            todo!()
        }
        else if local_port == CartridgeControllerLocalPorts::ProgramRomSize as i32 {
            todo!()
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
        // all these registers are read-only, so just ignore the request
        return false;
    }

    
}
impl CartridgeController
{
    pub fn new() -> CartridgeController
    {
        return CartridgeController {
            number_of_textures: 0,
            number_of_sounds: 0,

            cartridge_file_name: String::new(),
            cartridge_title: String::new(),
            cartridge_version: 0,
            cartridge_revision: 0,
        }
    }
}