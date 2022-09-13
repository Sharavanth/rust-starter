#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

use crate::hal::{
    adc::{config::AdcConfig, Adc},
    pac,
    prelude::*,
};

#[entry]
fn adc_1() -> ! {
    rtt_init_print!(); // You may prefer to initialize another way

    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Set up the system clock. We want to run at 48MHz for this one.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.MHz()).freeze();

        // Get the GPIO Port A pins
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // setting up the pwm output pin
        let pa8 = gpioa.pa8.into_alternate();
        let mut pwm = dp.TIM1.pwm_hz(pa8, 1.kHz(), &clocks).split();
        let max_duty = pwm.get_max_duty();
        pwm.enable();
        // setup the adc pin from gpio
        let mut pa3 = gpioa.pa3.into_analog();
        // setting up the inbuilt led
        let mut led = gpioc.pc13.into_push_pull_output();

        // Create a delay abstraction based on SysTick
        let mut delay = cp.SYST.delay(&clocks);

        // setup the adc converter
        let mut adc = Adc::adc1(dp.ADC1, true, AdcConfig::default());

        loop {
            if let Ok(sample) = adc.read(&mut pa3) {
                let millivolts = adc.sample_to_millivolts(sample);
                rprintln!("an1: {}mV", millivolts);

                // Scale 12bit ADC value to the range of 0..=max_duty
                let scale = sample as f32 / 0x0FFF as f32;
                pwm.set_duty((scale * max_duty as f32) as u16);
            } else {
                continue;
            }

            led.set_high();
            delay.delay_ms(200_u32);
        }
    }

    loop {}
}
