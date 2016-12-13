// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! An interface for retrieving an encoding (or a set of encodings) from a string/numeric label.

use all;
use types::EncodingRef;
use compat;

/// Returns an encoding from given label, defined in the WHATWG Encoding standard, if any.
/// Implements "get an encoding" algorithm: http://encoding.spec.whatwg.org/#concept-encoding-get
pub fn encoding_from_whatwg_label(label: &str) -> Option<EncodingRef> {
    compat::encoding_rs_for_label(label)
}

/// Returns an encoding from Windows code page number.
/// http://msdn.microsoft.com/en-us/library/windows/desktop/dd317756%28v=vs.85%29.aspx
/// Sometimes it can return a *superset* of the requested encoding, e.g. for several CJK encodings.
pub fn encoding_from_windows_code_page(cp: usize) -> Option<EncodingRef> {
    match cp {
        65001 => Some(all::UTF_8 as EncodingRef),
        866 => Some(all::IBM866 as EncodingRef),
        // 28591 => Some(all::ISO_8859_1 as EncodingRef),
        28592 => Some(all::ISO_8859_2 as EncodingRef),
        28593 => Some(all::ISO_8859_3 as EncodingRef),
        28594 => Some(all::ISO_8859_4 as EncodingRef),
        28595 => Some(all::ISO_8859_5 as EncodingRef),
        28596 => Some(all::ISO_8859_6 as EncodingRef),
        28597 => Some(all::ISO_8859_7 as EncodingRef),
        28598 => Some(all::ISO_8859_8 as EncodingRef),
        38598 => Some(all::whatwg::ISO_8859_8_I as EncodingRef),
        28603 => Some(all::ISO_8859_13 as EncodingRef),
        28605 => Some(all::ISO_8859_15 as EncodingRef),
        20866 => Some(all::KOI8_R as EncodingRef),
        21866 => Some(all::KOI8_U as EncodingRef),
        10000 => Some(all::MAC_ROMAN as EncodingRef),
        874 => Some(all::WINDOWS_874 as EncodingRef),
        1250 => Some(all::WINDOWS_1250 as EncodingRef),
        1251 => Some(all::WINDOWS_1251 as EncodingRef),
        1252 => Some(all::WINDOWS_1252 as EncodingRef),
        1253 => Some(all::WINDOWS_1253 as EncodingRef),
        1254 => Some(all::WINDOWS_1254 as EncodingRef),
        1255 => Some(all::WINDOWS_1255 as EncodingRef),
        1256 => Some(all::WINDOWS_1256 as EncodingRef),
        1257 => Some(all::WINDOWS_1257 as EncodingRef),
        1258 => Some(all::WINDOWS_1258 as EncodingRef),
        1259 => Some(all::MAC_CYRILLIC as EncodingRef),
        936 | 54936 => Some(all::GB18030 as EncodingRef), // XXX technically wrong
        950 => Some(all::BIG5_2003 as EncodingRef),
        20932 => Some(all::EUC_JP as EncodingRef),
        50220 => Some(all::ISO_2022_JP as EncodingRef),
        932 => Some(all::WINDOWS_31J as EncodingRef),
        949 => Some(all::WINDOWS_949 as EncodingRef),
        1201 => Some(all::UTF_16BE as EncodingRef),
        1200 => Some(all::UTF_16LE as EncodingRef),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use all;
    use super::encoding_from_whatwg_label;

    #[test]
    fn test_encoding_from_whatwg_label() {
        assert!(encoding_from_whatwg_label("utf-8").is_some());
        assert!(encoding_from_whatwg_label("UTF-8").is_some());
        assert!(encoding_from_whatwg_label("\t\n\x0C\r utf-8\t\n\x0C\r ").is_some());
        assert!(encoding_from_whatwg_label("\u{A0}utf-8").is_none(),
                "Non-ASCII whitespace should not be trimmed");
        assert!(encoding_from_whatwg_label("greek").is_some());
        assert!(encoding_from_whatwg_label("gree\u{212A}").is_none(),
                "Case-insensitive matching should be ASCII only. Kelvin sign does not match k.");

        // checks if the `whatwg_name` method returns the label that resolves back to that encoding
        for encoding in all::encodings() {
            if let Some(whatwg_name) = encoding.whatwg_name() {
                if whatwg_name == "replacement" {
                    continue;
                }
                assert_eq!(encoding_from_whatwg_label(whatwg_name).and_then(|e| e.whatwg_name()),
                           Some(whatwg_name));
            }
        }
    }

    #[bench]
    fn bench_encoding_from_whatwg_label(bencher: &mut test::Bencher) {
        bencher.iter(|| {
            test::black_box({
                encoding_from_whatwg_label("iso-8859-bazinga")
            })
        })
    }
}
