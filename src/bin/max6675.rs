#![deny(unsafe_code)]
#![allow(clippy::empty_loop)]
#![no_main]
#![no_std]

use crate::hal::{pac, prelude::*};
use bit_field::BitField;
use cortex_m_rt::entry;
use hal::{
    gpio::{Alternate, Output, Pin},
    pac::SPI2,
    spi::{Mode, Phase, Polarity, Spi},
};
use panic_probe as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f4xx_hal as hal;

pub const MODE: Mode = Mode {
    phase: Phase::CaptureOnSecondTransition,
    polarity: Polarity::IdleHigh,
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
        let gpiob = dp.GPIOB.split();
        let gpioc = dp.GPIOC.split();

        // setting up the inbuilt led
        let mut led = gpioc.pc13.into_push_pull_output();

        // SERIAL MAX6675 - SPI2
        let sck = gpiob.pb13;
        let miso = gpiob.pb14.into_alternate::<5>();
        let mosi = gpiob.pb15;
        let mut tc_spi = dp.SPI2.spi((sck, miso, mosi), MODE, 10.kHz(), &clocks);
        let mut pb12 = gpiob.pb12.into_push_pull_output();
        let mut delay = cp.SYST.delay(&clocks);

        loop {
            if let Ok(temp) = get_temp((&mut tc_spi, &mut pb12)) {
                rprintln!("Temp: {} deg C", temp);
            } else {
                continue;
            }

            led.set_high();
            delay.delay_ms(200_u32);
        }
    }

    loop {}
}

fn get_temp(
    tc: (
        &mut Spi<SPI2, (Pin<'B', 13>, Pin<'B', 14, Alternate<5>>, Pin<'B', 15>)>,
        &mut Pin<'B', 12, Output>,
    ),
) -> Result<f32, &'static str> {
    let (spi2, cs) = tc;
    spi2.enable(true);

    let mut buffer: [u8; 2] = [0; 2];
    cs.set_low();
    if let Ok(data) = spi2.transfer(&mut buffer) {
        cs.set_high();
        let raw = (data[0] as u16) << 8 | (data[1] as u16) << 0;

        let fault = raw.get_bit(2);
        if fault {
            return Err("Thermouple is open");
        }

        let count: f32 = (raw.get_bits(3..=14)).into();
        rprintln!("raw: {}, {}, {}, {}", fault, data[0], data[1], count);
        Ok((count * 0.25).floor())
    } else {
        Err("Could not read temp")
    }
}
