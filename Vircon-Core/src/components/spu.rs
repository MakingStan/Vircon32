use crate::components::vircon_component::VirconComponent;

pub struct Spu {

}

impl VirconComponent for Spu {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        todo!()
    }
}
impl Spu {
    pub fn new() -> Spu
    {
        return Spu {

        }
    }
}