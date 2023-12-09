use super::Stat;

pub struct Memory;

impl Memory {
    pub fn new() -> Memory {
        Memory
    }
}

impl Stat for Memory {
    fn run(&self) -> String{
        String::new()
    }
}

