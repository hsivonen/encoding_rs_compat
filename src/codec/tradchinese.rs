// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Legacy traditional Chinese encodings.

#[cfg(test)]
mod bigfive2003_tests {
    extern crate test;
    use testutils;
    use types::*;
    use compat;

    static BigFive2003Encoding: EncodingRef = &compat::BIG5;

    #[test]
    fn test_encoder_valid() {
        let mut e = BigFive2003Encoding.raw_encoder();
        assert_feed_ok!(e, "A", "", [0x41]);
        assert_feed_ok!(e, "BC", "", [0x42, 0x43]);
        assert_feed_ok!(e, "", "", []);
        assert_feed_ok!(e,
                        "\u{4e2d}\u{83ef}\u{6c11}\u{570b}",
                        "",
                        [0xa4, 0xa4, 0xb5, 0xd8, 0xa5, 0xc1, 0xb0, 0xea]);
        assert_feed_ok!(e, "1\u{20ac}/m", "", [0x31, 0xa3, 0xe1, 0x2f, 0x6d]);
        assert_feed_ok!(e, "\u{ffed}", "", [0xf9, 0xfe]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_invalid() {
        let mut e = BigFive2003Encoding.raw_encoder();
        assert_feed_err!(e, "", "\u{ffff}", "", []);
        assert_feed_err!(e, "?", "\u{ffff}", "!", [0x3f]);
        assert_feed_err!(e, "", "\u{3eec}", "\u{4e00}", []); // HKSCS-2008 addition
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_decoder_valid() {
        let mut d = BigFive2003Encoding.raw_decoder();
        assert_feed_ok!(d, [0x41], [], "A");
        assert_feed_ok!(d, [0x42, 0x43], [], "BC");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d,
                        [0xa4, 0xa4, 0xb5, 0xd8, 0xa5, 0xc1, 0xb0, 0xea],
                        [],
                        "\u{4e2d}\u{83ef}\u{6c11}\u{570b}");
        assert_feed_ok!(d, [], [0xa4], "");
        assert_feed_ok!(d, [0xa4, 0xb5, 0xd8], [0xa5], "\u{4e2d}\u{83ef}");
        assert_feed_ok!(d, [0xc1, 0xb0, 0xea], [], "\u{6c11}\u{570b}");
        assert_feed_ok!(d, [0x31, 0xa3, 0xe1, 0x2f, 0x6d], [], "1\u{20ac}/m");
        assert_feed_ok!(d, [0xf9, 0xfe], [], "\u{ffed}");
        assert_feed_ok!(d, [0x87, 0x7e], [], "\u{3eec}"); // HKSCS-2008 addition
        assert_feed_ok!(d,
                        [0x88, 0x62, 0x88, 0x64, 0x88, 0xa3, 0x88, 0xa5],
                        [],
                        "\u{ca}\u{304}\u{00ca}\u{30c}\u{ea}\u{304}\u{ea}\u{30c}"); // 2-byte output
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_immediate_test_finish() {
        for i in 0x81..0xff {
            let mut d = BigFive2003Encoding.raw_decoder();
            assert_feed_ok!(d, [], [i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }

        // 80/FF: immediate failure
        let mut d = BigFive2003Encoding.raw_decoder();
        assert_feed_err!(d, [], [0x80], [], "");
        assert_feed_err!(d, [], [0xff], [], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_followed_by_space() {
        for i in 0x80..0x100 {
            let i = i as u8;
            let mut d = BigFive2003Encoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x20], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_lead_followed_by_invalid_trail() {
        // unlike most other cases, valid lead + invalid MSB-set trail are entirely consumed.
        // https://www.w3.org/Bugs/Public/show_bug.cgi?id=16771
        for i in 0x81..0xff {
            let mut d = BigFive2003Encoding.raw_decoder();
            assert_feed_err!(d, [], [i, 0x80], [0x20], "");
            assert_feed_err!(d, [], [i, 0xff], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = BigFive2003Encoding.raw_decoder();
            assert_feed_ok!(d, [], [i], "");
            assert_feed_err!(d, [], [0x80], [0x20], "");
            assert_feed_ok!(d, [], [i], "");
            assert_feed_err!(d, [], [0xff], [0x20], "");
            assert_finish_ok!(d, "");
        }

        // 80/FF is not a valid lead and the trail is not consumed
        let mut d = BigFive2003Encoding.raw_decoder();
        assert_feed_err!(d, [], [0x80], [0x80], "");
        assert_feed_err!(d, [], [0x80], [0xff], "");
        assert_feed_err!(d, [], [0xff], [0x80], "");
        assert_feed_err!(d, [], [0xff], [0xff], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_feed_after_finish() {
        let mut d = BigFive2003Encoding.raw_decoder();
        assert_feed_ok!(d, [0xa4, 0x40], [0xa4], "\u{4e00}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xa4, 0x40], [], "\u{4e00}");
        assert_finish_ok!(d, "");
    }

    #[bench]
    fn bench_encode_short_text(bencher: &mut test::Bencher) {
        let s = testutils::TRADITIONAL_CHINESE_TEXT;
        bencher.bytes = s.len() as u64;
        bencher.iter(|| {
            test::black_box({
                BigFive2003Encoding.encode(&s, EncoderTrap::Strict)
            })
        })
    }

    #[bench]
    fn bench_decode_short_text(bencher: &mut test::Bencher) {
        let s = BigFive2003Encoding.encode(testutils::TRADITIONAL_CHINESE_TEXT,
                                           EncoderTrap::Strict)
                                   .ok()
                                   .unwrap();
        bencher.bytes = s.len() as u64;
        bencher.iter(|| {
            test::black_box({
                BigFive2003Encoding.decode(&s, DecoderTrap::Strict)
            })
        })
    }
}
