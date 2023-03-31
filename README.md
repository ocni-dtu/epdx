# EPDx

EPDx is a library for parsing EPD files into a common exchange format.

The library is written in Rust, to allow for cross-language package distribution.
Currently, we support Javascript/Typescript, Python and Rust.

EPDx is part of a larger project of making life cycle assessments more accessible and transparent to building
professionals.

Read more about our [overall goal](https://github.com/ocni-dtu/life-cycle-formats)

# Documentation

EPDx can have available packages for Javascript, Python and Rust.

To get started head over to our [documentation](https://epdx.kongsgaard.eu).

## Install NPM Package

```bash
npm install epdx
```

## Install Python Package

```bash
pip install epdx
```

## Install Rust Crate

```bash
cargo add epdx
```

# Contribute

## Install Rust
Head over to Rust's installation [page](https://www.rust-lang.org/tools/install)

## Run Tests

```bash
cargo test --package epdx --target x86_64-unknown-linux-gnu
```