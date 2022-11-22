use log::info;
use crate::vircon_word::VirconWord;
use crate::buses::control_bus::ControlBus;
use crate::buses::memory_bus::MemoryBus;

const STACK_SIZE: usize = 16; //set it so 16 for now
const REGISTER_AMOUNT: usize = 16;

pub struct Cpu {
    memory_bus: MemoryBus,
    control_bus: ControlBus,

    registers: [VirconWord; REGISTER_AMOUNT],
    instruction_pointer: VirconWord,
    instruction_register: VirconWord,
    immediate_value: VirconWord,
    halt_flag: bool,
    wait_flag: bool,
    stack: [VirconWord; STACK_SIZE]
}

impl Cpu {
    pub fn new(&mut self) -> self
    {
        info!("{} Creating new CPU...", CPU_PREFIX);
        return Cpu {
            memory_bus: MemoryBus::new(),
            control_bus: ControlBus::new(),

            registers: [VirconWord::new(); REGISTER_AMOUNT],
            instruction_pointer: VirconWord::new(),
            instruction_register: VirconWord::new(),
            immediate_value: VirconWord::new(),
            halt_flag: false,
            wait_flag: false,

            stack: [VirconWord; REGISTER_AMOUNT]
        };
    }

    fn stack_push(&mut self, value: i32)
    {
        self.stack[self.registers[15].as_integer].as_integer = value;

        self.registers[15].as_integer -= 1;
    }

    fn stack_pop(&mut self) -> VirconWord
    {
        self.registers[15].as_integer += 1;

        return self.stack[self.registers[15].as_integer];
    }

    pub fn cycle(&mut self)
    {
        //Read next instruction
        let instruction = read_next_instruction();
        self.instruction_register.as_integer = instruction;

        //Get immediate value from instruction
        let immediate_value_bit: u32 = (instruction & 0b11111111111111111111111111) >> 25;

        //Is the immediate value bit = 1?
        if immediate_value_bit == 1
        {
            //Store the instruction in the immediate value
            self.immediate_value.as_integer = read_next_instruction();
        }

        self.execute(instruction, immediate_value_bit);
    }

    fn read_next_instruction(&mut self) -> VirconWord
    {
        /* TODO fetch next instruction */
        self.instruction_pointer.as_integer +=1;

        //Make a full VirconWord type that consists of all of the data.
        return VirconWord::new();
    }

