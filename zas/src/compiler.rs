use crate::dictionary::Dictionary;
use colored::*;

pub struct Compiler {
    data: Vec<String>,
    line: usize,
    binary: Vec<u8>,
    dict: Dictionary<'static>
}

impl Compiler {
    pub fn new(data: Vec<String>) -> Compiler {
        Compiler {
            data,
            line: 0,
            binary: Vec::new(),
            dict: Dictionary::new()
        }
    }

    pub fn compile(&mut self) {
        while self.line < self.data.len() {
            let line = self.data[self.line].clone();
            let instr = if line.contains(' ') {
                line.splitn(2, ' ').collect::<Vec<&str>>()[0]
            } else {
                line.as_str()
            };
            print!("INSTR {} OPCODE ", instr);

            let (op_data, sec_byte) = self.dict.get_opcode(instr);
            let op = match op_data {
                Some(op) => *op,
                None => {
                    eprintln!("{}: no such instruction {} at line {}", "ERROR".bright_red(), instr.bold(), self.line);
                    std::process::exit(1);
                }
            };
            println!("{:X}", op);
            if sec_byte


            self.line += 1;
        }
    }

    pub fn get_binary(&self) -> Vec<u8> {
        self.binary.clone()
    }
}
