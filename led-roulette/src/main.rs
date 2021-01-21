//! Initialization code

#![deny(unsafe_code)]
#![no_std]
#![no_main]

extern crate panic_halt; // panic handler

pub use cortex_m_rt::entry;
pub use f3::{
    hal::{delay::Delay, prelude},
    led::Leds,
};

use f3::hal::{prelude::*, stm32f30x};

pub fn init() -> (Delay, Leds) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    let leds = Leds::new(dp.GPIOE.split(&mut rcc.ahb));

    (delay, leds)
}

#[entry]
fn main() -> ! {
    let (mut delay, mut leds) = init();

    let half_period = 1000_u16 / 8;
    let num_leds = leds.len();

    loop {
        (0..num_leds).into_iter().for_each(|led| {
            leds[(led+3) % num_leds].on();
            leds[led].off();
            delay.delay_ms(half_period);
        })
    }
}
