use crate::mem::Mem;

pub struct Cpu {
    registers: Registers,
    decoder: InstrDecoder,
    alu: Alu
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            registers: Registers::new(),
            decoder: InstrDecoder::new(),
            alu: Alu::new()
        }
    }

    pub fn reset(&mut self) {
        self.registers.reset();
        self.alu.reset();
    }

    pub fn execute(&mut self, rom: &Mem, ram: &mut Mem) -> bool {
        // First subcycle
        let opcode = rom.read(self.registers.program_counter as usize);
        self.decoder.load_opcode(opcode);
        self.registers.program_counter += 1;
        println!("OPCODE: {:X}\tINSTR: {:?}", opcode, self.decoder.get_type());

        // Second subcycle
        match self.decoder.get_type() {
            InstrType::HLT => return true,
            InstrType::MIV => {
                let byte = rom.read(self.registers.program_counter as usize);
                match self.decoder.get_register(true) {
                    Register::Accum => self.alu.accumulator = byte,
                    Register::B => self.registers.reg_b = byte,
                    Register::C => self.registers.reg_c = byte,
                    Register::D => self.registers.reg_d = byte,
                    Register::E => self.registers.reg_e = byte,
                    Register::H => self.registers.reg_h = byte,
                    Register::L => self.registers.reg_l = byte,
                }
                self.registers.program_counter += 1;
                return false;
            },
            _ => ()
        }
        false
    }

    pub fn dump(&self) {
        println!("CPU DUMP:\nRegisters:\tFlags:");
        println!("A  {:>0w$X}\t\tZ  {}", self.alu.accumulator, self.alu.flags.zero, w=2);
        println!("B  {:>0w$X}\t\tS  {}", self.registers.reg_b, self.alu.flags.sign, w=2);
        println!("C  {:>0w$X}\t\tP  {}", self.registers.reg_c, self.alu.flags.parity, w=2);
        println!("D  {:>0w$X}\t\tC  {}", self.registers.reg_d, self.alu.flags.carry, w=2);
        println!("E  {:>0w$X}", self.registers.reg_e, w=2);
        println!("HL {:>0w$X}{:>0w$X}", self.registers.reg_h, self.registers.reg_l, w=2);
        println!("SP {:>0w$X}", self.registers.stack_pointer, w=4);
        println!("PC {:>0w$X}", self.registers.program_counter, w=4);
    }
}

#[derive(Debug)]
enum InstrType {
    MOV { MemMov: bool, MemDir: bool },
    MIV,
    LSP,
    PUSH { Pop: bool },
    LPC,
    JS,
    JR,
    ALUOP { CMP: bool },
    HLT
}

enum Register {
    Accum,
    B,
    C,
    D,
    E,
    H,
    L
}

struct InstrDecoder {
    reg_instruction: u8,
    // decoder
}

impl InstrDecoder {
    pub fn new() -> InstrDecoder {
        InstrDecoder {
            reg_instruction: 0
        }
    }

    pub fn load_opcode(&mut self, opcode: u8) {
        self.reg_instruction = opcode;
    }

    pub fn get_type(&self) -> InstrType {
        match self.reg_instruction {
            0xFF => InstrType::HLT,
            0x3F => InstrType::LSP,
            0x24 => InstrType::LPC,
            0x00..=0x06 => InstrType::MIV,
            _ => std::process::exit(1)
        }
    }

    pub fn get_register(&self, ctrl: bool) -> Register {
        let code = if ctrl {
            (self.reg_instruction << 5) >> 5
        } else {
            (self.reg_instruction << 2) >> 5
        };
        match code {
            0 => Register::Accum,
            1 => Register::B,
            2 => Register::C,
            3 => Register::D,
            4 => Register::E,
            5 => Register::H,
            6 => Register::L,
            _ => std::process::exit(1)
        }
    }
}

struct Alu {
    pub accumulator: u8,
    pub temp: u8,
    pub flags: Flags
}

impl Alu {
    pub fn new() -> Alu {
        Alu {
            accumulator: 0,
            temp: 0,
            flags: Flags {
                zero: false,
                carry: false,
                sign: false,
                parity: false
            }
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.temp = 0;
        self.flags.zero = false;
        self.flags.carry = false;
        self.flags.sign = false;
        self.flags.parity = false;
    }
}

struct Flags {
    pub zero: bool,
    pub carry: bool,
    pub sign: bool,
    pub parity: bool
}

struct Registers {
    pub reg_b: u8,
    pub reg_c: u8,
    pub reg_d: u8,
    pub reg_e: u8,
    pub reg_h: u8,
    pub reg_l: u8,

    pub stack_pointer: u16,
    pub program_counter: u16
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            reg_b: 0,
            reg_c: 0,
            reg_d: 0,
            reg_e: 0,
            reg_h: 0,
            reg_l: 0,

            stack_pointer: 0,
            program_counter: 0
        }
    }

    fn reset(&mut self) {
        self.reg_b = 0;
        self.reg_c = 0;
        self.reg_d = 0;
        self.reg_e = 0;
        self.reg_h = 0;
        self.reg_l = 0;
        self.stack_pointer = 0;
        self.program_counter = 0;
    }
}
