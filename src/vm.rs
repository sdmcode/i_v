use instruction::Opcode;

#[derive(Debug)]
pub struct VM {
    pub registers: [i32; 32],
    pub pc: usize,
    pub program: Vec<u8>,
    heap: Vec<u8>,
    remainder: u32,
    equal_flag: bool,
}

impl VM {
    pub fn new() -> VM {
        VM {
            registers: [0; 32],
            program: vec![],
            heap: vec![],
            pc: 0,
            remainder: 0,
            equal_flag: false,
        }
    }

    fn skip_8_bits(&mut self) {
        self.pc += 1;
    }

    fn skip_16_bits(&mut self) {
        self.pc += 2;
    }

    fn skip_24_bits(&mut self) {
        self.pc += 3;
    }

    fn next_8_bits(&mut self) -> u8 {
        let result = self.program[self.pc];
        self.pc += 1;

        return result;
    }

    fn next_16_bits(&mut self) -> u16 {
        let result = ((self.program[self.pc] as u16) << 8)
                    | self.program[self.pc +1] as u16;

        self.pc += 2;

        return result;
    }

    fn decode_opcode(&mut self) -> Opcode {
        let opcode = Opcode::from(self.program[self.pc]);
        self.pc += 1;
        return opcode;
    }

    // Run until we run out of instructions to execute
    pub fn run(&mut self) {
        let mut is_done = false;

        while !is_done {
            is_done = self.execute_instruction();
        }
    }

    // Execute only a single instruction
    pub fn run_once(&mut self) {
        self.execute_instruction();
    }

    pub fn execute_instruction(&mut self) -> bool {
        // Check whether we've exceeded the max size of the program
        if self.pc >= self.program.len() {
            return true;
        }
        match self.decode_opcode() {

            Opcode::HLT => {
                println!("HLT encountered.. Exiting program");

                return true;
            },

            Opcode::ADD => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 + register2;
            },


            Opcode::SUB => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 - register2;
            },

