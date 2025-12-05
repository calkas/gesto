#!/bin/bash

# Check if argument is provided
if [ $# -eq 0 ]; then
    echo "Usage: $0 <gesture_capture|gesture_detector>"
    echo "Example: $0 gesture_detector"
    exit 1
fi

BINARY_NAME="$1"

# Validate binary name
if [ "$BINARY_NAME" != "gesture_capture" ] && [ "$BINARY_NAME" != "gesture_detector" ]; then
    echo "Error: Invalid binary name '$BINARY_NAME'"
    echo "Valid options: gesture_capture, gesture_detector"
    exit 1
fi

# ELF file path
ELF_PATH="target/thumbv7em-none-eabihf/debug/$BINARY_NAME"

# Check if the file exists
if [ ! -f "$ELF_PATH" ]; then
    echo "Error: ELF does not exist: $ELF_PATH"
    echo "Run: cargo build --bin $BINARY_NAME --target thumbv7em-none-eabihf"
    exit 1
fi

echo -e "\e[35mFlashing $BINARY_NAME...\e[0m"

# Flashing with OpenOCD
openocd -f interface/stlink.cfg -f target/stm32f4x.cfg \
  -c "program $ELF_PATH reset exit"

if [ $? -ne 0 ]; then
    echo -e "\e[31mFlashing failed!\e[0m"
    exit 1
fi  

echo -e "\e[32mFlashing completed successfully!\e[0m"
