use crate::components::vircon_component::VirconComponent;
pub struct Rng {

}

impl VirconComponent for Rng {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        todo!()
    }
}

impl Rng()
{
    pub fn new() -> Rng
    {
        return Rng {

        }
    }
    pub fn random_number()
    {
        // generate and return a random number
    }
}