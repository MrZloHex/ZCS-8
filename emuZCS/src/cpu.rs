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
        println!("OPCODE: {:>0w$X}\tINSTR: {:?}", opcode, self.decoder.get_type(&mut self.alu), w=2);

        // Second subcycle
        match self.decoder.get_type(&mut self.alu) {
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
            InstrType::LSP => (),
            InstrType::MOV {MemMov, MemDir} => {
                let val = if !MemMov || (MemMov && MemDir) {
                    match self.decoder.get_register(false) {
                        Register::Accum => self.alu.accumulator,
                        Register::B => self.registers.reg_b,
                        Register::C => self.registers.reg_c,
                        Register::D => self.registers.reg_d,
                        Register::E => self.registers.reg_e,
                        Register::H => self.registers.reg_h,
                        Register::L => self.registers.reg_l,
                    }
                } else { 0 };

                if !MemMov {
                    self.alu.temp = val;
                } else if MemDir && MemMov {
                    ram.write(self.registers.HL() as usize, val);
                    return false;
                }
            },
            InstrType::PUSH{Push} => {
                if Push {
                    match self.decoder.get_register(false) {
                        Register::Accum => ram.write(self.registers.stack_pointer as usize, self.alu.accumulator),
                        Register::B => ram.write(self.registers.stack_pointer as usize, self.registers.reg_b),
                        Register::C => ram.write(self.registers.stack_pointer as usize, self.registers.reg_c),
                        Register::D => ram.write(self.registers.stack_pointer as usize, self.registers.reg_d),
                        Register::E => ram.write(self.registers.stack_pointer as usize, self.registers.reg_e),
                        Register::H => ram.write(self.registers.stack_pointer as usize, self.registers.reg_h),
                        Register::L => ram.write(self.registers.stack_pointer as usize, self.registers.reg_l),
                    }
                } else {
                    self.registers.stack_pointer += 1;
                }
            },
            InstrType::LPC => {
                self.registers.program_counter = self.registers.HL();
                return false;
            },
            InstrType::JS => {
                let flag = match self.decoder.get_flag() {
                    Flag::Zero => self.alu.flags.zero,
                    Flag::Sign => self.alu.flags.sign,
                    Flag::Parity => self.alu.flags.parity,
                    Flag::Carry => self.alu.flags.carry
                };
                if flag {
                    self.registers.program_counter = self.registers.HL();
                }
                return false;
            },
            InstrType::JR => {
                let flag = match self.decoder.get_flag() {
                    Flag::Zero => self.alu.flags.zero,
                    Flag::Sign => self.alu.flags.sign,
                    Flag::Parity => self.alu.flags.parity,
                    Flag::Carry => self.alu.flags.carry
                };
                if !flag {
                    self.registers.program_counter = self.registers.HL();
                }
                return false;
            },
            InstrType::ALUOP{CMP} => {
                match self.decoder.get_register(false) {
                    Register::Accum => self.alu.temp = self.alu.accumulator,
                    Register::B => self.alu.temp = self.registers.reg_b,
                    Register::C => self.alu.temp = self.registers.reg_c,
                    Register::D => self.alu.temp = self.registers.reg_d,
                    Register::E => self.alu.temp = self.registers.reg_e,
                    Register::H => self.alu.temp = self.registers.reg_h,
                    Register::L => self.alu.temp = self.registers.reg_l,
                }
            },
            _ => std::process::exit(12)
        }

        // Third subcycle
        match self.decoder.get_type(&mut self.alu) {
            InstrType::LSP => {
                self.registers.stack_pointer = self.registers.HL();
                return false;
            },
            InstrType::MOV {MemMov, MemDir} => {
                let val = if !MemMov {
                    self.alu.temp
                } else if MemMov && !MemDir {
                    ram.read(self.registers.HL() as usize)
                } else { 0 };

                match self.decoder.get_register(true) {
                    Register::Accum => self.alu.accumulator = val,
                    Register::B => self.registers.reg_b = val,
                    Register::C => self.registers.reg_c = val,
                    Register::D => self.registers.reg_d = val,
                    Register::E => self.registers.reg_e = val,
                    Register::H => self.registers.reg_h = val,
                    Register::L => self.registers.reg_l = val,
                }
                return false;
            },
            InstrType::PUSH{Push} => {
                if Push {
                    self.registers.stack_pointer -= 1;
                } else {
                    match self.decoder.get_register(true) {
                        Register::Accum => self.alu.accumulator = ram.read(self.registers.stack_pointer as usize),
                        Register::B => self.registers.reg_b = ram.read(self.registers.stack_pointer as usize),
                        Register::C => self.registers.reg_c = ram.read(self.registers.stack_pointer as usize),
                        Register::D => self.registers.reg_d = ram.read(self.registers.stack_pointer as usize),
                        Register::E => self.registers.reg_e = ram.read(self.registers.stack_pointer as usize),
                        Register::H => self.registers.reg_h = ram.read(self.registers.stack_pointer as usize),
                        Register::L => self.registers.reg_l = ram.read(self.registers.stack_pointer as usize),
                    }
                }
                return false;
            },
            InstrType::ALUOP{CMP} => {
                if CMP {
                    self.alu.calculate();
                } else {
                    self.alu.accumulator = self.alu.calculate();
                }
                return false;
            },
            _ => std::process::exit(13)
        }
        unreachable!();
        true
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
    PUSH { Push: bool },
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

