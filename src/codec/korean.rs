// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Legacy Korean encodings based on KS X 1001.

#[cfg(test)]
mod windows949_tests {
    #[cfg(nightly)]
    extern crate test;
    use testutils;
    use types::*;
    use all;

    const Windows949Encoding: EncodingRef = all::WINDOWS_949;

    #[test]
    fn test_encoder_valid() {
        let mut e = Windows949Encoding.raw_encoder();
        assert_feed_ok!(e, "A", "", [0x41]);
        assert_feed_ok!(e, "BC", "", [0x42, 0x43]);
        assert_feed_ok!(e, "", "", []);
        assert_feed_ok!(e, "\u{ac00}", "", [0xb0, 0xa1]);
        assert_feed_ok!(e, "\u{b098}\u{b2e4}", "", [0xb3, 0xaa, 0xb4, 0xd9]);
        assert_feed_ok!(e,
                        "\u{bdc1}\u{314b}\u{d7a3}",
                        "",
                        [0x94, 0xee, 0xa4, 0xbb, 0xc6, 0x52]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_invalid() {
        let mut e = Windows949Encoding.raw_encoder();
        assert_feed_err!(e, "", "\u{ffff}", "", []);
        assert_feed_err!(e, "?", "\u{ffff}", "!", [0x3f]);
        assert_feed_err!(e, "?", "\u{fffd}", "!", [0x3f]); // for invalid table entries
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_decoder_valid() {
        let mut d = Windows949Encoding.raw_decoder();
        assert_feed_ok!(d, [0x41], [], "A");
        assert_feed_ok!(d, [0x42, 0x43], [], "BC");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d, [0xb0, 0xa1], [], "\u{ac00}");
        assert_feed_ok!(d, [0xb3, 0xaa, 0xb4, 0xd9], [], "\u{b098}\u{b2e4}");
        assert_feed_ok!(d,
                        [0x94, 0xee, 0xa4, 0xbb, 0xc6, 0x52, 0xc1, 0x64],
                        [],
                        "\u{bdc1}\u{314b}\u{d7a3}\u{d58f}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_valid_partial() {
        let mut d = Windows949Encoding.raw_decoder();
        assert_feed_ok!(d, [], [0xb0], "");
        assert_feed_ok!(d, [0xa1], [], "\u{ac00}");
        assert_feed_ok!(d, [0xb3, 0xaa], [0xb4], "\u{b098}");
        assert_feed_ok!(d, [0xd9], [0x94], "\u{b2e4}");
        assert_feed_ok!(d, [0xee, 0xa4, 0xbb], [0xc6], "\u{bdc1}\u{314b}");
        assert_feed_ok!(d, [0x52, 0xc1, 0x64], [], "\u{d7a3}\u{d58f}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_immediate_test_finish() {
        for i in 0x81..0xff {
            let mut d = Windows949Encoding.raw_decoder();
            assert_feed_ok!(d, [], [i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }

        // 80/FF: immediate failure
        let mut d = Windows949Encoding.raw_decoder();
        assert_feed_err!(d, [], [0x80], [], "");
        assert_feed_err!(d, [], [0xff], [], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_followed_by_space() {
        for i in 0x80..0x100 {
            let i = i as u8;
            let mut d = Windows949Encoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x20], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_lead_followed_by_invalid_trail() {
        // should behave similarly to Big5.
        // https://www.w3.org/Bugs/Public/show_bug.cgi?id=16691
        for i in 0x81..0xff {
            let mut d = Windows949Encoding.raw_decoder();
            assert_feed_err!(d, [], [i, 0x80], [0x20], "");
            assert_feed_err!(d, [], [i, 0xff], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = Windows949Encoding.raw_decoder();
            assert_feed_ok!(d, [], [i], "");
            assert_feed_err!(d, [], [0x80], [0x20], "");
            assert_feed_ok!(d, [], [i], "");
            assert_feed_err!(d, [], [0xff], [0x20], "");
            assert_finish_ok!(d, "");
        }

        let mut d = Windows949Encoding.raw_decoder();
        assert_feed_err!(d, [], [0x80], [0x80], "");
        assert_feed_err!(d, [], [0x80], [0xff], "");
        assert_feed_err!(d, [], [0xff], [0x80], "");
        assert_feed_err!(d, [], [0xff], [0xff], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_boundary() {
        // U+D7A3 (C6 52) is the last Hangul syllable not in KS X 1001, C6 53 is invalid.
        // note that since the trail byte may coincide with ASCII, the trail byte 53 is
        // not considered to be in the problem. this is compatible to WHATWG Encoding standard.
        let mut d = Windows949Encoding.raw_decoder();
        assert_feed_ok!(d, [], [0xc6], "");
        assert_feed_err!(d, [], [], [0x53], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_feed_after_finish() {
        let mut d = Windows949Encoding.raw_decoder();
        assert_feed_ok!(d, [0xb0, 0xa1], [0xb0], "\u{ac00}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xb0, 0xa1], [], "\u{ac00}");
        assert_finish_ok!(d, "");
    }

    #[cfg(nightly)]
    #[bench]
    fn bench_encode_short_text(bencher: &mut test::Bencher) {
        let s = testutils::KOREAN_TEXT;
        bencher.bytes = s.len() as u64;
        bencher.iter(|| {
            test::black_box({
                Windows949Encoding.encode(&s, EncoderTrap::Strict)
            })
        })
    }

    #[cfg(nightly)]
    #[bench]
    fn bench_decode_short_text(bencher: &mut test::Bencher) {
        let s = Windows949Encoding.encode(testutils::KOREAN_TEXT, EncoderTrap::Strict)
            .ok()
            .unwrap();
        bencher.bytes = s.len() as u64;
        bencher.iter(|| {
            test::black_box({
                Windows949Encoding.decode(&s, DecoderTrap::Strict)
            })
        })
    }
}
