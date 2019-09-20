# WASM beginners week

## Glossary

|WASM	|Web Assembly abbreviation	|
|---	|---	|
|WASI	|Web Assembly System Interface	|
|WAT	|Web Assembly Text	|
|	|	|
|	|	|
|	|	|

## Problem

WASM is a standard binary format for instruction stack-based virtual machines and it is designed to be a portable target for high level languages like C, C++ and Rust. Most modern browsers implement support for the WASM runtime and implementations of this binary format are not restricted to browsers.

WASI is a system interface to run WebAssembly outside of the browser and aims to expose access to filesystem, networking and other system facilities

With WASI it is possible to implement virtual machines to execute the wasm binary format outside of the browser 

## Observation

Initially C/C++ only then Rust team decided to dedicate a team to WebAssembly and since then has been dominating the tooling space of WASM/WASI, many tools and libraries are in heavy development and rapidly evolving.

Interesting use-cases for running WASM binaries on the web for native performance and security capabilities of the sandbox versus runtimes implemented to run WASM binaries outside of the browsers 

Rust tools for WASM/WASI are plenty, here are some examples:

wasm-pack       - Compile, build and test wasm with targets for webpack, nodejs and no-modules
wasmer             - Runtime implemented in Rust by Mozilla
wasm-bindgen - Rust library to generate wasm files and JS wrappers using `#[wasm_bindgen]` macro 

## Actions

* Compiled a hello world and generated a wasm file with webpack bundler to be loaded in the browser
    * Tweak the hello world example to use a native JS function (alert)
    * Tweak the hello world example to call console.log in JS namespace
* Compiled a regular rust program to `wasm32-wasi` target
    * TODO: gather files and upload to a repository
    * Ran with `wasmer`
    * Ran with `wasmtime`
    * Ran with `lucet`
        * Lucet only supports rust and C
        * wasmer is pretty solid
        * wasmtime is base for lucet and possibly wasmer as well, developed by mozilla
    * A libc replacement is necessary for these runtimes to execute the binary
    * [CloudABI.org](http://cloudabi.org/) format is commonly used and is part of web-sysroot and is used by C programs
    * Rust will use the alternative libc (musl + libpropen + [cloudabi.org](http://cloudabi.org/)) when the target is set to WASM
    * Capsicum (https://www.cl.cam.ac.uk/research/security/capsicum/) is the capability model and can be fined tuned for file system access as well as networking and system facilities
* 


