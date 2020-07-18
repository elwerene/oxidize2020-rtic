//! Example for the app macro.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;

#[app(device = nrf52840_hal::pac)]
const APP: () = {
    // RTIC app is written in here!
};
