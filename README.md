# l0g

Opinionated l0gging facade meant for the `no_std` libraries that are also sometimes used in a `std` context.

## Overview

Logging facilities differ depending on the environment, especially between `no_std` and `std`. This facade allows to delegate the decision which l0gging implementation shall be used to the top-level application.

Any call to `l0g::{error,warn,info,debug,trace}` turns into a
- noop when no feature is set
- `log::{error,warn,info,debug,trace}` from the [log](https://crates.io/crates/log) crate when `log` feature is set
- `defmt::{error,warn,info,debug,trace}` from the [defmt](https://crates.io/crates/defmt) crate when `defmt` feature is set

Moreover, proc-macro is provided to generalize over which formatting implementation should be used

```rust
#[l0g::format]
struct MyStruct {
    value: u8
}
```
turns into
```rust
#[derive(core::fmt::Debug)]
struct MyStruct {
    value: u8
}
```
if `log` feature is used or
```rust
#[derive(defmt::Format)]
struct MyStruct {
    value: u8
}
```
if `defmt` feature is used.

This allows a `{:?}` formatting to work, regardless of which logging implementation is chosen.

## Usage

In the `Cargo.toml` of your libraries just say

```toml
l0g = "1"
```

In the `no_std` top-level application specify the dependency with the `defmt` feature. For how to make `defmt` work, take a look into its respective documentation.

```toml
l0g = { version = "1", features = ["defmt"] }
```

In the std top-level application specify the dependency with the `log` feature. For how to make `log` work, take a look into its respective documentation.

```toml
l0g = { version = "1", features = ["log"] }
```
