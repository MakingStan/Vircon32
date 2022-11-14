use crate::constants::*;
use crate::components::vircon_component::VirconComponent;

use log::*;

pub struct Gpu
{
    pub drawing_buffer: [[u32; SCREEN_HEIGTH]; SCREEN_WIDTH as usize],
}

impl VirconComponent for Gpu {
    fn read_port(&mut self, local_port: i32, result: &mut i32) -> bool {
        todo!()
    }

    fn write_port(&mut self, local_port: i32, value: i32) -> bool {
        todo!()
    }
}


impl Gpu
{
    pub fn new() -> Gpu
    {
        return Gpu {
            drawing_buffer: [[0; SCREEN_HEIGTH]; SCREEN_WIDTH]
        }
    }

    pub fn clear(&mut self, color: u32)
    {
        info!("Clearing screen buffer...");

        //Set the drawing buffer equal to an array with only one color
        self.drawing_buffer = [[color: SCREEN_HEIGHT]; SCREEN_WIDTH];
    }
}