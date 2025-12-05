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

TARGET_PATH="thumbv7em-none-eabihf"

# Path to the ELF file
DEBUG_ELF_PATH="target/$TARGET_PATH/debug/$BINARY_NAME"

echo "Building: $BINARY_NAME for target: $TARGET_PATH"
cargo build --bin $BINARY_NAME --target $TARGET_PATH
echo ""

# Check if build was successful
if [ ! -f "$DEBUG_ELF_PATH" ]; then
    echo "Error: Build failed, ELF file not found: $DEBUG_ELF_PATH"
    exit 1
fi
echo -e "\e[35mBuild for $BINARY_NAME on target $TARGET_PATH\e[0m"
echo -e "\e[32mBuild completed successfully!\e[0m"
echo ""

echo "Section sizes:"
echo "Idx Name          Size      VMA       LMA       File off  Algn"
# Using arm-none-eabi-objdump to display section sizes
arm-none-eabi-objdump -h $DEBUG_ELF_PATH | grep "vector_table\|.text\|.data\|.bss"

echo ""
echo "Binary sizes:"
# Using arm-none-eabi-size to display binary sizes
arm-none-eabi-size $DEBUG_ELF_PATH
