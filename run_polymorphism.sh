#!/bin/bash

if [ $# -ne 1 ]; then
    echo "Usage: $0 <filename>"
    echo "Example: $0 basic_inheritance_and_method_override.st"
    exit 1
fi

filename="$1"
filepath="tests/lit/single/polymorphism/$filename"

if [ ! -f "$filepath" ]; then
    echo "Error: File $filepath does not exist"
    exit 1
fi

echo "Running polymorphism test: $filename"
echo "Command: cargo r -- --linker=clang -o integration.out tests/lit/util/printf.pli $filepath && ./integration.out"
echo ""

cargo r -- --linker=clang -o integration.out tests/lit/util/printf.pli "$filepath" && ./integration.out