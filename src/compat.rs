// Copyright 2016 Mozilla Foundation. See the COPYRIGHT
// file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE.txt or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::borrow::Cow;
extern crate encoding_rs;
use self::encoding_rs::Encoder;
use self::encoding_rs::Decoder;
use self::encoding_rs::EncoderResult;
use self::encoding_rs::DecoderResult;
use types;
use types::RawEncoder;
use types::RawDecoder;
use types::EncoderTrap;
use types::DecoderTrap;
use types::ByteWriter;
use types::StringWriter;
use types::CodecError;
use types::EncodingRef;
use all;

const DECODER_BUFFER_LENGTH: usize = 200;

const ENCODER_BUFFER_LENGTH: usize = 200;

pub struct EncodingWrap {
    /// The wrapped encoding_rs `Encoding`
    encoding: &'static encoding_rs::Encoding,
    /// The always-lowercase legacy WHATWG name. (Cannot be computed at
    /// run-time from the mixed-case `self.encoding.name()`, because this
    /// name needs to be returned with the `'static` lifetime.)
    whatwg_name: &'static str,
    /// The potentially non-WHATWG name used by rust-encoding.
    name: &'static str,
}

impl types::Encoding for EncodingWrap {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn whatwg_name(&self) -> Option<&'static str> {
        return Some(self.whatwg_name);
    }

    fn raw_encoder(&self) -> Box<RawEncoder> {
        Box::new(RawEncoderImpl::new(self.encoding))
    }

    fn raw_decoder(&self) -> Box<RawDecoder> {
        Box::new(RawDecoderImpl::new(self.encoding))
    }
}

struct RawDecoderImpl {
    decoder: Decoder,
    buffer: [u8; DECODER_BUFFER_LENGTH],
}

impl RawDecoderImpl {
    fn new(encoding: &'static encoding_rs::Encoding) -> RawDecoderImpl {
        RawDecoderImpl {
            decoder: encoding.new_decoder_without_bom_handling(),
            buffer: [0; DECODER_BUFFER_LENGTH],
        }
    }
}

impl RawDecoder for RawDecoderImpl {
    fn from_self(&self) -> Box<RawDecoder> {
        Box::new(RawDecoderImpl::new(self.decoder.encoding()))
    }

    fn is_ascii_compatible(&self) -> bool {
        self.decoder.encoding().is_ascii_compatible()
    }

    fn raw_feed(&mut self, input: &[u8], output: &mut StringWriter) -> (usize, Option<CodecError>) {
        output.writer_hint(self.decoder.max_utf8_buffer_length_without_replacement(input.len()));
        let mut total_read = 0usize;
        loop {
            let (result, read, written) = self.decoder
                .decode_to_utf8_without_replacement(&input[total_read..],
                                                    &mut self.buffer[..],
                                                    false);
            total_read += read;
            let as_str: &str = unsafe { ::std::mem::transmute(&self.buffer[..written]) };
            output.write_str(as_str);
            match result {
                DecoderResult::InputEmpty => {
                    return (total_read, None);
                }
                DecoderResult::OutputFull => {
                    continue;
                }
                DecoderResult::Malformed(_, _) => {
                    // TODO: Figure out the exact semantics of `upto`.
                    return (total_read,
                            Some(CodecError {
                        upto: total_read as isize,
                        cause: "invalid sequence".into(),
                    }));
                }
            }
        }
    }

    fn raw_finish(&mut self, output: &mut StringWriter) -> Option<CodecError> {
        let (result, read, written) = self.decoder
            .decode_to_utf8_without_replacement(b"", &mut self.buffer[..], true);
        let as_str: &str = unsafe { ::std::mem::transmute(&self.buffer[..written]) };
        output.write_str(as_str);
        match result {
            DecoderResult::InputEmpty => {
                return None;
            }
            DecoderResult::OutputFull => {
                unreachable!("No way buffer could get filled from empty input.");
            }
            DecoderResult::Malformed(_, _) => {
                // TODO: Figure out the exact semantics of `upto`.
                return Some(CodecError {
                    upto: read as isize,
                    cause: "invalid sequence".into(),
                });
            }
        }
    }
}

