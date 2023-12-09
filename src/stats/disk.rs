use super::Stat;

pub struct Disk;

impl Disk {
    pub fn new() -> Disk {
        Disk
    }
}

impl Stat for Disk {
    fn run(&self) -> String{
        String::new()
    }
}

