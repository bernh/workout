# WASM

## Build

we are not using a bundler like webpack and we have no interest in npm: 

    wasm-pack build --target web

is the way to go generate the wasm code and the accompaning JS suitable for direct usage
within a browser.

See https://rustwasm.github.io/docs/wasm-bindgen/reference/deployment.html and
https://rustwasm.github.io/docs/wasm-bindgen/examples/without-a-bundler.html

## Test

Using WASM modules directly from the file system does not work due to some MIME type
restriction therefore a local http server is needed. Easiest way to setup a local development
server:

    python -m http.server