struct RawEncoderImpl {
    encoder: Encoder,
    buffer: [u8; ENCODER_BUFFER_LENGTH],
}

impl RawEncoderImpl {
    fn new(encoding: &'static encoding_rs::Encoding) -> RawEncoderImpl {
        RawEncoderImpl {
            encoder: encoding.new_encoder(),
            buffer: [0; ENCODER_BUFFER_LENGTH],
        }
    }
}

impl RawEncoder for RawEncoderImpl {
    fn from_self(&self) -> Box<RawEncoder> {
        Box::new(RawEncoderImpl::new(self.encoder.encoding()))
    }

    fn is_ascii_compatible(&self) -> bool {
        self.encoder.encoding().is_ascii_compatible()
    }

    fn raw_feed(&mut self, input: &str, output: &mut ByteWriter) -> (usize, Option<CodecError>) {
        output.writer_hint(self.encoder.max_buffer_length_from_utf8_without_replacement(input.len()));
        let mut total_read = 0usize;
        loop {
            let (result, read, written) = self.encoder
                .encode_from_utf8_without_replacement(&input[total_read..],
                                                      &mut self.buffer[..],
                                                      false);
            total_read += read;
            output.write_bytes(&self.buffer[..written]);
            match result {
                EncoderResult::InputEmpty => {
                    return (total_read, None);
                }
                EncoderResult::OutputFull => {
                    continue;
                }
                EncoderResult::Unmappable(_) => {
                    // Move back until we find a UTF-8 sequence boundary.
                    // Note: This is a spec violation when the ISO-2022-JP
                    // encoder reports Basic Latin code points as unmappables
                    // with U+FFFD. The `RawEncoder` cannot represent that
                    // case in a spec-compliant manner.
                    let bytes = input.as_bytes();
                    let mut char_start = total_read - 1;
                    while (bytes[char_start] & 0xC0) == 0x80 {
                        char_start -= 1;
                    }
                    return (char_start,
                            Some(CodecError {
                        upto: total_read as isize,
                        cause: "unrepresentable character".into(),
                    }));
                }
            }
        }
    }

    fn raw_finish(&mut self, output: &mut ByteWriter) -> Option<CodecError> {
        let (result, read, written) = self.encoder
            .encode_from_utf8_without_replacement("", &mut self.buffer[..], false);
        output.write_bytes(&self.buffer[..written]);
        match result {
            EncoderResult::InputEmpty => {
                return None;
            }
            EncoderResult::OutputFull => {
                unreachable!("No way buffer could get filled from empty input.");
            }
            EncoderResult::Unmappable(_) => {
                // TODO: Figure out the exact semantics of `upto`.
                return Some(CodecError {
                    upto: read as isize,
                    cause: "unrepresentable character".into(),
                });
            }
        }
    }
}

pub fn from_encoding_rs(encoding: &'static encoding_rs::Encoding) -> EncodingRef {
    let mut it = WRAPS.iter();
    loop {
        match it.next() {
            None => unreachable!("How can an unlisted &'static encoding_rs::Encoding exist?"),
            Some(wrap) => {
                if wrap.encoding == encoding {
                    // Need this intermediate binding to keep the compiler
                    // happy.
                    let enc: &'static EncodingWrap = wrap;
                    return enc;
                }
            }
        }
    }
}

pub fn to_encoding_rs(encoding: EncodingRef) -> Option<&'static encoding_rs::Encoding> {
    let mut it = WRAPS.iter();
    loop {
        match it.next() {
            None => {
                return None;
            }
            Some(wrap) => {
                let enc: &'static EncodingWrap = wrap;
                let enc_ref: EncodingRef = enc;
                if (enc_ref as *const types::Encoding) == (encoding as *const types::Encoding) {
                    return Some(wrap.encoding);
                }
            }
        }
    }
}

pub fn encoding_rs_for_label(label: &str) -> Option<EncodingRef> {
    let enc = encoding_rs::Encoding::for_label(label.as_bytes());
    match enc {
        None => None,
        Some(encoding) => Some(from_encoding_rs(encoding)),
    }
}

