use crate::buses::control_bus::ControlBus;
use crate::buses::memory_bus::MemoryBus;

const STACK_SIZE: usize = 16; //set it so 16 for now
const REGISTER_AMOUNT: usize = 16;

pub struct Cpu {
    memory_bus: MemoryBus,
    control_bus: ControlBus,

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
           memory_bus: MemoryBus::new(),
           control_bus: ControlBus::new(),
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

    fn read_next_instruction(&mut self) -> u32
    {
        /* TODO fetch next instruction */
        self.instruction_pointer+=1;
        return 0;
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
                // TODO
            }
            //CALL variant 2
            (03, 0) => {
                // TODO
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

        /* TODO interpet these register values as floats */
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


            /*-----------------------------------------------------*/
            //FGT variant 1
            (15, 1) => {
                if self.registers[register_1] > self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //FGT variant 2
            (15, 0) => {
                if self.registers[register_1] > self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }


            /*-----------------------------------------------------*/
            //FGE variant 1
            (16, 1) => {
                if self.registers[register_1] >= self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //FGT variant 2
            (16, 0) => {
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
            //FLT variant 1
            (17, 1) => {
                if self.registers[register_1] < self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //FLT variant 2
            (17, 0) => {
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
            //FLE variant 1
            (18, 1) => {
                if self.registers[register_1] <= self.immediate_value
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
            //FLE variant 2
            (18, 0) => {
                if self.registers[register_1] <= self.registers[register_2]
                {
                    self.registers[register_1] = 1;
                }
                else
                {
                    self.registers[register_1] = 0;
                }
            }
        /* TODO end of sequence that has to be converted to float */


            /*-----------------------------------------------------*/
            //MOV
            (19, 1) | (19, 0) => {
                match addressing_mode
                {
                    //variant 1
                    0 => {
                        self.registers[register_1] = self.immediate_value;
                    }
                    //variant 2
                    1 => {
                        self.registers[register_1] = self.registers[register_2];
                    }
                    //variant 3
                    2 => {
                        self.registers[register_1] = self.memory_bus.ram[self.immediate_value];
                    }
                    //variant 4
                    3 => {
                        self.registers[register_1] = self.memory_bus.ram[self.registers[register_2]];
                    }
                    //variant 5
                    4 => {
                        self.registers[register_1] = self.memory_bus.ram(self.registers[register_2] + self.immediate_value);
                    }
                    //variant 6
                    5 => {
                        self.memory_bus.ram[immediate_value] = self.registers[register_2];
                    }
                    //variant 7
                    6 => {
                        self.memory_bus.ram[self.registers[register_1]] = self.registers[register_2];
                    }
                    //variant 7
                    7 => {
                        self.memory_bus.ram[self.registers[register_1] + self.immediate_value] = self.registers[register_2];
                    }
                    _ => {
                        /* This should be impossible to reach, because the max value that 3 bits can hold is 7 */
                    }
                }
            }


            /*-----------------------------------------------------*/
            //LEA variant 1
            (20, 0) => {
                self.registers[register_1] = self.registers[register_2];
            }
            //LEA variant 2
            (20, 1) => {
                self.registers[register_1] = self.registers[register_2]+self.immediate_value;
            }

            /*-----------------------------------------------------*/
            //PUSH
            (21, 0) | (21, 1) => {
                // TODO
            }

            /*-----------------------------------------------------*/
            //POP
            (22, 0) | (22, 1) => {
                // TODO
            }


            /*-----------------------------------------------------*/
            //IN
            (23, 0) | (23, 1) => {
                // TODO
            }

            /*-----------------------------------------------------*/
            //OUT variant 1
            (24, 1) => {
                // TODO
            }

            //OUT variant 2
            (24, 0) => {
                // TODO
            }

            /*-----------------------------------------------------*/
            //MOVS
            (25, 1) | (25, 0) => {
                //memory[DR] = memory[SR]
                self.memory_bus.ram[self.registers[13]] = self.memory_bus.ram[self.registers[12]];

                //DR += 1
                self.registers[13] = self.registers[13].overflowing_add(1).0;

                //SR += 1
                self.registers[12] = self.registers[12].overflowing_add(1).0;

                //CR -= 1
                self.registers[11] = self.registers[11].overflowing_sub(1).0;

                if self.registers[11] > 0
                {
                    self.instruction_pointer -= 1;
                }
            }

            /*-----------------------------------------------------*/
            //SETS
            (26, 0) | (26, 1) => {
                // Memory[DR] = SR
                self.memory_bus[self.registers[13]] = self.registers[12];

                // DR += 1
                self.registers[13] = self.registers[13].overflowing_add(1).0;

                // CR -= 1
                self.registers[11] = self.registers[11].overflowing_sub(1).0;

                if self.registers[11] > 0
                {
                    self.instruction_pointer -= 1;
                }

            }

            /*-----------------------------------------------------*/
            //CMPS
            (27, 0) | (27, 1) => {
                //register 1 = memory[DR] - Memory[SR]
                self.registers[register_1] = self.memory_bus.ram[self.registers[13]]
                    .overflowing_sub(self.memory_bus.ram[self.registers[12]]);

                if self.registers[register_1] != 0
                {
                    return;
                }

                //DR += 1
                self.registers[13] = self.registers[13].overflowing_add(1).0;

                //SR += 1
                self.registers[12] = self.registers[12].overflowing_add(1).0;

                //CR -= 1
                self.registers[11] = self.registers[11].overflowing_sub(1).0;

                if self.registers[11] > 0
                {
                    self.instruction_pointer -= 1;
                }
            }

            /*-----------------------------------------------------*/
            //CMPS
            (28, 0) | (28, 1) => {
                //convert u32 to i32
                let binary_register_1 = u32::to_be_bytes(self.registers[register_1]);
                let i32_register_1 = i32::from_be_bytes(binary_register_1);

                //TODO convert i32 to float and store that binary form in the register
            }
            _ => {
                unimplemented!("Opcode {} could not be found.", opcode)
            }
        }
    }
}