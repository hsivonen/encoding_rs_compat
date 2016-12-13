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

const DECODER_BUFFER_LENGTH: usize = 1024;

const ENCODER_BUFFER_LENGTH: usize = 1024;

fn encode_char(c: char, buffer: &mut [u8; 4]) -> &str {
    let u = c as u32;
    let len = if u <= 0x7F {
        buffer[0] = u as u8;
        1usize
    } else if u <= 0x07FF {
        buffer[0] = ((u >> 6) | 0xC0u32) as u8;
        buffer[1] = ((u & 0x3Fu32) | 0x80u32) as u8;
        2usize
    } else if u <= 0xFFFF {
        buffer[0] = ((u >> 12) | 0xE0u32) as u8;
        buffer[1] = (((u & 0xFC0u32) >> 6) | 0x80u32) as u8;
        buffer[2] = ((u & 0x3Fu32) | 0x80u32) as u8;
        3usize
    } else {
        buffer[0] = ((u >> 18) | 0xF0u32) as u8;
        buffer[1] = (((u & 0x3F000u32) >> 12) | 0x80u32) as u8;
        buffer[2] = (((u & 0xFC0u32) >> 6) | 0x80u32) as u8;
        buffer[3] = ((u & 0x3Fu32) | 0x80u32) as u8;
        4usize
    };
    let slice = &buffer[..len];
    unsafe { ::std::mem::transmute(slice) }
}

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

impl EncodingWrap {
    fn encode_to_vec(&self,
                     input: &str,
                     trap: EncoderTrap,
                     output: &mut Vec<u8>)
                     -> Result<(), Cow<'static, str>> {
        let mut unmappable_buffer = [0u8; 4];
        let mut raw_encoder = RawEncoderImpl::new(self.encoding);
        {
            let RawEncoderImpl(ref mut encoder) = raw_encoder;
            output.reserve(encoder.max_buffer_length_from_utf8_without_replacement(input.len()));
        }
        let mut total_read = 0usize;
        loop {
            let (result, read) =
                raw_encoder.encode_to_vec_without_replacement(&input[total_read..], output, true);
            total_read += read;
            match result {
                RawEncoderResult::Done => {
                    return Ok(());
                }
                RawEncoderResult::Unmappable(c) => {
                    if trap.trap(&mut raw_encoder,
                                 encode_char(c, &mut unmappable_buffer),
                                 output) {
                        continue;
                    } else {
                        return Err("unrepresentable character".into());
                    }
                }
            }
        }
    }

    fn encode_to_writer(&self,
                        input: &str,
                        trap: EncoderTrap,
                        output: &mut ByteWriter)
                        -> Result<(), Cow<'static, str>> {
        let mut unmappable_buffer = [0u8; 4];
        let mut raw_encoder = RawEncoderImpl::new(self.encoding);
        {
            let RawEncoderImpl(ref mut encoder) = raw_encoder;
            output.writer_hint(encoder.max_buffer_length_from_utf8_without_replacement(input.len()));
        }
        let mut total_read = 0usize;
        loop {
            let (result, read) =
                raw_encoder.encode_to_writer_without_replacement(&input[total_read..],
                                                                 output,
                                                                 true);
            total_read += read;
            match result {
                RawEncoderResult::Done => {
                    return Ok(());
                }
                RawEncoderResult::Unmappable(c) => {
                    if trap.trap(&mut raw_encoder,
                                 encode_char(c, &mut unmappable_buffer),
                                 output) {
                        continue;
                    } else {
                        return Err("unrepresentable character".into());
                    }
                }
            }
        }
    }

    fn decode_to_string(&self,
                        input: &[u8],
                        trap: DecoderTrap,
                        output: &mut String)
                        -> Result<(), Cow<'static, str>> {
        let mut raw_decoder = RawDecoderImpl::new(self.encoding);
        {
            let RawDecoderImpl(ref mut decoder) = raw_decoder;
            output.reserve(decoder.max_utf8_buffer_length_without_replacement(input.len()));
        }
        let mut total_read = 0usize;
        loop {
            let (result, read) =
                raw_decoder.decode_to_string_without_replacement(&input[total_read..],
                                                                 output,
                                                                 true);
            total_read += read;
            match result {
                RawDecoderResult::Done => {
                    return Ok(());
                }
                RawDecoderResult::Malformed(bad, good) => {
                    let end = total_read - (good as usize);
                    let start = end - (bad as usize);
                    if trap.trap(&mut raw_decoder, &input[start..end], output) {
                        continue;
                    } else {
                        return Err("unrepresentable character".into());
                    }
                }
            }
        }
    }

    fn decode_to_writer(&self,
                        input: &[u8],
                        trap: DecoderTrap,
                        output: &mut StringWriter)
                        -> Result<(), Cow<'static, str>> {
        let mut raw_decoder = RawDecoderImpl::new(self.encoding);
        {
            let RawDecoderImpl(ref mut decoder) = raw_decoder;
            output.writer_hint(decoder.max_utf8_buffer_length_without_replacement(input.len()));
        }
        let mut total_read = 0usize;
        loop {
            let (result, read) =
                raw_decoder.decode_to_writer_without_replacement(&input[total_read..],
                                                                 output,
                                                                 true);
            total_read += read;
            match result {
                RawDecoderResult::Done => {
                    return Ok(());
                }
                RawDecoderResult::Malformed(bad, good) => {
                    let end = total_read - (good as usize);
                    let start = end - (bad as usize);
                    if trap.trap(&mut raw_decoder, &input[start..end], output) {
                        continue;
                    } else {
                        return Err("unrepresentable character".into());
                    }
                }
            }
        }
    }

    fn panic_if_utf16(&self) {
        if self.encoding == encoding_rs::UTF_16LE || self.encoding == encoding_rs::UTF_16BE {
            panic!("Can't encode to UTF-16.");
        }
    }
}

