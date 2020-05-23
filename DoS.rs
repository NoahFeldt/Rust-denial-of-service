use std::net::UdpSocket;
use std::env::args;
use std::process::exit;

fn main() {
    let args: Vec<String> = args().collect();

    let address: &str = {
        if args.len() > 1 {
            args[1].trim()
        }
        else {
            println!("Expected argument in form of ip-addres:port");
            exit(1);
        }
    };

    let socket: UdpSocket = {
        match UdpSocket::bind("0.0.0.0:0") {
            Ok(s) => s,
            Err(_) => { println!("Could not bind socket"); exit(2); },
        }
    };

    let data: Vec<u8> = vec![0; 65507];

    loop {
        match socket.send_to(&data, address) {
            Ok(_) => (),
            Err(_) => { println!("Could not send data"); exit(3); },
        }
    }
}
