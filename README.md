# Dalted with WebAssembly

![alt text](./static/img/logo.svg "Dalted logo black")  

Web application to simulate color blindness across the spectrum in WASM.

> For the server version with actix check https://github.com/carrascomj/dalted.

## License

Licensed under either of

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.

## Run locally

The first step is to [install Rust](https://www.rust-lang.org/tools/install):

```bash
# Unix-like OS
curl https://sh.rustup.rs -sSf | sh
```


After cloning this repository, install [trunk](https://trunkrs.dev/) with [cargo](https://doc.rust-lang.org/cargo/guide/creating-a-new-project.html) and run it:

```bash
git clone https://github.com/carrascomj/dalted.git
cargo install trunk
cd dalted
# we need the nightly toolchain for some leptos QOL
rustup default nightly
# enable --release for optimized compilation (but slower build process)
trunk server --open #--release
```