impl types::Encoding for EncodingWrap {
    fn name(&self) -> &'static str {
        return self.name;
    }

    fn whatwg_name(&self) -> Option<&'static str> {
        return Some(self.whatwg_name);
    }

    fn raw_encoder(&self) -> Box<RawEncoder> {
        self.panic_if_utf16();
        Box::new(RawEncoderImpl::new(self.encoding))
    }

    fn raw_decoder(&self) -> Box<RawDecoder> {
        Box::new(RawDecoderImpl::new(self.encoding))
    }

    fn encode(&self, input: &str, trap: EncoderTrap) -> Result<Vec<u8>, Cow<'static, str>> {
        self.panic_if_utf16();
        if self.encoding.output_encoding() == encoding_rs::UTF_8 {
            let mut vec = Vec::with_capacity(input.len());
            vec.extend_from_slice(input.as_bytes());
            return Ok(vec);
        }
        match trap {
            EncoderTrap::NcrEscape => {
                let (out, _, _) = self.encoding.encode(input);
                return Ok(out.into_owned());
            }
            _ => {
                let mut out = Vec::new();
                return self.encode_to_vec(input, trap, &mut out).map(|_| out);
            }
        }
    }

    fn encode_to(&self,
                 input: &str,
                 trap: EncoderTrap,
                 output: &mut ByteWriter)
                 -> Result<(), Cow<'static, str>> {
        self.panic_if_utf16();
        if self.encoding.output_encoding() == encoding_rs::UTF_8 {
            output.write_bytes(input.as_bytes());
            return Ok(());
        }
        match output.as_mut_vec() {
            None => {}
            Some(vec) => return self.encode_to_vec(input, trap, vec),
        }
        self.encode_to_writer(input, trap, output)
    }

    fn decode(&self, input: &[u8], trap: DecoderTrap) -> Result<String, Cow<'static, str>> {
        match trap {
            DecoderTrap::Replace => {
                let (out, _) = self.encoding.decode_without_bom_handling(input);
                return Ok(out.into_owned());
            }
            _ => {
                let mut out = String::new();
                return self.decode_to_string(input, trap, &mut out).map(|_| out);
            }
        }
    }

    fn decode_to(&self,
                 input: &[u8],
                 trap: DecoderTrap,
                 output: &mut StringWriter)
                 -> Result<(), Cow<'static, str>> {
        match output.as_mut_string() {
            None => {}
            Some(string) => return self.decode_to_string(input, trap, string),
        }
        self.decode_to_writer(input, trap, output)
    }
}

