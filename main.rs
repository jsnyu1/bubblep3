use std::env;
use std::io;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::net::UdpSocket;
use std::thread;

const SOCKS5VER: u8 = 0x05;
const RESERVED: u8 = 0x00;

struct SOCKSReply {
    buffer: [u8; 10],
}

enum replycode {
    succeeded = 0x00,
    failure = 0x01,
    notallowed = 0x02,
    networkunreachable = 0x03,
    hostunreachable = 0x04,
    ConnectionRefused = 0x05,
    TTlexpired = 0x06,
    Commandunsupported = 0x07,
    Addressunsupported = 0x08,
}

impl SOCKSReply {
    fn new(reply: replycode) -> Self {
        let buffer = [SOCKS5VER, reply as u8, RESERVED, 1, 0, 0, 0, 0, 0, 0];
        Self { buffer }
    }
}

enum addresstype {
    IPv4 = 0x01,
    Domain = 0x03,
    IPv6 = 0x04,
}

impl addresstype {
    fn by(i: usize) -> Option<addresstype> {
        match i {
            1 => Some(addresstype::IPv4),
            3 => Some(addresstype::Domain),
            4 => Some(addresstype::IPv6),
            _ => None,
        }
    }
}

struct tcp {
    listener: TcpListener
}

impl tcp {
    fn new(port: u16, ip: &str) {
        let listener = TcpListener::bind((ip,port)).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");

            thread::spawn(|| {
                handle_connection_tcp(stream);
            });
        }

    }
}


fn handle_connection_tcp(mut stream: TcpStream) {
    for _ in 0..1000 {

        //reading in the stream and having the message broken into a struct
        //clientmessage with VER, NMETHODS, and METHODS

        let mut buffer: [u8; 8] = [0; 8];
        let read = stream.read(&mut buffer).unwrap();
        stream.write(&buffer[..read]);

        println!("from sender: {}", String::from_utf8_lossy(&buffer));

        let mut data: Vec<usize> = Vec::new();
        let mut datastring = String::new();

        for char in String::from_utf8_lossy(&buffer).chars() {

            if char != ',' {
                datastring.push(char);

            } else {
                data.push(datastring.parse::<usize>().unwrap());
                println!("Data: {}", datastring);
                datastring = String::new();
            }

        }



    }
}

// fn handle_connection_udp(socket: UdpSocket) {
//     let mut buffer:[u8; 3] = [0 ; 3];

//     let (amt, src) = socket.recv_from(&mut buffer).expect("Didn't receive data");

//     let buffer = &mut buffer[..amt];

//     socket.send_to(buffer,src).expect("Couldn't send data");

//     println!("from sender: {}", String::from_utf8_lossy(&buffer));
// }

fn main() {
    let args: Vec<String> = env::args().collect();
    let protocol = &args[1];
    let address = &args[2];

    println!("Protocol: {}", protocol);
    println!("Address: {}", address);

    if protocol == "tcp" {
        let listener = TcpListener::bind(address).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            println!("Connection established!");

            thread::spawn(|| {
                handle_connection_tcp(stream);
            });
        }
    }
    // } else if protocol == "udp" {

    //     let socket = UdpSocket::bind(protocol).expect("couldn't find to address.");

    //     thread::spawn(|| {
    //         handle_connection_udp(socket);
    //     });

    // } else {
    //     println!("Wrong connection Type!")
    // }
}
