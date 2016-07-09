# rulinalg

---

## Summary

Rulinalg is a linear algebra library written in Rust that doesn't require heavy external dependencies.

The goal of rulinalg is to provide efficient implementations of common linear algebra techniques
in Rust.

Rulinalg was initially a part of [rusty-machine](https://github.com/AtheMathmo/rusty-machine), a machine
learning library in Rust.

#### Contributing

This project is currently [looking for contributors](CONTRIBUTING.md) of all capacities!

---

## Implementation

This project is implemented using [Rust](https://www.rust-lang.org/).

Currently the library does not make use of any external dependencies - though hopefully
we will have BLAS/LAPACK bindings soon.

---

## Usage

The library usage is described well in the [API documentation](https://AtheMathmo.github.io/rulinalg/) - including example code.

### Installation

The library is most easily used with [cargo](http://doc.crates.io/guide.html). Simply include the following in your Cargo.toml file:

```toml
[dependencies]
rulinalg="0.1.0"
```

And then import the library using:

```rust
extern crate rulinalg;
```

Then import the modules and you're done!

```rust
use rulinalg::matrix::Matrix;

let a = Matrix::new(2,2, vec![1.0, 2.0, 3.0, 4.0]); // Create a 2x2 matrix [[1,2],[3,4]]

let b = Matrix::new(2,3, vec![1.0,2.0,3.0,4.0,5.0,6.0]); // Create a 2x3 matrix [[1.0,2.0,3.0],[4.0,5.0,6.0]]

let c = &a * &b; // Matrix product of a and b
```

More detailed coverage can be found in the [API documentation](https://AtheMathmo.github.io/rulinalg/).