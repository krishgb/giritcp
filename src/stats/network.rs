use super::Stat;

pub struct Network;

impl Network {
    pub fn new() -> Network {
        Network
    }
}

impl Stat for Network {
    fn run(&self) -> String{
        String::new()
    }
}

