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

// ---- Parametry wejścia do modelu ----
const TIMESTEPS: usize = 100; // liczba próbek w oknie
const FEATS: usize = 3; // X/Y/Z
const INPUT_LEN: usize = TIMESTEPS * FEATS;

const SAMPLE_DELAY_MS: u32 = 10; // odstęp próbkowania
const DEBOUNCE_MS: u32 = 20;

// Model trzymany w FLASH (rodata)
const MODEL: &[u8] = include_bytes!("../gesture_model_accu86.tflite");

// Prosta normalizacja:
// LIS302DL (±2g): czułość ~18 mg/LSB => 0.018 g/LSB
#[inline]
fn norm_acc(raw: i8) -> f32 {
    const G_PER_LSB: f32 = 0.018;
    (raw as f32) * G_PER_LSB
}

#[inline]
fn argmax(xs: &[f32]) -> usize {
    let mut best_i = 0usize;
    let mut best_v = f32::NEG_INFINITY;
    for (i, &v) in xs.iter().enumerate() {
        if v > best_v {
            best_v = v;
            best_i = i;
        }
    }
    best_i
}

#[entry]
fn main() -> ! {
    // ---------------- CONFIGURATION ----------------

    // 1) Peripherals i zegary
    let dp = pac::Peripherals::take().unwrap();
    let rcc = dp.RCC.constrain();
    let clocks = rcc
        .cfgr
        .use_hse(8.MHz()) // zewnętrzny kwarc 8 MHz
        .sysclk(168.MHz()) // system 168 MHz (F407 max)
        .pclk1(42.MHz()) // APB1
        .pclk2(84.MHz()) // APB2
        .freeze();

    // 2) Przycisk (USER)
    let gpioa = dp.GPIOA.split();
    let button = gpioa.pa0.into_input();

    // 3) LEDy
    let gpiod = dp.GPIOD.split();
    let mut led = Led {
        green: gpiod.pd12.into_push_pull_output(),
        red: gpiod.pd14.into_push_pull_output(),
        orange: gpiod.pd13.into_push_pull_output(),
        blue: gpiod.pd15.into_push_pull_output(),
    };

    // 4) UART (USART1, AF7)
    let gpiob = dp.GPIOB.split();
    let usart_tx_pin = gpiob.pb6.into_alternate::<7>();
    let usart_rx_pin = gpiob.pb7.into_alternate::<7>();
    let serial: Serial<pac::USART1, u8> = Serial::new(
        dp.USART1,
        (usart_tx_pin, usart_rx_pin),
        Config::default()
            .baudrate(115_200.bps())
            .wordlength_8()
            .parity_none(),
        &clocks,
    )
    .unwrap();
    let (mut tx, _rx) = serial.split();

    // 5) SPI1 (AF5) dla LIS302DL
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

    // 6) Delay (TIM1 dla >65 MHz)
    let mut delay = dp.TIM1.delay_us(&clocks);

    // 7) Akcelerometr
    let mut accelerometer = LIS302DL { spi, spi_cs };
    accelerometer.init();

    // ---------------- CONFIGURATION DONE ----------------

    writeln!(
        tx,
        "WHO_AM_I: {:#X} - Config OK",
        accelerometer.get_device_id()
    )
    .ok();
    led.blue.set_high();

    // ---- Inicjalizacja modelu ----
    match init_model(MODEL) {
        Ok(()) => {
            writeln!(tx, "Model init: OK").ok();
        }
        Err(_) => {
            writeln!(tx, "Model init: ERR").ok();
            led.orange.set_high();
        }
    }

    writeln!(tx, "Press USER button to capture {TIMESTEPS} samples…").ok();

    let mut last_button = false;
    loop {
        let b = button.is_high();

        if b && !last_button {
            delay.delay(DEBOUNCE_MS.millis());
            if button.is_high() {
                led.green.set_high();
                led.red.set_low();
                writeln!(
                    tx,
                    "Start capture window ({TIMESTEPS} samples @ {} ms)…",
                    SAMPLE_DELAY_MS
                )
                .unwrap();
                let mut input: [f32; INPUT_LEN] = [0.0; INPUT_LEN];

                for i in 0..TIMESTEPS {
                    let x_raw = accelerometer.read_x_axis();
                    let y_raw = accelerometer.read_y_axis();
                    let z_raw = accelerometer.read_z_axis();

                    // Normalizacja -> f32
                    let x = norm_acc(x_raw);
                    let y = norm_acc(y_raw);
                    let z = norm_acc(z_raw);

                    // Spakowanie do formatu modelu: [x0,y0,z0, x1,y1,z1, ...]
                    let base = i * FEATS;
                    input[base + 0] = x;
                    input[base + 1] = y;
                    input[base + 2] = z;

                    delay.delay(SAMPLE_DELAY_MS.millis());
                }

                set_input(&input);
                let status = invoke();
                if status.is_err() {
                    writeln!(tx, "invoke: ERR").unwrap();
                    led.red.set_high();
                } else {
                    let mut output: [f32; 2] = [0.0; 2];
                    get_output(&mut output);
                    let cls = argmax(&output);
                    // 0 → idle, 1 → swipe
                    match cls {
                        0 => writeln!(tx, "idle").unwrap(),
                        1 => writeln!(tx, "swipe").unwrap(),
                        _ => {}
                    }
                }
                led.green.set_low();
                writeln!(tx, "DONE.\n").ok();
            }
        }

        last_button = b;
        delay.delay(1.millis());
    }
}
