#![deny(unsafe_code)]
#![no_std]
#![no_main]

pub use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};
pub use cortex_m_rt::entry;
use f3::hal::{prelude::*, delay::Delay, stm32f30x};

#[allow(unused_imports)]
use panic_itm; // hello-world handler

pub fn init() -> (ITM, Delay) {
    let cp = cortex_m::Peripherals::take().unwrap();
    let dp = stm32f30x::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let delay = Delay::new(cp.SYST, clocks);

    (cp.ITM, delay)
}


#[entry]
fn main() -> ! {
    let (mut itm, mut delay) = init();
    let period = 1000_u16 / 8;
    let mut count = 0;

    loop {
        iprintln!(&mut itm.stim[0], "Hello, world! {}", count);
        delay.delay_ms(period);
        if count == i32::MAX {
            count = 0;
        } else {
            count += 1;
        }
    }
}
