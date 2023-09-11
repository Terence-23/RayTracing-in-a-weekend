# Overview
My attempt at [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) project. I wanted to attempt to write it in multiple languages. For now I have settled for C++, Rust and Zig. After finishing the first book I decided to implement the second book: [_Ray Tracing: The Next Week_](https://raytracing.github.io/books/RayTracingTheNextWeek.html), but only in Rust.

# Build instructions
## C++
Needs `cmake>=3.10`. Go to the c++ directory. First run 
``` 
git submodule update --init
``` 
In file `C++/lib/tqdm.cpp/include/tqdm/tqdm.h` change line 37 from `#include"tqdm/utils.h"` to  `#include"utils.h"`. Then 
```
mkdir build
cd build
cmake ..
cmake --build .
```
## Rust
For Rust go to Rust directory and use `cargo build` to download dependencies and build the project.
## Zig
For Zig download the submodules, then go to Zig directory and use `zig build`. Needs at least Zig master (0.11.0+) with stage2 self-hosted compiler for zigimg. 
