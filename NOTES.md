# WASM

https://developer.mozilla.org/en-US/docs/WebAssembly/Rust_to_wasm basically also explains
exactly how to develop a simple Wasm module and integrate it into a web page (without bundlers, 
npm, etc.).

## Build

we are not using a bundler like webpack and we have no interest in npm: 

    wasm-pack build --target web

is the way to go generate the wasm code and the accompaning JS suitable for direct usage
within a browser.

See https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html and
https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html

Attention: Looks like wasm-pack creates a .gitignore file inside the pkg directory. Has to
be considered when commiting and pushing to a webpage.


## Test

Using WASM modules directly from the file system does not work due to some MIME type
restriction therefore a local http server is needed. Easiest way to setup a local development
server:

    python -m http.server

This is what the `serve.sh` script is doing.

# egui

