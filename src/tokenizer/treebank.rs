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

use std::borrow::Cow;

use once_cell::sync::Lazy;
use regex::Regex;

use super::StringTokenizer;

static REGEX_INITIAL_GROUP: Lazy<[(Regex, &str); 11]> = Lazy::new(|| {
    [
        // Starting Quotes
        (Regex::new(r#"^\""#).unwrap(), r"``"),
        (Regex::new(r"(``)").unwrap(), r" \1 "),
        (Regex::new(r#"([ \(\[{<])(\"|\'{2})"#).unwrap(), r"\1 `` "),
        // Punctuation
        (Regex::new(r"([:,])([^\d])").unwrap(), r" \1 \2"),
        (Regex::new(r"([:,])$").unwrap(), r" \1 "),
        (Regex::new(r"\.\.\.").unwrap(), r" ... "),
        (Regex::new(r"[;@#$%&]").unwrap(), r" \g<0> "),
        (
            Regex::new(r#"([^\.])(\.)([\]\)}>"\']*)\s*$"#).unwrap(),
            r"\1 \2\3 ",
        ),
        (Regex::new(r"[?!]").unwrap(), r" \g<0> "),
        (Regex::new(r"([^'])' ").unwrap(), r"\1 ' "),
        // Pads parentheses
        (Regex::new(r"[\]\[\(\)\{\}\<\>]").unwrap(), r" \g<0> "),
    ]
});

static REGEX_MID_GROUP: Lazy<[(Regex, &str); 7]> = Lazy::new(|| {
    [
        // Convert parentheses
        (Regex::new(r"\(").unwrap(), "-LRB-"),
        (Regex::new(r"\)").unwrap(), "-RRB-"),
        (Regex::new(r"\[").unwrap(), "-LSB-"),
        (Regex::new(r"\]").unwrap(), "-RSB-"),
        (Regex::new(r"\{").unwrap(), "-LCB-"),
        (Regex::new(r"\}").unwrap(), "-RCB-"),
        // Double dash
        (Regex::new(r"--").unwrap(), r" -- "),
    ]
});

static REGEX_FINAL_GROUP: Lazy<[(Regex, &str); 4]> = Lazy::new(|| {
    [
        // Ending quotes
        (Regex::new(r"''").unwrap(), " '' "),
        (Regex::new(r#"""#).unwrap(), " '' "),
        (
            Regex::new(r"([^' ])('[sS]|'[mM]|'[dD]|') ").unwrap(),
            r"\1 \2 ",
        ),
        (
            Regex::new(r"([^' ])('ll|'LL|'re|'RE|'ve|'VE|n't|N'T) ").unwrap(),
            r"\1 \2 ",
        ),
    ]
});

static REGEX_CONTRACTIONS: Lazy<[Regex; 10]> = Lazy::new(|| {
    [
        Regex::new(r"(?i)\b(can)(?#X)(not)\b").unwrap(),
        Regex::new(r"(?i)\b(d)(?#X)('ye)\b").unwrap(),
        Regex::new(r"(?i)\b(gim)(?#X)(me)\b").unwrap(),
        Regex::new(r"(?i)\b(gon)(?#X)(na)\b").unwrap(),
        Regex::new(r"(?i)\b(got)(?#X)(ta)\b").unwrap(),
        Regex::new(r"(?i)\b(lem)(?#X)(me)\b").unwrap(),
        Regex::new(r"(?i)\b(more)(?#X)('n)\b").unwrap(),
        Regex::new(r"(?i)\b(wan)(?#X)(na)(?=\s)").unwrap(),
        Regex::new(r"(?i) ('t)(?#X)(is)\b").unwrap(),
        Regex::new(r"(?i) ('t)(?#X)(was)\b").unwrap(),
    ]
});

const CONTRACTION_SUBSTITUTION: &str = r" \1 \2 ";

pub struct TreebankWordTokenizer<'a> {
    text: &'a str,
}

impl<'a> StringTokenizer<'a> for TreebankWordTokenizer<'a> {
    fn new(text: &'a str) -> Self {
        Self { text: text.trim() }
    }

    fn tokenize(&'a self) -> Vec<String> {
        let mut string_text = Cow::from(self.text);

        REGEX_INITIAL_GROUP.iter().for_each(|(rgx, sub)| {
            string_text = Cow::from(rgx.replace_all(&string_text, *sub).into_owned());
        });
        REGEX_MID_GROUP.iter().for_each(|(rgx, sub)| {
            string_text = Cow::from(rgx.replace_all(&string_text, *sub).into_owned());
        });

        let mut add_whitespace = String::from(" ");
        add_whitespace.push_str(&string_text);
        add_whitespace.push(' ');
        string_text = Cow::from(&add_whitespace);

        REGEX_FINAL_GROUP.iter().for_each(|(rgx, sub)| {
            string_text = Cow::from(rgx.replace_all(&string_text, *sub).into_owned());
        });
        REGEX_CONTRACTIONS.iter().for_each(|rgx| {
            string_text = Cow::from(
                rgx.replace_all(&string_text, CONTRACTION_SUBSTITUTION)
                    .into_owned(),
            );
        });

        string_text
            .split_whitespace()
            .into_iter()
            .map(|s| s.to_string())
            .collect()
    }
}
