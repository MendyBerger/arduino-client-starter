#![no_std]

extern crate alloc;

use alloc::{string::String, vec::Vec};
use bincode::{Decode, Encode, config::Configuration};

#[derive(Clone, Debug, Decode, Encode)]
pub enum ClientToArduinoMessage {
    SetLed(bool),
}

#[derive(Clone, Debug, Decode, Encode)]
pub enum ArduinoToClientMessage {
    Print(String),
}


#[derive(Debug)]
pub struct MessageHandler {
    message: [u8; u8::MAX as usize],
    next_index: u8,
    len: u8,
}

impl MessageHandler {
    pub fn new() -> Self {
        Self {
            message: [0; u8::MAX as usize],
            len: 0,
            next_index: 0,
        }
    }

    pub fn got_byte(&mut self, byte: u8) -> Option<Vec<u8>> {
        // first element in message is length, rest is the actual message
        if self.len == 0 { // start
            self.len = byte;
        } else if self.next_index < self.len - 1 {
            self.message[self.next_index as usize] = byte;
            self.next_index += 1;
        } else { // end
            self.message[self.next_index as usize] = byte;

            let message = &self.message[..self.len as usize];
            let message = message.to_vec();
            self.len = 0;
            self.next_index = 0;
            return Some(message);
        };
        None
    }

    pub fn create_message<T: Encode>(message: T) -> Vec<u8> {
        // first element in message is length, rest is the actual message
        let mut slice = [0u8; u8::MAX as usize];
    
        let length = bincode::encode_into_slice(
            message,
            &mut slice[1..],
            Configuration::standard()
        ).unwrap();
    
        if length > u8::MAX as usize - 1 {
            panic!("message to long");
        }
    
        slice[0] = length as u8;
        let slice = &slice[..=length];
    
        slice.to_vec()
    }
}