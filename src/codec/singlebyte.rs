// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Common codec implementation for single-byte encodings.

#[cfg(test)]
mod tests {
    use all::ISO_8859_2;
    use types::*;

    #[test]
    fn test_encoder_non_bmp() {
        let mut e = ISO_8859_2.raw_encoder();
        assert_feed_err!(e, "A", "\u{FFFF}", "B", [0x41]);
        assert_feed_err!(e, "A", "\u{10000}", "B", [0x41]);
    }
}
