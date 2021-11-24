#![allow(dead_code)]

#[link(wasm_import_module = "arduino")]
extern {
    #[link_name = "millis"]                     fn _millis() -> u32;
    #[link_name = "delay"]                      fn _delay(ms: u32);
    #[link_name = "pinMode"]                    fn _pin_mode(pin: u32, mode: u32);
    #[link_name = "digitalWrite"]               fn _digital_write(pin: u32, value: bool);
    #[link_name = "getPinLED"]                  fn _get_pin_led() -> u32;
    #[link_name = "serialAvailable"]            fn _serial_available() -> u32;
    #[link_name = "serialRead"]                 fn _serial_read() -> u32;
    #[link_name = "serialWrite"]                fn _serial_write(c: u8);
}

pub enum PinMode {
    Input = 0,
    Output = 1,
    InputPullup = 2,
}

pub fn millis           () -> u32                 { unsafe { _millis() } }
pub fn delay            (ms: u32)                 { unsafe { _delay(ms); } }
pub fn pin_mode         (pin: u32, mode: PinMode) { unsafe { _pin_mode(pin, mode as u32) } }
pub fn digital_write    (pin: u32, value: bool)   { unsafe { _digital_write(pin, value) } }
pub fn get_pin_led      () -> u32                 { unsafe { _get_pin_led() } }
pub fn serial_available () -> u32                 { unsafe { _serial_available() } }
pub fn serial_read      () -> u32                 { unsafe { _serial_read() } }
pub fn serial_write     (c: u8)                   { unsafe { _serial_write(c) } }
