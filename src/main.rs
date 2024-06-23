use std::io::{Read, Write};
use std::time::Duration;
use nmea_parser::{NmeaParser, ParsedMessage};
use serialport5::SerialPort;

fn main() {
    if let Some(port) = std::env::args().nth(1) {
        let mut port = SerialPort::builder()
            .baud_rate(9600)
            .read_timeout(Some(Duration::from_millis(10)))
            .open(port)
            .expect("Failed to open port");
        let mut buf: Vec<u8> = vec![0; 1024];
        let mut buffer_ordinary = [0u8; 512];
        let mut byte_counter = 0;
        let mut parser = NmeaParser::new();

        loop {
            match port.read(buf.as_mut_slice()) {
                Ok(t) => {
                    let slice = &buf[..t];
                    for value in slice.iter(){
                        if *value == b'\r'
                        {
                            continue;
                        }
                        if *value == b'\n' {
                            let line = String::from_utf8_lossy(&buffer_ordinary[0..byte_counter as usize]);

                            byte_counter = 0;

                            match parser.parse_sentence(&line) {
                                Ok(sentence) => {
                                    match sentence {
                                        ParsedMessage::Rmc(rmc) => {
                                            if let Some(lon) = rmc.longitude {
                                                if let Some(lat) = rmc.latitude {
                                                    println!("Lon {:.2} Lat {:.2}", lon, lat);
                                                }
                                            }
                                            if let Some(timestamp) = rmc.timestamp {
                                                println!("Time: {:?}", timestamp);
                                            }
                                        }
                                        _ => {}
                                    }
                                },
                                Err(error) => {
                                    println!("error parsing sentence: {:?}", error);
                                }
                            }
                            continue;
                        }
                        buffer_ordinary[byte_counter as usize] = *value;
                        byte_counter += 1;
                    }
                },
                Err(ref e) if e.kind() == std::io::ErrorKind::TimedOut => (),
                Err(e) => eprintln!("{:?}", e),
            }
        }

    } else {
        eprintln!("No serial port specified!");
        eprintln!("Available ports:");
        let ports = serialport5::available_ports().unwrap();
        for port in ports {
            eprintln!("{}", port.port_name);
        }
    }
}
