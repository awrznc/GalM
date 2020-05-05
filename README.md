# GalM

[![Actions Status](https://github.com/awrznc/GalM/workflows/Build/badge.svg)](https://github.com/awrznc/GalM/actions)
[![Crate](https://img.shields.io/crates/v/galm.svg)](https://crates.io/crates/galm)

GalM is pattern matching library.

![galm](https://awrznc.github.io/GalM/assets/image/galm.png)

Inspired by Galmoji.

## Quick start

Put the following in your project's Cargo.toml file:

```toml
[dependencies]
galm = "0.0.6"
```

And overwrite in your project's main.rs file:

```rust
// Get the matching rate.
fn main() {

    // Initialize GalM Database instance.
    let galm: galm::Database = galm::new();

    // Get characters similar to the passed in the argument.
    let distance: u8 = galm.get_distance("王", "玉");

    assert_eq!(distance, 30);
}
```

Corresponds to the following characters.

```text
一右雨円王音下火花貝
学気九休玉金空月犬見
五口校左三山子四糸字
耳七車手十出女小上森
```

## Example

Print the most similar string from the strings separated by commas.

```bash
# build
cargo build --example galm --release

# use galm
echo -e "皇様\n玉様\n大様" | ./target/release/examples/galm "王様"
# => 玉様
```

## Install GalM Command

```bash
# install galm
cargo install galm --example galm

# using galm
echo -e "皇様\n玉様\n大様" | galm "王様"
# => 玉様
# => 皇様
# => 大様
```
