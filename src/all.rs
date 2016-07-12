// This is a part of rust-encoding.
// Copyright (c) 2013, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! A list of all supported encodings. Useful for encodings fixed in the compile time.

use codec;
use compat;
use types::EncodingRef;

pub const ASCII: &'static codec::ascii::ASCIIEncoding = &codec::ascii::ASCIIEncoding {};

// BEGIN GENERATED CODE. PLEASE DO NOT EDIT.
// Instead, please regenerate using generate_constants.py

/// The Big5 encoding.
pub const BIG5_2003: &'static compat::EncodingWrap = compat::BIG5;

/// The EUC-JP encoding.
pub const EUC_JP: &'static compat::EncodingWrap = compat::EUC_JP;

/// The EUC-KR encoding.
pub const WINDOWS_949: &'static compat::EncodingWrap = compat::EUC_KR;

/// The GBK encoding.
pub const GBK: &'static compat::EncodingWrap = compat::GBK;

/// The IBM866 encoding.
pub const IBM866: &'static compat::EncodingWrap = compat::IBM866;

/// The ISO-2022-JP encoding.
pub const ISO_2022_JP: &'static compat::EncodingWrap = compat::ISO_2022_JP;

/// The ISO-8859-10 encoding.
pub const ISO_8859_10: &'static compat::EncodingWrap = compat::ISO_8859_10;

/// The ISO-8859-13 encoding.
pub const ISO_8859_13: &'static compat::EncodingWrap = compat::ISO_8859_13;

/// The ISO-8859-14 encoding.
pub const ISO_8859_14: &'static compat::EncodingWrap = compat::ISO_8859_14;

/// The ISO-8859-15 encoding.
pub const ISO_8859_15: &'static compat::EncodingWrap = compat::ISO_8859_15;

/// The ISO-8859-16 encoding.
pub const ISO_8859_16: &'static compat::EncodingWrap = compat::ISO_8859_16;

/// The ISO-8859-2 encoding.
pub const ISO_8859_2: &'static compat::EncodingWrap = compat::ISO_8859_2;

/// The ISO-8859-3 encoding.
pub const ISO_8859_3: &'static compat::EncodingWrap = compat::ISO_8859_3;

/// The ISO-8859-4 encoding.
pub const ISO_8859_4: &'static compat::EncodingWrap = compat::ISO_8859_4;

/// The ISO-8859-5 encoding.
pub const ISO_8859_5: &'static compat::EncodingWrap = compat::ISO_8859_5;

/// The ISO-8859-6 encoding.
pub const ISO_8859_6: &'static compat::EncodingWrap = compat::ISO_8859_6;

/// The ISO-8859-7 encoding.
pub const ISO_8859_7: &'static compat::EncodingWrap = compat::ISO_8859_7;

/// The ISO-8859-8 encoding.
pub const ISO_8859_8: &'static compat::EncodingWrap = compat::ISO_8859_8;

/// The KOI8-R encoding.
pub const KOI8_R: &'static compat::EncodingWrap = compat::KOI8_R;

/// The KOI8-U encoding.
pub const KOI8_U: &'static compat::EncodingWrap = compat::KOI8_U;

/// The Shift_JIS encoding.
pub const WINDOWS_31J: &'static compat::EncodingWrap = compat::SHIFT_JIS;

/// The UTF-16BE encoding.
pub const UTF_16BE: &'static compat::EncodingWrap = compat::UTF_16BE;

/// The UTF-16LE encoding.
pub const UTF_16LE: &'static compat::EncodingWrap = compat::UTF_16LE;

/// The UTF-8 encoding.
pub const UTF_8: &'static compat::EncodingWrap = compat::UTF_8;

/// The gb18030 encoding.
pub const GB18030: &'static compat::EncodingWrap = compat::GB18030;

/// The macintosh encoding.
pub const MAC_ROMAN: &'static compat::EncodingWrap = compat::MACINTOSH;

/// The windows-1250 encoding.
pub const WINDOWS_1250: &'static compat::EncodingWrap = compat::WINDOWS_1250;

/// The windows-1251 encoding.
pub const WINDOWS_1251: &'static compat::EncodingWrap = compat::WINDOWS_1251;

/// The windows-1252 encoding.
pub const WINDOWS_1252: &'static compat::EncodingWrap = compat::WINDOWS_1252;

/// The windows-1253 encoding.
pub const WINDOWS_1253: &'static compat::EncodingWrap = compat::WINDOWS_1253;

/// The windows-1254 encoding.
pub const WINDOWS_1254: &'static compat::EncodingWrap = compat::WINDOWS_1254;

/// The windows-1255 encoding.
pub const WINDOWS_1255: &'static compat::EncodingWrap = compat::WINDOWS_1255;

/// The windows-1256 encoding.
pub const WINDOWS_1256: &'static compat::EncodingWrap = compat::WINDOWS_1256;

/// The windows-1257 encoding.
pub const WINDOWS_1257: &'static compat::EncodingWrap = compat::WINDOWS_1257;

/// The windows-1258 encoding.
pub const WINDOWS_1258: &'static compat::EncodingWrap = compat::WINDOWS_1258;

/// The windows-874 encoding.
pub const WINDOWS_874: &'static compat::EncodingWrap = compat::WINDOWS_874;

/// The x-mac-cyrillic encoding.
pub const MAC_CYRILLIC: &'static compat::EncodingWrap = compat::X_MAC_CYRILLIC;

// END GENERATED CODE
pub mod whatwg {
    use compat;
    pub const X_USER_DEFINED: &'static compat::EncodingWrap = compat::X_USER_DEFINED;
    pub const ISO_8859_8_I: &'static compat::EncodingWrap = compat::ISO_8859_8_I;
    pub const REPLACEMENT: &'static compat::EncodingWrap = compat::REPLACEMENT;
}

/// Returns a list of references to the encodings available.
pub fn encodings() -> &'static [EncodingRef] {
    // TODO should be generated automatically
    const ENCODINGS: &'static [EncodingRef] = &[ASCII,
                                                IBM866,
                                                //        ISO_8859_1,
                                                ISO_8859_2,
                                                ISO_8859_3,
                                                ISO_8859_4,
                                                ISO_8859_5,
                                                ISO_8859_6,
                                                ISO_8859_7,
                                                ISO_8859_8,
                                                ISO_8859_10,
                                                ISO_8859_13,
                                                ISO_8859_14,
                                                ISO_8859_15,
                                                ISO_8859_16,
                                                KOI8_R,
                                                KOI8_U,
                                                MAC_ROMAN,
                                                WINDOWS_874,
                                                WINDOWS_1250,
                                                WINDOWS_1251,
                                                WINDOWS_1252,
                                                WINDOWS_1253,
                                                WINDOWS_1254,
                                                WINDOWS_1255,
                                                WINDOWS_1256,
                                                WINDOWS_1257,
                                                WINDOWS_1258,
                                                MAC_CYRILLIC,
                                                UTF_8,
                                                UTF_16LE,
                                                UTF_16BE,
                                                WINDOWS_949,
                                                EUC_JP,
                                                WINDOWS_31J,
                                                ISO_2022_JP,
                                                GBK,
                                                GB18030,
                                                BIG5_2003,
                                                whatwg::X_USER_DEFINED,
                                                whatwg::ISO_8859_8_I,
                                                whatwg::REPLACEMENT];
    ENCODINGS
}