            Opcode::MUL => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1 * register2;
            },

            Opcode::DIV => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                self.registers[self.next_8_bits() as usize] = register1  / register2;

                self.remainder = ( register1 % register2 ) as u32;
            },

            Opcode::LOAD => {
                let register = self.next_8_bits() as usize;
                let number = self.next_16_bits() as u16;

                self.registers[register] = number as i32;
            },

            Opcode::JMP => {
                let target = self.registers[self.next_8_bits() as usize];
                self.pc = target as usize;
            },

            Opcode::JMPF => {
                let offset = self.registers[self.next_8_bits() as usize] as usize;
                self.pc += offset;
            },

            Opcode::JMPB => {
                let offset = self.registers[self.next_8_bits() as usize] as usize;
                self.pc -= offset;
            },

            Opcode::EQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 == register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.skip_8_bits();
            },

            Opcode::NEQ => {
                let register1 = self.registers[self.next_8_bits() as usize];
                let register2 = self.registers[self.next_8_bits() as usize];

                if register1 != register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.skip_8_bits();
            },

            Opcode::JEQ => {
                if self.equal_flag {
                    let register = self.next_8_bits() as usize;
                    let target = self.registers[register];

                    self.pc = target as usize;
                }
            },

            Opcode::JNE => {
                let register = self.next_8_bits() as usize;
                let target = self.registers[register];

                if !self.equal_flag {
                    self.pc = target as usize;
                }
            },

            Opcode::GTE => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;

                if register1 >= register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.skip_8_bits();
            },

            Opcode::LTE => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;

                if register1 <= register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.skip_8_bits();
            },

            Opcode::LT => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;

                if register1 < register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.skip_8_bits();
            },

            Opcode::GT => {
                let register1 = self.registers[self.next_8_bits() as usize] as usize;
                let register2 = self.registers[self.next_8_bits() as usize] as usize;

                if register1 > register2 {
                    self.equal_flag = true;
                } else {
                    self.equal_flag = false;
                }

                self.skip_8_bits();
            },

            Opcode::NOP => {
                self.skip_24_bits();
            },

            Opcode::ALOC => {
                let register = self.next_8_bits() as usize;
                let bytes = self.registers[register];
                let new_len = self.heap.len() as i32 + bytes;

                self.heap.resize(new_len as usize, 0);

                self.skip_16_bits();
            }

            _ => {
                println!("Illegal operation encountered");
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_test_vm() -> VM {
        let mut test_vm = VM::new();

        test_vm.registers[0] = 5;
        test_vm.registers[1] = 10;

        return test_vm
    }

    #[test]
    fn test_create_vm() {
        let test_vm = VM::new();
        assert_eq!(test_vm.registers[0], 0);
    }

    #[test]
    fn test_opcode_hlt() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![5, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_igl() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![254, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 1);
    }

    #[test]
    fn test_opcode_load() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![0, 0, 1, 244];
        test_vm.run();

        assert_eq!(test_vm.registers[0], 500);
    }

    #[test]
    fn test_opcode_add() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![1, 0, 1, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 15);
    }

    #[test]
    fn test_opcode_sub() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![2, 1, 0, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 5);
    }

    #[test]
    fn test_opcode_mul() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![3, 0, 1, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 50);
    }

    #[test]
    fn test_opcode_div() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![4, 1, 0, 2];
        test_vm.run();

        assert_eq!(test_vm.registers[2], 2);
    }

    #[test]
    fn test_opcode_jmpf() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 2;
        test_vm.program = vec![7, 0, 0, 0, 5, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_opcode_jmpb() {
        let mut test_vm = get_test_vm();

        test_vm.registers[1] = 6;
        test_vm.program = vec![0, 0, 0, 10, 8, 1 ,0, 0];
        test_vm.run_once();
        test_vm.run_once();

        assert_eq!(test_vm.pc, 0);
    }

    #[test]
    fn test_opcode_eq() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.program = vec![9, 0, 1, 0, 9, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 20;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_neq() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.program = vec![12, 0, 1, 0, 12, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[1] = 20;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_jeq() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 7;
        test_vm.equal_flag = true;
        test_vm.program = vec![10, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_opcode_jne() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 7;
        test_vm.equal_flag = false;
        test_vm.program = vec![11, 0, 0, 0, 17, 0, 0, 0, 17, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 7);
    }

    #[test]
    fn test_opcode_lte() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.program = vec![14, 0, 1, 0, 14, 0, 1, 0, 14, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 20;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 6;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_gte() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 10;
        test_vm.registers[1] = 10;

        test_vm.program = vec![13, 0, 1, 0, 13, 0, 1, 0, 13, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);

        test_vm.registers[1] = 20;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[1] = 6;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_lt() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 7;
        test_vm.registers[1] = 7;

        test_vm.program = vec![16, 0, 1, 0, 16, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[1] = 17;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, true);
    }

    #[test]
    fn test_opcode_gt() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 7;
        test_vm.registers[1] = 7;

        test_vm.program = vec![15, 0, 1, 0, 15, 0, 1, 0];
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);

        test_vm.registers[1] = 17;
        test_vm.run_once();

        assert_eq!(test_vm.equal_flag, false);
    }

    #[test]
    fn test_opcode_aloc() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 1024;

        test_vm.program = vec![18, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.heap.len(), 1024);
    }

    #[test]
    fn test_opcode_nop() {
        let mut test_vm = get_test_vm();

        test_vm.program = vec![17, 0, 0, 0];
        test_vm.run_once();

        assert_eq!(test_vm.pc, 4);
    }

    #[test]
    fn test_program() {
        let mut test_vm = get_test_vm();

        test_vm.registers[0] = 12;
        test_vm.registers[1] = 17;

        test_vm.program = vec![
                                1, 0, 1, 2,
                                3, 1, 2, 3,
                                3, 1, 3, 4,
                                4, 2, 1, 5,
                                5
                            ];

        test_vm.run();

        assert_eq!(test_vm.pc, 17);
    }
}
