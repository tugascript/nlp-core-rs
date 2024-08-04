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

pub trait Tokenizer<'a> {
    fn new(text: &'a str) -> Self;
    fn tokenize(&'a self) -> Vec<&str>;
}

pub trait FullTokenizer<'a> {
    fn new(text: &'a str) -> Self;
    fn word_split(&'a self) -> Vec<&str>;
    fn sentence_split(&'a self) -> Vec<&str>;
    fn line_split(&'a self) -> Vec<&str>;
}

pub fn process_str(s: &str) -> Option<&str> {
    let trimmed = s.trim();
    if s.is_empty() {
        return None;
    }

    Some(trimmed)
}
