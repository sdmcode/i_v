#[derive(Debug, PartialEq)]
pub enum Opcode {
    HLT,
    LT,
    GT,
    LTE,
    GTE,
    NEQ,
    EQ,
    JEQ,
    JNE,
    JMP,
    JMPF,
    JMPB,
    IGL,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    NOP,
    ALOC,
    LBL,
}

#[derive(Debug, PartialEq)]
pub struct Instruction {
    opcode: Opcode
}

impl From<u8> for Opcode {
    fn from(v: u8) -> Self {
        match v {
            19 => return Opcode::LBL,
            18 => return Opcode::ALOC,
            17 => return Opcode::NOP,
            16 => return Opcode::LT,
            15 => return Opcode::GT,
            14 => return Opcode::LTE,
            13 => return Opcode::GTE,
            12 => return Opcode::NEQ,
            11 => return Opcode::JNE,
            10 => return Opcode::JEQ,
            9 => return Opcode::EQ,
            8 => return Opcode::JMPB,
            7 => return Opcode::JMPF,
            6 => return Opcode::JMP,
            5 => return Opcode::HLT,
            4 => return Opcode::DIV,
            3 => return Opcode::MUL,
            2 => return Opcode::SUB,
            1 => return Opcode::ADD,
            0 => return Opcode::LOAD,
            _ => return Opcode::IGL
        }
    }
}

impl<'a> From<&'a str> for Opcode {
    fn from(str: &'a str) -> Self {
        match str.to_lowercase().as_ref() {
            "aloc" => return Opcode::ALOC,
            "nop" => return Opcode::NOP,
            "lt" => return Opcode::LT,
            "gt" => return Opcode::GT,
            "lte" => return Opcode::LTE,
            "gte" => return Opcode::GTE,
            "neq" => return Opcode::NEQ,
            "jne" => return Opcode::JNE,
            "jeq" => return Opcode::JEQ,
            "eq" => return Opcode::EQ,
            "jmpb" => return Opcode::JMPB,
            "jmpf" => return Opcode::JMPF,
            "jmp" => return Opcode::JMP,
            "hlt" => return Opcode::HLT,
            "div" => return Opcode::DIV,
            "mul" => return Opcode::MUL,
            "sub" => return Opcode::SUB,
            "add" => return Opcode::ADD,
            "load" => return Opcode::LOAD,
            _ => return Opcode::IGL
        }
    }
}

impl Instruction {
    pub fn new(opcode: Opcode) -> Instruction {
        Instruction {
            opcode: opcode
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_hlt() {
        let opcode = Opcode::HLT;
        assert_eq!(opcode, Opcode::HLT);
    }

    #[test]
    fn test_create_instruction() {
        let instruction = Instruction::new(Opcode::HLT);
        assert_eq!(instruction.opcode, Opcode::HLT);
    }

    #[test]
    fn test_instruction_from_string() {
        let instruction = Instruction::new(Opcode::from("HLT"));
        assert_eq!(instruction.opcode, Opcode::HLT);
    }
}
