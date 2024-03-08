# Around Crate

This is the `around` crate, a Rust procedural macro library for calling other functions inside your desired ones.

## Getting Started

To use the `around` crate, add it to your `Cargo.toml` file:

```toml
[dependencies]
around = { version = "0.1.0"}
```

## Usage

The `around` crate provides a procedural macros `before`, `afer` and `both` that enables running another function when expected.

The function must exist and be in scope.

Here's a basic example of how to use it:

```rust
#[around(database_clean)]
fn fancy_func(){
    // Your function code here...
}
```
In this example `database_clean` would be called twice, once before and once after your original function code.

Currently this literally translates to:

```rust
fn fancy_func(){
    database_clean();
    // Your function code here...
    database_clean();
}
```


#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>