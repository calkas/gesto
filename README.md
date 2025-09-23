# gesto
This project implements a gesture recognition system based on accelerometer data, running on the STM32F4 Discovery development board, using the Rust programming language and TinyML for lightweight machine learning inference.

## Hardware
`STM32F4-Discovery Board` - STM32F407VGT6 microcontroller featuring 32-bit Arm® Cortex-M4 with FPU core, 1-Mbyte flash memory and 192-Kbyte RAM in an LQFP100 package.

[Data Sheet](https://www.st.com/resource/en/datasheet/dm00037051.pdf)


## Software

### Setup

https://github.com/arthurggordon/emb-rust



Cortex-M4 Thumb-2 Instruction Set

Compilator
> thumbv7em-none-eabi 

Instruction:

1. Add the Cortex-M4 support target to the Rust compiler

> rustup target add thumbv7em-none-eabi

2. Build
> cargo build --target thumbv7em-none-eabi

Created .cargo/config file to use cargo build

## Libs
 - cortex-m - Dostęp do instrukcji i rejestrów ARM
 - cortex-m-rt - Runtime i przerwania
 - stm32f4xx-hal - Obsługa peryferiów STM32
