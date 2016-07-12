# encoding_rs_compat 0.2.32

[![Build Status](https://travis-ci.org/hsivonen/encoding_rs_compat.svg?branch=master)](https://travis-ci.org/hsivonen/encoding_rs_compat)

encoding_rs_compat will provide the
[rust-encoding](https://lifthrasiir.github.io/rust-encoding/) 0.2.32 API
implemented on top of [encoding_rs](https://hsivonen.fi/rs/encoding_rs/).
Technically, encoding_rs_compat will be fork of rust-encoding 0.2.32 with the 
internals replaced. The use case is to allow Gecko to use crates that depend
on rust-encoding without having to include duplicate data tables and converter
functionality.

_Currently, this repo is just a fork. The internals haven't been replaced, yet._
