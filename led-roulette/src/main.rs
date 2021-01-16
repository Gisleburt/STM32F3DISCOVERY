//! Initialization code

#![deny(unsafe_code)]
#![no_std]
#![no_main]

#[allow(unused_extern_crates)] // NOTE(allow) bug rust-lang/rust#53964
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

    let half_period = 500_u16;

    loop {
        (0..8).into_iter().for_each(|led| {
            leds[led].on();
            delay.delay_ms(half_period);
            leds[led].off();
        })
    }
}
