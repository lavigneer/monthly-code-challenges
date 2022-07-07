use std::net::TcpListener;
use std::io::{BufRead, BufReader};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        let reader = BufReader::new(&stream);
        let lines = reader.lines();
        for line in lines {
            println!("Data: {}", line.unwrap())
        }

        println!("Connection established!");
    }
}
