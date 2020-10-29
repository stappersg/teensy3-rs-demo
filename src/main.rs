#![no_std]
#![no_main]

#[macro_use]
extern crate teensy3;

use teensy3::bindings;
use teensy3::serial::Serial;

use core::convert::TryInto;

#[no_mangle]
pub unsafe extern fn main() {
    // Blink Loop

    bindings::pinMode(13, bindings::OUTPUT as u8);
    bindings::digitalWrite(13, bindings::LOW as u8);
    let ser = Serial{};
    let mut i = 0;

    loop {
        // Send a message over the USB Serial port
        let msg = "Hello !\n";
        // If the serial write fails, we will halt (no more alive blinks)
        ser.write_bytes(msg.as_bytes()).unwrap_or_else(|_|{println!("Write unsuccesfull!");});
        println!("Count: {}", i);
        i += 1;
        // Show we are alive
        alive();

        if i > 2 {
            panic!("Test panic");
        }

        // Don't spam the console
        bindings::delay(2000);
    }
}

/// Blink the light twice to know we're alive
pub unsafe fn alive() {
    for _ in 0..3 {
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(100);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(100);
    }
    for _ in 0..2 {
        bindings::digitalWrite(13, bindings::HIGH as u8);
        bindings::delay(500);
        bindings::digitalWrite(13, bindings::LOW as u8);
        bindings::delay(500);
    }
    // bindings::Keyboard.press(bindings::KEY_H.try_into().unwrap());
    // bindings::Keyboard.release(bindings::KEY_H.try_into().unwrap());
    // bindings::Keyboard.press(bindings::KEY_E.try_into().unwrap());
    // bindings::Keyboard.release(bindings::KEY_E.try_into().unwrap());
    // bindings::Keyboard.press(bindings::KEY_I.try_into().unwrap());
    // bindings::Keyboard.release(bindings::KEY_I.try_into().unwrap());
    // bindings::Keyboard.press(bindings::KEY_SPACE.try_into().unwrap());
    // bindings::Keyboard.release(bindings::KEY_SPACE.try_into().unwrap());
}


