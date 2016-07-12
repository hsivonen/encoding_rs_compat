// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! UTF-16.

use std::convert::Into;
use std::marker::PhantomData;
use util::as_char;
use types::*;

#[cfg(test)]
mod tests {
    // little endian and big endian is symmetric to each other, there's no need to test both.
    // since big endian is easier to inspect we test UTF_16BE only.

    use super::UTF_16BE_ENCODING as UTF_16BE;
    use types::*;

    #[test]
    fn test_encoder_valid() {
        let mut e = UTF_16BE.raw_encoder();
        assert_feed_ok!(e, "\u{0}\
                            \u{1}\u{02}\u{004}\u{0008}\
                            \u{10}\u{020}\u{0040}\u{80}\
                            \u{100}\u{0200}\u{400}\u{800}\
                            \u{1000}\u{2000}\u{4000}\u{8000}\
                            \u{ffff}", "",
                        [0x00, 0x00,
                         0x00, 0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08,
                         0x00, 0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80,
                         0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08, 0x00,
                         0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80, 0x00,
                         0xff, 0xff]);
        assert_feed_ok!(e, "\u{10000}\
                            \u{10001}\u{010002}\
                            \u{10004}\u{010008}\
                            \u{10010}\u{010020}\
                            \u{10040}\u{010080}\
                            \u{10100}\u{010200}\
                            \u{10400}\u{010800}\
                            \u{11000}\u{012000}\
                            \u{14000}\u{018000}\
                            \u{20000}\u{030000}\
                            \u{50000}\u{090000}\
                            \u{10FFFF}", "",
                        [0xd8, 0x00, 0xdc, 0x00,
                         0xd8, 0x00, 0xdc, 0x01, 0xd8, 0x00, 0xdc, 0x02,
                         0xd8, 0x00, 0xdc, 0x04, 0xd8, 0x00, 0xdc, 0x08,
                         0xd8, 0x00, 0xdc, 0x10, 0xd8, 0x00, 0xdc, 0x20,
                         0xd8, 0x00, 0xdc, 0x40, 0xd8, 0x00, 0xdc, 0x80,
                         0xd8, 0x00, 0xdd, 0x00, 0xd8, 0x00, 0xde, 0x00,
                         0xd8, 0x01, 0xdc, 0x00, 0xd8, 0x02, 0xdc, 0x00,
                         0xd8, 0x04, 0xdc, 0x00, 0xd8, 0x08, 0xdc, 0x00,
                         0xd8, 0x10, 0xdc, 0x00, 0xd8, 0x20, 0xdc, 0x00,
                         0xd8, 0x40, 0xdc, 0x00, 0xd8, 0x80, 0xdc, 0x00,
                         0xd9, 0x00, 0xdc, 0x00, 0xda, 0x00, 0xdc, 0x00,
                         0xdb, 0xff, 0xdf, 0xff]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_decoder_valid() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [0x00, 0x00,
                            0x00, 0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08,
                            0x00, 0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80,
                            0x01, 0x00, 0x02, 0x00, 0x04, 0x00, 0x08, 0x00,
                            0x10, 0x00, 0x20, 0x00, 0x40, 0x00, 0x80, 0x00,
                            0xff, 0xff], [],
                        "\u{0}\
                         \u{1}\u{02}\u{004}\u{0008}\
                         \u{10}\u{020}\u{0040}\u{80}\
                         \u{100}\u{0200}\u{400}\u{800}\
                         \u{1000}\u{2000}\u{4000}\u{8000}\
                         \u{ffff}");
        assert_feed_ok!(d, [0xd8, 0x00, 0xdc, 0x00,
                            0xd8, 0x00, 0xdc, 0x01, 0xd8, 0x00, 0xdc, 0x02,
                            0xd8, 0x00, 0xdc, 0x04, 0xd8, 0x00, 0xdc, 0x08,
                            0xd8, 0x00, 0xdc, 0x10, 0xd8, 0x00, 0xdc, 0x20,
                            0xd8, 0x00, 0xdc, 0x40, 0xd8, 0x00, 0xdc, 0x80,
                            0xd8, 0x00, 0xdd, 0x00, 0xd8, 0x00, 0xde, 0x00,
                            0xd8, 0x01, 0xdc, 0x00, 0xd8, 0x02, 0xdc, 0x00,
                            0xd8, 0x04, 0xdc, 0x00, 0xd8, 0x08, 0xdc, 0x00,
                            0xd8, 0x10, 0xdc, 0x00, 0xd8, 0x20, 0xdc, 0x00,
                            0xd8, 0x40, 0xdc, 0x00, 0xd8, 0x80, 0xdc, 0x00,
                            0xd9, 0x00, 0xdc, 0x00, 0xda, 0x00, 0xdc, 0x00,
                            0xdb, 0xff, 0xdf, 0xff], [],
                        "\u{10000}\
                         \u{10001}\u{010002}\
                         \u{10004}\u{010008}\
                         \u{10010}\u{010020}\
                         \u{10040}\u{010080}\
                         \u{10100}\u{010200}\
                         \u{10400}\u{010800}\
                         \u{11000}\u{012000}\
                         \u{14000}\u{018000}\
                         \u{20000}\u{030000}\
                         \u{50000}\u{090000}\
                         \u{10FFFF}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_valid_partial_bmp() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0x12], "");
        assert_feed_ok!(d, [0x34], [], "\u{1234}");
        assert_feed_ok!(d, [], [0x56], "");
        assert_feed_ok!(d, [0x78], [], "\u{5678}");
        assert_finish_ok!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0x12], "");
        assert_feed_ok!(d, [0x34], [0x56], "\u{1234}");
        assert_feed_ok!(d, [0x78, 0xab, 0xcd], [], "\u{5678}\u{abcd}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_valid_partial_non_bmp() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8], "");
        assert_feed_ok!(d, [], [0x08], "");
        assert_feed_ok!(d, [], [0xdf], "");
        assert_feed_ok!(d, [0x45], [0xd9], "\u{12345}");
        assert_feed_ok!(d, [], [0x5e], "");
        assert_feed_ok!(d, [], [0xdc], "");
        assert_feed_ok!(d, [0x90], [], "\u{67890}");
        assert_finish_ok!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8], "");
        assert_feed_ok!(d, [], [0x08, 0xdf], "");
        assert_feed_ok!(d, [0x45], [0xd9, 0x5e], "\u{12345}");
        assert_feed_ok!(d, [0xdc, 0x90], [], "\u{67890}");
        assert_finish_ok!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8, 0x08, 0xdf], "");
        assert_feed_ok!(d, [0x45], [0xd9, 0x5e, 0xdc], "\u{12345}");
        assert_feed_ok!(d, [0x90], [], "\u{67890}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_partial() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0x12], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8, 0x08], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8, 0x08, 0xdf], "");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_upper_surrogate() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8, 0x00], "");
        assert_feed_err!(d, [], [], [0x12, 0x34], "");
        assert_feed_err!(d, [], [0xd8, 0x00], [0x56, 0x78], "");
        assert_feed_ok!(d, [], [0xd8, 0x00], "");
        assert_feed_err!(d, [], [], [0xd8, 0x00], "");
        assert_feed_ok!(d, [], [0xd8, 0x00], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xdb, 0xff], "");
        assert_feed_err!(d, [], [], [0x12, 0x34], "");
        assert_feed_err!(d, [], [0xdb, 0xff], [0x56, 0x78], "");
        assert_feed_ok!(d, [], [0xdb, 0xff], "");
        assert_feed_err!(d, [], [], [0xdb, 0xff], "");
        assert_feed_ok!(d, [], [0xdb, 0xff], "");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_upper_surrogate_partial() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8], "");
        assert_feed_err!(d, [], [0x00], [0x12, 0x34], "");
        assert_feed_ok!(d, [], [0xd8, 0x00, 0x56], "");
        assert_feed_err!(d, -1, [], [], [0x56, 0x78], "");
        assert_feed_ok!(d, [], [0xd8], "");
        assert_feed_err!(d, [], [0x00], [0xd8, 0x00], "");
        assert_feed_ok!(d, [], [0xd8, 0x00, 0xdb], "");
        assert_feed_err!(d, -1, [], [], [0xdb, 0xff], "");
        assert_feed_ok!(d, [], [0xd8], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xdb], "");
        assert_feed_err!(d, [], [0xff], [0x12, 0x34], "");
        assert_feed_ok!(d, [], [0xdb, 0xff, 0x56], "");
        assert_feed_err!(d, -1, [], [], [0x56, 0x78], "");
        assert_feed_ok!(d, [], [0xdb], "");
        assert_feed_err!(d, [], [0xff], [0xdb, 0xff], "");
        assert_feed_ok!(d, [], [0xdb, 0xff, 0xd8], "");
        assert_feed_err!(d, -1, [], [], [0xd8, 0x00], "");
        assert_feed_ok!(d, [], [0xdb], "");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lower_surrogate() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_err!(d, [], [0xdc, 0x00], [], "");
        assert_feed_err!(d, [0x12, 0x34], [0xdc, 0x00], [0x56, 0x78], "\u{1234}");
        assert_finish_ok!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_err!(d, [], [0xdf, 0xff], [], "");
        assert_feed_err!(d, [0x12, 0x34], [0xdf, 0xff], [0x56, 0x78], "\u{1234}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lower_surrogate_partial() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xdc], "");
        assert_feed_err!(d, [], [0x00], [], "");
        assert_feed_ok!(d, [0x12, 0x34], [0xdc], "\u{1234}");
        assert_feed_err!(d, [], [0x00], [0x56, 0x78], "");
        assert_finish_ok!(d, "");

        assert_feed_ok!(d, [], [0xdf], "");
        assert_feed_err!(d, [], [0xff], [], "");
        assert_feed_ok!(d, [0x12, 0x34], [0xdf], "\u{1234}");
        assert_feed_err!(d, [], [0xff], [0x56, 0x78], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_one_byte_before_finish() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0x12], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [0x12, 0x34], [0x56], "\u{1234}");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_invalid_three_bytes_before_finish() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8, 0x00, 0xdc], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [0x12, 0x34], [0xd8, 0x00, 0xdc], "\u{1234}");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_invalid_three_bytes_before_finish_partial() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [], [0xd8], "");
        assert_feed_ok!(d, [], [0x00], "");
        assert_feed_ok!(d, [], [0xdc], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [0x12, 0x34], [0xd8], "\u{1234}");
        assert_feed_ok!(d, [], [0x00, 0xdc], "");
        assert_finish_err!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [0x12, 0x34], [0xd8, 0x00], "\u{1234}");
        assert_feed_ok!(d, [], [0xdc], "");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_feed_after_finish() {
        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [0x12, 0x34], [0x12], "\u{1234}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0x12, 0x34], [], "\u{1234}");
        assert_finish_ok!(d, "");

        let mut d = UTF_16BE.raw_decoder();
        assert_feed_ok!(d, [0xd8, 0x08, 0xdf, 0x45], [0xd8, 0x08, 0xdf], "\u{12345}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xd8, 0x08, 0xdf, 0x45], [0xd8, 0x08], "\u{12345}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xd8, 0x08, 0xdf, 0x45], [0xd8], "\u{12345}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xd8, 0x08, 0xdf, 0x45], [], "\u{12345}");
        assert_finish_ok!(d, "");
    }
}

