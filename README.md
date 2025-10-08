# gesto
This project implements a gesture recognition system based on accelerometer data, running on the `STM32F4 Discovery` development board, using the Rust programming language and TinyML for lightweight machine learning inference.

## Hardware
`STM32F4-Discovery Board` - STM32F407VGT6 microcontroller featuring 32-bit Arm® Cortex-M4 with FPU core, 1-Mbyte flash memory and 192-Kbyte RAM in an LQFP100 package and frequency up to 168 MHz.

[Data Sheet](https://www.st.com/resource/en/datasheet/dm00037051.pdf)

[Schematic](https://www.st.com/resource/en/schematic_pack/mb997-f407vgt6-b02_schematic.pdf)

### Memory layout

```cpp
---------------------------
|      MEMORY LAYOUT      |
---------------------------
| SRAM                    |
| 0x20000000 - 0x2001FFFF |
---------------------------
| CCM data RAM - STACK    |
| 0x10000000 - 0x1000FFFF |
---------------------------
| FLASH                   |
| 0x08000000 - 0x080FFFFF |
---------------------------
```

### Clock configuration

```
fSYSCLK ​= (fHSE / PLLm) * PLLn / PLLp

Calculation:
 * fHSE = 8MHz
 * PLL_M = 8
 * PLL_N = 336
 * PLL_P = 2

fSYSCLK = (8/8) * 336 / 2 = 168MHz
```

### LEDs

| LED   | Color   | Pin   | Port   |
|-------|---------|-------|--------|
| LD3   | RED     | PD13  | GPIOD  |
| LD4   | BLUE    | PD15  | GPIOD  |
| LD5   | ORANGE  | PD14  | GPIOD  |
| LD6   | GREEN   | PD12  | GPIOD  |


### USART_1

Communication with `FT232` hardware to transfer data from accelerometer to PC

| STM32 PIN   | FT232 PIN  |
|-------------|------------|
| PB6 (TX)    | RX         |
| PB7 (RX)    | TX         |
| GND         | GND        |


### SPI_1

Communication with accelerometer [LIS302DL](https://www.st.com/resource/en/application_note/an2335-lis302dl-3axis--2g8g-digital-output-ultracompact-linear-accelerometer-stmicroelectronics.pdf)

| STM32 PIN   | FUNCTION   |
|-------------|------------|
| PE3         | CS         |
| PA5         | SCK        |
| PA6         | MISO       |
| PA7         | MOSI       |

## Software

### Project configuration

Cortex-M4 Thumb-2 Instruction Set (Compilator __thumbv7em-none-eabi__)

Setup:

[Example Link](https://github.com/arthurggordon/emb-rust)

1. Add the Cortex-M4 support target to the Rust compiler

> rustup target add thumbv7em-none-eabihf

2. Next setup the (memory.x) linker according to:

[Requirements](https://docs.rs/cortex-m-rt/latest/cortex_m_rt/)

3. Build
> cargo build --target thumbv7em-none-eabihf

Created .cargo/config file to use cargo build

## Libs
 - cortex-m - Low level access to Cortex-M processors
 - cortex-m-rt - startup code and minimal runtime for Cortex-M microcontrollers
 - stm32f4xx-hal - Multi device hardware abstraction on top of the peripheral access API for the STMicro STM32F4 series microcontrollers.

## Links
 - [Useful](https://gist.github.com/BlinkingApe/9b4f5202c0294ce47a883633fc94e71b#file-config-toml)
