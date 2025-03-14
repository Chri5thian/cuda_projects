#!/bin/bash

set -e  # Exit on error

echo "🚀 Compiling CUDA Kernel..."
nvcc -c src/cuda/kernel.cu -o src/cuda/kernel.o -Xcompiler -fPIC

echo "🚀 Compiling C++ Wrapper..."
g++ -c src/cpp/wrapper.cpp -o src/cpp/wrapper.o -fPIC

echo "🚀 Compiling C Helper Functions..."
gcc -c src/c/helper.c -o src/c/helper.o -fPIC

echo "🚀 Creating Shared Library: libcuda_wrapper.so..."
g++ -shared -o libcuda_wrapper.so src/cuda/kernel.o src/cpp/wrapper.o src/c/helper.o -lcudart

echo "🚀 Moving Shared Library to Target Directory..."
mkdir -p target/debug
mv libcuda_wrapper.so target/debug/

echo "🚀 Setting RUSTFLAGS for linker path..."
export RUSTFLAGS="-L target/debug"

echo "🚀 Building Rust Project..."
cargo build

echo "🚀 Running Rust Project..."
cargo run