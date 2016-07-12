// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Legacy Japanese encodings based on JIS X 0208 and JIS X 0212.

use std::convert::Into;
use std::default::Default;
use util::StrCharIndex;
use types::*;

#[cfg(test)]
mod eucjp_tests {
    extern crate test;
    use super::EUCJPEncoding;
    use testutils;
    use types::*;

    #[test]
    fn test_encoder_valid() {
        let mut e = EUCJPEncoding.raw_encoder();
        assert_feed_ok!(e, "A", "", [0x41]);
        assert_feed_ok!(e, "BC", "", [0x42, 0x43]);
        assert_feed_ok!(e, "", "", []);
        assert_feed_ok!(e, "\u{a5}", "", [0x5c]);
        assert_feed_ok!(e, "\u{203e}", "", [0x7e]);
        assert_feed_ok!(e, "\u{306b}\u{307b}\u{3093}", "", [0xa4, 0xcb, 0xa4, 0xdb, 0xa4, 0xf3]);
        assert_feed_ok!(e, "\u{ff86}\u{ff8e}\u{ff9d}", "", [0x8e, 0xc6, 0x8e, 0xce, 0x8e, 0xdd]);
        assert_feed_ok!(e, "\u{65e5}\u{672c}", "", [0xc6, 0xfc, 0xcb, 0xdc]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_double_mapped() {
        // these characters are double-mapped to both EUDC area and Shift_JIS extension area
        // but only the former should be used. (note that U+FFE2 is triple-mapped!)
        let mut e = EUCJPEncoding.raw_encoder();
        assert_feed_ok!(e, "\u{9ed1}\u{2170}\u{ffe2}", "", [0xfc, 0xee, 0xfc, 0xf1, 0xa2, 0xcc]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_invalid() {
        let mut e = EUCJPEncoding.raw_encoder();
        assert_feed_err!(e, "", "\u{ffff}", "", []);
        assert_feed_err!(e, "?", "\u{ffff}", "!", [0x3f]);
        // JIS X 0212 is not supported in the encoder
        assert_feed_err!(e, "", "\u{736c}", "\u{8c78}", []);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_decoder_valid() {
        let mut d = EUCJPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x41], [], "A");
        assert_feed_ok!(d, [0x42, 0x43], [], "BC");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d, [0x5c], [], "\\");
        assert_feed_ok!(d, [0x7e], [], "~");
        assert_feed_ok!(d, [0xa4, 0xcb, 0xa4, 0xdb, 0xa4, 0xf3], [], "\u{306b}\u{307b}\u{3093}");
        assert_feed_ok!(d, [0x8e, 0xc6, 0x8e, 0xce, 0x8e, 0xdd], [], "\u{ff86}\u{ff8e}\u{ff9d}");
        assert_feed_ok!(d, [0xc6, 0xfc, 0xcb, 0xdc], [], "\u{65e5}\u{672c}");
        assert_feed_ok!(d, [0x8f, 0xcb, 0xc6, 0xec, 0xb8], [], "\u{736c}\u{8c78}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_valid_partial() {
        let mut d = EUCJPEncoding.raw_decoder();
        assert_feed_ok!(d, [], [0xa4], "");
        assert_feed_ok!(d, [0xcb], [0xa4], "\u{306b}");
        assert_feed_ok!(d, [0xdb], [0xa4], "\u{307b}");
        assert_feed_ok!(d, [0xf3], [], "\u{3093}");
        assert_feed_ok!(d, [], [0x8e], "");
        assert_feed_ok!(d, [0xc6], [0x8e], "\u{ff86}");
        assert_feed_ok!(d, [0xce], [0x8e], "\u{ff8e}");
        assert_feed_ok!(d, [0xdd], [], "\u{ff9d}");
        assert_feed_ok!(d, [], [0xc6], "");
        assert_feed_ok!(d, [0xfc], [0xcb], "\u{65e5}");
        assert_feed_ok!(d, [0xdc], [], "\u{672c}");
        assert_feed_ok!(d, [], [0x8f], "");
        assert_feed_ok!(d, [], [0xcb], "");
        assert_feed_ok!(d, [0xc6], [0xec], "\u{736c}");
        assert_feed_ok!(d, [0xb8], [], "\u{8c78}");
        assert_feed_ok!(d, [], [0x8f, 0xcb], "");
        assert_feed_ok!(d, [0xc6, 0xec, 0xb8], [], "\u{736c}\u{8c78}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_immediate_test_finish() {
        for i in 0x8e..0x90 {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }

        for i in 0xa1..0xff {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }

        // immediate failures
        let mut d = EUCJPEncoding.raw_decoder();
        for i in 0x80..0x8e {
            assert_feed_err!(d, [], [i], [], "");
        }
        for i in 0x90..0xa1 {
            assert_feed_err!(d, [], [i], [], "");
        }
        assert_feed_err!(d, [], [0xff], [], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_followed_by_space() {
        for i in 0x80..0x100 {
            let i = i as u8;
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x20], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_lead_followed_by_invalid_trail() {
        for i in 0x80..0x100 {
            let i = i as u8;
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x80], "");
            assert_feed_err!(d, [], [i], [0xff], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_lone_lead_for_0212_immediate_test_finish() {
        for i in 0xa1..0xff {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [0x8f, i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_lone_lead_for_0212_immediate_test_finish_partial() {
        for i in 0xa1..0xff {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [0x8f], "");
            assert_feed_ok!(d, [], [i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_trail_for_0201() {
        for i in 0..0xa1 {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_err!(d, [], [0x8e], [i], "");
            assert_finish_ok!(d, "");
        }

        for i in 0xe0..0xff {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_err!(d, [], [0x8e, i], [], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_trail_for_0201_partial() {
        for i in 0..0xa1 {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [0x8e], "");
            assert_feed_err!(d, [], [], [i], "");
            assert_finish_ok!(d, "");
        }

        for i in 0xe0..0xff {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [0x8e], "");
            assert_feed_err!(d, [], [i], [], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_middle_for_0212() {
        for i in 0..0xa1 {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_err!(d, [], [0x8f], [i], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_middle_for_0212_partial() {
        for i in 0..0xa1 {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [0x8f], "");
            assert_feed_err!(d, [], [], [i], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_trail_for_0212() {
        for i in 0..0xa1 {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_err!(d, [], [0x8f, 0xa1], [i], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_trail_for_0212_partial() {
        for i in 0..0xa1 {
            let mut d = EUCJPEncoding.raw_decoder();
            assert_feed_ok!(d, [], [0x8f], "");
            assert_feed_ok!(d, [], [0xa1], "");
            assert_feed_err!(d, [], [], [i], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_feed_after_finish() {
        let mut d = EUCJPEncoding.raw_decoder();
        assert_feed_ok!(d, [0xa4, 0xa2], [0xa4], "\u{3042}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xa4, 0xa2], [], "\u{3042}");
        assert_finish_ok!(d, "");
    }

    #[bench]
    fn bench_encode_short_text(bencher: &mut test::Bencher) {
        let s = testutils::JAPANESE_TEXT;
        bencher.bytes = s.len() as u64;
        bencher.iter(|| test::black_box({
            EUCJPEncoding.encode(&s, EncoderTrap::Strict)
        }))
    }

    #[bench]
    fn bench_decode_short_text(bencher: &mut test::Bencher) {
        let s = EUCJPEncoding.encode(testutils::JAPANESE_TEXT,
                                     EncoderTrap::Strict).ok().unwrap();
        bencher.bytes = s.len() as u64;
        bencher.iter(|| test::black_box({
            EUCJPEncoding.decode(&s, DecoderTrap::Strict)
        }))
    }
}

#[cfg(test)]
mod windows31j_tests {
    extern crate test;
    use super::Windows31JEncoding;
    use testutils;
    use types::*;

    #[test]
    fn test_encoder_valid() {
        let mut e = Windows31JEncoding.raw_encoder();
        assert_feed_ok!(e, "A", "", [0x41]);
        assert_feed_ok!(e, "BC", "", [0x42, 0x43]);
        assert_feed_ok!(e, "", "", []);
        assert_feed_ok!(e, "\u{a5}", "", [0x5c]);
        assert_feed_ok!(e, "\u{203e}", "", [0x7e]);
        assert_feed_ok!(e, "\u{306b}\u{307b}\u{3093}", "", [0x82, 0xc9, 0x82, 0xd9, 0x82, 0xf1]);
        assert_feed_ok!(e, "\u{ff86}\u{ff8e}\u{ff9d}", "", [0xc6, 0xce, 0xdd]);
        assert_feed_ok!(e, "\u{65e5}\u{672c}", "", [0x93, 0xfa, 0x96, 0x7b]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_no_eudc() {
        let mut e = Windows31JEncoding.raw_encoder();
        assert_feed_err!(e, "", "\u{e000}", "", []);
        assert_feed_err!(e, "", "\u{e757}", "", []);
        assert_feed_err!(e, "", "\u{e758}", "", []);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_double_mapped() {
        // these characters are double-mapped to both EUDC area and Shift_JIS extension area
        // but only the latter should be used. (note that U+FFE2 is triple-mapped!)
        let mut e = Windows31JEncoding.raw_encoder();
        assert_feed_ok!(e, "\u{9ed1}\u{2170}\u{ffe2}", "", [0xfc, 0x4b, 0xfa, 0x40, 0x81, 0xca]);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_invalid() {
        let mut e = Windows31JEncoding.raw_encoder();
        assert_feed_err!(e, "", "\u{ffff}", "", []);
        assert_feed_err!(e, "?", "\u{ffff}", "!", [0x3f]);
        assert_feed_err!(e, "", "\u{736c}", "\u{8c78}", []);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_decoder_valid() {
        let mut d = Windows31JEncoding.raw_decoder();
        assert_feed_ok!(d, [0x41], [], "A");
        assert_feed_ok!(d, [0x42, 0x43], [], "BC");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d, [0x5c], [], "\\");
        assert_feed_ok!(d, [0x7e], [], "~");
        assert_feed_ok!(d, [0x80], [], "\u{80}"); // compatibility
        assert_feed_ok!(d, [0x82, 0xc9, 0x82, 0xd9, 0x82, 0xf1], [], "\u{306b}\u{307b}\u{3093}");
        assert_feed_ok!(d, [0xc6, 0xce, 0xdd], [], "\u{ff86}\u{ff8e}\u{ff9d}");
        assert_feed_ok!(d, [0x93, 0xfa, 0x96, 0x7b], [], "\u{65e5}\u{672c}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_eudc() {
        let mut d = Windows31JEncoding.raw_decoder();
        assert_feed_ok!(d, [], [0xf0], "");
        assert_feed_ok!(d, [0x40], [], "\u{e000}");
        assert_feed_ok!(d, [0xf9, 0xfc], [], "\u{e757}");
        assert_feed_err!(d, [], [0xf0], [0x00], "");
        assert_feed_err!(d, [], [0xf0], [0xff], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_immediate_test_finish() {
        for i in 0x81..0xa0 {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_ok!(d, [], [i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }

        for i in 0xe0..0xfd {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_ok!(d, [], [i], ""); // wait for a trail
            assert_finish_err!(d, "");
        }

        // A0/FD/FE/FF: immediate failure
        let mut d = Windows31JEncoding.raw_decoder();
        assert_feed_err!(d, [], [0xa0], [], "");
        assert_feed_err!(d, [], [0xfd], [], "");
        assert_feed_err!(d, [], [0xfe], [], "");
        assert_feed_err!(d, [], [0xff], [], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_lone_lead_followed_by_space() {
        for i in 0x81..0xa0 {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x20], "");
            assert_finish_ok!(d, "");
        }

        for i in 0xe0..0xfd {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x20], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_lead_followed_by_invalid_trail() {
        for i in 0x81..0xa0 {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x3f], "");
            assert_feed_err!(d, [], [i], [0x7f], "");
            assert_feed_err!(d, [], [i], [0xfd], "");
            assert_feed_err!(d, [], [i], [0xfe], "");
            assert_feed_err!(d, [], [i], [0xff], "");
            assert_finish_ok!(d, "");
        }

        for i in 0xe0..0xfd {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_err!(d, [], [i], [0x3f], "");
            assert_feed_err!(d, [], [i], [0x7f], "");
            assert_feed_err!(d, [], [i], [0xfd], "");
            assert_feed_err!(d, [], [i], [0xfe], "");
            assert_feed_err!(d, [], [i], [0xff], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_invalid_lead_followed_by_invalid_trail_partial() {
        for i in 0x81..0xa0 {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_ok!(d, [], [i], "");
            assert_feed_err!(d, [], [], [0xff], "");
            assert_finish_ok!(d, "");
        }

        for i in 0xe0..0xfd {
            let mut d = Windows31JEncoding.raw_decoder();
            assert_feed_ok!(d, [], [i], "");
            assert_feed_err!(d, [], [], [0xff], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_decoder_feed_after_finish() {
        let mut d = Windows31JEncoding.raw_decoder();
        assert_feed_ok!(d, [0x82, 0xa0], [0x82], "\u{3042}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0x82, 0xa0], [], "\u{3042}");
        assert_finish_ok!(d, "");
    }

    #[bench]
    fn bench_encode_short_text(bencher: &mut test::Bencher) {
        let s = testutils::JAPANESE_TEXT;
        bencher.bytes = s.len() as u64;
        bencher.iter(|| test::black_box({
            Windows31JEncoding.encode(&s, EncoderTrap::Strict)
        }))
    }

    #[bench]
    fn bench_decode_short_text(bencher: &mut test::Bencher) {
        let s = Windows31JEncoding.encode(testutils::JAPANESE_TEXT,
                                          EncoderTrap::Strict).ok().unwrap();
        bencher.bytes = s.len() as u64;
        bencher.iter(|| test::black_box({
            Windows31JEncoding.decode(&s, DecoderTrap::Strict)
        }))
    }
}

#[cfg(test)]
mod iso2022jp_tests {
    extern crate test;
    use super::ISO2022JPEncoding;
    use testutils;
    use types::*;

    #[test]
    fn test_encoder_valid() {
        let mut e = ISO2022JPEncoding.raw_encoder();
        assert_feed_ok!(e, "A", "", [0x41]);
        assert_feed_ok!(e, "BC", "", [0x42, 0x43]);
        assert_feed_ok!(e, "\x1b\x24\x42", "", [0x1b, 0x24, 0x42]); // no round-trip guarantee
        assert_feed_ok!(e, "", "", []);
        assert_feed_ok!(e, "\u{a5}", "", [0x5c]);
        assert_feed_ok!(e, "\u{203e}", "", [0x7e]);
        assert_feed_ok!(e, "\u{306b}\u{307b}\u{3093}", "", [0x1b, 0x24, 0x42,
                                                            0x24, 0x4b, 0x24, 0x5b, 0x24, 0x73]);
        assert_feed_ok!(e, "\u{65e5}\u{672c}", "", [0x46, 0x7c, 0x4b, 0x5c]);
        assert_feed_ok!(e, "\u{ff86}\u{ff8e}\u{ff9d}", "", [0x1b, 0x28, 0x49,
                                                            0x46, 0x4e, 0x5d]);
        assert_feed_ok!(e, "XYZ", "", [0x1b, 0x28, 0x42,
                                       0x58, 0x59, 0x5a]);
        assert_finish_ok!(e, []);

        // one ASCII character and two similarly looking characters:
        // - A: U+0020 SPACE (requires ASCII state)
        // - B: U+30CD KATAKANA LETTER NE (requires JIS X 0208 Lead state)
        // - C: U+FF88 HALFWIDTH KATAKANA LETTER NE (requires Katakana state)
        // - D is omitted as the encoder does not support JIS X 0212.
        // a (3,2) De Bruijn near-sequence "ABCACBA" is used to test all possible cases.
        const AD: &'static str = "\x20";
        const BD: &'static str = "\u{30cd}";
        const CD: &'static str = "\u{ff88}";
        const AE: &'static [u8] = &[0x1b, 0x28, 0x42, 0x20];
        const BE: &'static [u8] = &[0x1b, 0x24, 0x42, 0x25, 0x4d];
        const CE: &'static [u8] = &[0x1b, 0x28, 0x49, 0x48];
        let mut e = ISO2022JPEncoding.raw_encoder();
        let decoded: String = ["\x20",      BD, CD, AD, CD, BD, AD].concat();
        let encoded: Vec<_> = [&[0x20][..], BE, CE, AE, CE, BE, AE].concat();
        assert_feed_ok!(e, decoded, "", encoded);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_encoder_invalid() {
        let mut e = ISO2022JPEncoding.raw_encoder();
        assert_feed_err!(e, "", "\u{ffff}", "", []);
        assert_feed_err!(e, "?", "\u{ffff}", "!", [0x3f]);
        // JIS X 0212 is not supported in the encoder
        assert_feed_err!(e, "", "\u{736c}", "\u{8c78}", []);
        assert_finish_ok!(e, []);
    }

    #[test]
    fn test_decoder_valid() {
        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x41], [], "A");
        assert_feed_ok!(d, [0x42, 0x43], [], "BC");
        assert_feed_ok!(d, [0x1b, 0x28, 0x4a,
                            0x44, 0x45, 0x46], [], "DEF");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d, [0x5c], [], "\\");
        assert_feed_ok!(d, [0x7e], [], "~");
        assert_feed_ok!(d, [0x1b, 0x24, 0x42,
                            0x24, 0x4b,
                            0x1b, 0x24, 0x42,
                            0x24, 0x5b, 0x24, 0x73], [], "\u{306b}\u{307b}\u{3093}");
        assert_feed_ok!(d, [0x46, 0x7c, 0x4b, 0x5c], [], "\u{65e5}\u{672c}");
        assert_feed_ok!(d, [0x1b, 0x28, 0x49,
                            0x46, 0x4e, 0x5d], [], "\u{ff86}\u{ff8e}\u{ff9d}");
        assert_feed_ok!(d, [0x1b, 0x24, 0x28, 0x44,
                            0x4b, 0x46,
                            0x1b, 0x24, 0x40,
                            0x6c, 0x38], [], "\u{736c}\u{8c78}");
        assert_feed_ok!(d, [0x1b, 0x28, 0x42,
                            0x58, 0x59, 0x5a], [], "XYZ");
        assert_finish_ok!(d, "");

        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x1b, 0x24, 0x42,
                            0x24, 0x4b, 0x24, 0x5b, 0x24, 0x73], [], "\u{306b}\u{307b}\u{3093}");
        assert_finish_ok!(d, "");

        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x1b, 0x28, 0x49,
                            0x46, 0x4e, 0x5d], [], "\u{ff86}\u{ff8e}\u{ff9d}");
        assert_finish_ok!(d, "");

        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x1b, 0x24, 0x28, 0x44,
                            0x4b, 0x46], [], "\u{736c}");
        assert_finish_ok!(d, "");

        // one ASCII character and three similarly looking characters:
        // - A: U+0020 SPACE (requires ASCII state)
        // - B: U+30CD KATAKANA LETTER NE (requires JIS X 0208 Lead state)
        // - C: U+FF88 HALFWIDTH KATAKANA LETTER NE (requires Katakana state)
        // - D: U+793B CJK UNIFIED IDEOGRAPH-793B (requires JIS X 0212 Lead state)
        // a (4,2) De Bruijn sequence "AABBCCACBADDBDCDA" is used to test all possible cases.
        const AD: &'static str = "\x20";
        const BD: &'static str = "\u{30cd}";
        const CD: &'static str = "\u{ff88}";
        const DD: &'static str = "\u{793b}";
        const AE: &'static [u8] = &[0x1b, 0x28, 0x42,       0x20];
        const BE: &'static [u8] = &[0x1b, 0x24, 0x42,       0x25, 0x4d];
        const CE: &'static [u8] = &[0x1b, 0x28, 0x49,       0x48];
        const DE: &'static [u8] = &[0x1b, 0x24, 0x28, 0x44, 0x50, 0x4b];
        let mut d = ISO2022JPEncoding.raw_decoder();
        let dec: String = ["\x20",     AD,BD,BD,CD,CD,AD,CD,BD,AD,DD,DD,BD,DD,CD,DD,AD].concat();
        let enc: Vec<_> = [&[0x20][..],AE,BE,BE,CE,CE,AE,CE,BE,AE,DE,DE,BE,DE,CE,DE,AE].concat();
        assert_feed_ok!(d, enc, [], dec);
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_valid_partial() {
        let mut d = ISO2022JPEncoding.raw_decoder();

        assert_feed_ok!(d, [], [0x1b], "");
        assert_feed_ok!(d, [], [0x28], "");
        assert_feed_ok!(d, [0x4a, 0x41], [], "A");
        assert_feed_ok!(d, [], [0x1b, 0x28], "");
        assert_feed_ok!(d, [0x4a, 0x42], [0x1b], "B");
        assert_feed_ok!(d, [0x28, 0x4a, 0x43], [], "C");

        assert_feed_ok!(d, [], [0x1b], "");
        assert_feed_ok!(d, [], [0x24], "");
        assert_feed_ok!(d, [0x42], [0x24], "");
        assert_feed_ok!(d, [0x4b], [0x1b, 0x24], "\u{306b}");
        assert_feed_ok!(d, [0x42, 0x24, 0x5b], [], "\u{307b}");
        assert_feed_ok!(d, [], [0x1b], "");
        assert_feed_ok!(d, [0x24, 0x42, 0x24, 0x73], [], "\u{3093}");

        assert_feed_ok!(d, [], [0x1b], "");
        assert_feed_ok!(d, [], [0x28], "");
        assert_feed_ok!(d, [0x49, 0x46], [], "\u{ff86}");
        assert_feed_ok!(d, [], [0x1b, 0x28], "");
        assert_feed_ok!(d, [0x49, 0x4e], [0x1b], "\u{ff8e}");
        assert_feed_ok!(d, [0x28, 0x49, 0x5d], [], "\u{ff9d}");

        assert_feed_ok!(d, [], [0x1b, 0x24], "");
        assert_feed_ok!(d, [], [0x28], "");
        assert_feed_ok!(d, [0x44], [0x4b], "");
        assert_feed_ok!(d, [0x46], [0x1b, 0x24, 0x28], "\u{736c}");
        assert_feed_ok!(d, [0x44, 0x4b, 0x46], [], "\u{736c}");

        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_carriage_return() {
        // CR in Lead state "resets to ASCII"
        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x1b, 0x24, 0x42,
                            0x25, 0x4d,
                            0x0a,
                            0x25, 0x4d], [], "\u{30cd}\n\x25\x4d");
        assert_feed_ok!(d, [0x1b, 0x24, 0x28, 0x44,
                            0x50, 0x4b,
                            0x0a,
                            0x50, 0x4b], [], "\u{793b}\n\x50\x4b");
        assert_finish_ok!(d, "");

        // other states don't allow CR
        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_err!(d, [0x1b, 0x28, 0x49, 0x48], [0x0a], [], "\u{ff88}"); // Katakana
        assert_feed_err!(d, [0x1b, 0x24, 0x42], [0x25, 0x0a], [], ""); // Trail
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_partial() {
        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x1b, 0x24, 0x42, 0x24, 0x4b], [0x24], "\u{306b}");
        assert_finish_err!(d, "");

        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x1b, 0x24, 0x28, 0x44, 0x4b, 0x46], [0x50], "\u{736c}");
        assert_finish_err!(d, "");
    }

    #[test]
    fn test_decoder_invalid_partial_escape() {
        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [], [0x1b], "");
        assert_finish_err!(d, "");

        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [], [0x1b, 0x24], "");
        assert_finish_err!(d, ""); // no backup

        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [], [0x1b, 0x24, 0x28], "");
        assert_finish_err!(d, -1, ""); // backup of -1, not -2

        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [], [0x1b, 0x28], "");
        assert_finish_err!(d, ""); // no backup

        assert_eq!(ISO2022JPEncoding.decode(&[0x1b], DecoderTrap::Replace),
                   Ok("\u{fffd}".to_string()));
        assert_eq!(ISO2022JPEncoding.decode(&[0x1b, 0x24], DecoderTrap::Replace),
                   Ok("\u{fffd}".to_string()));
        assert_eq!(ISO2022JPEncoding.decode(&[0x1b, 0x24, 0x28], DecoderTrap::Replace),
                   Ok("\u{fffd}\x28".to_string()));
        assert_eq!(ISO2022JPEncoding.decode(&[0x1b, 0x28], DecoderTrap::Replace),
                   Ok("\u{fffd}".to_string()));
    }

    #[test]
    fn test_decoder_invalid_escape() {
        // also tests allowed but never used escape codes in ISO 2022
        let mut d = ISO2022JPEncoding.raw_decoder();
        macro_rules! reset(() => (
            assert_feed_ok!(d, [0x41, 0x42, 0x43, 0x1b, 0x24, 0x42, 0x21, 0x21], [],
                            "ABC\u{3000}")
        ));

        reset!();
        assert_feed_ok!(d, [], [0x1b], "");
        assert_feed_err!(d, [], [], [0x00], "");
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x0a], "");
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x20], "");
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x21, 0x5a], ""); // ESC ! Z (CZD)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x22, 0x5a], ""); // ESC " Z (C1D)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x5a], ""); // ESC $ Z (GZDM4)
        reset!();
        assert_feed_ok!(d, [], [0x1b, 0x24], "");
        assert_feed_err!(d, -1, [], [], [0x24, 0x5a], "");
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x28, 0x5a], ""); // ESC $ ( Z (GZDM4)
        reset!();
        assert_feed_ok!(d, [], [0x1b, 0x24, 0x28], "");
        assert_feed_err!(d, -2, [], [], [0x24, 0x28, 0x5a], "");
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x29, 0x5a], ""); // ESC $ ) Z (G1DM4)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x2a, 0x5a], ""); // ESC $ * Z (G2DM4)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x2b, 0x5a], ""); // ESC $ + Z (G3DM4)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x2d, 0x5a], ""); // ESC $ - Z (G1DM6)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x2e, 0x5a], ""); // ESC $ . Z (G2DM6)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x24, 0x2f, 0x5a], ""); // ESC $ / Z (G3DM6)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x25, 0x5a], ""); // ESC % Z (DOCS)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x25, 0x2f, 0x5a], ""); // ESC % / Z (DOCS)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x28, 0x5a], ""); // ESC ( Z (GZD4)
        reset!();
        assert_feed_ok!(d, [], [0x1b, 0x28], "");
        assert_feed_err!(d, -1, [], [], [0x28, 0x5a], "");
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x29, 0x5a], ""); // ESC ) Z (G1D4)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x2a, 0x5a], ""); // ESC * Z (G2D4)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x2b, 0x5a], ""); // ESC + Z (G3D4)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x2d, 0x5a], ""); // ESC - Z (G1D6)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x2e, 0x5a], ""); // ESC . Z (G2D6)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x2f, 0x5a], ""); // ESC / Z (G3D6)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x4e], ""); // ESC N (SS2)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x4f], ""); // ESC O (SS3)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x6e], ""); // ESC n (LS2)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x6f], ""); // ESC o (LS3)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x7c], ""); // ESC | (LS3R)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x7d], ""); // ESC } (LS2R)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0x7e], ""); // ESC ~ (LS1R)
        reset!();
        assert_feed_err!(d, [], [0x1b], [0xff], "");
        reset!();
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_invalid_out_or_range() {
        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_err!(d, [], [0x80], [], "");
        assert_feed_err!(d, [], [0xff], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x42], [0x80, 0x21], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x42], [0x21, 0x80], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x42], [0x20, 0x21], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x42], [0x21, 0x20], [], "");
        assert_feed_err!(d, [0x1b, 0x28, 0x49], [0x20], [], "");
        assert_feed_err!(d, [0x1b, 0x28, 0x49], [0x60], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x28, 0x44], [0x80, 0x21], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x28, 0x44], [0x21, 0x80], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x28, 0x44], [0x20, 0x21], [], "");
        assert_feed_err!(d, [0x1b, 0x24, 0x28, 0x44], [0x21, 0x20], [], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_decoder_feed_after_finish() {
        let mut d = ISO2022JPEncoding.raw_decoder();
        assert_feed_ok!(d, [0x24, 0x22,
                            0x1b, 0x24, 0x42,
                            0x24, 0x22], [0x24], "\x24\x22\u{3042}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0x24, 0x22,
                            0x1b, 0x24, 0x42,
                            0x24, 0x22], [], "\x24\x22\u{3042}");
        assert_finish_ok!(d, "");
    }

    #[bench]
    fn bench_encode_short_text(bencher: &mut test::Bencher) {
        let s = testutils::JAPANESE_TEXT;
        bencher.bytes = s.len() as u64;
        bencher.iter(|| test::black_box({
            ISO2022JPEncoding.encode(&s, EncoderTrap::Strict)
        }))
    }

    #[bench]
    fn bench_decode_short_text(bencher: &mut test::Bencher) {
        let s = ISO2022JPEncoding.encode(testutils::JAPANESE_TEXT,
                                         EncoderTrap::Strict).ok().unwrap();
        bencher.bytes = s.len() as u64;
        bencher.iter(|| test::black_box({
            ISO2022JPEncoding.decode(&s, DecoderTrap::Strict)
        }))
    }
}