/// Result of a (potentially partial) decode operation without replacement.
#[derive(Debug)]
enum RawDecoderResult {
    /// The input was exhausted.
    ///
    /// If this result was returned from a call where `last` was `true`, the
    /// decoding process has completed. Otherwise, the caller should call a
    /// decode method again with more input.
    Done,

    /// The decoder encountered a malformed byte sequence.
    ///
    /// The caller must either treat this as a fatal error or must append one
    /// REPLACEMENT CHARACTER (U+FFFD) to the output and then re-push the
    /// the remaining input to the decoder.
    ///
    /// The first wrapped integer indicates the length of the malformed byte
    /// sequence. The second wrapped integer indicates the number of bytes
    /// that were consumed after the malformed sequence. If the second
    /// integer is zero, the last byte that was consumed is the last byte of
    /// the malformed sequence. Note that the malformed bytes may have been part
    /// of an earlier input buffer.
    Malformed(u8, u8), // u8 instead of usize to avoid useless bloat
}

struct RawDecoderImpl(Decoder);

impl RawDecoderImpl {
    fn new(encoding: &'static encoding_rs::Encoding) -> RawDecoderImpl {
        RawDecoderImpl(encoding.new_decoder_without_bom_handling())
    }

    fn decode_to_writer_without_replacement(&mut self,
                                            src: &[u8],
                                            dst: &mut StringWriter,
                                            last: bool)
                                            -> (RawDecoderResult, usize) {
        let &mut RawDecoderImpl(ref mut decoder) = self;
        let mut buffer: [u8; DECODER_BUFFER_LENGTH] = unsafe { ::std::mem::uninitialized() };
        let mut total_read = 0usize;
        loop {
            let (result, read, written) =
                decoder.decode_to_utf8_without_replacement(&src[total_read..],
                                                           &mut buffer[..],
                                                           last);
            total_read += read;
            let as_str: &str = unsafe { ::std::mem::transmute(&buffer[..written]) };
            dst.write_str(as_str);
            match result {
                DecoderResult::InputEmpty => {
                    return (RawDecoderResult::Done, total_read);
                }
                DecoderResult::OutputFull => {
                    continue;
                }
                DecoderResult::Malformed(bad, good) => {
                    return (RawDecoderResult::Malformed(bad, good), total_read);
                }
            }
        }
    }

    fn decode_to_string_without_replacement(&mut self,
                                            src: &[u8],
                                            dst: &mut String,
                                            last: bool)
                                            -> (RawDecoderResult, usize) {
        let &mut RawDecoderImpl(ref mut decoder) = self;
        let needed = decoder.max_utf8_buffer_length_without_replacement(src.len());
        let available = dst.capacity() - dst.len();
        if needed > available {
            dst.reserve(needed);
        }
        let (result, read) = decoder.decode_to_string_without_replacement(src, dst, last);
        match result {
            DecoderResult::InputEmpty => {
                return (RawDecoderResult::Done, read);
            }
            DecoderResult::OutputFull => {
                unreachable!("Input buffer should have been long enough.");
            }
            DecoderResult::Malformed(bad, good) => {
                return (RawDecoderResult::Malformed(bad, good), read);
            }
        }
    }

