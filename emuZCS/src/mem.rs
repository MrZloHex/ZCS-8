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
}