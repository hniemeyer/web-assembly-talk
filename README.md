# web-assembly-talk
Slides and Code for my talk "WebAssembly - Running C++ and Rust in a Web Browser"

## Code

The folder code contains the examples for generating and running WebAssembly from C++
and Rust code.

### cpp-clang

Compiling C++ to WebAssembly using just clang. 

Dependencies:
* clang compiler
* Python for running the web server (can be exchanged easily for any web server you want)
* [wasm2wat](https://github.com/WebAssembly/wabt) if you want to decompile the wasm file to human readable WAT (WebAssemblyText) yourself 

Usage:
* compile.sh will compile the C++ to wasm.
* run.sh will start a web server on port 9000 using pythons http.server (then you can go to localhost:9000/index.html)


### cpp-emscripten

Compiling C++ to WebAssembly using emscripten. Both WebAssembly embedded in a HTML webpage for running inside the browser
and freestanding WebAssembly for running with a wasm-runtime directly on the machine are generated.

Dependencies:
* [Emscripten](https://emscripten.org/)
* [Wasmer](https://wasmer.io/) for running the freestanding WebAssembly example outside of the browser

### rust

Compiling rust to WebAssembly and running it in the browser.

Dependencies:
* [Rust toolchain](https://www.rust-lang.org/)
* [NodeJS](https://nodejs.org/en/)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/)

## Slides

The folder slides contains the presentation as a PDF.