//crate lvl macro
#![no_main]
#![no_std]

use core::fmt::Write;
use cortex_m_rt::entry;
use gesto::st_disco_handler::leds::Led;
use gesto::st_disco_handler::mems::LIS302DL;
use gesto::tflm_wrapper::tflm_wrapper::*;
use panic_halt as _;
use stm32f4xx_hal::spi::{Mode, Phase, Polarity, Spi};
use stm32f4xx_hal::timer::*;
use stm32f4xx_hal::uart::{Config, Serial};
use stm32f4xx_hal::{pac, prelude::*};

const GESTURE_SAMPLES: usize = 100;
const SAMPLE_DELAY: u32 = 10;
const DEBOUNCING_DELAY: u32 = 20;

const MODEL: &[u8] = include_bytes!("../gesture_model_accu86.tflite");

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
    let mut led = Led {
        green: gpiod.pd12.into_push_pull_output(),
        red: gpiod.pd14.into_push_pull_output(),
        orange: gpiod.pd13.into_push_pull_output(),
        blue: gpiod.pd15.into_push_pull_output(),
    };

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
    let spi_cs = gpioe.pe3.into_push_pull_output();
    let spi_sck = gpioa.pa5.into_alternate::<5>();
    let spi_miso = gpioa.pa6.into_alternate::<5>();
    let spi_mosi = gpioa.pa7.into_alternate::<5>();

    let spi_mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };

    let spi = Spi::new(
        dp.SPI1,
        (spi_sck, spi_miso, spi_mosi),
        spi_mode,
        1.MHz(),
        &clocks,
    );

    // 6. Delay config
    // For system frequency more than 65 MHz
    let mut delay = dp.TIM1.delay_us(&clocks);

    let mut accelerometer = LIS302DL { spi, spi_cs };
    accelerometer.init();

    //let sampling_timer = Timer::new(dp.TIM5, &clocks);

    // ---------------- CONFIGURATION DONE ----------------

    let (mut tx, _rx) = serial.split();

    writeln!(
        tx,
        "WHO_AM_I: {:#X} - All Config Done",
        accelerometer.get_device_id()
    )
    .unwrap();

    led.blue.set_high();

    let mut last_button_state = false;
    let mut start_gesture_sampling = false;

    writeln!(tx, "t,x,y,z,label").unwrap();

    // ---------------- DUMMY FFI INVOKE ----------------

    let res = init_model(MODEL);
    if res.is_err() {
        led.orange.set_high();
    }

    // let input = [0.0; 300];
    // set_input(&input);

    // invoke().unwrap();
    // led.red.set_high();

    // let mut output_model = [0.0_f32; 2];
    // get_output(&mut output_model);

    loop {
        let current_button_state = button.is_high();

        if current_button_state && !last_button_state {
            delay.delay(DEBOUNCING_DELAY.millis());
            if button.is_high() {
                led.green.set_high();
                writeln!(tx, "Start Gesture Sampling").unwrap();
                start_gesture_sampling = true;
            }
        }

        if start_gesture_sampling {
            // SWIPE
            writeln!(tx, "Start collecting swipe:").unwrap();
            for i in 0..GESTURE_SAMPLES {
                let x = accelerometer.read_x_axis();
                let y = accelerometer.read_y_axis();
                let z = accelerometer.read_z_axis();

                writeln!(
                    tx,
                    "{:.2},{},{},{},swipe",
                    (i as f32 * SAMPLE_DELAY as f32 / 1000.0),
                    x,
                    y,
                    z
                )
                .unwrap();
                delay.delay(SAMPLE_DELAY.millis());
            }

            delay.delay(500.millis());

            // IDLE
            writeln!(tx, "Start collecting idle:").unwrap();
            for i in 0..GESTURE_SAMPLES {
                let x = accelerometer.read_x_axis();
                let y = accelerometer.read_y_axis();
                let z = accelerometer.read_z_axis();

                writeln!(
                    tx,
                    "{:.2},{},{},{},idle",
                    (i as f32 * SAMPLE_DELAY as f32 / 1000.0),
                    x,
                    y,
                    z
                )
                .unwrap();
                delay.delay(SAMPLE_DELAY.millis());
            }
            start_gesture_sampling = false;
            led.green.set_low();
            writeln!(tx, "DONE!").unwrap();
        }

        last_button_state = current_button_state;

        //Small delay for CPU
        delay.delay(1.millis());
    }
}
