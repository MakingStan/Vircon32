use crate::components::vircon_component::VirconComponent;


pub struct Timer {

}

impl VirconComponent for Timer {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        todo!()
    }
}

impl Timer {
    pub fn new() -> Timer
    {
        return Timer
        {

        }
    }
}