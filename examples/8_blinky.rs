//! Example on using HAL and blinking a LED.
//! The LED is a resource.

#![no_main]
#![no_std]

use embedded_hal::digital::v2::OutputPin as _;
use nrf52840_hal::gpio::{p0, Level, Output, Pin, PushPull};
use panic_halt as _;
use rtic::{app, cyccnt::U32Ext};

#[app(device = nrf52840_hal::pac, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Pin<Output<PushPull>>,
    }

    #[init(spawn = [blinky])]
    fn init(cx: init::Context) -> init::LateResources {
        // When using schedule and a monotonic timer, remember to start the timer!

        // This is the `cortex_m::Peripherals` struct without the SysTick which RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic timer (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Enable logging
        app::init();
        log::info!("Hello from init!");

        // Set up a LED
        let periph = nrf52840_hal::pac::Peripherals::take().unwrap();
        let pins = p0::Parts::new(periph.P0);
        let mut led = pins.p0_13.degrade().into_push_pull_output(Level::High);
        let _ = led.set_high();

        // Start the blinky task!
        cx.spawn.blinky().ok();

        init::LateResources {
            // Move the LED to the resources.
            led,
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        log::info!("Hello from idle!");

        loop {
            continue;
        }
    }

    #[task(schedule = [blinky], resources = [led])]
    fn blinky(cx: blinky::Context) {
        // RTIC's safe static muts!
        static mut FLAG: bool = false;

        // Extract the LED
        let led = cx.resources.led;

        if !(*FLAG) {
            let _ = led.set_low();
            log::info!("LED Off");
        } else {
            let _ = led.set_high();
            log::info!("LED On");
        }

        cx.schedule.blinky(cx.scheduled + 64_000_000.cycles()).ok();

        *FLAG = !*FLAG;
    }

    // Here we list unused interrupt vectors that can be used to dispatch software tasks
    //
    // One needs one free interrupt per priority level used in software tasks.
    extern "C" {
        fn TIMER1();
        fn TIMER2();
    }
};
