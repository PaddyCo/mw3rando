#!/usr/bin/env bash

cd "${0%/*}" || return

for filename in *.asm; do
    echo "Building $filename..."
    vasmm68k_mot "$filename" -o "obj/$(basename "$filename" .asm)".o -ldots -spaces -align -Fbin -Iinclude
done
