use std::env;
use std::io::{Write, BufReader, BufRead};
use std::net::TcpStream;

fn start_sender() {
    match TcpStream::connect("localhost:7878") {
        Ok(mut stream) => {
            let msg = b"Hello!";

            stream.write_all(msg).unwrap();
            println!("Sent Hello, awaiting reply...");
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn start_receiver() {
    match TcpStream::connect("localhost:7879") {
        Ok(stream) => {
            println!("Connected to server. Local port: {}", stream.local_addr().unwrap());

            let reader = BufReader::new(stream);

            let lines = reader.lines();
            for line in lines {
                match line {
                    Ok(s) => println!("Message: {}", s),
                    Err(e) => println!("Error: {}", e)
                };
            }
        }
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated.");
}

fn main() {
    match env::args().nth(1).as_deref() {
        Some("sender") => {
            start_sender();
        }
        Some("receiver") => {
            start_receiver();
        }
        Some(c) => panic!("Invalid client type: {}", c),
        None => panic!("You must pass a client type"),
    }
}
