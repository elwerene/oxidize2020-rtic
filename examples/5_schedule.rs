//! Example on scheduling tasks in the future.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::{app, cyccnt::U32Ext};

#[app(device = nrf52840_hal::pac, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    #[init(schedule = [hello_world_task])]
    fn init(cx: init::Context) {
        // When using schedule and a monotonic timer, remember to start the timer!

        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic timer (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Enable logging
        app::init();

        // Schedule the task 3s into the future (the MCU runs at 64 MHz).
        cx.schedule
            .hello_world_task(cx.start + 192_000_000.cycles())
            .ok();

        log::info!("Hello from init!");
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        log::info!("Hello from idle!");

        loop {
            continue;
        }
    }

    #[task]
    fn hello_world_task(_cx: hello_world_task::Context) {
        log::info!("Hello world from the future!");
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIMER1();
    }
};