/// All `EncodingWrap` objects in guestimatic order of frequency of usage.
static WRAPS: [&'static EncodingWrap; 40] = [UTF_8,
                                             WINDOWS_1252,
                                             GBK,
                                             SHIFT_JIS,
                                             BIG5,
                                             EUC_KR,
                                             EUC_JP,
                                             GB18030,
                                             WINDOWS_1250,
                                             WINDOWS_1251,
                                             WINDOWS_1253,
                                             WINDOWS_1254,
                                             WINDOWS_1255,
                                             WINDOWS_1256,
                                             WINDOWS_1257,
                                             WINDOWS_1258,
                                             WINDOWS_874,
                                             ISO_8859_2,
                                             ISO_8859_15,
                                             IBM866,
                                             KOI8_R,
                                             KOI8_U,
                                             ISO_8859_3,
                                             ISO_8859_4,
                                             ISO_8859_5,
                                             ISO_8859_6,
                                             ISO_8859_7,
                                             ISO_8859_8,
                                             X_MAC_CYRILLIC,
                                             REPLACEMENT,
                                             ISO_2022_JP,
                                             ISO_8859_8_I,
                                             X_USER_DEFINED,
                                             UTF_16BE,
                                             UTF_16LE,
                                             MACINTOSH,
                                             ISO_8859_10,
                                             ISO_8859_13,
                                             ISO_8859_14,
                                             ISO_8859_16];

// BEGIN GENERATED CODE. PLEASE DO NOT EDIT.
// Instead, please regenerate using generate_constants.py

/// The Big5 encoding.
pub const BIG5: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::BIG5,
    whatwg_name: "big5",
    name: "big5-2003",
};

/// The EUC-JP encoding.
pub const EUC_JP: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::EUC_JP,
    whatwg_name: "euc-jp",
    name: "euc-jp",
};

/// The EUC-KR encoding.
pub const EUC_KR: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::EUC_KR,
    whatwg_name: "euc-kr",
    name: "windows-949",
};

/// The GBK encoding.
pub const GBK: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::GBK,
    whatwg_name: "gbk",
    name: "gbk",
};

/// The IBM866 encoding.
pub const IBM866: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::IBM866,
    whatwg_name: "ibm866",
    name: "ibm866",
};

/// The ISO-2022-JP encoding.
pub const ISO_2022_JP: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_2022_JP,
    whatwg_name: "iso-2022-jp",
    name: "iso-2022-jp",
};

/// The ISO-8859-10 encoding.
pub const ISO_8859_10: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_10,
    whatwg_name: "iso-8859-10",
    name: "iso-8859-10",
};

/// The ISO-8859-13 encoding.
pub const ISO_8859_13: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_13,
    whatwg_name: "iso-8859-13",
    name: "iso-8859-13",
};

/// The ISO-8859-14 encoding.
pub const ISO_8859_14: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_14,
    whatwg_name: "iso-8859-14",
    name: "iso-8859-14",
};

/// The ISO-8859-15 encoding.
pub const ISO_8859_15: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_15,
    whatwg_name: "iso-8859-15",
    name: "iso-8859-15",
};

/// The ISO-8859-16 encoding.
pub const ISO_8859_16: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_16,
    whatwg_name: "iso-8859-16",
    name: "iso-8859-16",
};

/// The ISO-8859-2 encoding.
pub const ISO_8859_2: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_2,
    whatwg_name: "iso-8859-2",
    name: "iso-8859-2",
};

/// The ISO-8859-3 encoding.
pub const ISO_8859_3: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_3,
    whatwg_name: "iso-8859-3",
    name: "iso-8859-3",
};

/// The ISO-8859-4 encoding.
pub const ISO_8859_4: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_4,
    whatwg_name: "iso-8859-4",
    name: "iso-8859-4",
};

/// The ISO-8859-5 encoding.
pub const ISO_8859_5: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_5,
    whatwg_name: "iso-8859-5",
    name: "iso-8859-5",
};

/// The ISO-8859-6 encoding.
pub const ISO_8859_6: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_6,
    whatwg_name: "iso-8859-6",
    name: "iso-8859-6",
};

