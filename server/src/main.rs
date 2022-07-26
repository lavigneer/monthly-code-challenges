use std::collections::HashMap;
use std::io::{BufRead, BufReader, BufWriter, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, RwLock};
use std::thread::{self};

#[derive(Clone, Debug)]
pub struct Server {
    id: Arc<RwLock<u32>>,
    sender_connections: Arc<RwLock<HashMap<u32, BufReader<TcpStream>>>>,
    receiver_connections: Arc<RwLock<HashMap<u32, BufWriter<TcpStream>>>>,
}

impl Default for Server {
    fn default() -> Self {
        Server::new()
    }
}

impl Server {
    pub fn new() -> Server {
        Server {
            id: Arc::new(RwLock::new(0)),
            sender_connections: Arc::new(RwLock::new(HashMap::new())),
            receiver_connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub fn start_sender_listener(&self) -> std::thread::JoinHandle<()> {
        let me = self.clone();
        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:7878").expect("Could not start the server");

            println!("Sender server started succesfully");

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => me.on_sender_client_connect(stream),
                    Err(_error) => eprintln!("Error when tried to use stream"),
                }
            }
        })
    }

    fn on_sender_client_connect(&self, stream: TcpStream) {
        let mut id = self.id.write().unwrap();

        let mut reader = BufReader::new(stream);
        let lines = reader.by_ref().lines();
        for line in lines {
            let mut data = line.unwrap();
            data.push('\n');
            let me = self.clone();
            thread::spawn(move || {
                me.send_to_all_clients(data);
            });
        }

        self.sender_connections.write().unwrap().insert(*id, reader);
        *id += 1;
    }

    fn send_to_all_clients(&self, data: String) {
        let mut client_connections = self.receiver_connections.write().unwrap();

        for connection in client_connections.values_mut() {
            connection.write_all(data.as_bytes()).unwrap();
            connection.flush().unwrap();

        }
    }

    pub fn start_receiver_listener(&self) -> std::thread::JoinHandle<()> {
        let me = self.clone();
        thread::spawn(move || {
            let listener = TcpListener::bind("127.0.0.1:7879").expect("Could not start the server");

            println!("Receiver server started succesfully");

            for stream in listener.incoming() {
                match stream {
                    Ok(stream) => me.on_receiver_client_connect(stream),
                    Err(_error) => eprintln!("Error when tried to use stream"),
                }
            }
        })
    }

    fn on_receiver_client_connect(&self, stream: TcpStream) {
        println!("Client connected: {}", stream.peer_addr().unwrap());
        let mut id = self.id.write().unwrap();
        let writer = BufWriter::new(stream);
        self.receiver_connections
            .write()
            .unwrap()
            .insert(*id, writer);
        *id += 1;
    }
}

fn main() {
    let server: Server = Server::new();
    let receiver_handle = server.start_receiver_listener();
    let sender_handle = server.start_sender_listener();

    receiver_handle.join().unwrap();
    sender_handle.join().unwrap();
}
