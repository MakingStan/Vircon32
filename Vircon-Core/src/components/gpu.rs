use crate::constants::*;

pub struct Gpu
{
    drawing_buffer: [[u32; SCREEN_HEIGTH]; SCREEN_WIDTH as usize],
}

impl Gpu
{
    pub fn new() -> Gpu
    {
        return Gpu {
            drawing_buffer: [[0; SCREEN_HEIGTH]; SCREEN_WIDTH]
        }
    }

    pub fn draw_region()
    {

    }

    pub fn clear(&mut self, color: u32)
    {
        //loop through the whole drawing_buffer and make all of the pixels the clear color
        for x in self.drawing_buffer.len()
        {
            for y in self.drawing_buffer[x].len()
            {
                self.drawing_buffer[x][y] = color;
            }
        }
    }
}