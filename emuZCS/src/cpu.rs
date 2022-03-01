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

    pub fn execute(&mut self) -> bool {
        true
    }
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
}

struct Alu {
    accumulator: u8,
    flags: Flags
}

impl Alu {
    pub fn new() -> Alu {
        Alu {
            accumulator: 0,
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
    reg_b: u8,
    reg_c: u8,
    reg_d: u8,
    reg_e: u8,
    reg_h: u8,
    reg_l: u8,

    stack_pointer: u16,
    program_counter: u16
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
