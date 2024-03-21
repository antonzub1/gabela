use std::time::Duration;

use serialport::{self, SerialPortType};

const VID: u16 = 1155;
const PID: u16 = 22336;

fn main() {
    let ports = serialport::available_ports().unwrap();
    for port in ports {
        match port.port_type {
            SerialPortType::UsbPort(info) => {
                if info.vid == VID && info.pid == PID {
                    println!("{:?}", info);
                    println!("{:?}", port.port_name);
                    let port = serialport::new(port.port_name, 115200)
                        .timeout(Duration::from_secs(1))
                        .open()
                        .expect("Failed to open a port");
                    println!("{:?}", port);
                } 
            },
            _ => { println!("Uknown device") }
        }
    }
    println!("Hello, Gabela!");
}
