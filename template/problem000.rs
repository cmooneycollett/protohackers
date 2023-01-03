use std::io::{prelude::*, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

use protohackers::utils::thread::ThreadPool;

const PROBLEM_NAME: &str = "###";
const PROBLEM_NUMBER: u64 = 00;

const IP_ADDR: &str = "0.0.0.0";
const PORT_TCP: u16 = 80;
const NUM_WORKERS: usize = 10;

/// Entry point function for Protohackers problem 00.
pub fn main() {
    println!(
        "Protohackers // Problem {} - \"{}\"",
        PROBLEM_NUMBER, PROBLEM_NAME
    );
    println!("==================================================");
    // Bind TCP listener to accept incoming connections
    let addr = format!("{}:{}", IP_ADDR, PORT_TCP);
    let listener = TcpListener::bind(addr).unwrap();
    println!("[+] Listening on: {}:{} ...", IP_ADDR, PORT_TCP);
    // Create thread pool and handle incoming connections
    let threadpool = ThreadPool::new(NUM_WORKERS);
    for stream in listener.incoming().flatten() {
        println!(
            "[+] Incoming connection from: {}",
            stream.peer_addr().unwrap()
        );
        threadpool.execute(|| handle_connection(stream));
    }
}

/// Handle connection from client // ###
fn handle_connection(mut stream: TcpStream) {
    unimplemented!();
}
