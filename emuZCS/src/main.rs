mod cpu;
mod mem;

fn main() {
    let mut rom = mem::Mem::new(usize::pow(2, 15));
    let mut ram = mem::Mem::new(usize::pow(2, 16));
    let mut cpu = cpu::Cpu::new();

    let prog: Vec<u8> = vec![0x05, 0x80, 0x06, 0x10, 0x00, 0xff, 0x01, 0xff, 0x03, 0x01, 0x04, 0x01, 0x42, 0x3F, 0x90, 0x07, 0x60, 0x98, 0x8C, 0x44, 0x38, 0x05, 0x00, 0x06, 0x0E, 0xC4, 0xFF];

    rom.load(prog);

    cpu.reset();

    loop {
        let hlt = cpu.execute(&rom, &mut ram);
//        let mut input = String::new();
  //          let _string = std::io::stdin().read_line(&mut input).ok().expect("Failed to read line");
    //        let bytes = input.bytes().nth(0).expect("no byte read");
      //      // 'q'
        //    if bytes == 113 || bytes == 81 {break;}
        if hlt { break; }
    }   
    cpu.dump();

}
