use std::io::{prelude::*, BufReader, BufWriter};
use std::net::{TcpListener, TcpStream};

use protohackers::utils::thread::ThreadPool;

const PROBLEM_NAME: &str = "Smoke Test";
const PROBLEM_NUMBER: u64 = 0;

const IP_ADDR: &str = "0.0.0.0";
const PORT_TCP: u16 = 80;
const NUM_WORKERS: usize = 10;

/// Entry point function for Protohackers problem 0.
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

/// Reads all bytes from the stream until EOF is reached, then writes the same bytes to the stream.
fn handle_connection(mut stream: TcpStream) {
    // Read from stream
    let mut buf_reader = BufReader::new(&mut stream);
    let mut buf: Vec<u8> = vec![];
    match buf_reader.read_to_end(&mut buf) {
        Ok(_) => (),
        Err(_) => {
            return;
        }
    }
    // Write to stream - ignore error
    let mut buf_writer = BufWriter::new(&mut stream);
    if buf_writer.write_all(&buf).is_ok() {}
}
