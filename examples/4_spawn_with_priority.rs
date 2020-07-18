//! Example on spawning tasks with different priority.

#![no_main]
#![no_std]

use panic_halt as _;
use rtic::app;


#[app(device = nrf52840_hal::pac)]
const APP: () = {
    struct Resources {
        // Resources go here!
    }

    #[init(spawn = [low_prio_task, high_prio_task])]
    fn init(cx: init::Context) {
        // Enable logging
        app::init();

        // Spawn the low priority task first and the the high priority task.
        cx.spawn.low_prio_task().ok();
        cx.spawn.high_prio_task().ok(); // Even though it is spawned later it will run first!

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
    fn low_prio_task(_cx: low_prio_task::Context) {
        log::info!("Low prio task!");
    }

    #[task(priority = 2)]
    fn high_prio_task(_cx: high_prio_task::Context) {
        log::info!("High prio task!");
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIMER1();
        fn TIMER2();
    }
};