    fn decode_without_replacement(&mut self,
                                  src: &[u8],
                                  dst: &mut StringWriter,
                                  last: bool)
                                  -> (RawDecoderResult, usize) {
        match dst.as_mut_string() {
            None => {}
            Some(string) => return self.decode_to_string_without_replacement(src, string, last),
        }
        self.decode_to_writer_without_replacement(src, dst, last)
    }
}

impl RawDecoder for RawDecoderImpl {
    fn from_self(&self) -> Box<RawDecoder> {
        let &RawDecoderImpl(ref decoder) = self;
        Box::new(RawDecoderImpl::new(decoder.encoding()))
    }

    fn is_ascii_compatible(&self) -> bool {
        let &RawDecoderImpl(ref decoder) = self;
        decoder.encoding().is_ascii_compatible()
    }

    fn raw_feed(&mut self, input: &[u8], output: &mut StringWriter) -> (usize, Option<CodecError>) {
        {
            let &mut RawDecoderImpl(ref mut decoder) = self;
            output.writer_hint(decoder.max_utf8_buffer_length_without_replacement(input.len()));
        }
        let (result, read) = self.decode_without_replacement(input, output, false);
        match result {
            RawDecoderResult::Done => {
                return (read, None);
            }
            RawDecoderResult::Malformed(_, _) => {
                // Report a zero-length sequence as being in error by
                // setting `upto` to `read`.
                return (read,
                        Some(CodecError {
                    upto: read as isize,
                    cause: "invalid sequence".into(),
                }));
            }
        }
    }

    fn raw_finish(&mut self, output: &mut StringWriter) -> Option<CodecError> {
        let (result, _) = self.decode_without_replacement(b"", output, false);
        match result {
            RawDecoderResult::Done => {
                return None;
            }
            RawDecoderResult::Malformed(_, _) => {
                // Report a zero-length sequence as being in error by
                // setting `upto` to 0.
                return Some(CodecError {
                    upto: 0isize,
                    cause: "invalid sequence".into(),
                });
            }
        }
    }
}

/// Result of a (potentially partial) encode operation without replacement.
#[derive(Debug)]
enum RawEncoderResult {
    /// The input was exhausted.
    ///
    /// If this result was returned from a call where `last` was `true`, the
    /// decoding process has completed. Otherwise, the caller should call a
    /// decode method again with more input.
    Done,

    /// The encoder encountered an unmappable character.
    ///
    /// The caller must either treat this as a fatal error or must append
    /// a placeholder to the output and then re-push the the remaining input to
    /// the encoder.
    Unmappable(char),
}

struct RawEncoderImpl(Encoder);

impl RawEncoderImpl {
    fn new(encoding: &'static encoding_rs::Encoding) -> RawEncoderImpl {
        RawEncoderImpl(encoding.new_encoder())
    }

    fn encode_to_writer_without_replacement(&mut self,
                                            src: &str,
                                            dst: &mut ByteWriter,
                                            last: bool)
                                            -> (RawEncoderResult, usize) {
        let &mut RawEncoderImpl(ref mut encoder) = self;
        let mut buffer: [u8; ENCODER_BUFFER_LENGTH] = unsafe { ::std::mem::uninitialized() };
        dst.writer_hint(encoder.max_buffer_length_from_utf8_without_replacement(src.len()));
        let mut total_read = 0usize;
        loop {
            let (result, read, written) =
                encoder.encode_from_utf8_without_replacement(&src[total_read..],
                                                             &mut buffer[..],
                                                             last);
            total_read += read;
            dst.write_bytes(&buffer[..written]);
            match result {
                EncoderResult::InputEmpty => {
                    return (RawEncoderResult::Done, total_read);
                }
                EncoderResult::OutputFull => {
                    continue;
                }
                EncoderResult::Unmappable(c) => {
                    return (RawEncoderResult::Unmappable(c), total_read);
                }
            }
        }
    }

