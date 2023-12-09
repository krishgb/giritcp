use giri_tcp::ThreadPool;
use std::env;
use std::io::{prelude::*, BufReader};
use std::net::{TcpListener, TcpStream};
use std::process;

use crate::stats::{
    cpu::CPU,
    disk::Disk,
    memory::Memory,
    network::Network,
    platform::Platform,
    Stat,
};

pub mod stats;

fn main() {
    let port = env::var("PORT").unwrap_or(String::from("9090"));

    let threads = env::var("THREADS").unwrap_or(String::from("5"));
    let threads: usize = str::parse(&threads).unwrap();

    let pool = ThreadPool::new(threads);

    let listener = match TcpListener::bind(format!("localhost:{port}")) {
        Ok(i) => i,
        Err(err) => {
            eprintln!("error creating a TCP network: {:?}", err.to_string());
            process::exit(1);
        }
    };

    println!("TCP connection established at port: {port} with {threads} threads");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_conneciton(stream);
        });
    }
}


fn handle_conneciton(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let splitted_uri: Vec<_> = request_line.split_terminator(' ').collect();
    
    let request: Box<dyn Stat>  = match splitted_uri[1] {
        "/platform" | "/1" => Box::new(Platform::new()),
        "/cpu" | "/2" => Box::new(CPU::new()),
        "/memory" | "/3" => Box::new(Memory::new()),
        "/disk" | "/4" => Box::new(Disk::new()),
        "/network" | "/5" => Box::new(Network::new()),
        _ => {
            stream.write_all("HTTP/1.1 404 NOT FOUND".as_bytes()).unwrap();
            return;
        },
    };

    let response_status = "HTTP/1.1 200 OK";
    let response_body = request.run();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", response_status, response_body.len(), response_body);

    stream.write_all(response.as_bytes()).unwrap();

}
