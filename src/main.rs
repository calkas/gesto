//crate lvl macro
#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use panic_halt as _;
use stm32f4xx_hal::gpio::gpiod::Parts;
use stm32f4xx_hal::gpio::{Output, Pin, PushPull};
use stm32f4xx_hal::spi::{Mode, Phase, Polarity, Spi};
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

    // 5. SPI1 config alternative AF5
    let gpioe = dp.GPIOE.split();
    let mut spi_cs = gpioe.pe3.into_push_pull_output();
    let spi_sck = gpioa.pa5.into_alternate::<5>();
    let spi_miso = gpioa.pa6.into_alternate::<5>();
    let spi_mosi = gpioa.pa7.into_alternate::<5>();

    let spi_mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };

    let mut spi = Spi::new(
        dp.SPI1,
        (spi_sck, spi_miso, spi_mosi),
        spi_mode,
        1.MHz(),
        &clocks,
    );

    // 6. Delay config
    // For system frequency more than 65 MHz
    let mut delay = dp.TIM1.delay_us(&clocks);

    // ---------------- CONFIGURATION DONE ----------------

    let (mut tx, _rx) = serial.split();
    writeln!(tx, "Config Done").unwrap();

    led.3.set_high();

    let mut last_button_state = false;
    loop {
        let current_button_state = button.is_high();

        if current_button_state && !last_button_state {
            delay.delay(20.millis());
            if button.is_high() {
                led.2.toggle();

                // WHO_AM_I (0Fh) to LIS302DL
                spi_cs.set_low();

                //Bit 7    = 1 (odczyt)
                //Bit 6    = 0 (pojedynczy odczyt)
                //Bity 5-0 = adres rejestru

                let read_cmd = 0x80 | 0x0F; // 0x8F
                spi.write(&[read_cmd]).unwrap();
                let mut who_am_i_id = [0];
                spi.write(&[0x00]).unwrap(); // dummy write
                spi.read(&mut who_am_i_id).unwrap();
                spi_cs.set_high();

                // Should be 3Bh
                write!(tx, "LIS302DL Id: {:#X} ", who_am_i_id[0]).unwrap();
            }
        }

        last_button_state = current_button_state;

        //Small delay for CPU
        delay.delay(1.millis());
    }
}
