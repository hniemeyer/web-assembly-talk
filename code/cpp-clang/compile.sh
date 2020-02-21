#!/bin/bash
clang++ --target=wasm32 -nostdlib -Wl,--no-entry -Wl,--export-all -o add.wasm -O3 add.cpp