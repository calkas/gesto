use stm32f4xx_hal::gpio::Pin;
use stm32f4xx_hal::gpio::{Output, PushPull};

#[allow(dead_code)]
pub struct Led {
    pub green: Pin<'D', 12, Output<PushPull>>,
    pub red: Pin<'D', 14, Output<PushPull>>,
    pub orange: Pin<'D', 13, Output<PushPull>>,
    pub blue: Pin<'D', 15, Output<PushPull>>,
}
