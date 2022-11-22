use crate::vircon_word::VirconWord;

pub trait VirconComponent
{
    fn read_port(&mut self, local_port: i32, result: &mut VirconWord) -> bool;
    fn write_port(&mut self, local_port: i32, value: VirconWord) -> bool;
}