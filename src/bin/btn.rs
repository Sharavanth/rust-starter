#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
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
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // setting up the PA8 output pin
        let mut pa8 = gpioa.pa8.into_push_pull_output();
        // setup the btn read pin from gpio
        let pa3 = gpioa.pa3.into_pull_down_input();
        // setting up the inbuilt led
        let pc13 = gpioc.pc13.into_push_pull_output_in_state(hal::gpio::PinState::Low);

        loop {
            rprintln!(
                "PA3:{}, PA8:{}, LED:{}",
                pa3.is_high(),
                pa8.is_set_high(),
                pc13.is_set_high()
            );

            if pa3.is_high() {
                pa8.set_high();
            } else {
                pa8.set_low();
            }
            delay.delay_ms(100_u32);
        }
    }

    loop {}
}
