use std::{env, process};
use socketcan::{CanSocket, EmbeddedFrame, Frame, Socket, StandardId};
use socketcan::Id::Standard;


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Must specify the can interface as first parameter!");
        println!("Exiting...");

        process::exit(0);
    }

    let my_can_id: u16 = 0x152;
    let ref interface = &args[1];
    println!("Using can interface: {}", interface);

    if let Ok(socket) = CanSocket::open(interface) {
        println!("Opened the socket successfully!");
        println!("Listening for connections babbbby");
        loop {
            if let Ok(res) = socket.read_frame() {

                let id = res.id();
                let raw_id = match id {
                    Standard(std_id) => std_id.as_raw(),
                    _ => 0
                };
                let data = res.data();
                let data_len = res.len();
             
                if raw_id == 0x152 {
                    println!("its id 0x152");
                    //let's check what byte 8 is
                    if data_len != 8 {
                        println!("not 8 bytes in length :(");
                    }
                    else {
                        let bit8 = data[7];
                
                        match bit8 {
                            0x80 => println!("lights off!"),
                            0x84 => println!("parking lights"),
                            0x8C => println!("full headlights"),
                            0x98 => println!("high beans"),
                            _ => {}
                        }
                    }
                }
                //println!("{:X?}|{:X?}", raw_id, res.data())
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
