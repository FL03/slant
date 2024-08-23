# slant

[![license](https://img.shields.io/crates/l/rstm.svg)](https://crates.io/crates/rstm)
[![clippy](https://github.com/FL03/slant/actions/workflows/clippy.yml/badge.svg)](https://github.com/FL03/slant/actions/workflows/clippy.yml)
[![rust](https://github.com/FL03/slant/actions/workflows/rust.yml/badge.svg)](https://github.com/FL03/slant/actions/workflows/rust.yml)

[![crates.io](https://img.shields.io/crates/v/slant.svg)](https://crates.io/crates/slant)
[![docs.rs](https://docs.rs/slant/badge.svg)](https://docs.rs/slant)

***

_**The library is currently in the early stages of development and is not yet ready for production use.**_

Welcome to `slant`! Slant aims to be a complete mathematical toolbox providing an interactive sandbox for visualizing abstract mathematical concepts. The library is inspired by 3blue1brown's [manim](https://github.com/3b1b/manim) animation engine.

## Features

- [ ] `slant-anim` - Animation library
- [ ] `slant-plot` - Plotting library

## Getting Started

### Building from the source

Start by cloning the repository

```bash
git clone https://github.com/FL03/slant.git
cd slant
```

#### _Build the project_

```bash
cargo build --all-features --workspace
```

#### _Run the tests_

```bash
cargo test -F full --workspace
```

## Examples

### Basic Usage

```rust
    extern crate slant;

    fn main() -> Result<(), Box<dyn std::error::Error>> {
        tracing_subscriber::fmt::init();
        tracing::info!("Welcome to slant!");


        Ok(())
    }
```

## Contributing

Pull requests are welcome. For major changes, please open an issue first
to discuss what you would like to change.

Please make sure to update tests as appropriate.