/// The ISO-8859-7 encoding.
pub const ISO_8859_7: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_7,
    whatwg_name: "iso-8859-7",
    name: "iso-8859-7",
};

/// The ISO-8859-8 encoding.
pub const ISO_8859_8: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_8,
    whatwg_name: "iso-8859-8",
    name: "iso-8859-8",
};

/// The ISO-8859-8-I encoding.
pub const ISO_8859_8_I: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::ISO_8859_8_I,
    whatwg_name: "iso-8859-8-i",
    name: "iso-8859-8-i",
};

/// The KOI8-R encoding.
pub const KOI8_R: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::KOI8_R,
    whatwg_name: "koi8-r",
    name: "koi8-r",
};

/// The KOI8-U encoding.
pub const KOI8_U: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::KOI8_U,
    whatwg_name: "koi8-u",
    name: "koi8-u",
};

/// The Shift_JIS encoding.
pub const SHIFT_JIS: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::SHIFT_JIS,
    whatwg_name: "shift_jis",
    name: "windows-31j",
};

/// The UTF-16BE encoding.
pub const UTF_16BE: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::UTF_16BE,
    whatwg_name: "utf-16be",
    name: "utf-16be",
};

/// The UTF-16LE encoding.
pub const UTF_16LE: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::UTF_16LE,
    whatwg_name: "utf-16le",
    name: "utf-16le",
};

/// The UTF-8 encoding.
pub const UTF_8: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::UTF_8,
    whatwg_name: "utf-8",
    name: "utf-8",
};

/// The gb18030 encoding.
pub const GB18030: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::GB18030,
    whatwg_name: "gb18030",
    name: "gb18030",
};

/// The macintosh encoding.
pub const MACINTOSH: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::MACINTOSH,
    whatwg_name: "macintosh",
    name: "mac-roman",
};

/// The replacement encoding.
pub const REPLACEMENT: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::REPLACEMENT,
    whatwg_name: "replacement",
    name: "encoder-only-utf-8",
};

/// The windows-1250 encoding.
pub const WINDOWS_1250: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1250,
    whatwg_name: "windows-1250",
    name: "windows-1250",
};

/// The windows-1251 encoding.
pub const WINDOWS_1251: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1251,
    whatwg_name: "windows-1251",
    name: "windows-1251",
};

/// The windows-1252 encoding.
pub const WINDOWS_1252: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1252,
    whatwg_name: "windows-1252",
    name: "windows-1252",
};

/// The windows-1253 encoding.
pub const WINDOWS_1253: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1253,
    whatwg_name: "windows-1253",
    name: "windows-1253",
};

/// The windows-1254 encoding.
pub const WINDOWS_1254: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1254,
    whatwg_name: "windows-1254",
    name: "windows-1254",
};

/// The windows-1255 encoding.
pub const WINDOWS_1255: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1255,
    whatwg_name: "windows-1255",
    name: "windows-1255",
};

/// The windows-1256 encoding.
pub const WINDOWS_1256: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1256,
    whatwg_name: "windows-1256",
    name: "windows-1256",
};

/// The windows-1257 encoding.
pub const WINDOWS_1257: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1257,
    whatwg_name: "windows-1257",
    name: "windows-1257",
};

/// The windows-1258 encoding.
pub const WINDOWS_1258: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_1258,
    whatwg_name: "windows-1258",
    name: "windows-1258",
};

/// The windows-874 encoding.
pub const WINDOWS_874: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::WINDOWS_874,
    whatwg_name: "windows-874",
    name: "windows-874",
};

/// The x-mac-cyrillic encoding.
pub const X_MAC_CYRILLIC: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::X_MAC_CYRILLIC,
    whatwg_name: "x-mac-cyrillic",
    name: "mac-cyrillic",
};

/// The x-user-defined encoding.
pub const X_USER_DEFINED: &'static EncodingWrap = &EncodingWrap {
    encoding: encoding_rs::X_USER_DEFINED,
    whatwg_name: "x-user-defined",
    name: "pua-mapped-binary",
};

// END GENERATED CODE
