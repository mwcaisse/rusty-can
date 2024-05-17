use std::{env, process};
use socketcan::{CanSocket, EmbeddedFrame, Frame, Socket, StandardId};
use socketcan::Id::Standard;
use eframe::egui;

fn main() {
   // read_some_cans();
    create_some_gui();
}

fn create_some_gui() {
    env_logger::init();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.2, 240.0]),
        ..Default::default()
    };

    // Application state
    let mut name = "Mitchell".to_owned();
    let mut age = 23;

    let _ = eframe::run_simple_native("Rusty CAN", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Rusty CAN");
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            if ui.button("Increment").clicked() {
                age += 1;
            }
            ui.label(format!("Hello '{name}', age {age}"));
        });
    });


}
fn read_some_cans() {
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
                    //let's check what byte 8 is
                    if data_len != 8 {
                        println!("not 8 bytes in length :(");
                    }
                    else {
                        let byte8 = data[7];

                        match byte8 {
                            0x80 => println!("lights off!"),
                            0x84 => println!("parking lights"),
                            0x8C => println!("full headlights"),
                            0x98 => println!("high beans"),
                            _ => {}
                        }
                    }
                }
                else if raw_id == 0x140 {
                    // accelerator pedal pressure
                }
                else if raw_id == 0x0D1 {
                    // vehicle speed
                    // brake pedal pressure
                }
                else if raw_id == 0x0D4 {
                    // wheel speeds
                }
                else if raw_id == 0x375 {
                    // door locking state

                    // door open state
                    let byte2 = data[1];
                    if byte2 == 0 {
                        println!("All doors closed")
                    }
                    else {
                        if byte2 & 1 > 0 {
                            println!("Driver front door open");
                        }
                        if byte2 & (1 << 1) > 0 {
                            println!("Passenger front door open");
                        }
                        if byte2 & (1 << 2) > 0 {
                            println!("Passenger rear door open")
                        }
                        if byte2 & (1 << 3) > 0 {
                            println!("Driver rear door open")
                        }
                        if byte2 & (1 << 5) > 0 {
                            println!("Trunk door open")
                        }
                    }
                }
                else if raw_id == 0x002 {
                    // steering wheel position

                    let left = ((data[0] as f64) * 255.0 + (data[1] as f64)) / ((26 * 255) as f64);
                    println!("Steering wheel left: {}  0: {}, 1: {}", left, data[0], data[1]);
                }
                else if raw_id == 0x281 {
                    // climate control
                }
                else if raw_id == 0x282 {
                    // seat buckle
                }
                else if raw_id == 0x6D1 {
                    // odometer reading
                    let a = data[3] as u32;
                    let b = data[2] as u32;
                    let c = data[1] as u32;
                    let d= data[0] as u32;
                    let odo = (a * ( 1 << 24) + b * (1 << 16) + c * (1 << 8) + d) / 10;

                    println!("Odometer: {}km", odo);
                }
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