enum Flag {
    Zero,
    Sign,
    Parity,
    Carry
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

    pub fn get_type(&self, alu: &mut Alu) -> InstrType {
        match self.reg_instruction {
            0xFF => InstrType::HLT,
            0x3F => InstrType::LSP,
            0x24 => InstrType::LPC,
            0x00..=0x06 => InstrType::MIV,
            0x40..=0x46 | 0x48..=0x4E | 0x50..=0x56 | 0x58..=0x5E | 0x60..=0x66 | 0x68..=0x6E | 0x70..=0x76 => InstrType::MOV{MemMov: false, MemDir:false},
            0x47 | 0x4F | 0x57 | 0x5F | 0x67 | 0x6F | 0x77 => InstrType::MOV {MemMov: true, MemDir: true},
            0x78 | 0x79 | 0x7A | 0x7B | 0x7C | 0x7D | 0x7E => InstrType::MOV {MemMov: true, MemDir: false},
            0x07 | 0x0F | 0x17 | 0x1F | 0x27 | 0x2f | 0x37 => InstrType::PUSH { Push: true },
            0x38 | 0x39 | 0x3A | 0x3B | 0x3C | 0x3D | 0x3E => InstrType::PUSH { Push: false },
            0xCC | 0xDC | 0xEC | 0xFC => InstrType::JS,
            0xC4 | 0xD4 | 0xE4 | 0xF4 => InstrType::JR,
            _ => {
                if (self.reg_instruction >> 6) == 2 {
                    match self.get_register(true) {
                        Register::Accum => alu.set_op(Operation::ADD),
                        Register::B => alu.set_op(Operation::ADDC),
                        Register::C => alu.set_op(Operation::SUB),
                        Register::D => alu.set_op(Operation::SUBB),
                        Register::E => alu.set_op(Operation::SUB),
                        _ => unreachable!()
                    }
                    let cmp = (self.reg_instruction & 0x7) == 4;
                    InstrType::ALUOP { CMP: cmp }
                } else if (self.reg_instruction & 0xC0 == 0xC0) && (self.reg_instruction & 0x7 != 4){
                    match self.get_register(true) {
                        Register::Accum => alu.set_op(Operation::AND),
                        Register::B => alu.set_op(Operation::OR),
                        Register::C => alu.set_op(Operation::XOR),
                        Register::D => alu.set_op(Operation::NOT),
                        _ => unreachable!()
                    }
                    InstrType::ALUOP { CMP: false }
                } else {
                    unreachable!()
                }
            },
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

    pub fn get_flag(&self) -> Flag {
        let code = (self.reg_instruction << 2) >> 6;
        match code {
            0 => Flag::Zero,
            1 => Flag::Sign,
            2 => Flag::Parity,
            3 => Flag::Carry,
            _ => std::process::exit(1)
        }
    }
}

enum Operation {
    ADD,
    ADDC,
    SUB,
    SUBB,
    AND,
    OR,
    XOR,
    NOT,
    NOP
}

struct Alu {
    pub accumulator: u8,
    pub temp: u8,
    pub flags: Flags,
    op: Operation
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
            },
            op: Operation::NOP
        }
    }

    pub fn reset(&mut self) {
        self.accumulator = 0;
        self.temp = 0;
        self.flags.zero = false;
        self.flags.carry = false;
        self.flags.sign = false;
        self.flags.parity = false;
        self.op = Operation::NOP;
    }

    pub fn set_op(&mut self, op: Operation) {
        self.op = op;
    }

    pub fn calculate(&mut self) -> u8 {
        let val: i16 = match self.op {
            Operation::ADD => self.accumulator as i16 + self.temp as i16,
            Operation::ADDC => {
                let c: i16 = if self.flags.carry { 1 } else { 0 };
                self.accumulator as i16 + self.temp as i16 + c
            },
            Operation::SUB => self.accumulator as i16 - self.temp as i16,
            Operation::SUBB => {
                let c: i16 = if self.flags.carry { 1 } else { 0 };
                self.accumulator as i16 - self.temp as i16 - c
            },
            Operation::AND => (self.accumulator & self.temp) as i16,
            Operation::OR => (self.accumulator | self.temp) as i16,
            Operation::XOR => (self.accumulator ^ self.temp) as i16,
            Operation::NOT => (!self.temp) as i16,
            Operation::NOP => unreachable!("NOP")
        };
        // println!("ALU: {}", val);
        self.flags.zero = val == 0;
        self.flags.parity = (val & 1) == 0;
        self.flags.sign = (val & 128) == 1;
        self.flags.carry = (val < 0) || (val > 255);
        val as u8
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

    pub fn HL(&self) -> u16 {
        (self.reg_h as u16 * 0x100) + self.reg_l as u16 
    }
}
