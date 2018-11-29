# encoding_rs_compat 0.2.33

[![Build Status](https://travis-ci.org/hsivonen/encoding_rs_compat.svg?branch=master)](https://travis-ci.org/hsivonen/encoding_rs_compat)

encoding_rs_compat provides the
[rust-encoding](https://lifthrasiir.github.io/rust-encoding/) 0.2.33 API
implemented on top of [encoding_rs](https://hsivonen.fi/rs/encoding_rs/).
Technically, encoding_rs_compat is a fork of rust-encoding 0.2.32 with the 
internals replaced. The use case was to allow Gecko to use crates that depend
on rust-encoding without having to include duplicate data tables and converter
functionality, but Gecko ended up not needing this compatibility crate after
all.

## Usage

Put this in the `Cargo.toml` file of your top-level Cargo artifact:

```
[replace]
"encoding:0.2.33" = { git = 'https://github.com/hsivonen/encoding_rs_compat' }
```

Upon `cargo build`, ensure you see don't see the `encoding-index-*` crates being built.


## Differences from rust-encoding

* The bugs in the converters and the spec snapshot they implement are those
  of encoding_rs.

* ISO-8859-1 as an encoding distinct from windows-1252, HZ, and the error
  encoding are not supported.

* Attempting to encode to UTF-16LE or UTF-16BE panics.

* The types of the constants in `all::*` differ.

* The "constants" in `all::*` are `static` instead of `const`, because they
  need to refer to `static`s and Rust, very annoyingly, doesn't allow `const`
  to refer to even an address of a `static`.

* There is no direct access to the indices.

* The `codec` module isn't visible.

* `RawDecoder.raw_feed()` always identifies a zero-length byte sequence as
  being the erroneous one. (This is due to encoding_rs not backtracking when
  a byte sequence that might turn out to be erroneous later is split across
  a buffer boundary. The entry points that take a `DecoderTrap` pass the
  erroneous bytes to the `DecoderTrapFunc` correctly, however.)

* While `RawEncoder.raw_feed()` signals unmappable characters the same way as
  rust-encoding, which cannot represent the current spec requiring certain
  unmappables in ISO-2022-JP to be reported as U+FFFD, unmappable characters
  passed to `EncoderTrap` are reported as U+FFFD where required by the spec for
  ISO-2022-JP.

* The performance profile of custom `ByteWriter` and `StringWriter` differs from
  the performance profile of the default `Vec<u8>` and `String`. The former
  involve an extra intermediate copy of the output while the latter run at the
  native speed of encoding_rs.
