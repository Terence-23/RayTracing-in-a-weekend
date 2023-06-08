# Overview
My attempt at [RayTracing in one weekend](https://raytracing.github.io) project. I wanted to attempt to write it in multiple languages. For now i have settled for C++, Rust and Zig.

# Build instructions
## C++
Needs `cmake>=3.10`. Go to the c++ directory. Then 
```
mkdir build
cd build
cmake ..
cmake --build .
```
## Rust
For Rust go to Rust directory and use `cargo build` to download dependencies and build the project.
## Zig
For Zig go to Zig directory and use `zig build`. Needs at least Zig master (0.11.0+) with stage2 self-hosted compiler for zigimg. 
