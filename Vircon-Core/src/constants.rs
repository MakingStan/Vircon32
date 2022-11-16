// version of these definitions
pub const VIRCON_VERSION: i32 = 1;
pub const VIRCON_REVISION: i32 = 0;

// timing
pub const FRAMES_PER_SECOND: i32 = 60;
pub const CYCLES_PER_SECOND: i32 = 9000000;
pub const CYCLES_PER_FRAME: i32 = CyclesPerSecond / FramesPerSecond;

// display
pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 360;
pub const SCREEN_PIXELS: i32 = SCREEN_HEIGHT*SCREEN_WIDTH;

// GPU specs
pub const GPU_TEXTURE_SIZE: i32 = 1024;
pub const GPU_MAXIMUM_CARTRIDGE_TEXTURES: i32 = 256;
pub const GPU_REGIONS_PER_TEXTURE: i32 = 4096;
pub const GPU_PIXEL_CAPACITY_PER_FRAME: i32 = 9*SCREEN_PIXELS;
pub const GPU_CLEAR_SCREEN_PENALTY: f32 = -0.5;
pub const GPU_SCALING_PENALTY: f32 = 0.15;
pub const GPU_ROTATION_PENALTY: f32 = 0.25;

// SPU specs
pub const SPU_MAXIMUM_CARTRIDGE_SOUNDS: i32 = 1024;
pub const SPU_MAXIMUM_CARTRIDGE_SAMPLES: i32 = 1024*1024*256;
pub const SPU_MAXIMUM_BIOS_SAMPLES: i32 = 1024 * 1024 * 1;
pub const SPU_SOUND_CHANNELS: i32 = 16;
pub const SPU_SAMPLING_RATE: i32 = 44100;

// memory specs (all sizes in words)
pub const MAXIMUM_CARTRIDGE_PROGRAM_ROM: i32 = 1024*1024*128;
pub const MAXIMUM_BIOS_PROGRAM_ROM: i32 = 1024*1024*1;
pub const RAM_SIZE: i32 = 1024*1024*1;
pub const MEMORY_CARD_SIZE: i32 = 1024*256;

// bus specs
pub const MEMORY_BUS_SLAVES: i32 = 4;
pub const CONTROL_BUS_SLAVES: i32 =8;

// other specs
pub const MAXIMUM_GAMEPADS: i32 = 4;

// base memory addresses
pub const RAM_FIRST_ADDRESS: i32 = 0x00000000;
pub const BIOS_PROGRAM_ROMFIRST_ADDRESS: i32 = 0x10000000;
pub const CARTRIDGE_PROGRAM_ROMFIRST_ADDRESS: i32 = 0x20000000;
pub const MEMORY_CARD_RAM_FIRST_ADDRESS: i32 = 0x30000000;

// base control port numbers
pub const TIM_FIRST_PORT: i32 = 0x000;
pub const RNG_FIRST_PORT: i32 = 0x100;
pub const GPU_FIRST_PORT: i32 = 0x200;
pub const SPU_FIRST_PORT: i32 = 0x300;
pub const INP_FIRST_PORT: i32 = 0x400;
pub const CAR_FIRST_PORT: i32 = 0x500;
pub const MEM_FIRST_PORT: i32 = 0x600;
