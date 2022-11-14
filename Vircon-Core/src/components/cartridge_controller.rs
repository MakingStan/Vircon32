use crate::components::vircon_component::VirconComponent;

pub struct CartridgeController {

}

impl VirconComponent for CartridgeController {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        todo!()
    }
}
impl CartridgeController
{
    pub fn new() -> CartridgeController
    {
        return CartridgeController
        {

        }
    }
}