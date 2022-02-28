mod cpu;
mod mem;

fn main() {
    let mut rom = mem::Mem::new(usize::pow(2, 15));
    let mut rom = mem::Mem::new(usize::pow(2, 15));
    let mut cpu = cpu::Cpu::new();


    cpu.reset();

    loop {
        cpu.execute();
    }   
}