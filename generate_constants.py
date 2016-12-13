#!/usr/bin/python

# Copyright 2013-2016 Mozilla Foundation. See the COPYRIGHT
# file at the top-level directory of this distribution.
#
# Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
# https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
# <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
# option. This file may not be copied, modified, or distributed
# except according to those terms.

import json
import subprocess

class Label:
  def __init__(self, label, preferred):
    self.label = label
    self.preferred = preferred
  def __cmp__(self, other):
    return cmp(self.label, other.label)

NAMES_TO_RUST_ENCODING = {
  "macintosh": "mac-roman",
  "x-mac-cyrillic": "mac-cyrillic",
  "x-user-defined": "pua-mapped-binary",
  "replacement": "encoder-only-utf-8",
  "Big5": "big5-2003",
  "EUC-KR": "windows-949",
  "Shift_JIS": "windows-31j",
}

RUST_ENCODING_CONSTANT_USES_RUST_ENCODING_NAME = [
  "macintosh",
  "x-mac-cyrillic",
  "Big5",
  "EUC-KR",
  "Shift_JIS",
]

ON_WHATWG_LIST_IN_RUST_ENCODING = [
  "ISO-8859-8-I",
  "replacement",
  "x-user-defined",
]

preferred = []

data = json.load(open("../encoding/encodings.json", "r"))

def to_camel_name(name):
  if name == u"iso-8859-8-i":
    return u"Iso8I"
  if name.startswith(u"iso-8859-"):
    return name.replace(u"iso-8859-", u"Iso")
  return name.title().replace(u"X-", u"").replace(u"-", u"").replace(u"_", u"")

def to_constant_name(name):
  return name.replace(u"-", u"_").upper()

def to_snake_name(name):
  return name.replace(u"-", u"_").lower()

# Legacy WHATWG name as used in rust-encoding
def to_whatwg_name(name):
  return name.lower()

# rust-encoding name
def to_rust_encoding_name(name):
  if NAMES_TO_RUST_ENCODING.has_key(name):
    return NAMES_TO_RUST_ENCODING[name]
  return name.lower()

# rust-encoding constant name
def to_rust_encoding_constant_name(name):
  if name in RUST_ENCODING_CONSTANT_USES_RUST_ENCODING_NAME:
    return to_constant_name(to_rust_encoding_name(name))
  return to_constant_name(name)

#

for group in data:
  for encoding in group["encodings"]:
    preferred.append(encoding["name"])

preferred.sort()

def read_non_generated(path):
  partially_generated_file = open(path, "r")
  full = partially_generated_file.read()
  partially_generated_file.close()

  generated_begin = "// BEGIN GENERATED CODE. PLEASE DO NOT EDIT."
  generated_end = "// END GENERATED CODE"

  generated_begin_index = full.find(generated_begin)
  if generated_begin_index < 0:
    print "Can't find generated code start marker in %s. Exiting." % path
    sys.exit(-1)
  generated_end_index = full.find(generated_end)
  if generated_end_index < 0:
    print "Can't find generated code end marker in %s. Exiting." % path
    sys.exit(-1)

  return (full[0:generated_begin_index + len(generated_begin)],
          full[generated_end_index:])

(compat_rs_begin, compat_rs_end) = read_non_generated("src/compat.rs")

compat_file = open("src/compat.rs", "w")

compat_file.write(compat_rs_begin)
compat_file.write("""
// Instead, please regenerate using generate_constants.py

""")

for name in preferred:
  compat_file.write('''/// The %s encoding.
pub static %s: EncodingWrap = EncodingWrap {
    encoding: &encoding_rs::%s_INIT,
    whatwg_name: "%s",
    name: "%s",
};

''' % (name, to_constant_name(name), to_constant_name(name), to_whatwg_name(name), to_rust_encoding_name(name)))

compat_file.write(compat_rs_end)
compat_file.close()

(all_rs_begin, all_rs_end) = read_non_generated("src/all.rs")

all_file = open("src/all.rs", "w")

all_file.write(all_rs_begin)
all_file.write("""
// Instead, please regenerate using generate_constants.py

""")

for name in preferred:
  if name in ON_WHATWG_LIST_IN_RUST_ENCODING:
    continue
  all_file.write('''/// The %s encoding.
pub static %s: &'static compat::EncodingWrap = &compat::%s;

''' % (name, to_rust_encoding_constant_name(name), to_constant_name(name)))

all_file.write(all_rs_end)
all_file.close()

subprocess.call(["cargo", "fmt"])

