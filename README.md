# Rust recipes

Rust recipes code based on https://www.packtpub.com/programming/rust-programming-recipes-video

## Requirements

* rustc stable >= 1.30
* gcc >= 9
* cmake >= 3
* python >= 2.7

## Stucture

Recipes are store in `src/recipes/$recipe_name`, each recipe is being used in tests for each module/file:

* `src/recipes/error`: error hanlding code and custom error setup
* `src/recipes/iterators`: iterator code
* `src/recipes/lifetime`: lifetime usage and recipes
* `src/recipes/patterns`: common patterns in rust
* `src/recipes/ffi`: FFI usage in rust and other laguages
* `src/recipes/testing`: testing and mocking recipes

## Usage

### Tests

Runs recipes tests

```sh
cargo test
```

### Python FFI

Builds the rust library and runs a python script which loads the rust shared library into it.

```sh
cargo build
./runfromrust.py
```