pub trait VirconComponent
{
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool;
    fn write_port(&mut self, local_port: i32, value: i32) -> bool;
}