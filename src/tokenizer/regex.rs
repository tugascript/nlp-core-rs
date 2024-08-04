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

use regex::{Regex, RegexBuilder};

use super::{process_str, Tokenizer};

fn process_regex_split_tokenizer<'a>(text: &'a str, rgx: &'a Regex) -> Vec<&'a str> {
    rgx.split(text).filter_map(|s| process_str(s)).collect()
}

fn process_regex_match_tokenizer<'a>(text: &'a str, rgx: &'a Regex) -> Vec<&'a str> {
    rgx.find_iter(text)
        .filter_map(|s| process_str(s.as_str()))
        .collect()
}

fn build_regex(pattern: &str) -> Regex {
    RegexBuilder::new(pattern)
        .unicode(true)
        .multi_line(true)
        .build()
        .unwrap()
}

pub enum RegexType<'a> {
    SplitRegex(Regex),
    MatchRegex(Regex),
    SplitRegexPattern(&'a str),
    MatchRegexPattern(&'a str),
}

pub trait RegexTokenizer<'a> {
    fn new(text: &'a str, regex_type: RegexType<'a>) -> Self;
    fn tokenize(&'a self) -> Vec<&str>;
}

pub struct CustomRegexTokenizer<'a> {
    regex: Regex,
    split: bool,
    text: &'a str,
}

impl<'a> RegexTokenizer<'a> for CustomRegexTokenizer<'a> {
    fn new(text: &'a str, regex_type: RegexType<'a>) -> Self {
        let trimmed = text.trim();
        match regex_type {
            RegexType::SplitRegex(regex) => Self {
                regex,
                split: true,
                text: trimmed,
            },
            RegexType::MatchRegex(regex) => Self {
                regex,
                split: false,
                text: trimmed,
            },
            RegexType::SplitRegexPattern(pattern) => Self {
                regex: build_regex(pattern),
                split: true,
                text: trimmed,
            },
            RegexType::MatchRegexPattern(pattern) => Self {
                regex: build_regex(pattern),
                split: false,
                text: trimmed,
            },
        }
    }

    fn tokenize(&'a self) -> Vec<&str> {
        if self.split {
            return process_regex_split_tokenizer(self.text, &self.regex);
        }

        process_regex_match_tokenizer(self.text, &self.regex)
    }
}

const WHITESPACE_PATTERN: &str = r"\s+";

pub struct WhitespaceRegexTokenizer<'a> {
    regex: Regex,
    text: &'a str,
}

impl<'a> Tokenizer<'a> for WhitespaceRegexTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            regex: build_regex(WHITESPACE_PATTERN),
            text: text.trim(),
        }
    }

    fn tokenize(&'a self) -> Vec<&str> {
        process_regex_split_tokenizer(self.text, &self.regex)
    }
}

const BLANKLINE_PATTERN: &str = r"\s*\n\s*\n\s*";

pub struct BlanklineRegexTokenizer<'a> {
    regex: Regex,
    text: &'a str,
}

impl<'a> Tokenizer<'a> for BlanklineRegexTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            regex: build_regex(BLANKLINE_PATTERN),
            text: text.trim(),
        }
    }

    fn tokenize(&'a self) -> Vec<&str> {
        process_regex_split_tokenizer(self.text, &self.regex)
    }
}

const WORD_PUNCTUATION_PATTERN: &str = r"\w+|[^\w\s]+";

pub struct WordPunctuationRegexTokenizer<'a> {
    regex: Regex,
    text: &'a str,
}

impl<'a> Tokenizer<'a> for WordPunctuationRegexTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self {
            regex: build_regex(WORD_PUNCTUATION_PATTERN),
            text: text.trim(),
        }
    }

    fn tokenize(&'a self) -> Vec<&str> {
        process_regex_match_tokenizer(&self.text, &self.regex)
    }
}
