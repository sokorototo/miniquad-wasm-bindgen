# Miniquad

[![Github Actions](https://github.com/not-fl3/miniquad/workflows/Cross-compile/badge.svg)](https://github.com/not-fl3/miniquad/actions?query=workflow%3A)
[![Docs](https://docs.rs/miniquad/badge.svg?version=0.3.13)](https://docs.rs/miniquad/0.3.13/miniquad/index.html)
[![Crates.io version](https://img.shields.io/crates/v/miniquad.svg)](https://crates.io/crates/miniquad)
[![Discord chat](https://img.shields.io/discord/710177966440579103.svg?label=discord%20chat)](https://discord.gg/WfEp6ut)
[![Matrix](https://img.shields.io/matrix/quad-general:matrix.org?label=matrix%20chat)](https://matrix.to/#/#quad-general:matrix.org)

Miniquad is a manifestation of a dream in a world where we do not need a deep dependencies tree and thousands lines of code to draw things with a computer.

Miniquad aims to provide a graphics abstraction that works the same way on any platform with a GPU, being as light weight as possible while covering as many machines as possible.

## Supported Platforms

* Windows, OpenGL 3, OpenGL 2.2;
* Linux, OpenGL 2.2, OpenGL 3, GLES 2, GLES 3;
* WASM, WebGL 2 - tested on Firefox, Chrome;

## Examples

![Imgur](https://i.imgur.com/TRI50rk.gif)

[examples/quad.rs](https://github.com/not-fl3/miniquad/blob/master/examples/quad.rs): [web demo](https://not-fl3.github.io/miniquad-samples/quad.html)<br/>
[examples/offscreen.rs](https://github.com/not-fl3/miniquad/blob/master/examples/offscreen.rs): [web demo](https://not-fl3.github.io/miniquad-samples/offscreen.html)<br/>

[PonasKovas/miniquad-mandelbrot](https://github.com/PonasKovas/miniquad-mandelbrot): [web demo](https://ponaskovas.github.io/miniquad-mandelbrot-wasm-demo/)

# Building examples

## Linux

```bash
cargo run --example quad
```

On NixOS Linux you can use [`shell.nix`](shell.nix) to start a development
environment where Miniquad can be built and run.

## Windows

```bash
# both MSVC and GNU target is supported:
rustup target add x86_64-pc-windows-msvc
# or
rustup target add x86_64-pc-windows-gnu

cargo run --example quad
```

## WASM

... uses wasm-bindgen, should work out of the box in such a context

## Cross Compilation

```bash

# windows target from linux host:
# this is how windows builds are tested from linux machine:
rustup target add x86_64-pc-windows-gnu
cargo run --example quad --target x86_64-pc-windows-gnu
```
