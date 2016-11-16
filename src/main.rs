extern crate rand;
extern crate encoding;

use std::io;
use rand::Rng;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use encoding::{Encoding, EncoderTrap};
use encoding::all::UTF_8;

fn handle_client(stream: TcpStream) {
    println!("Client connected: {}",
             stream.local_addr().expect("no address"));
    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut stream_writer = io::BufWriter::new(stream);
    let secret_msg = format!("The secret_number is: {}", secret_number);
    let bytes_msg = &UTF_8.encode(&secret_msg, EncoderTrap::Strict).expect("unable to encode msg");
    stream_writer.write_all(bytes_msg).expect("unable to write to stream");
    stream_writer.flush().expect("couldn't flush on tcp stream");
}

fn main() {
    let connection = TcpListener::bind("0.0.0.0:3030").expect("Unable to bind at :3030");
    for stream in connection.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
    drop(connection);
}