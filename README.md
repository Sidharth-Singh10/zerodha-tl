# zerodha-tl

A small Rust crate for working with Zerodha (trading/instrument timeline) or websocket timelines. This repository provides a library (crate) and an example demonstrating how to use the code to connect/process timeline-like data streams.

> Crate: `zerodha-tl` — version 0.1.0 (see `Cargo.toml`)

## Table of contents

- [Overview](#overview)
- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Examples](#examples)
- [Development](#development)

## Overview

`zerodha-tl` is a small Rust library and example set that provides utilities and typed models to work with timeline-style data, websockets, and asynchronous streams. The crate is built on top of Tokio and uses `tokio-tungstenite` for websocket support and `serde` for (de)serialization.

The project structure is minimal and opinionated for library + example usage:

- `src/lib.rs` — crate library entrypoint
- `src/config.rs` — configuration helpers
- `src/models.rs` — domain models and serde types
- `src/utils.rs` — utility helpers
- `examples/try.rs` — an example binary demonstrating usage

## Features

- Async runtime powered by Tokio
- Websocket support via tokio-tungstenite
- Serde-compatible models for JSON parsing
- Small, focused example to get started quickly

## Installation

Ensure you have Rust and Cargo installed (stable toolchain recommended). Then clone and build:

```sh
git clone <repo-url>
cd zerodha-tl
cargo build --release
```

Or for development builds:

```sh
cargo build
```

## Usage

This crate is primarily a library with an example. To run the included example:

```sh
# Run the example named `try` from the examples/ folder
cargo run --example try
```

If you'd like to use the crate in another project, add it to your `Cargo.toml` as a dependency (path or crates.io if published):

```toml
[dependencies]
zerodha-tl = { path = "../zerodha-tl" }
```

Then import the crate in your code and call the public API exposed in `lib.rs`.

## Examples

The `examples/try.rs` file includes a small demonstration of using the crate (connect to a source, use models and utilities). Inspect the file for details and run it as shown in the Usage section.


## Development

Developer workflow:

- Build: `cargo build`
- Run examples: `cargo run --example try`
- Add dependencies via `cargo add` (or edit `Cargo.toml`) and run `cargo build`