    fn execute(&mut self, instruction: u32, immediate_value_bit: u32)
    {
        /* Watch out if a hardware error happens */

        //Extract all of the "components" from the instruction
        let opcode: u32 = (instruction & 429_496_729_5) >> 26; //u32 max (I don't want to use std)
        let register_1: u32 = (instruction & 0b1111111111111111111111111) >> 21;
        let register_2: u32 = (instruction & 0b111111111111111111111) >> 17;
        let addressing_mode: u32 = (instruction & 0b11111111111111111) >> 14;
        let port_number: u32 = instruction & 0b11111111111111;



        match (opcode, immediate_value_bit) {
            /*-----------------------------------------------------*/
            //HLT
            (00, _) =>  {
                self.halt_flag = true;
            }

            /*-----------------------------------------------------*/
            //WAIT
            (01, _) => {
                self.wait_flag = true;
            }

            /*-----------------------------------------------------*/
            //JMP variant 1
            (02, 1) => {
                self.instruction_pointer.as_integer = self.immediate_value.as_integer;
            }
            //JMP variant 2
            (02, 0) => {
                self.instruction_pointer.as_integer = self.registers[register_1].as_integer;
            }


            /*-----------------------------------------------------*/
            //CALL variant 1
            (03, 1) => {
                self.stack_push(self.instruction_pointer.as_integer);
                self.instruction_pointer.as_integer = self.immediate_value.as_integer;
            }
            //CALL variant 2
            (03, 0) => {
                self.stack_push(self.instruction_pointer.as_integer);
                self.instruction_pointer.as_integer = self.registers[register_1].as_integer;
            }

            /*-----------------------------------------------------*/
            //RET
            (04, _) => {
                self.registers[15].as_integer -= 1;

                self.instruction_pointer = self.stack[self.registers[15].as_integer];
            }

            /*-----------------------------------------------------*/
            //JT variant 1
            (05, 1) => {
                if self.registers[register_1].as_integer != 0
                {
                    self.instruction_pointer.as_integer = self.immediate_value.as_integer;
                }
            }
            //JT variant 2
            (05, 0) => {
                if self.registers[register_1].as_integer != 0
                {
                    self.instruction_pointer.as_integer = self.registers[register_2].as_integer;
                }
            }

            /*-----------------------------------------------------*/
            //JF variant 1
            (06, 1) => {
                if self.registers[register_1].as_integer == 0
                {
                    self.instruction_pointer.as_integer = self.immediate_value.as_integer;
                }
            }
            //JF variant 2
            (06, 0) => {
                if self.registers[register_1].as_integer == 0
                {
                    self.instruction_pointer = self.registers[register_2].as_integer;
                }
            }

            /*-----------------------------------------------------*/
            //EIQ variant 1
            (07, 1) => {
                if self.registers[register_1].as_integer == self.immediate_value
                {
                    self.registers[register_1].as_integer = 1;
                }
                else {
                    self.registers[register_2].as_integer = 0;
                }
            }
            //EIQ variant 2
            (07, 0) => {
                if self.registers[register_1].as_integer == self.registers[register_2].as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else {
                    self.registers[register_2].as_integer = 0;
                }
            }

            /*-----------------------------------------------------*/
            //INE variant 1
            (08, 1) => {
                if self.registers[register_1].as_integer != self.immediate_value.as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else {
                    self.registers[register_2].as_integer = 0;
                }
            }
            //INE variant 2
            (08, 0) => {
                if self.registers[register_1].as_integer != self.registers[register_2]
                {
                    self.registers[register_1].as_integer = 1;
                }
                else {
                    self.registers[register_2].as_integer = 0;
                }
            }

            /*-----------------------------------------------------*/
            //IGT variant 1
            (09, 1) => {
                if self.registers[register_1].as_integer > self.immediate_value.as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }
            //IGT variant 2
            (09, 0) => {
                if self.registers[register_1].as_integer > self.registers[register_2].as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }

            /*-----------------------------------------------------*/
            //IGE variant 1
            (10, 1) => {
                if self.registers[register_1].as_integer >= self.immediate_value.as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }
            //IGE variant 2
            (10, 0) => {
                if self.registers[register_1].as_integer >= self.registers[register_2].as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }


            /*-----------------------------------------------------*/
            //ILT variant 1
            (11, 1) => {
                if self.registers[register_1].as_integer < self.immediate_value.as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }
            //ILT variant 2
            (11, 0) => {
                if self.registers[register_1].as_integer < self.registers[register_2].as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }

            /*-----------------------------------------------------*/
            //ILE variant 1
            (12, 1) => {
                if self.registers[register_1].as_integer <= self.immediate_value.as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }
            //ILT variant 2
            (12, 0) => {
                if self.registers[register_1].as_integer <= self.registers[register_2].as_integer
                {
                    self.registers[register_1].as_integer = 1;
                }
                else
                {
                    self.registers[register_1].as_integer = 0;
                }
            }

            /*-----------------------------------------------------*/
            //FEQ variant 1
            (13, 1) => {
                if self.registers[register_1].as_float == self.immediate_value.as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }
            //FEQ variant 2
            (13, 0) => {
                if self.registers[register_1].as_float == self.registers[register_2].as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }

            /*-----------------------------------------------------*/
            //FNE variant 1
            (14, 1) => {
                if self.registers[register_1].as_float != self.immediate_value.as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }
            //FNE variant 2
            (14, 0) => {
                if self.registers[register_1].as_float != self.registers[register_2].as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }


            /*-----------------------------------------------------*/
            //FGT variant 1
            (15, 1) => {
                if self.registers[register_1].as_float > self.immediate_value.as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }
            //FGT variant 2
            (15, 0) => {
                if self.registers[register_1].as_float > self.registers[register_2].as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }


            /*-----------------------------------------------------*/
            //FGE variant 1
            (16, 1) => {
                if self.registers[register_1].as_float >= self.immediate_value.as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }
            //FGT variant 2
            (16, 0) => {
                if self.registers[register_1].as_float >= self.registers[register_2].as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }

            /*-----------------------------------------------------*/
            //FLT variant 1
            (17, 1) => {
                if self.registers[register_1].as_float < self.immediate_value.as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }
            //FLT variant 2
            (17, 0) => {
                if self.registers[register_1].as_float < self.registers[register_2].as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }

            /*-----------------------------------------------------*/
            //FLE variant 1
            (18, 1) => {
                if self.registers[register_1].as_float <= self.immediate_value.as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }
            //FLE variant 2
            (18, 0) => {
                if self.registers[register_1].as_float <= self.registers[register_2].as_float
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }


            /*-----------------------------------------------------*/
            //MOV
            (19, _) => {
                let mut success = true;
                match addressing_mode
                {
                    //variant 1
                    0 => {
                        self.registers[register_1] = self.immediate_value.clone();
                    }
                    //variant 2
                    1 => {
                        self.registers[register_1] = self.registers[register_2].clone();
                    }
                    //variant 3
                    2 => {
                        success = self.memory_bus.ram.read_address(self.immediate_value.as_integer, &mut self.registers[register_1])
                    }
                    //variant 4
                    3 => {
                        success = self.memory_bus.ram.read_address(self.registers[register_2].as_integer, &mut self.registers[register_1]);
                    }
                    //variant 5
                    4 => {
                        success = self.memory_bus.ram.read_address(self.registers[register_2].as_integer + self.immediate_value.as_integer, &mut self.registers[register_1]);
                    }
                    //variant 6
                    5 => {
                        success = self.memory_bus.ram.write_address(self.immediate_value.as_integer, self.registers[register_2]);
                    }
                    //variant 7
                    6 => {
                        success = self.memory_bus.ram.write_address(self.registers[register_1].as_integer, self.registers[register_2]);
                    }
                    //variant 8
                    7 => {
                        success = self.memory_bus.ram.write_address(self.registers[register_1].as_integer + self.immediate_value.as_integer, self.registers[register_2]);
                    }
                    _ => {
                        /* This should be impossible to reach, because the max value that 3 bits can hold is 7 */
                    }
                }

                //handle hardware errors if any read and write didn't go properly
                if !success {
                    //handle the hardware error
                }
            }


            /*-----------------------------------------------------*/
            //LEA variant 1
            (20, 0) => {
                self.registers[register_1].as_integer = self.registers[register_2].as_integer;
            }
            //LEA variant 2
            (20, 1) => {
                self.registers[register_1].as_integer = self.registers[register_2].as_integer+self.immediate_value.as_integer;
            }

            /*-----------------------------------------------------*/
            //PUSH
            (21, _) => {
                self.stack_push(self.registers[register_1]);
            }

            /*-----------------------------------------------------*/
            //POP
            (22, _) => {
                self.registers[register_1] = self.stack_pop();
            }


            /*-----------------------------------------------------*/
            //IN
            (23, _) => {
                self.control_bus.read_port(port_number as i32, &mut self.registers[register_1]);
            }

            /*-----------------------------------------------------*/
            //OUT variant 1
            (24, 1) => {
                self.control_bus.write_port(port_number as i32, self.immediate_value.clone());
            }

            //OUT variant 2
            (24, 0) => {
                self.control_bus.write_port(port_number as i32, self.registers[register_1].clone());
            }

            /*-----------------------------------------------------*/
            //MOVS
            (25, _) => {
                //memory[DR] = memory[SR]
                let mut memory_sr = &mut self.registers[12];

                let success = self.memory_bus.ram.read_address(self.registers[13].as_integer, &mut memory_sr);

                if !success {
                    //handle hardware error
                }

                //DR += 1
                self.registers[13].as_integer += 1;

                //SR += 1
                self.registers[12].as_integer += 1;

                //CR -= 1
                self.registers[11].as_integer -= 1;

                if self.registers[11] > 0
                {
                    self.instruction_pointer -= 1;
                }
            }

            /*-----------------------------------------------------*/
            //SETS
            (26, _) => {
                // Memory[DR] = SR
                self.memory_bus.ram.write_address(self.registers[13].as_integer, self.registers[12].clone());

                //DR += 1
                self.registers[13].as_integer += 1;

                //CR -= 1
                self.registers[11].as_integer -= 1;

                if self.registers[11] > 0
                {
                    self.instruction_pointer -= 1;
                }

            }

            /*-----------------------------------------------------*/
            //CMPS
            (27, _) => {
                //register 1 = memory[DR] - Memory[SR]
                let mut memory_dr: VirconWord = VirconWord::new();
                let mut memory_sr: VirconWord = VirconWord::new();
                let mut success: bool = false;

                success = self.memory_bus.ram.read_address(self.registers[12].as_integer, &mut memory_sr);
                if !success {
                    //handle hardware error
                }

                success = self.memory_bus.ram.read_address(self.registers[13].as_integer, &mut memory_dr);
                if !success {
                    //handle hardware error
                }


                self.registers[register_1] = memory_dr.overflowing_sub(memory_sr);

                if self.registers[register_1] != 0
                {
                    return;
                }

                //DR += 1
                self.registers[13].as_integer += 1;

                //SR += 1
                self.registers[12].as_integer += 1;

                //CR -= 1
                self.registers[11].as_integer -= 1;

                if self.registers[11].as_integer > 0
                {
                    self.instruction_pointer -= 1;
                }
            }

            /*-----------------------------------------------------*/
            //CIF
            (28, _) => {
                self.registers[register_1].as_float = self.registers[register_1].as_integer as f32;
            }

            /*-----------------------------------------------------*/
            //CFI
            (29, _) => {
                self.registers[register_1].as_integer = self.registers[register_1].as_float as i32;
            }


            /*-----------------------------------------------------*/
            //CIB
            (30, _) => {
                if self.registers[register_1].as_integer != 0
                {
                    self.registers[register_1].as_integer = 1;
                }
            }


            /*-----------------------------------------------------*/
            //CFB
            (31, _) => {
                if self.registers[register_1].as_float != 0.0 {
                    self.registers[register_1].as_integer = 1;
                }
                else {
                    self.registers[register_1].as_integer = 0;
                }
            }


            /*-----------------------------------------------------*/
            //NOT
            (32, _) => {
                self.registers[register_1].as_binary = !self.registers[register_1].as_binary;
            }


            /*-----------------------------------------------------*/
            //AND variant 1
            (33, 1) => {
                self.registers[register_1].as_binary &= self.immediate_value.as_binary;
            }
            //AND variant 2
            (33, 0) => {
                self.registers[register_1].as_binary &= self.registers[register_2].as_binary;
            }


            /*-----------------------------------------------------*/
            //OR variant 1
            (34, 1) => {
                self.registers[register_1].as_binary |= self.immediate_value.as_binary;
            }
            //OR variant 2
            (34, 0) => {
                self.registers[register_1].as_binary |= self.registers[register_2].as_binary;
            }


            /*-----------------------------------------------------*/
            //XOR variant 1
            (35, 1) => {
                self.registers[register_1].as_binary ^= self.immediate_value.as_binary;
            }
            //XOR variant 2
            (35, 0) => {
                self.registers[register_1].as_binary ^= self.registers[register_2].as_binary;
            }


            /*-----------------------------------------------------*/
            //BNOT
            (36, _) => {
                if self.registers[register_1].as_binary == 0
                {
                    self.registers[register_1].as_binary = 1;
                }
                else
                {
                    self.registers[register_1].as_binary = 0;
                }
            }


            /*-----------------------------------------------------*/
            //SHL variant 1
            (37, 1) => {
                // Allow negative shifts
                if self.immediate_value.as_integer > 0
                {
                    self.registers[register_1].as_binary <<= self.immediate_value.as_integer;
                }
                else
                {
                    self.registers[register_1].as_binary >>= -self.immediate_value.as_integer;
                }
            }
            //SHL variant 2
            (37, 0) => {
                // Allow negative shifts
                if self.registers[register_2].as_integer > 0
                {
                    self.registers[register_1].as_binary <<= self.registers[register_2].as_integer;
                }
                else
                {
                    self.registers[register_1].as_binary >>= -self.registers[register_2].as_integer;
                }
            }


            /*-----------------------------------------------------*/
            //IADD variant 1
            (38, 1) => {
                self.registers[register_1].as_integer += self.immediate_value.as_integer;
            }
            //IADD variant 2
            (38, 0) => {
                self.registers[register_1].as_integer += self.registers[register_2].as_integer;
            }


            /*-----------------------------------------------------*/
            //ISUB variant 1
            (39, 1) => {
                self.registers[register_1].as_integer -= self.immediate_value.as_integer;
            }
            //ISUB variant 2
            (39, 0) => {
                self.registers[register_1].as_integer -= self.registers[register_2].as_integer;
            }


            /*-----------------------------------------------------*/
            //IMUL variant 1
            (40, 1) => {
                self.registers[register_1].as_integer *= self.immediate_value.as_integer;
            }
            //IMUL variant 2
            (40, 0) => {
                self.registers[register_1].as_integer *= self.registers[register_2].as_integer;
            }


            /*-----------------------------------------------------*/
            //IDIV variant 1
            (41, 1) => {
                self.registers[register_1].as_integer /= self.immediate_value.as_integer;
            }
            //IDIV variant 2
            (41, 0) => {
                self.registers[register_1].as_integer /= self.registers[register_2].as_integer;
            }


            /*-----------------------------------------------------*/
            //IMOD variant 1
            (42, 1) => {
                self.registers[register_1].as_integer %= self.immediate_value.as_integer;
            }
            //IMOD variant 2
            (42, 0) => {
                self.registers[register_1].as_integer %= self.registers[register_2].as_integer;
            }


            /*-----------------------------------------------------*/
            //ISIGN
            (43, _) => {
                self.registers[register_1].as_integer = -self.registers[register_1].as_integer;
            }


            /*-----------------------------------------------------*/
            //IMIN variant 1
            (44, 1) => {
                self.registers[register_1].as_integer = min(self.registers[register_1].as_integer, self.immediate_value.as_integer);
            }
            //IMIN variant 2
            (44, 0) => {
                self.registers[register_1].as_integer = min(self.registers[register_1].as_integer, self.registers[register_2].as_integer);
            }


            /*-----------------------------------------------------*/
            //IMAX variant 1
            (45, 1) => {
                self.registers[register_1].as_integer = max(self.registers[register_1].as_integer, self.immediate_value.as_integer);
            }
            //IMAX variant 2
            (45, 0) => {
                self.registers[register_1].as_integer = max(self.registers[register_1].as_integer, self.registers[register_2].as_integer);
            }

            /*-----------------------------------------------------*/
            //IABS
            (46, _) => {
                if self.registers[register_1].as_integer < 0
                {
                    self.registers[register_1].as_integer = -self.registers[register_1].as_integer;
                }
            }

            _ => {
                unimplemented!("Opcode {} could not be found.", opcode)
            }
        }
        fn min(value1: i32, value2: i32) -> i32
        {
            if value1 > value2
            {
                return value2;
            }
            else
            {
                return value1;
            }
        }

        fn max(value1: i32, value2: i32) -> i32
        {
            if value1 > value2
            {
                return value1;
            }
            else
            {
                return value2;
            }
        }
    }


}