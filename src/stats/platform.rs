use super::Stat;

pub struct Platform;

impl Platform {
    pub fn new() -> Platform {
        Platform
    }
}


impl Stat for Platform {
    fn run(&self) -> String{
        String::new()
    }
}