    fn encode_to_vec_without_replacement(&mut self,
                                         src: &str,
                                         dst: &mut Vec<u8>,
                                         last: bool)
                                         -> (RawEncoderResult, usize) {
        let &mut RawEncoderImpl(ref mut encoder) = self;
        let needed = encoder.max_buffer_length_from_utf8_without_replacement(src.len());
        let available = dst.capacity() - dst.len();
        if needed > available {
            dst.reserve(needed);
        }
        let (result, read) = encoder.encode_from_utf8_to_vec_without_replacement(src, dst, last);
        match result {
            EncoderResult::InputEmpty => {
                return (RawEncoderResult::Done, read);
            }
            EncoderResult::OutputFull => {
                unreachable!("The output buffer should have been long enough.");
            }
            EncoderResult::Unmappable(c) => {
                return (RawEncoderResult::Unmappable(c), read);
            }
        }
    }

    fn encode_without_replacement(&mut self,
                                  src: &str,
                                  dst: &mut ByteWriter,
                                  last: bool)
                                  -> (RawEncoderResult, usize) {
        match dst.as_mut_vec() {
            None => {}
            Some(vec) => return self.encode_to_vec_without_replacement(src, vec, last),
        }
        self.encode_to_writer_without_replacement(src, dst, last)
    }
}

impl RawEncoder for RawEncoderImpl {
    fn from_self(&self) -> Box<RawEncoder> {
        let &RawEncoderImpl(ref encoder) = self;
        Box::new(RawEncoderImpl::new(encoder.encoding()))
    }

    fn is_ascii_compatible(&self) -> bool {
        let &RawEncoderImpl(ref encoder) = self;
        encoder.encoding().is_ascii_compatible()
    }

    fn raw_feed(&mut self, input: &str, output: &mut ByteWriter) -> (usize, Option<CodecError>) {
        {
            let &mut RawEncoderImpl(ref mut encoder) = self;
            output.writer_hint(encoder.max_buffer_length_from_utf8_without_replacement(input.len()));
        }
        let (result, read) = self.encode_without_replacement(input, output, false);
        match result {
            RawEncoderResult::Done => {
                return (read, None);
            }
            RawEncoderResult::Unmappable(_) => {
                // Move back until we find a UTF-8 sequence boundary.
                // Note: This is a spec violation when the ISO-2022-JP
                // encoder reports Basic Latin code points as unmappables
                // with U+FFFD. The `RawEncoder` cannot represent that
                // case in a spec-compliant manner.
                let bytes = input.as_bytes();
                let mut char_start = read - 1;
                while (bytes[char_start] & 0xC0) == 0x80 {
                    char_start -= 1;
                }
                return (char_start,
                        Some(CodecError {
                    upto: read as isize,
                    cause: "unrepresentable character".into(),
                }));
            }
        }
    }

