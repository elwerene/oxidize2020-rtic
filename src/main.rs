//! This is not the file you are looking for!
//!
//! Go to the `examples` folder for all the code. :)

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;

#[app(device = nrf52840_hal::pac)]
const APP: () = {
    #[init]
    fn init(_: init::Context) {}
};
