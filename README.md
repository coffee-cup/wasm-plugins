# WebAssembly Plugin Experimentation

This repo is an experimentation of using WASM modules with a shared component interface as plugins.

## Features

- Shared component interface defined in the [WIT](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md) format. (`wit/handler.wit`)
  - Plugins load the component interface via [wit-bindgen](https://github.com/bytecodealliance/wit-bindgen) which enforces the implementation of the _guest_ side of the component
  - Hosts load the interface via [wasmtime](https://docs.rs/wasmtime/9.0.1/wasmtime/component/macro.bindgen.html) which enforces the implementation of the _host_ side of the component
  - The plugin and server and currently both implemented in Rust, but the interface is language agnostic. As long as the WASM component module confirms to the WIT format, it can be used by as a handler.
- Route handlers are loaded as WASM modules. Currently the only one available is `hello-handler`. These handlers are compiled separately to the `wasm32-wasi` or `wasm32-unknown-unknown` targets
- [Actix](https://actix.rs/) server located in `server/` that dynamically loads WASM modules and executes them as route handlers (currently only 1 on `/` but I will add more when it is not super late)

## How to Use

- Install [wasm-tools](https://github.com/bytecodealliance/wasm-tools)
- Compile the handlers and convert into WASM components: `./build-handlers.sh`. These will show up in `handlers/*.wasm`
- Run the server: `cargo run --package server`
- Go to [localhost:9090](http://localhost:9090)

The response of the request is the result of calling the WASM handler.

The handlers are also dynamically loaded on each request, which means you can change `hello-handler/src/lib.rs`, run `./build-handlers.sh` and refresh the page to see the changes (no need to re-run the server).

## TODO

- Add more handlers and expand the API interface
- Figure out how to run the modules with WASI available
- Re-use the execution engine so we are not re-creating it for each request
- Dynamically register handlers while the server is already running (possible with a request to POST /handler)
- Implement a handler in a non-Rust language
- See what a host in JS looks like

## Resources

_Note: Documentation for a lot of these libraries/tools is very sparse and often outdated. I've tried to compile a list of resources here, but use at your own risk._

- [Introduction to WebAssembly components](https://radu-matei.com/blog/intro-wasm-components/)
  - All of Radu's blog posts on wasm are great (even of 2-3 years old and often outdated)
- [WIT Bindgen](https://github.com/bytecodealliance/wit-bindgen)
- [Wasmtime crate](https://docs.rs/wasmtime/latest/wasmtime/index.html)
- [WIT spec](https://github.com/WebAssembly/component-model/blob/main/design/mvp/WIT.md)
- [WebAssembly binary toolkit](https://github.com/WebAssembly/wabt)
- [WASI preview1 spec](https://github.com/WebAssembly/WASI/blob/main/legacy/preview1/docs.md)
