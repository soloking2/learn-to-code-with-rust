#[derive(Debug)]
struct Computer {
    cpu: String,
    memory: u32,
    hard_disk_capacity: u32
}



impl Computer {
     fn new(cpu: String, memory: u32, hard_disk_capacity: u32) -> Self {
        Self { cpu, memory, hard_disk_capacity }
    }
    fn upgrade_cpu(&mut self, name: String) -> &mut Self {
        self.cpu = name;
        self
    }

    fn upgrade_memory(self: &mut Self, memory: u32) -> &mut Self {
        self.memory = memory;
        self
    }

    fn upgrade_hard_disk_capacity(&mut self, capacity: u32) -> &mut Self {
        self.hard_disk_capacity = capacity;
        self
    }
}
fn main() {
    let mut computer = Computer::new(String::from("M2 Chip"), 16, 512);

    computer
                .upgrade_cpu(String::from("M4 pro"))
                .upgrade_memory(32)
                .upgrade_hard_disk_capacity(1054);

   println!("{:#?}", computer);
    
    
}