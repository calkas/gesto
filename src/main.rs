//crate lvl macro
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::{pac, prelude::*};

//iteam lvl macro
#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let _clocks = rcc
        .cfgr
        .use_hse(8.MHz()) // zewnętrzny kwarc 8 MHz
        .sysclk(168.MHz()) // systemowy zegar 168 MHz
        .pclk1(42.MHz()) // APB1 max 42 MHz
        .pclk2(84.MHz()) // APB2 max 84 MHz
        .freeze(); // zatwierdzenie konfiguracji

    let gpiod = dp.GPIOD.split();
    let mut led = gpiod.pd15.into_push_pull_output();
    led.set_high();
    loop {
        continue;
    }
}
