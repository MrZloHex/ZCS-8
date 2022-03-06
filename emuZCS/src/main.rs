mod cpu;
mod mem;

fn main() {
    let mut rom = mem::Mem::new(usize::pow(2, 15));
    let mut ram = mem::Mem::new(usize::pow(2, 15));
    let mut cpu = cpu::Cpu::new();

    let prog: [u8; 1] = [0xFF];

    rom.load(&prog);

    cpu.reset();

    loop {
        let hlt = cpu.execute(&rom, &mut ram);
        if hlt { break; }
    }   
}