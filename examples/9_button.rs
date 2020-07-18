//! Example on using HAL to blink a LED via a button.
//! The LED and button is a resource.

#![no_main]
#![no_std]

use embedded_hal::digital::v2::OutputPin as _;
use nrf52840_hal::{
    gpio::{Level, Output, Pin, PushPull},
    gpiote::*,
};
use panic_halt as _;
use rtic::{app, cyccnt::U32Ext};

#[app(device = nrf52840_hal::pac, peripherals = true, monotonic = rtic::cyccnt::CYCCNT)]
const APP: () = {
    struct Resources {
        led: Pin<Output<PushPull>>,
        gpiote: Gpiote,
    }

    #[init]
    fn init(cx: init::Context) -> init::LateResources {
        // When using schedule and a monotonic timer, remember to start the timer!

        // This is the `cortex_m::Peripherals` struct without the SysTick which
        // RTIC has taken ownership of.
        let mut cp = cx.core;

        // Initialize (enable) the monotonic timer (CYCCNT)
        cp.DCB.enable_trace();
        cp.DWT.enable_cycle_counter();

        // Enable logging
        app::init();
        log::info!("Hello from init!");

        let p0 = nrf52840_hal::gpio::p0::Parts::new(cx.device.P0);

        // Set up a LED
        let mut led = p0.p0_13.degrade().into_push_pull_output(Level::High);
        let _ = led.set_high();

        // Set up the button for interrupts
        let button = p0.p0_11.into_pullup_input().degrade();
        let gpiote = Gpiote::new(cx.device.GPIOTE);
        gpiote
            .channel0()
            .input_pin(&button)
            .hi_to_lo()
            .enable_interrupt();

        init::LateResources {
            // Move the LED to the resources.
            led,
            // Move Gpiote to the resources.
            gpiote,
        }
    }

    #[idle]
    fn idle(_cx: idle::Context) -> ! {
        log::info!("Hello from idle!");

        loop {
            //continue;
            cortex_m::asm::wfi(); // NEEDED?
        }
    }

    #[task(binds = GPIOTE, resources = [gpiote], spawn = [blinky])]
    fn button_event(cx: button_event::Context) {
        let gpiote = cx.resources.gpiote;

        log::info!("Spawning blinky from button!");

        cx.spawn.blinky().ok();

        gpiote.channel0().reset_events();
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
        fn TIMER0();
    }
};
