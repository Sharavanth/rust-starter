#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::rtt_init_print;
use stm32f4xx_hal as hal;

use crate::hal::{pac, prelude::*};

#[entry]
fn main() -> ! {
    rtt_init_print!(); // You may prefer to initialize another way

    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        // Get the GPIO Port pins
        let gpioc = dp.GPIOC.split();
        // setting up the inbuilt led
        let mut pc13 = gpioc.pc13.into_push_pull_output();

        loop {
            // On for 1s, off for 1s.
            pc13.set_high();
            delay.delay_ms(500_u32);
            pc13.set_low();
            delay.delay_ms(4000_u32);
        }
    }

    loop {}
}
