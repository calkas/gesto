//crate lvl macro
#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::gpio::gpiod::Parts;
use stm32f4xx_hal::gpio::{Output, Pin, PushPull};
use stm32f4xx_hal::timer::*;
use stm32f4xx_hal::uart::{Config, Serial};
use stm32f4xx_hal::{pac, prelude::*};

type LedPinOut = (
    Pin<'D', 12, Output<PushPull>>,
    Pin<'D', 13, Output<PushPull>>,
    Pin<'D', 14, Output<PushPull>>,
    Pin<'D', 15, Output<PushPull>>,
);

fn get_led(led_port: Parts) -> LedPinOut {
    let led_green = led_port.pd12.into_push_pull_output();
    let led_red = led_port.pd13.into_push_pull_output();
    let led_orange = led_port.pd14.into_push_pull_output();
    let led_blue = led_port.pd15.into_push_pull_output();

    (led_green, led_red, led_orange, led_blue)
}

//iteam lvl macro
#[entry]
fn main() -> ! {
    // ---------------- CONFIGURATION ----------------

    // 1. Device Peripherals and clocks
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz()) // zewnętrzny kwarc 8 MHz
        .sysclk(168.MHz()) // systemowy zegar 168 MHz
        .pclk1(42.MHz()) // APB1 max 42 MHz
        .pclk2(84.MHz()) // APB2 max 84 MHz
        .freeze(); // zatwierdzenie konfiguracji

    // 2. User Button config
    let gpioa = dp.GPIOA.split();
    let button = gpioa.pa0.into_input();

    // 3. Led config
    let gpiod = dp.GPIOD.split();
    let mut led = get_led(gpiod);
    led.0.set_high();

    // 4. USART1 config alternative AF7
    let gpiob = dp.GPIOB.split();
    let usart_tx_pin = gpiob.pb6.into_alternate::<7>();
    let usart_rx_pin = gpiob.pb7.into_alternate::<7>();

    let serial: Serial<pac::USART1, u8> = Serial::new(
        dp.USART1,
        (usart_tx_pin, usart_rx_pin),
        Config::default()
            .baudrate(115200.bps())
            .wordlength_8()
            .parity_none(),
        &clocks,
    )
    .unwrap();

    // 5. Delay config
    // For system frequency more than 65 MHz
    let mut delay = dp.TIM1.delay_us(&clocks);

    // ---------------- CONFIGURATION DONE ----------------

    let (mut tx, _rx) = serial.split();
    writeln!(tx, "Config Done\r").unwrap();

    led.3.set_high();

    let mut last_button_state = false;
    loop {
        let current_button_state = button.is_high();

        if current_button_state && !last_button_state {
            delay.delay(20.millis());
            if button.is_high() {
                led.2.toggle();
                writeln!(tx, "UART calling ").unwrap();
            }
        }

        last_button_state = current_button_state;

        //Small delay for CPU
        delay.delay(1.millis());
    }
}
