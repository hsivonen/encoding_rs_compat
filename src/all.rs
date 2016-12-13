// This is a part of rust-encoding.
// Copyright (c) 2013, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! A list of all supported encodings. Useful for encodings fixed in the compile time.

use codec;
use compat;
use types::EncodingRef;

static ASCII_INIT: codec::ascii::ASCIIEncoding = codec::ascii::ASCIIEncoding {};

pub static ASCII: &'static codec::ascii::ASCIIEncoding = &ASCII_INIT;

// BEGIN GENERATED CODE. PLEASE DO NOT EDIT.
// Instead, please regenerate using generate_constants.py

/// The Big5 encoding.
pub static BIG5_2003: &'static compat::EncodingWrap = &compat::BIG5;

/// The EUC-JP encoding.
pub static EUC_JP: &'static compat::EncodingWrap = &compat::EUC_JP;

/// The EUC-KR encoding.
pub static WINDOWS_949: &'static compat::EncodingWrap = &compat::EUC_KR;

/// The GBK encoding.
pub static GBK: &'static compat::EncodingWrap = &compat::GBK;

/// The IBM866 encoding.
pub static IBM866: &'static compat::EncodingWrap = &compat::IBM866;

/// The ISO-2022-JP encoding.
pub static ISO_2022_JP: &'static compat::EncodingWrap = &compat::ISO_2022_JP;

/// The ISO-8859-10 encoding.
pub static ISO_8859_10: &'static compat::EncodingWrap = &compat::ISO_8859_10;

/// The ISO-8859-13 encoding.
pub static ISO_8859_13: &'static compat::EncodingWrap = &compat::ISO_8859_13;

/// The ISO-8859-14 encoding.
pub static ISO_8859_14: &'static compat::EncodingWrap = &compat::ISO_8859_14;

/// The ISO-8859-15 encoding.
pub static ISO_8859_15: &'static compat::EncodingWrap = &compat::ISO_8859_15;

/// The ISO-8859-16 encoding.
pub static ISO_8859_16: &'static compat::EncodingWrap = &compat::ISO_8859_16;

/// The ISO-8859-2 encoding.
pub static ISO_8859_2: &'static compat::EncodingWrap = &compat::ISO_8859_2;

/// The ISO-8859-3 encoding.
pub static ISO_8859_3: &'static compat::EncodingWrap = &compat::ISO_8859_3;

/// The ISO-8859-4 encoding.
pub static ISO_8859_4: &'static compat::EncodingWrap = &compat::ISO_8859_4;

/// The ISO-8859-5 encoding.
pub static ISO_8859_5: &'static compat::EncodingWrap = &compat::ISO_8859_5;

/// The ISO-8859-6 encoding.
pub static ISO_8859_6: &'static compat::EncodingWrap = &compat::ISO_8859_6;

/// The ISO-8859-7 encoding.
pub static ISO_8859_7: &'static compat::EncodingWrap = &compat::ISO_8859_7;

/// The ISO-8859-8 encoding.
pub static ISO_8859_8: &'static compat::EncodingWrap = &compat::ISO_8859_8;

/// The KOI8-R encoding.
pub static KOI8_R: &'static compat::EncodingWrap = &compat::KOI8_R;

/// The KOI8-U encoding.
pub static KOI8_U: &'static compat::EncodingWrap = &compat::KOI8_U;

/// The Shift_JIS encoding.
pub static WINDOWS_31J: &'static compat::EncodingWrap = &compat::SHIFT_JIS;

/// The UTF-16BE encoding.
pub static UTF_16BE: &'static compat::EncodingWrap = &compat::UTF_16BE;

/// The UTF-16LE encoding.
pub static UTF_16LE: &'static compat::EncodingWrap = &compat::UTF_16LE;

/// The UTF-8 encoding.
pub static UTF_8: &'static compat::EncodingWrap = &compat::UTF_8;

/// The gb18030 encoding.
pub static GB18030: &'static compat::EncodingWrap = &compat::GB18030;

