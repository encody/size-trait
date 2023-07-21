# size-trait

This crate provides traits for restricting the size of type parameters.

## Example

### Zero-sized types

```rust
#![feature(generic_const_exprs)]

use size_trait::ZeroSize;

// This struct can only contain `T` when `T` has a size of 0.
struct Zst<T: ZeroSize<true>>(T);

let _ = Zst([0u8; 0]);
let _ = Zst(());
```

### Fixed-sized types

```rust
#![feature(generic_const_exprs)]

use size_trait::Size;

// This struct can only contain `T` when `T` has a size of 4 bytes.
struct Fixed4Bytes<T: Size<4>>(T);

let _ = Fixed4Bytes([0u8; 4]);
let _ = Fixed4Bytes(0u32);
```

## Warning

This crate relies on the unstable feature `generic_const_exprs`. This feature is only available on nightly Rust. It is also not guaranteed to be stable in the future. Tracking issue [#76560](https://github.com/rust-lang/rust/issues/76560).

In order for this crate to work properly, you must enable the `generic_const_exprs` feature in your crate:

```rust
#![feature(generic_const_exprs)]
```

# Author

- Jacob Lindahl [@sudo_build](https://twitter.com/sudo_build) [geeklaunch.io](https://geeklaunch.io)
