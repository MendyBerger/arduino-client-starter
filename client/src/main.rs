use std::io::{self, Write};
use std::thread;
use std::time::Duration;

use bincode::{
    config::Configuration,
    error::DecodeError,
};
use serialport::SerialPort;
use shared::{ArduinoToClientMessage, ClientToArduinoMessage, MessageHandler};

fn main() {
    let port_name = "COM3";
    let baud_rate = 9_600;

    let port = serialport::new(port_name, baud_rate)
        .timeout(Duration::from_millis(10))
        .open();

    match port {
        Ok(port) => {
            println!("Opened port {} at {} baud:", &port_name, &baud_rate);

            setup_port_listener(port.try_clone().unwrap());

            setup_blink_loop(port);
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

fn on_new_message(message: ArduinoToClientMessage) {
    match message {
        ArduinoToClientMessage::Print(message) => {
            println!("Message from arduino: \"{}\"", message)
        },
    }
}

fn setup_port_listener(mut port: Box<dyn SerialPort>) {
    // listen for messages on different thread
    thread::spawn(move || {
        // let mut app = App::new();
        let mut message_handler = MessageHandler::new();
        let mut serial_buf: Vec<u8> = vec![0; 1000];
        
        loop {
            match port.read(serial_buf.as_mut_slice()) {
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                    // No need to handle this case, this just means got nothing
                },
                Err(e) => {
                    eprintln!("{:?}", e);
                },
                Ok(t) => {
                    serial_buf[..t].iter().for_each(|byte| {
                        let message = on_new_byte(&mut message_handler, *byte);
                        if let Some(message) = message {
                            on_new_message(message);
                        };
                    });
                },
            }
        }
    });
}

fn on_new_byte(message_handler: &mut MessageHandler, byte: u8) -> Option<ArduinoToClientMessage> {
    let message = message_handler.got_byte(byte)?;
    let parsed_message: Result<ArduinoToClientMessage, DecodeError> = bincode::decode_from_slice(
        &message,
        Configuration::standard()
    );
    match parsed_message {
        Err(_) => {
            println!("Error {:?}", String::from_utf8(message).unwrap_or_default());
            None
        },
        Ok(message) => {
            Some(message)
        },
    }
}

fn setup_blink_loop(mut port: Box<dyn SerialPort>) {
    // blink light every half second
    let mut on = false;
    loop {
        thread::sleep(Duration::from_millis(500));

        on = !on;
        let message = ClientToArduinoMessage::SetLed(on);
        let message = MessageHandler::create_message(message);

        port.write(&message).expect("Write failed!");
    }
}
