use super::Stat;
use std::{
    fs::{self, File},
    io::{self, prelude::*}, 
    thread,
    time::Duration,
};

pub struct CPU;

impl CPU {
    pub fn new() -> CPU {
        CPU
    }
}

impl Stat for CPU {
    fn run(&self) -> String {
        // read `/proc/cpuinfo` to get CPU cores and model name
        let cpuinfo = fs::read_to_string("/proc/cpuinfo").unwrap();
        let mut cores = 0;
        let mut model = String::new();

        for line in cpuinfo.lines() {
            let parts: Vec<&str> = line.split(":").collect();

            if parts.len() != 2 {
                continue;
            }

            let (key, value) = (parts[0].trim(), parts[1].trim());
            if key == "cpu cores" {
                cores += 1;
            } else if key == "model name" {
                model = String::from(value);
            }
        }

        // read `/proc/cpuinfo` to get CPU usage in a interval of 500ms
        let file = File::open("/proc/stat").unwrap();
        let reader = io::BufReader::new(file);

        let mut total_time = 0;
        let mut idle_time = 0;

        for line in reader.lines() {
            let line = line.unwrap();

            if line.starts_with("cpu ") {
                let values: Vec<&str> = line.split_whitespace().collect();
                if values.len() >= 5 {
                    total_time = values[1..5]
                        .iter()
                        .map(|v| v.parse::<u64>().unwrap_or_default())
                        .sum();

                    idle_time = values[4].parse::<u64>().unwrap_or_default();
                    break;
                }
            }
        }

        thread::sleep(Duration::from_millis(100));

        let new_file = File::open("/proc/stat").unwrap();
        let new_reader = io::BufReader::new(new_file);

        let mut new_total_time = 0;
        let mut new_idle_time = 0;

        for line in new_reader.lines() {
            let line = line.unwrap();

            if line.starts_with("cpu ") {
                let values: Vec<&str> = line.split_whitespace().collect();
                if values.len() >= 5 {
                    new_total_time = values[1..5]
                        .iter()
                        .map(|v| v.parse::<u64>().unwrap_or_default())
                        .sum();

                    new_idle_time = values[4].parse::<u64>().unwrap_or_default();
                    break;
                }
            }
        }

        let total_diff = new_total_time - total_time;
        let idle_diff = new_idle_time - idle_time;

        let usage = 100.0 * (1.0 - idle_diff as f32 / total_diff as f32);

       format!("Number of Cores: {}\nCPU Model: {}\nCPU Usage: {:.2}%", cores, model, usage)
    }
}
