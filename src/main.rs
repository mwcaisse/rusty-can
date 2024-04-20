use std::{env, process};
use socketcan::{CanSocket, Socket};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify the can interface as first parameter!");
        println!("Exiting...");

        process::exit(0);
    }


    let ref interface = &args[1];
    println!("Using can interface: {}", interface);

    if let Ok(socket) = CanSocket::open(interface) {
        println!("Opened the socket successfully!");
        println!("Listening for connections babbbby");
        loop {
            if let Ok(res) = socket.read_frame() {
                println!("read a frame!");
            }
            else {
                println!("Failed to read a frame from the can interface");
                break;
            }
        }
    }
    else {
        println!("Failed to open the socket")
    }

}
