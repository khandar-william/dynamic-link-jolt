# Recreating Jolt Issue with `cdylib`

I intend to use Jolt as dynamic link library (.so file + `libloading` crate) but I encountered an issue where the `libloading` fails to load the .so file.

# Content

```
├── guest
│   ├── Cargo.toml
│   └── src
│       └── lib.rs
├── lib-producer
│   ├── Cargo.toml
│   ├── rust-toolchain.toml
│   └── src
│       └── lib.rs
├── lib-consumer
│   ├── Cargo.toml
│   └── src
│       └── main.rs
```

- `guest` is the sample fibonacci guest code
- `lib-producer` is a Rust library package with target
  ```
  [lib]
  crate-type = ["rlib", "cdylib"]
  ```
  - this will produce `lib-producer/target/release/liblib_producer.so` file
- `lib-consumer` is a Rust app that tries to load dynamic library file from `lib-producer`

# Steps to reproduce

1. Go to `lib-producer/`
   1. Run `cargo build -r`
   1. This will produce file `lib-producer/target/release/liblib_producer.so`
1. Go to `lib-consumer/`
   1. Run `cargo run`
   1. The output showing errors

## The Error

```
lib-consumer$ cargo run
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.07s
     Running `target/debug/lib-consumer`
Entered lib-consumer
thread 'main' panicked at src/main.rs:8:58:
called `Result::unwrap()` on an `Err` value: DlOpen { desc: "../lib-producer/target/release/liblib_producer.so: undefined symbol: _HEAP_PTR" }
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```
