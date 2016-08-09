# encoding_rs_compat 0.2.32

[![Build Status](https://travis-ci.org/hsivonen/encoding_rs_compat.svg?branch=master)](https://travis-ci.org/hsivonen/encoding_rs_compat)

encoding_rs_compat provides the
[rust-encoding](https://lifthrasiir.github.io/rust-encoding/) 0.2.32 API
implemented on top of [encoding_rs](https://hsivonen.fi/rs/encoding_rs/).
Technically, encoding_rs_compat is a fork of rust-encoding 0.2.32 with the 
internals replaced. The use case is to allow Gecko to use crates that depend
on rust-encoding without having to include duplicate data tables and converter
functionality.

## Differences from rust-encoding

* The bugs in the converters and the spec snapshot they implement are those
  of encoding_rs.

* HZ, ISO-8859-1 as an encoding distinct from windows-1252 and the error
  encoding are not supported.

* The types of the constants in `all::*` differ.

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

