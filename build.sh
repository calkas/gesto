#!/bin/bash

TARGET_PATH="thumbv7em-none-eabihf"

# Path to the ELF file
DEBUG_ELF_PATH="target/$TARGET_PATH/debug/gesto"

echo "Building the project..."
cargo build --target thumbv7em-none-eabihf
echo ""
echo "Build completed !"

echo "Section sizes:"
echo "Idx Name          Size      VMA       LMA       File off  Algn"
# Using arm-none-eabi-objdump to display section sizes
arm-none-eabi-objdump -h $DEBUG_ELF_PATH | grep "vector_table\|.text\|.data\|.bss"

echo "Binary sizes:"
# Using arm-none-eabi-size to display binary sizes
arm-none-eabi-size $DEBUG_ELF_PATH