    fn raw_finish(&mut self, output: &mut ByteWriter) -> Option<CodecError> {
        let (result, _) = self.encode_without_replacement("", output, false);
        match result {
            RawEncoderResult::Done => {
                return None;
            }
            RawEncoderResult::Unmappable(_) => {
                unreachable!("Encoders never report unmappables upon finish.");
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

/// All `EncodingWrap` objects in guestimated order of frequency of usage.
static WRAPS: [&'static EncodingWrap; 40] = [&UTF_8,
                                             &WINDOWS_1252,
                                             &GBK,
                                             &SHIFT_JIS,
                                             &BIG5,
                                             &EUC_KR,
                                             &EUC_JP,
                                             &GB18030,
                                             &WINDOWS_1250,
                                             &WINDOWS_1251,
                                             &WINDOWS_1253,
                                             &WINDOWS_1254,
                                             &WINDOWS_1255,
                                             &WINDOWS_1256,
                                             &WINDOWS_1257,
                                             &WINDOWS_1258,
                                             &WINDOWS_874,
                                             &ISO_8859_2,
                                             &ISO_8859_15,
                                             &IBM866,
                                             &KOI8_R,
                                             &KOI8_U,
                                             &ISO_8859_3,
                                             &ISO_8859_4,
                                             &ISO_8859_5,
                                             &ISO_8859_6,
                                             &ISO_8859_7,
                                             &ISO_8859_8,
                                             &X_MAC_CYRILLIC,
                                             &REPLACEMENT,
                                             &ISO_2022_JP,
                                             &ISO_8859_8_I,
                                             &X_USER_DEFINED,
                                             &UTF_16BE,
                                             &UTF_16LE,
                                             &MACINTOSH,
                                             &ISO_8859_10,
                                             &ISO_8859_13,
                                             &ISO_8859_14,
                                             &ISO_8859_16];

// BEGIN GENERATED CODE. PLEASE DO NOT EDIT.
// Instead, please regenerate using generate_constants.py

/// The Big5 encoding.
pub static BIG5: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::BIG5_INIT,
    whatwg_name: "big5",
    name: "big5-2003",
};

/// The EUC-JP encoding.
pub static EUC_JP: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::EUC_JP_INIT,
    whatwg_name: "euc-jp",
    name: "euc-jp",
};

/// The EUC-KR encoding.
pub static EUC_KR: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::EUC_KR_INIT,
    whatwg_name: "euc-kr",
    name: "windows-949",
};

/// The GBK encoding.
pub static GBK: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::GBK_INIT,
    whatwg_name: "gbk",
    name: "gbk",
};

/// The IBM866 encoding.
pub static IBM866: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::IBM866_INIT,
    whatwg_name: "ibm866",
    name: "ibm866",
};

/// The ISO-2022-JP encoding.
pub static ISO_2022_JP: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_2022_JP_INIT,
    whatwg_name: "iso-2022-jp",
    name: "iso-2022-jp",
};

/// The ISO-8859-10 encoding.
pub static ISO_8859_10: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_10_INIT,
    whatwg_name: "iso-8859-10",
    name: "iso-8859-10",
};

/// The ISO-8859-13 encoding.
pub static ISO_8859_13: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_13_INIT,
    whatwg_name: "iso-8859-13",
    name: "iso-8859-13",
};

/// The ISO-8859-14 encoding.
pub static ISO_8859_14: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_14_INIT,
    whatwg_name: "iso-8859-14",
    name: "iso-8859-14",
};

/// The ISO-8859-15 encoding.
pub static ISO_8859_15: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_15_INIT,
    whatwg_name: "iso-8859-15",
    name: "iso-8859-15",
};

/// The ISO-8859-16 encoding.
pub static ISO_8859_16: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_16_INIT,
    whatwg_name: "iso-8859-16",
    name: "iso-8859-16",
};

/// The ISO-8859-2 encoding.
pub static ISO_8859_2: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_2_INIT,
    whatwg_name: "iso-8859-2",
    name: "iso-8859-2",
};

/// The ISO-8859-3 encoding.
pub static ISO_8859_3: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_3_INIT,
    whatwg_name: "iso-8859-3",
    name: "iso-8859-3",
};

/// The ISO-8859-4 encoding.
pub static ISO_8859_4: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_4_INIT,
    whatwg_name: "iso-8859-4",
    name: "iso-8859-4",
};

/// The ISO-8859-5 encoding.
pub static ISO_8859_5: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_5_INIT,
    whatwg_name: "iso-8859-5",
    name: "iso-8859-5",
};

/// The ISO-8859-6 encoding.
pub static ISO_8859_6: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_6_INIT,
    whatwg_name: "iso-8859-6",
    name: "iso-8859-6",
};

/// The ISO-8859-7 encoding.
pub static ISO_8859_7: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_7_INIT,
    whatwg_name: "iso-8859-7",
    name: "iso-8859-7",
};

/// The ISO-8859-8 encoding.
pub static ISO_8859_8: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_8_INIT,
    whatwg_name: "iso-8859-8",
    name: "iso-8859-8",
};

