//! Example on spawning a software/hardware task.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;

#[app(device = nrf52840_hal::pac)]
const APP: () = {
    struct Resources {
        // Resources go here!
    }

    #[init(spawn = [hello_world_task])]
    fn init(cx: init::Context) {
        // Enable logging
        app::init();

        // Any spawn in init will run after init finishes.
        cx.spawn.hello_world_task().ok();

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
        log::info!("Hello world from task!");
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks

    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIMER1();
    }
};
