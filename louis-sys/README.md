# louis-sys
Rust bindings to liblouis (low-level crate)

## Dependencies

- `pkg-config` to locate liblouis
- `libclang` (for bindgen)

### liblouis-dev

If a reasonably up-to-date (>=3.1.0) version of liblouis including headers and a `liblouis.pc` file is present,
this crate will find it and link against it.
Debian and Ubuntu users can install it by running the following command:

```
apt install liblouis-dev
```

If liblouis is not present or too old, this crate will automatically compile a vendored version of liblouis.
To do this, it will require:

- A C compiler
- make