/// The ISO-8859-8-I encoding.
pub static ISO_8859_8_I: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::ISO_8859_8_I_INIT,
    whatwg_name: "iso-8859-8-i",
    name: "iso-8859-8-i",
};

/// The KOI8-R encoding.
pub static KOI8_R: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::KOI8_R_INIT,
    whatwg_name: "koi8-r",
    name: "koi8-r",
};

/// The KOI8-U encoding.
pub static KOI8_U: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::KOI8_U_INIT,
    whatwg_name: "koi8-u",
    name: "koi8-u",
};

/// The Shift_JIS encoding.
pub static SHIFT_JIS: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::SHIFT_JIS_INIT,
    whatwg_name: "shift_jis",
    name: "windows-31j",
};

/// The UTF-16BE encoding.
pub static UTF_16BE: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::UTF_16BE_INIT,
    whatwg_name: "utf-16be",
    name: "utf-16be",
};

/// The UTF-16LE encoding.
pub static UTF_16LE: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::UTF_16LE_INIT,
    whatwg_name: "utf-16le",
    name: "utf-16le",
};

/// The UTF-8 encoding.
pub static UTF_8: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::UTF_8_INIT,
    whatwg_name: "utf-8",
    name: "utf-8",
};

/// The gb18030 encoding.
pub static GB18030: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::GB18030_INIT,
    whatwg_name: "gb18030",
    name: "gb18030",
};

/// The macintosh encoding.
pub static MACINTOSH: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::MACINTOSH_INIT,
    whatwg_name: "macintosh",
    name: "mac-roman",
};

/// The replacement encoding.
pub static REPLACEMENT: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::REPLACEMENT_INIT,
    whatwg_name: "replacement",
    name: "encoder-only-utf-8",
};

/// The windows-1250 encoding.
pub static WINDOWS_1250: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1250_INIT,
    whatwg_name: "windows-1250",
    name: "windows-1250",
};

/// The windows-1251 encoding.
pub static WINDOWS_1251: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1251_INIT,
    whatwg_name: "windows-1251",
    name: "windows-1251",
};

/// The windows-1252 encoding.
pub static WINDOWS_1252: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1252_INIT,
    whatwg_name: "windows-1252",
    name: "windows-1252",
};

/// The windows-1253 encoding.
pub static WINDOWS_1253: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1253_INIT,
    whatwg_name: "windows-1253",
    name: "windows-1253",
};

/// The windows-1254 encoding.
pub static WINDOWS_1254: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1254_INIT,
    whatwg_name: "windows-1254",
    name: "windows-1254",
};

/// The windows-1255 encoding.
pub static WINDOWS_1255: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1255_INIT,
    whatwg_name: "windows-1255",
    name: "windows-1255",
};

/// The windows-1256 encoding.
pub static WINDOWS_1256: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1256_INIT,
    whatwg_name: "windows-1256",
    name: "windows-1256",
};

/// The windows-1257 encoding.
pub static WINDOWS_1257: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1257_INIT,
    whatwg_name: "windows-1257",
    name: "windows-1257",
};

/// The windows-1258 encoding.
pub static WINDOWS_1258: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_1258_INIT,
    whatwg_name: "windows-1258",
    name: "windows-1258",
};

/// The windows-874 encoding.
pub static WINDOWS_874: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::WINDOWS_874_INIT,
    whatwg_name: "windows-874",
    name: "windows-874",
};

/// The x-mac-cyrillic encoding.
pub static X_MAC_CYRILLIC: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::X_MAC_CYRILLIC_INIT,
    whatwg_name: "x-mac-cyrillic",
    name: "mac-cyrillic",
};

/// The x-user-defined encoding.
pub static X_USER_DEFINED: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::X_USER_DEFINED_INIT,
    whatwg_name: "x-user-defined",
    name: "pua-mapped-binary",
};

// END GENERATED CODE
