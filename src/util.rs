// This is a part of rust-encoding.
// Copyright (c) 2013-2015, Kang Seonghoon.
// See README.md and LICENSE.txt for details.

//! Internal utilities.

use std::{str, char};

/// External iterator for a string's characters with its corresponding byte offset range.
pub struct StrCharIndexIterator<'r> {
    index: usize,
    chars: str::Chars<'r>,
}

impl<'r> Iterator for StrCharIndexIterator<'r> {
    type Item = ((usize, usize), char);

    #[inline]
    fn next(&mut self) -> Option<((usize, usize), char)> {
        if let Some(ch) = self.chars.next() {
            let prev = self.index;
            let next = prev + ch.len_utf8();
            self.index = next;
            Some(((prev, next), ch))
        } else {
            None
        }
    }
}

/// A trait providing an `index_iter` method.
pub trait StrCharIndex<'r> {
    fn index_iter(&self) -> StrCharIndexIterator<'r>;
}

impl<'r> StrCharIndex<'r> for &'r str {
    /// Iterates over each character with corresponding byte offset range.
    fn index_iter(&self) -> StrCharIndexIterator<'r> {
        StrCharIndexIterator {
            index: 0,
            chars: self.chars(),
        }
    }
}
