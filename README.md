# Rust Core NLP

THIS PROJECT IS CURRENTLY IN DEVELOPMENT AND IS NOT YET READY FOR USE. EXPECTED RELEASE DATE NOVEMBER 2024

A Rust library for Natural Language Processing (NLP) tasks. It mainly focuses on tokenization, stemming, lemmatization, and part-of-speech tagging.

By default this library will only support the English language, but generic implementations will be added in the future.

## Features

All features are mainly for information extraction and text analysis. The following features are planned to be implemented in the future:

- Tokenization
- Stemming
- Lemmatization
- Part-of-Speech Tagging

### Status

⚠️ WARNING NO FEATURE IS YET TESTED!!!!

- Tokenization:
  - [x] Simple (Unicode) Tokenization
  - [x] Regex Tokenization
  - [x] Treebank Tokenization
  - [ ] Stanford Tokenization
- Stemming:
  - [ ] Porter Stemmer
  - [ ] Snowball Stemmer
  - [ ] Lancaster Stemmer
- Lemmatization:
  - [ ] WordNet Lemmatizer
- Basic Tagging:
  - [ ] Ngram Tagger
  - [ ] Affix Tagger
  - [ ] Regex Tagger
  - [ ] Classifier Tagger
- POS Tagging:
  - [ ] Brill Tagger
  - [ ] Classifier Tagger

  The implementation of this library is based on the NLTK book found (here)[https://www.nltk.org/book/]

## License

This project is licensed under the GNU Lesser General Public License v3.0. See the [Copying](COPYING)
and [Copying Lesser](COPYING.LESSER) files for details.