/// The macintosh encoding.
pub static MAC_ROMAN: &'static compat::EncodingWrap = &compat::MACINTOSH;

/// The windows-1250 encoding.
pub static WINDOWS_1250: &'static compat::EncodingWrap = &compat::WINDOWS_1250;

/// The windows-1251 encoding.
pub static WINDOWS_1251: &'static compat::EncodingWrap = &compat::WINDOWS_1251;

/// The windows-1252 encoding.
pub static WINDOWS_1252: &'static compat::EncodingWrap = &compat::WINDOWS_1252;

/// The windows-1253 encoding.
pub static WINDOWS_1253: &'static compat::EncodingWrap = &compat::WINDOWS_1253;

/// The windows-1254 encoding.
pub static WINDOWS_1254: &'static compat::EncodingWrap = &compat::WINDOWS_1254;

/// The windows-1255 encoding.
pub static WINDOWS_1255: &'static compat::EncodingWrap = &compat::WINDOWS_1255;

/// The windows-1256 encoding.
pub static WINDOWS_1256: &'static compat::EncodingWrap = &compat::WINDOWS_1256;

/// The windows-1257 encoding.
pub static WINDOWS_1257: &'static compat::EncodingWrap = &compat::WINDOWS_1257;

/// The windows-1258 encoding.
pub static WINDOWS_1258: &'static compat::EncodingWrap = &compat::WINDOWS_1258;

/// The windows-874 encoding.
pub static WINDOWS_874: &'static compat::EncodingWrap = &compat::WINDOWS_874;

/// The x-mac-cyrillic encoding.
pub static MAC_CYRILLIC: &'static compat::EncodingWrap = &compat::X_MAC_CYRILLIC;

// END GENERATED CODE
pub mod whatwg {
    use compat;
    pub static X_USER_DEFINED: &'static compat::EncodingWrap = &compat::X_USER_DEFINED;
    pub static ISO_8859_8_I: &'static compat::EncodingWrap = &compat::ISO_8859_8_I;
    pub static REPLACEMENT: &'static compat::EncodingWrap = &compat::REPLACEMENT;
}

static ENCODINGS: &'static [EncodingRef] = &[&ASCII_INIT,
                                             &compat::IBM866,
                                             // ISO_8859_1,
                                             &compat::ISO_8859_2,
                                             &compat::ISO_8859_3,
                                             &compat::ISO_8859_4,
                                             &compat::ISO_8859_5,
                                             &compat::ISO_8859_6,
                                             &compat::ISO_8859_7,
                                             &compat::ISO_8859_8,
                                             &compat::ISO_8859_10,
                                             &compat::ISO_8859_13,
                                             &compat::ISO_8859_14,
                                             &compat::ISO_8859_15,
                                             &compat::ISO_8859_16,
                                             &compat::KOI8_R,
                                             &compat::KOI8_U,
                                             &compat::MACINTOSH,
                                             &compat::WINDOWS_874,
                                             &compat::WINDOWS_1250,
                                             &compat::WINDOWS_1251,
                                             &compat::WINDOWS_1252,
                                             &compat::WINDOWS_1253,
                                             &compat::WINDOWS_1254,
                                             &compat::WINDOWS_1255,
                                             &compat::WINDOWS_1256,
                                             &compat::WINDOWS_1257,
                                             &compat::WINDOWS_1258,
                                             &compat::X_MAC_CYRILLIC,
                                             &compat::UTF_8,
                                             &compat::UTF_16LE,
                                             &compat::UTF_16BE,
                                             &compat::EUC_KR,
                                             &compat::EUC_JP,
                                             &compat::SHIFT_JIS,
                                             &compat::ISO_2022_JP,
                                             &compat::GBK,
                                             &compat::GB18030,
                                             &compat::BIG5,
                                             &compat::X_USER_DEFINED,
                                             &compat::ISO_8859_8_I,
                                             &compat::REPLACEMENT];

/// Returns a list of references to the encodings available.
pub fn encodings() -> &'static [EncodingRef] {
    ENCODINGS
}
