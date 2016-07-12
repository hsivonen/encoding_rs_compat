If you send a pull request / patch, please observe the following.

## Licensing

Since the goal is to make this crate dual-licensed in the future, subject
to upstream relicensing situation (see the file `COPYRIGHT` for more info),
contributions to this crate are required to be dual-licensed under the
following dual license:

"Licensed under the Apache License, Version 2.0 or the MIT license, at your
option." where "your" refers to the downstream recipient.

Please do not contribute if you aren't willing or allowed to license your
contributions in this manner and please indicate in pull request comments
that your contribution is licensed in this manner.

## Copyright Notices

If you require the addition of your copyright notice, it's up to you to edit in
your notice as part of your Contribution. Not adding a copyright notice is
taken as a waiver of copyright notice.

## Compatibility with Stable Rust

Please ensure that your Contribution compiles with the latest stable-channel
rustc.

## rustfmt

Please install [`rustfmt`](https://github.com/rust-lang-nursery/rustfmt) and
run `cargo fmt` before creating a pull request. (It's OK for `cargo fmt` to
exit with an error due to too long lines.)

## Unit tests

Please ensure that `cargo test` succeeds.
