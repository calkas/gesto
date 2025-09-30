#!/bin/bash

TARGET_PATH="thumbv7em-none-eabihf"

# Path to the ELF file
ELF_PATH="target/$TARGET_PATH/debug/gesto"

echo "Building the project..."
cargo build --target thumbv7em-none-eabihf
echo "Build completed."

echo "Section sizes:"
# Using arm-none-eabi-objdump to display section sizes
arm-none-eabi-objdump -h $ELF_PATH | grep "vector_table\|.text\|.data\|.bss"
