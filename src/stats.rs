pub mod cpu;
pub mod disk;
pub mod memory;
pub mod network;
pub mod platform;


pub trait Stat{
    fn run(&self) -> String;
}


