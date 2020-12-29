#!/bin/bash

echo "Building standard library..."
echo ""

cargo build --release

cwd=`pwd`
export lilac="$cwd/target/release/lilac"

cd target

if [[ -d ./std ]] ; then
    rm -r ./std
fi

if [[ -f liblila.so ]] ; then
    rm liblila.so
fi

# Order matters
# Build the core library
$lilac ../corelib/x86_64.ls -o x86_64.o --no-link --pic --no-corelib
$lilac ../corelib/mem.ls -o mem.o --no-link --pic --no-corelib

ar -rc liblila_core.a \
    x86_64.o \
    mem.o

# Build the standard library
$lilac ../stdlib/string.ls -o string.o --no-link --pic
$lilac ../stdlib/io.ls -o io.o --no-link --pic
$lilac ../stdlib/fs.ls -o fs.o --no-link --pic
$lilac ../stdlib/text_io.ls -o text_io.o --no-link --pic

$lilac -o liblila.so --lib \
    string.o \
    io.o \
    fs.o \
    text_io.o
    
rm *.o

as ../stdlib/x64_start.asm -o lrt.o

cd ..

echo "Done"

