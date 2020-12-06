# Teensy with Rust

This is an example "Hello world" project that uses bindings from [teensy3-rs](https://github.com/tolvanea/teensy3-rs) crate. This aims to be simple template to get teensy project running. It is intended to be forked or copied into a new project.

## About in this fork
This fork is based on James Munns' [teensy3-rs-demo](https://github.com/jamesmunns/teensy3-rs-demo). Main modifications are:
* Add [Cross](https://github.com/rust-embedded/cross) support alongside with Cargo. (Cross does compilation in Docker container.)
* Support for Teensy 3.0-3.6
* Reduce need for device specific configuration: Teensy model is only needed to be specified once in Makefile.
* Place teensy3-rs crate in as a git sub module. Now library build process is easy to tweak as the crate is in the same directory.
* Other modification notes are listed in [teensy3-rs](https://github.com/tolvanea/teensy3-rs).

## Package manager
This project supports [Cross](https://github.com/rust-embedded/cross) package manager, which is basically just Cargo inside a Docker container. Docker container is useful here, because binding generation and cross compilation are sensitive to installed dependency versions. For example version numbering differences in gcc may cause failing compilations. 

Of course, Cargo can be used as well, and makefile can be configured to use that instead. However, if Cargo is used, many dependencies must be installed on system. Ubuntu dependencies can be found from `Dockerfile`.(Docker container is based on ubuntu 16.04.)

## Installations
Install [teensy-loader-cli](https://www.pjrc.com/teensy/loader_cli.html), which will be used to flash binaries on Teensy. Also, install _objcopy_, which is used to transform compiled binaries into correct. Installation on Ubuntu is:
```
sudo apt-get install teensy-loader-cli binutils-arm-none-eabi
```
Install [Cross](https://github.com/rust-embedded/cross):
```
cargo install cross
```

Install Docker. On ubuntu it can be done by following "Step 1" in [this link](https://www.digitalocean.com/community/tutorials/how-to-install-and-use-docker-on-ubuntu-20-04).

Docker requires super user rights by default. It's useful to permit rights for current user, so that Cross and Docker need not be called with root every time:
```
sudo usermod -aG docker ${USER}
```
However, the above command takes effect only after logging out and in. To avoid relogging, it can be temporarily fixed with 
```
su - $USER                                           
```
which works only in that one terminal window. This command resets current working directory, so there is need to navigate back to the teensy3-rs-demo directory. Also GUI programs can not be started from this terminal window.

## Setting up with Cross
Clone this repository with command
```
git clone --recurse-submodules https://github.com/tolvanea/teensy3-rs-demo
```
Flag `--recurse-submodules` is needed, because all git submodules need to be downloaded too.

Then build docker image of `Dockerfile` with
```
docker build -t teensy3/cross:tag .
``` 
This docker image will be used by cross. To check that docker is working, all images on system can be listed with command
```
docker images
```

Lastly, Teensy model should be specified. Uncomment the line corresponding the Teensy model in `Makefile`. For example, it may look like:
```
#MODEL=TEENSY32
#MODEL=TEENSY35
MODEL=TEENSY36
```

## Compiling, flashing and running
Build project in release mode with
```
make
```
Plug Teensy in with usb cable, and flash it with
```
make flash
```
You may need to press button on board to finish the flash. If everything worked out, you should see quickly blinking led now. Reading stdout from Teensy is important, and Linux users can do it by running provided shell script:
```
./read_output_from_usb
``` 
This script prints sometimes excess new lines, so keep that in mind if output seems sparse.


## Other makefile usage
Debuggin build:
```
make debug
```
Run clippy
```
make clippy
```
Generate documentation for the project and bindings:
```
make doc
```
The generated documentantation can be opened from file
```
target/thumbv7em-none-eabi/doc/teensy3_sys/index.html
```



## Safe Components

Items used from the `teensy3` crate directly can be used as safe rust code. Notice how there is no `unsafe` marker:

```rust
extern crate teensy3;
use teensy3::util::{delay};
use teensy3::pins::{Pin, PinRow};

#[no_mangle]
pub extern fn main() {
    let mut pinrow = PinRow::new_once();
    let mut led = pinrow.get_led();
    loop {
        blink_safe(&mut led);
    }
}

pub fn blink_safe(led: &mut Pin) {
    // Blink led with custom wrapper
    for _ in 0..10 {
        led.digital_write(true);
        delay(50);
        led.digital_write(false);
        delay(50);
    }
}
```

Items used from the `teensy3::bindings` module are NOT marked as safe (because they are direct C++ code mappings). These require an `unsafe` mark:

```rust
extern crate teensy3;

#[no_mangle]
pub extern fn main() {
    loop {
        blink_unsafe();
    }
}

pub fn blink_unsafe() {
    // Blink led with raw bindings
    for _ in 0..10 {
        unsafe{bindings::digitalWrite(13, bindings::HIGH as u8)};
        unsafe{bindings::delay(50)};
        unsafe{bindings::digitalWrite(13, bindings::LOW as u8)};
        unsafe{bindings::delay(50)};
    }
}
```
Only few selected interfaces have safe wrapper in `teensy3`, so in most cases use of unsafe bindings are needed.

# License

Rust contributions are licensed under the MIT License.

**Please Note:** ASM, C, C++, and Linker Components of the `teensy3-sys` crate (a dependency of the `teensy3` crate) contain components licensed under the MIT License, PJRC's modified MIT License, and the LGPL v2.1. Please refer to individual components for more details.
