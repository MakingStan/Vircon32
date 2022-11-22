use log::info;
use crate::components::vircon_component::VirconComponent;
use crate::constants::SPU_PREFIX;
use crate::vircon_word::VirconWord;

pub struct Spu {

}

impl VirconComponent for Spu {
    fn read_port(&mut self, local_port: i32, result: &mut VirconWord) -> bool {
        info!("{} Reading local port \"{}\"", SPU_PREFIX, local_port);
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: VirconWord) -> bool {
        info!("{} Writing value \"{}\" to local port \"{}\"", SPU_PREFIX, value.as_integer, local_port);
        todo!()
    }
}
impl Spu {
    pub fn new() -> Spu
    {
        info!("{} Creating new SPU...", SPU_PREFIX);

        return Spu {

        }
    }
}