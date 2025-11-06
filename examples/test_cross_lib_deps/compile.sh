#!/bin/sh

# This script compiles lib1 as a shared lib, and then compiles lib2 and main linking against lib1

echo "Compiling lib1.st to shared library lib1.so"
cargo r -- "lib1/base.st" -o lib1.so --shared --linker=cc -g -Onone # --no-linker-script

echo "Compiling lib2.st to shared library lib2.so, linking against lib1.so"
cargo r -- "lib2/*.st" -o app -i "lib1/*.pli" -L lib1 -l1 --linker=cc -g
# cargo r -- "lib2/*.st" -o app "lib1/*.st" -L lib1 -l1 --linker=cc -g
#
# echo "Compiling lib1.st to LLVM IR"
# cargo r -- "lib1/*.st" -o lib1.ll --ir
# echo "Compiling lib2.st to LLVM IR, linking against lib1"
# cargo r -- "lib2/*.st" -o lib2.ll -i "lib1/*.pli"  --ir
