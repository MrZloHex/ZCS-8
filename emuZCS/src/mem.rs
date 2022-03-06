pub struct Mem {
    capacity: usize,
    data: Vec<u8>
}

impl Mem {
    pub fn new(capacity: usize) -> Mem {
        Mem {
            capacity,
            data: Vec::with_capacity(capacity)
        }
    }

    pub fn load(&mut self, data: &[u8]) {
        self.data.clone_from_slice(data)
    }

    pub fn read(&self, address: usize) -> u8 {
        if address >= self.capacity {
            eprintln!("ERROR: tried to read at unreal address");
            std::process::exit(1);
        }
        self.data[address].clone()
    }
}

pub struct AddressDecoder {

}

impl AddressDecoder {
    pub fn read_byte(&self, address: usize) -> u8 {
        2
    }
}