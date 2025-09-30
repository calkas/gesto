//crate lvl macro
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;

use stm32f4xx_hal::gpio::gpiod::Parts;
use stm32f4xx_hal::gpio::{Output, Pin, PushPull};
use stm32f4xx_hal::{pac, prelude::*};

fn get_led(
    led_port: Parts,
) -> (
    Pin<'D', 12, Output<PushPull>>,
    Pin<'D', 13, Output<PushPull>>,
    Pin<'D', 14, Output<PushPull>>,
    Pin<'D', 15, Output<PushPull>>,
) {
    let mut led_green = led_port.pd12.into_push_pull_output();
    let mut led_red = led_port.pd13.into_push_pull_output();
    let mut led_orange = led_port.pd14.into_push_pull_output();
    let mut led_blue = led_port.pd15.into_push_pull_output();

    (led_green, led_red, led_orange, led_blue)
}

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

    let mut led = get_led(gpiod);
    led.0.set_high();
    led.1.set_high();

    loop {
        continue;
    }
}
