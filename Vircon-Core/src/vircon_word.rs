// note that all Vircon 32-bit primitive types are little endian
#[repr(C)]
#[derive(Clone)]
pub union VirconWord {
    pub as_float: f32,
    pub as_integer: i32,
    pub as_binary: u32,
    pub as_instruction: CPUInstruction,
    pub as_color: GPUColor,
    pub as_sample: SPUSample
}

impl VirconWord {
    pub fn new() -> VirconWord
    {
        return VirconWord {
            as_integer: 0
        }
    }
}
// ordering, from most to least significant:          // CHECK WITH WAV EXAMPLES
//   byte 0    byte 1     byte 2    byte 3
// [Left LSB][Left MSB][Right LSB][Right MSB]
// this is the order used in 16-bit stereo WAV files
pub struct SPUSample {
    pub left_sample: i16,
    pub right_sample: i16
}

// ordering: "RGBA", from most to least significant bytes     // CHECK WITH PNG->RGBA EXAMPLES
//   byte 0   byte 1   byte 2   byte 3
//    Red      Green    Blue     Alpha
// this is the order used in both PNG files and OpenGL RGBA
pub struct GPUColor
{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

// ordering: opcode is in the 6 most significant bits
pub struct CPUInstruction {
    pub port_number: u32,
    pub addressing_mode: u32,
    pub register_1: u32,
    pub register_2: u32,
    pub uses_immediate: u32,
    pub opcode: u32
}