# Assembly files

The sources files in this directory should be assembled using
[vasm](http://sun.hasenbraten.de/vasm/) targeting `Motorola 68000` with the [Motorola syntax](http://sun.hasenbraten.de/vasm/release/vasm_4.html#Mot-Syntax-Module)

To not force everyone who wants to build this project to compile vasm we are
checking in the assembled files into the repository

# VASM

Compile `vasm` with the following arguments:
`make CPU=m68k SYNTAX=mot`

This will build a file called `vasmm68k_mot`, make sure you put it on path so it
works with the provided build script

# Building ASM files

Run the `build.sh` script in this folder

# Notes

*Do not make changes directly to the assembled files, as all changes will be overwritten
when somebody re-builds it*
