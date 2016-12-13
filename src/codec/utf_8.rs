// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.
//
// Portions Copyright (c) 2008-2009 Bjoern Hoehrmann <bjoern@hoehrmann.de>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! UTF-8, the universal encoding.

extern crate encoding_rs;

/// Almost equivalent to `std::str::from_utf8`.
/// This function is provided for the fair benchmark against the stdlib's UTF-8 conversion
/// functions, as rust-encoding always allocates a new string.
#[cfg(test)]
pub fn from_utf8<'a>(input: &'a [u8]) -> Option<&'a str> {
    // encoding_rs needs an output buffer, so the purpose of this function is
    // somewhat defeated.
    let mut buffer: [u8; 124400] = unsafe { ::std::mem::uninitialized() };
    let mut decoder = encoding_rs::UTF_8.new_decoder_without_bom_handling();
    let (result, _, _) = decoder.decode_to_utf8_without_replacement(input, &mut buffer[..], true);
    match result {
        encoding_rs::DecoderResult::InputEmpty => Some(unsafe { ::std::mem::transmute(input) }),
        encoding_rs::DecoderResult::OutputFull => {
            unreachable!("Stack buffer too small.");
        }
        encoding_rs::DecoderResult::Malformed(_, _) => None,
    }
}

#[cfg(test)]
mod tests {
    // portions of these tests are adopted from Markus Kuhn's UTF-8 decoder capability and
    // stress test: <http://www.cl.cam.ac.uk/~mgk25/ucs/examples/UTF-8-test.txt>.

    use super::from_utf8;
    use std::str;
    use testutils;
    use types::*;
    use compat;

    static UTF8Encoding: EncodingRef = &compat::UTF_8;

