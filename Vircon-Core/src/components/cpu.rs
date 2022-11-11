const STACK_SIZE: usize = 16; //set it so 16 for now
const REGISTER_AMOUNT: usize = 16;

pub struct Cpu {
    registers: [u32; REGISTER_AMOUNT],
    instruction_pointer: u32,
    instruction_register: u32,
    immediate_value: u32,
    halt_flag: bool,
    wait_flag: bool,
    stack: [i32; STACK_SIZE]
}

impl Cpu {
    pub fn new(&mut self) -> self
    {
       Cpu {
           registers: [0; REGISTER_AMOUNT],
           instruction_pointer: 0,
           instruction_register: 0,
           immediate_value: 0,
           halt_flag: false,
           wait_flag: false,
           stack: [0; REGISTER_AMOUNT]
       }
    }

    pub fn cycle(&mut self)
    {
        //Read next instruction
        let instruction = read_next_instruction();
        self.instruction_register = instruction;

        //Get immediate value from instruction
        let immediate_value: u32 = (instruction & 0b11111111111111111111111111) >> 25;

        //Is the immediate value bit = 1?
        if immediate_value == 1
        {
            //Store the instruciton in the immediate value
            self.immediate_value = read_next_instruction();
        }

        self.execute(instruction, immediate_value);
    }

    fn execute(&mut self, instruction: u32, immediate_value: u32)
    {
        /* Watch out if a hardware error happens */

        //Extract all of the "components" from the instruction
        let opcode: u32 = (instruction & 429_496_729_5) >> 26; //u32 max (I don't want to use std)
        let register_1: u32 = (instruction & 0b1111111111111111111111111) >> 21;
        let register_2: u32 = (instruction & 0b111111111111111111111) >> 17;
        let addressing_mode: u32 = (instruction & 0b11111111111111111) >> 14;
        let port_number: u32 = instruction & 0b11111111111111;

        match (opcode, immediate_value) {
            /*-----------------------------------------------------*/
            //HLT
            (00, 0) | (00, 1) =>  {
                self.halt_flag = true;
            }

            /*-----------------------------------------------------*/
            //WAIT
            (01, 0) | (01, 1) => {
                self.wait_flag = true;
            }

            /*-----------------------------------------------------*/
            //JMP variant 1
            (02, 1) => {
                self.instruction_pointer = self.immediate_value;
            }
            //JMP variant 2
            (02, 0) => {
                self.instruction_pointer = self.registers[register_1];
            }


            /*-----------------------------------------------------*/
            //CALL variant 1
            (03, 1) => {
                todo!();
            }
            //CALL variant 2
            (03, 0) => {
                todo!()
            }

            /*-----------------------------------------------------*/
            //RET
            (04, 0) | (04, 1) => {
                self.registers[15] -= 1;

                self.instruction_pointer = self.stack[self.registers[15]];
            }

            /*-----------------------------------------------------*/
            //JT variant 1
            (05, 1) => {
                if self.registers[register_1] != 0
                {
                    self.instruction_pointer = self.immediate_value;
                }
            }
            //JT variant 2
            (05, 0) => {
                if self.registers[register_1] != 0
                {
                    self.instruction_pointer = self.registers[register_2];
                }
            }

            /*-----------------------------------------------------*/
            //JF variant 1
            (06, 1) => {
                if self.registers[register_1] == 0
                {
                    self.instruction_pointer = self.immediate_value;
                }
            }
            //JF variant 2
            (06, 0) => {
                if self.registers[register_1] == 0
                {
                    self.instruction_pointer = self.registers[register_2];
                }
            }

            /*-----------------------------------------------------*/
            //EIQ variant 1
            (07, 1) => {
                if self.registers[register_1] == self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else {
                    self.registers[register_2] = 0;
                }
            }
            //EIQ variant 2
            (07, 0) => {
                if self.registers[register_1] == self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else {
                    self.registers[register_2] = 0;
                }
            }

            /*-----------------------------------------------------*/
            //INE variant 1
            (08, 1) => {
                if self.registers[register_1] != self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else {
                    self.registers[register_2] = 0;
                }
            }
            //INE variant 2
            (08, 0) => {
                if self.registers[register_1] != self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else {
                    self.registers[register_2] = 0;
                }
            }

            /*-----------------------------------------------------*/
            //IGT variant 1
            (09, 1) => {
                if self.registers[register_1] > self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //IGT variant 2
            (09, 0) => {
                if self.registers[register_1] > register_2
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }

            /*-----------------------------------------------------*/
            //IGE variant 1
            (10, 1) => {
                if self.registers[register_1] >= self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //IGE variant 2
            (10, 0) => {
                if self.registers[register_1] >= self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }


            /*-----------------------------------------------------*/
            //ILT variant 1
            (11, 1) => {
                if self.registers[register_1] < self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //ILT variant 2
            (11, 0) => {
                if self.registers[register_1] < self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }

            /*-----------------------------------------------------*/
            //ILE variant 1
            (12, 1) => {
                if self.registers[register_1] <= self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //ILT variant 2
            (12, 0) => {
                if self.registers[register_1] <= self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }

            /*-----------------------------------------------------*/
            //FEQ variant 1
            (13, 1) => {
                if self.registers[register_1] == self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //FEQ variant 2
            (13, 0) => {
                if self.registers[register_1] == self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }

            /*-----------------------------------------------------*/
            //FNE variant 1
            (14, 1) => {
                f32::from_
                if self.registers[register_1] != self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //FNE variant 2
            (14, 0) => {
                if self.registers[register_1] != self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }

            _ => {
                unimplemented!("Opcode {} could not be found.", opcode)
            }
        }
    }

    fn read_next_instruction(&mut self) -> u32
    {
        self.instruction_pointer+=1;
        return 0;
    }
}