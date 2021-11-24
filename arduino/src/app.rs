use alloc::string::String;
use bincode::{config::Configuration, error::DecodeError};
use shared::{ArduinoToClientMessage, ClientToArduinoMessage, MessageHandler};
use crate::arduino;

pub struct App {
    message_handler: MessageHandler,
}

impl App {
    pub fn new() -> Self {
        let led = arduino::get_pin_led();
        arduino::pin_mode(led, arduino::PinMode::Output);

        println("Started");

        Self {
            message_handler: MessageHandler::new(),
        }
    }

    pub fn run(&mut self) {
        self.check_for_serial_messages();

    }

    fn check_for_serial_messages(&mut self) {
        if arduino::serial_available() > 0 {
            let byte = arduino::serial_read();
            match self.message_handler.got_byte(byte as u8) {
                None => {},
                Some(message) => {
                    on_new_message(&message);
                },
            };
        }
    }
}

fn on_new_message(message: &[u8]) {
    let parsed_message: Result<ClientToArduinoMessage, DecodeError> = bincode::decode_from_slice(
        message,
        Configuration::standard()
    );
    match parsed_message {
        Ok(message) => {
            handle_new_message(message);
        },
        Err(_) => {
            println("Error parsing message");
        },
    }
}
fn handle_new_message(message: ClientToArduinoMessage) {
    match message {
        ClientToArduinoMessage::SetLed(value) => {
            arduino::digital_write(arduino::get_pin_led(), value);
            println("Setting led");
        },
    }
}

pub fn println(message: &str) {
    let message = ArduinoToClientMessage::Print(String::from(message));
    let message = MessageHandler::create_message(message);
    message.iter().for_each(|c| {
        arduino::serial_write(*c);
    });
}