    #[test]
    fn test_valid() {
        // one byte
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0x41], [], "A");
        assert_feed_ok!(d, [0x42, 0x43], [], "BC");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d, [0x44, 0x45, 0x46], [], "DEF");
        assert_finish_ok!(d, "");

        // two bytes
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xc2, 0xa2], [], "\u{a2}");
        assert_feed_ok!(d, [0xc2, 0xac, 0xc2, 0xa9], [], "\u{ac}\u{0a9}");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d,
                        [0xd5, 0xa1, 0xd5, 0xb5, 0xd5, 0xa2, 0xd5, 0xb8, 0xd6, 0x82, 0xd5, 0xa2,
                         0xd5, 0xa5, 0xd5, 0xb6],
                        [],
                        "\u{561}\u{0575}\u{562}\u{578}\u{582}\u{562}\u{565}\u{576}");
        assert_finish_ok!(d, "");

        // three bytes
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xed, 0x92, 0x89], [], "\u{d489}");
        assert_feed_ok!(d,
                        [0xe6, 0xbc, 0xa2, 0xe5, 0xad, 0x97],
                        [],
                        "\u{6f22}\u{5b57}");
        assert_feed_ok!(d, [], [], "");
        assert_feed_ok!(d,
                        [0xc9, 0x99, 0xc9, 0x94, 0xc9, 0x90],
                        [],
                        "\u{259}\u{0254}\u{250}");
        assert_finish_ok!(d, "");

        // four bytes
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xf0, 0x90, 0x82, 0x82], [], "\u{10082}");
        assert_feed_ok!(d, [], [], "");
        assert_finish_ok!(d, "");

        // we don't test encoders as it is largely a no-op.
    }

    #[test]
    fn test_valid_boundary() {
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0x00], [], "\x00");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0x7f], [], "\x7f");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xc2, 0x80], [], "\u{80}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xdf, 0xbf], [], "\u{7ff}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xe0, 0xa0, 0x80], [], "\u{800}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xed, 0x9f, 0xbf], [], "\u{d7ff}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xee, 0x80, 0x80], [], "\u{e000}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xef, 0xbf, 0xbf], [], "\u{ffff}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xf0, 0x90, 0x80, 0x80], [], "\u{10000}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xf4, 0x8f, 0xbf, 0xbf], [], "\u{10ffff}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_valid_partial() {
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [], [0xf0], "");
        assert_feed_ok!(d, [], [0x90], "");
        assert_feed_ok!(d, [], [0x82], "");
        assert_feed_ok!(d, [0x82], [0xed], "\u{10082}");
        assert_feed_ok!(d, [0x92, 0x89], [], "\u{d489}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [], [0xc2], "");
        assert_feed_ok!(d, [0xa9, 0x20], [], "\u{a9}\u{020}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_invalid_continuation() {
        for c in 0x80..0xc0 {
            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [c], [], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [c], [c], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [c], [c, c], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_invalid_surrogate() {
        // surrogates should fail at the second byte.

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xed], [0xa0, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xed], [0xad, 0xbf], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xed], [0xae, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xed], [0xaf, 0xbf], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xed], [0xb0, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xed], [0xbe, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xed], [0xbf, 0xbf], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_invalid_boundary() {
        // as with surrogates, should fail at the second byte.
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xf4], [0x90, 0x90, 0x90], ""); // U+110000
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_invalid_start_immediate_test_finish() {
        for c in 0xf5..0x100 {
            let c = c as u8;
            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [c], [], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_invalid_start_followed_by_space() {
        for c in 0xf5..0x100 {
            let c = c as u8;

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [c], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [c], [], "");
            assert_feed_ok!(d, [0x20], [], "\x20");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_invalid_lone_start_immediate_test_finish() {
        for c in 0xc2..0xf5 {
            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [c], ""); // wait for cont. bytes
            assert_finish_err!(d, "");
        }
    }

    #[test]
    fn test_invalid_lone_start_followed_by_space() {
        for c in 0xc2..0xf5 {
            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [c], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [c], ""); // wait for cont. bytes
            assert_feed_err!(d, [], [], [0x20], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_invalid_incomplete_three_byte_seq_followed_by_space() {
        for b in 0xe0..0xf5 {
            let c = if b == 0xe0 || b == 0xf0 {
                0xa0
            } else {
                0x80
            };

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [b, c], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [b, c], ""); // wait for cont. bytes
            assert_feed_err!(d, [], [], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [b], ""); // wait for cont. bytes
            assert_feed_err!(d, [], [c], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [b], ""); // wait for cont. bytes
            assert_feed_ok!(d, [], [c], ""); // wait for cont. bytes
            assert_feed_err!(d, [], [], [0x20], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_invalid_incomplete_four_byte_seq_followed_by_space() {
        for a in 0xf0..0xf5 {
            let b = if a == 0xf0 {
                0xa0
            } else {
                0x80
            };
            let c = 0x80;

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_err!(d, [], [a, b, c], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [a], ""); // wait for cont. bytes
            assert_feed_ok!(d, [], [b], ""); // wait for cont. bytes
            assert_feed_ok!(d, [], [c], ""); // wait for cont. bytes
            assert_feed_err!(d, [], [], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [a, b], ""); // wait for cont. bytes
            assert_feed_err!(d, [], [c], [0x20], "");
            assert_finish_ok!(d, "");

            let mut d = UTF8Encoding.raw_decoder();
            assert_feed_ok!(d, [], [a, b, c], ""); // wait for cont. bytes
            assert_feed_err!(d, [], [], [0x20], "");
            assert_finish_ok!(d, "");
        }
    }

    #[test]
    fn test_invalid_too_many_cont_bytes() {
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [0xc2, 0x80], [0x80], [], "\u{80}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [0xe0, 0xa0, 0x80], [0x80], [], "\u{800}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [0xf0, 0x90, 0x80, 0x80], [0x80], [], "\u{10000}");
        assert_finish_ok!(d, "");

        // no continuation byte is consumed after 5/6-byte sequence starters and FE/FF
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xf8], [0x88, 0x80, 0x80, 0x80, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xfc], [0x84, 0x80, 0x80, 0x80, 0x80, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xfe], [0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xff], [0x80], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_invalid_too_many_cont_bytes_partial() {
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [], [0xc2], "");
        assert_feed_err!(d, [0x80], [0x80], [], "\u{80}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [], [0xe0, 0xa0], "");
        assert_feed_err!(d, [0x80], [0x80], [], "\u{800}");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [], [0xf0, 0x90, 0x80], "");
        assert_feed_err!(d, [0x80], [0x80], [], "\u{10000}");
        assert_finish_ok!(d, "");

        // no continuation byte is consumed after 5/6-byte sequence starters and FE/FF
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xf8], [], "");
        assert_feed_err!(d, [], [0x88], [0x80, 0x80, 0x80, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xfc], [], "");
        assert_feed_err!(d, [], [0x84], [0x80, 0x80, 0x80, 0x80, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xfe], [], "");
        assert_feed_err!(d, [], [0x80], [], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xff], [], "");
        assert_feed_err!(d, [], [0x80], [], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_invalid_overlong_minimal() {
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xc0], [0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xe0], [0x80, 0x80], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xf0], [0x80, 0x80, 0x80], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_invalid_overlong_maximal() {
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xc1], [0xbf], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xe0], [0x9f, 0xbf], "");
        assert_finish_ok!(d, "");

        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_err!(d, [], [0xf0], [0x8f, 0xbf, 0xbf], "");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_feed_after_finish() {
        let mut d = UTF8Encoding.raw_decoder();
        assert_feed_ok!(d, [0xc2, 0x80], [0xc2], "\u{80}");
        assert_finish_err!(d, "");
        assert_feed_ok!(d, [0xc2, 0x80], [], "\u{80}");
        assert_finish_ok!(d, "");
    }

    #[test]
    fn test_correct_from_utf8() {
        let s = testutils::ASCII_TEXT.as_bytes();
        assert_eq!(from_utf8(s), str::from_utf8(s).ok());

        let s = testutils::KOREAN_TEXT.as_bytes();
        assert_eq!(from_utf8(s), str::from_utf8(s).ok());

        let s = testutils::INVALID_UTF8_TEXT;
        assert_eq!(from_utf8(s), str::from_utf8(s).ok());
    }

    mod bench_ascii {
        #[cfg(nightly)]
        extern crate test;
        use super::super::from_utf8;
        use std::str;
        use testutils;
        use types::*;
        use compat;

        static UTF8Encoding: EncodingRef = &compat::UTF_8;

        #[cfg(nightly)]
        #[bench]
        fn bench_encode(bencher: &mut test::Bencher) {
            let s = testutils::ASCII_TEXT;
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    UTF8Encoding.encode(s, EncoderTrap::Strict)
                })
            })
        }

        #[cfg(nightly)]
        #[bench]
        fn bench_decode(bencher: &mut test::Bencher) {
            let s = testutils::ASCII_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    UTF8Encoding.decode(s, DecoderTrap::Strict)
                })
            })
        }

        #[cfg(nightly)]
        #[bench]
        fn bench_from_utf8(bencher: &mut test::Bencher) {
            let s = testutils::ASCII_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    from_utf8(s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8(bencher: &mut test::Bencher) {
            let s = testutils::ASCII_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    str::from_utf8(s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8_lossy(bencher: &mut test::Bencher) {
            let s = testutils::ASCII_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    String::from_utf8_lossy(s)
                })
            })
        }
    }

    // why Korean? it has an excellent mix of multibyte sequences and ASCII sequences
    // unlike other CJK scripts, so it reflects a practical use case a bit better.
    mod bench_korean {
        #[cfg(nightly)]
        extern crate test;
        use super::super::from_utf8;
        use std::str;
        use testutils;
        use types::*;
        use compat;

        static UTF8Encoding: EncodingRef = &compat::UTF_8;

        #[cfg(nightly)]
        #[bench]
        fn bench_encode(bencher: &mut test::Bencher) {
            let s = testutils::KOREAN_TEXT;
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    UTF8Encoding.encode(s, EncoderTrap::Strict)
                })
            })
        }

        #[cfg(nightly)]
        #[bench]
        fn bench_decode(bencher: &mut test::Bencher) {
            let s = testutils::KOREAN_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    UTF8Encoding.decode(s, DecoderTrap::Strict)
                })
            })
        }

        #[cfg(nightly)]
        #[bench]
        fn bench_from_utf8(bencher: &mut test::Bencher) {
            let s = testutils::KOREAN_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    from_utf8(s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8(bencher: &mut test::Bencher) {
            let s = testutils::KOREAN_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    str::from_utf8(s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8_lossy(bencher: &mut test::Bencher) {
            let s = testutils::KOREAN_TEXT.as_bytes();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    String::from_utf8_lossy(s)
                })
            })
        }
    }

    mod bench_lossy_invalid {
        #[cfg(nightly)]
        extern crate test;
        use super::super::from_utf8;
        use std::str;
        use testutils;
        use types::*;
        use types::DecoderTrap::Replace as DecodeReplace;
        use compat;

        static UTF8Encoding: EncodingRef = &compat::UTF_8;

        #[cfg(nightly)]
        #[bench]
        fn bench_decode_replace(bencher: &mut test::Bencher) {
            let s = testutils::INVALID_UTF8_TEXT;
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    UTF8Encoding.decode(s, DecodeReplace)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_from_utf8_failing(bencher: &mut test::Bencher) {
            let s = testutils::INVALID_UTF8_TEXT;
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    from_utf8(s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8_failing(bencher: &mut test::Bencher) {
            let s = testutils::INVALID_UTF8_TEXT;
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    str::from_utf8(s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8_lossy(bencher: &mut test::Bencher) {
            let s = testutils::INVALID_UTF8_TEXT;
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    String::from_utf8_lossy(s)
                })
            })
        }
    }

    mod bench_lossy_external {
        #[cfg(nightly)]
        extern crate test;
        use super::super::from_utf8;
        use std::str;
        use testutils;
        use types::*;
        use types::DecoderTrap::Replace as DecodeReplace;
        use compat;

        static UTF8Encoding: EncodingRef = &compat::UTF_8;

        #[cfg(nightly)]
        #[bench]
        fn bench_decode_replace(bencher: &mut test::Bencher) {
            let s = testutils::get_external_bench_data();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    UTF8Encoding.decode(&s, DecodeReplace)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_from_utf8_failing(bencher: &mut test::Bencher) {
            let s = testutils::get_external_bench_data();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    from_utf8(&s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8_failing(bencher: &mut test::Bencher) {
            let s = testutils::get_external_bench_data();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    str::from_utf8(&s)
                })
            })
        }

        #[cfg(nightly)]
        #[bench] // for the comparison
        fn bench_stdlib_from_utf8_lossy(bencher: &mut test::Bencher) {
            let s = testutils::get_external_bench_data();
            bencher.bytes = s.len() as u64;
            bencher.iter(|| {
                test::black_box({
                    String::from_utf8_lossy(&s)
                })
            })
        }
    }
}
