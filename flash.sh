#!/bin/bash

# ELF file path
ELF_PATH="target/thumbv7em-none-eabihf/debug/gesto"

# Check if the file exists
if [ ! -f "$ELF_PATH" ]; then
    echo "Error: ELF does not exist: $ELF_PATH"
    exit 1
fi

# Flashing with OpenOCD
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg \
  -c "program $ELF_PATH verify reset exit"
