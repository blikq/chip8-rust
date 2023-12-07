pub struct Ram{
    memory: [u8; 4096]
}

impl Ram{
    pub fn new() -> Self{
        let mut ram = Ram{memory: [0; 4096]};

        ram
    }

    pub fn write(&mut self, address: u16, value: u8) {
        self.memory[address as usize] = value;
    }

    pub fn read(&self, address: u16) -> u8 {
        self.memory[address as usize]
    }
}