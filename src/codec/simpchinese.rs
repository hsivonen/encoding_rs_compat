// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Legacy simplified Chinese encodings based on GB 2312 and GB 18030.

#[cfg(test)]
mod gb18030_tests {
    #[cfg(nightly)]
    extern crate test;
    use testutils;
    use types::*;
    use all;

    const GB18030_ENCODING: EncodingRef = all::GB18030;

    #[test]
    fn test_encoder() {
        let mut e = GB18030_ENCODING.raw_encoder();
        assert_feed_ok!(e, "A", "", [0x41]);
        assert_feed_ok!(e, "BC", "", [0x42, 0x43]);
        assert_feed_ok!(e, "", "", []);
        assert_feed_ok!(e,
                        "\u{4e2d}\u{534e}\u{4eba}\u{6c11}\u{5171}\u{548c}\u{56fd}",
                        "",
                        [0xd6, 0xd0, 0xbb, 0xaa, 0xc8, 0xcb, 0xc3, 0xf1, 0xb9, 0xb2, 0xba, 0xcd,
                         0xb9, 0xfa]);
        assert_feed_ok!(e, "1\u{20ac}/m", "", [0x31, 0xa2, 0xe3, 0x2f, 0x6d]);
        assert_feed_ok!(e,
                        "\u{ff21}\u{ff22}\u{ff23}",
                        "",
                        [0xa3, 0xc1, 0xa3, 0xc2, 0xa3, 0xc3]);
        assert_feed_ok!(e, "\u{80}", "", [0x81, 0x30, 0x81, 0x30]);
        assert_feed_ok!(e, "\u{81}", "", [0x81, 0x30, 0x81, 0x31]);
        assert_feed_ok!(e, "\u{a3}", "", [0x81, 0x30, 0x84, 0x35]);
        assert_feed_ok!(e, "\u{a4}", "", [0xa1, 0xe8]);
        assert_feed_ok!(e, "\u{a5}", "", [0x81, 0x30, 0x84, 0x36]);
        assert_feed_ok!(e, "\u{10ffff}", "", [0xe3, 0x32, 0x9a, 0x35]);
        assert_feed_ok!(e,
                        "\u{2a6a5}\u{3007}",
                        "",
                        [0x98, 0x35, 0xee, 0x37, 0xa9, 0x96]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_decoder_valid() {
        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [0x41], [], "A");
        assert_feed_ok!(d, [0x42, 0x43], [], "BC");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d,
                        [0xd6, 0xd0, 0xbb, 0xaa, 0xc8, 0xcb, 0xc3, 0xf1, 0xb9, 0xb2, 0xba, 0xcd,
                         0xb9, 0xfa],
                        [],
                        "\u{4e2d}\u{534e}\u{4eba}\u{6c11}\u{5171}\u{548c}\u{56fd}");
        assert_feed_ok!(d, [0x31, 0x80, 0x2f, 0x6d], [], "1\u{20ac}/m");
        assert_feed_ok!(d,
                        [0xa3, 0xc1, 0xa3, 0xc2, 0xa3, 0xc3],
                        [],
                        "\u{ff21}\u{ff22}\u{ff23}");
        assert_feed_ok!(d, [0x81, 0x30, 0x81, 0x30], [], "\u{80}");
        assert_feed_ok!(d, [0x81, 0x30, 0x81, 0x31], [], "\u{81}");
        assert_feed_ok!(d, [0x81, 0x30, 0x84, 0x35], [], "\u{a3}");
        assert_feed_ok!(d, [0xa1, 0xe8], [], "\u{a4}");
        assert_feed_ok!(d, [0x81, 0x30, 0x84, 0x36], [], "\u{a5}");
        assert_feed_ok!(d, [0xe3, 0x32, 0x9a, 0x35], [], "\u{10ffff}");
        assert_feed_ok!(d,
                        [0x98, 0x35, 0xee, 0x37, 0xa9, 0x96],
                        [],
                        "\u{2a6a5}\u{3007}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_valid_partial() {
        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [], [0xa1], "");
        assert_feed_ok!(d, [0xa1], [], "\u{3000}");
        assert_feed_ok!(d, [], [0x81], "");
        assert_feed_ok!(d, [], [0x30], "");
        assert_feed_ok!(d, [], [0x81], "");
        assert_feed_ok!(d, [0x30], [], "\u{80}");
        assert_feed_ok!(d, [], [0x81], "");
        assert_feed_ok!(d, [], [0x30], "");
        assert_feed_ok!(d, [0x81, 0x31], [], "\u{81}");
        assert_feed_ok!(d, [], [0x81], "");
        assert_feed_ok!(d, [0x30, 0x81, 0x32], [], "\u{82}");
        assert_feed_ok!(d, [], [0x81], "");
        assert_feed_ok!(d, [], [0x30, 0x81], "");
        assert_feed_ok!(d, [0x33], [], "\u{83}");
        assert_feed_ok!(d, [], [0x81, 0x30], "");
        assert_feed_ok!(d, [], [0x81], "");
        assert_feed_ok!(d, [0x34], [], "\u{84}");
        assert_feed_ok!(d, [], [0x81, 0x30], "");
        assert_feed_ok!(d, [0x81, 0x35], [], "\u{85}");
        assert_feed_ok!(d, [], [0x81, 0x30, 0x81], "");
        assert_feed_ok!(d, [0x36], [], "\u{86}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_partial() {
        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [], [0xa1], "");
        assert_finish_err!(d, "");

        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [], [0x81], "");
        assert_finish_err!(d, "");

        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [], [0x81, 0x30], "");
        assert_finish_err!(d, "");

        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [], [0x81, 0x30, 0x81], "");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_invalid_out_of_range() {
        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_err!(d, [], [0xff], [], "");
        assert_feed_err!(d, [], [0x81], [0x00], "");
        assert_feed_err!(d, [], [0x81], [0x7f], "");
        assert_feed_err!(d, [], [0x81], [0xff], "");
        assert_feed_err!(d, [], [0x81], [0x31, 0x00], "");
        assert_feed_err!(d, [], [0x81], [0x31, 0x80], "");
        assert_feed_err!(d, [], [0x81], [0x31, 0xff], "");
        assert_feed_err!(d, [], [0x81], [0x31, 0x81, 0x00], "");
        assert_feed_err!(d, [], [0x81], [0x31, 0x81, 0x2f], "");
        assert_feed_err!(d, [], [0x81], [0x31, 0x81, 0x3a], "");
        assert_feed_err!(d, [], [0x81], [0x31, 0x81, 0xff], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_boundary() {
        // U+10FFFF (E3 32 9A 35) is the last Unicode codepoint, E3 32 9A 36 is invalid.
        // note that since the 2nd to 4th bytes may coincide with ASCII, bytes 32 9A 36 is
        // not considered to be in the problem. this is compatible to WHATWG Encoding standard.
        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [], [0xe3], "");
        assert_feed_err!(d, [], [], [0x32, 0x9a, 0x36], "");
        assert_finish_ok!(d, "");

        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [], [0xe3], "");
        assert_feed_ok!(d, [], [0x32, 0x9a], "");
        assert_feed_err!(d, -2, [], [], [0x32, 0x9a, 0x36], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_feed_after_finish() {
        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [0xd2, 0xbb], [0xd2], "\u{4e00}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xd2, 0xbb], [], "\u{4e00}");
        assert_finish_ok!(d, "");

        let mut d = GB18030_ENCODING.raw_decoder();
        assert_feed_ok!(d, [0x98, 0x35, 0xee, 0x37], [0x98, 0x35, 0xee], "\u{2a6a5}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0x98, 0x35, 0xee, 0x37], [0x98, 0x35], "\u{2a6a5}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0x98, 0x35, 0xee, 0x37], [0x98], "\u{2a6a5}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0x98, 0x35, 0xee, 0x37], [], "\u{2a6a5}");
        assert_finish_ok!(d, "");
    }

    #[cfg(nightly)]
    #[bench]
    fn bench_encode_short_text(bencher: &mut test::Bencher) {
        let s = testutils::SIMPLIFIED_CHINESE_TEXT;
        bencher.bytes = s.len() as u64;
        bencher.iter(|| {
            test::black_box({
                GB18030_ENCODING.encode(&s, EncoderTrap::Strict)
            })
        })
    }

    #[cfg(nightly)]
    #[bench]
    fn bench_decode_short_text(bencher: &mut test::Bencher) {
        let s = GB18030_ENCODING.encode(testutils::SIMPLIFIED_CHINESE_TEXT, EncoderTrap::Strict)
            .ok()
            .unwrap();
        bencher.bytes = s.len() as u64;
        bencher.iter(|| {
            test::black_box({
                GB18030_ENCODING.decode(&s, DecoderTrap::Strict)
            })
        })
    }
}

#[cfg(test)]
mod gbk_tests {
    #[cfg(nightly)]
    extern crate test;
    use testutils;
    use types::*;
    use all;

    const GBK_ENCODING: EncodingRef = all::GBK;

    // GBK and GB 18030 share the same decoder logic.

    #[test]
    fn test_encoder() {
        let mut e = GBK_ENCODING.raw_encoder();
        assert_feed_ok!(e, "A", "", [0x41]);
        assert_feed_ok!(e, "BC", "", [0x42, 0x43]);
        assert_feed_ok!(e, "", "", []);
        assert_feed_ok!(e,
                        "\u{4e2d}\u{534e}\u{4eba}\u{6c11}\u{5171}\u{548c}\u{56fd}",
                        "",
                        [0xd6, 0xd0, 0xbb, 0xaa, 0xc8, 0xcb, 0xc3, 0xf1, 0xb9, 0xb2, 0xba, 0xcd,
                         0xb9, 0xfa]);
        assert_feed_ok!(e, "1\u{20ac}/m", "", [0x31, 0x80, 0x2f, 0x6d]);
        assert_feed_ok!(e,
                        "\u{ff21}\u{ff22}\u{ff23}",
                        "",
                        [0xa3, 0xc1, 0xa3, 0xc2, 0xa3, 0xc3]);
        assert_feed_err!(e, "", "\u{80}", "", []);
        assert_feed_err!(e, "", "\u{81}", "", []);
        assert_feed_err!(e, "", "\u{a3}", "", []);
        assert_feed_ok!(e, "\u{a4}", "", [0xa1, 0xe8]);
        assert_feed_err!(e, "", "\u{a5}", "", []);
        assert_feed_err!(e, "", "\u{10ffff}", "", []);
        assert_feed_err!(e, "", "\u{2a6a5}", "\u{3007}", []);
        assert_feed_err!(e, "\u{3007}", "\u{2a6a5}", "", [0xa9, 0x96]);
        assert_finish_ok!(e, []);
    }

    #[cfg(nightly)]
    #[bench]
    fn bench_encode_short_text(bencher: &mut test::Bencher) {
        let s = testutils::SIMPLIFIED_CHINESE_TEXT;
        bencher.bytes = s.len() as u64;
        bencher.iter(|| {
            test::black_box({
                GBK_ENCODING.encode(&s, EncoderTrap::Strict)
            })
        })
    }
}
