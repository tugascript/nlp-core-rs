// Copyright (C) 2024 Afonso Barracha
//
// Rust NLP Core is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// Rust NLP is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with Rust NLP Core. If not, see <http://www.gnu.org/licenses/>.

use unicode_segmentation::UnicodeSegmentation;

use super::{process_str, FullTokenizer, Tokenizer};

pub struct SimpleTokenizer<'a> {
    text: &'a str,
}

impl<'a> FullTokenizer<'a> for SimpleTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self { text: text.trim() }
    }

    fn word_split(&'a self) -> Vec<&str> {
        self.text
            .split_word_bounds()
            .filter_map(|s| process_str(s))
            .collect()
    }

    fn sentence_split(&'a self) -> Vec<&str> {
        self.text
            .unicode_sentences()
            .filter_map(|s| process_str(s))
            .collect()
    }

    fn line_split(&'a self) -> Vec<&str> {
        self.text.lines().filter_map(|s| process_str(s)).collect()
    }
}

pub struct SimpleWordTokenizer<'a> {
    text: &'a str,
}

impl<'a> Tokenizer<'a> for SimpleWordTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self { text: text.trim() }
    }

    fn tokenize(&'a self) -> Vec<&str> {
        self.text
            .split_word_bounds()
            .filter_map(|s| process_str(s))
            .collect()
    }
}

pub struct SimpleSentenceTokenizer<'a> {
    text: &'a str,
}

impl<'a> Tokenizer<'a> for SimpleSentenceTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self { text: text.trim() }
    }

    fn tokenize(&'a self) -> Vec<&str> {
        self.text
            .unicode_sentences()
            .filter_map(|s| process_str(s))
            .collect()
    }
}

pub struct SimpleLineTokenizer<'a> {
    text: &'a str,
}

impl<'a> Tokenizer<'a> for SimpleLineTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self { text: text.trim() }
    }

    fn tokenize(&'a self) -> Vec<&str> {
        self.text.lines().filter_map(|s| process_str(s)).collect()
    }
}
