#![no_std]
#![no_main]
#![deny(unused_must_use)]
#![allow(clippy::needless_return)]

#[macro_use]
extern crate teensy3;

use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow};
use teensy3::bindings;

fn setup() -> PinRow {
    PinRow::new_once()
}


#[no_mangle]
pub extern fn main() {
    let mut pinrow = setup();
    let mut led = pinrow.get_led();
    let mut i = 0;

    // Blink Loop
    loop {
        // Send a message over the USB Serial port
        // Print with println! wrapper macro, which just uses serial write on background
        println!("Hello! Count: {}", i);
        i += 1;
        // Show we are alive by blinking
        blink_safe(&mut led);

        // Keep 2 second pause in blinking the led, also don't spam the console
        delay(2000);
    }
}

/// Blink the light 10 times to know we're alive
pub fn blink_safe(led: &mut Pin) {
    // Blink led with custom wrapper
    for _ in 0..10 {
        led.digital_write(true);
        delay(50);
        led.digital_write(false);
        delay(50);
    }
}

/// Blink the light 10 times to know we're alive
pub fn blink_unsafe() {
    // Blink led with raw bindings
    for _ in 0..10 {
        unsafe{bindings::digitalWrite(13, bindings::HIGH as u8)};
        unsafe{bindings::delay(50)};
        unsafe{bindings::digitalWrite(13, bindings::LOW as u8)};
        unsafe{bindings::delay(50)};
    }
}


