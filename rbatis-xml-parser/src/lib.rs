// Copyright 2015 The rbatis_xml_parser Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! This crate provides a push based XML parser library that
//! adheres to XML5 specification. In other words this library
//! trades well-formedness for error recovery.
//!
//! The idea behind this, was to minimize number of errors from
//! tools that generate XML (e.g. `&#83` won't just return `&#83`
//! as text, but will parse it into `S` ).
//! You can check out full specification [here](https://ygg01.github.io/xml5_draft/).
//!
//! What this library provides is a solid XML parser that can:
//!
//!   * Parse somewhat erroneous XML input
//!   * Provide support for [Numeric character references](https://en.wikipedia.org/wiki/Numeric_character_reference).
//!   * Provide partial [XML namespace](http://www.w3.org/TR/xml-names11/) support.
//!   * Provide full set of SVG/MathML entities
//!
//! What isn't in scope for this library:
//!
//!   * Document Type Definition parsing - this is pretty hard to do right and nowadays, its used
//!

#![crate_name="rbatis_xml_parser"]
#![crate_type="dylib"]
#![deny(missing_docs)]

#[macro_use] extern crate html5ever_atoms;
#[macro_use] extern crate log;
#[macro_use] extern crate mac;
extern crate phf;
extern crate time;

pub use html5ever_atoms::{Prefix, Namespace, LocalName, QualName};

/// Re-export the tendril crate so that users don’t need to depend on it.
pub mod tendril {
    extern crate tendril;
    pub use self::tendril::*;
}

macro_rules! time {
    ($e:expr) => {{
        let t0 = std::time::Instant::now();
        let result = $e;
        let dt = t0.elapsed().as_secs();
        (result, dt)
    }}
}

#[macro_use] mod util;

/// XML5 tokenizer - converts input into tokens
pub mod tokenizer;
/// XML5 tree builder - converts tokens into a tree like structure
pub mod tree_builder;
/// A simple reference-counted that serves as a default tree structure
pub mod rcdom;

use tokenizer::{XmlTokenizerOpts, XmlTokenizer, TokenSink};
use tree_builder::{TreeSink, XmlTreeBuilder};


/// Parse and send results to a `TreeSink`.
///
/// ## Example
///
/// ```ignore
/// let mut sink = MySink;
/// parse_to(&mut sink, iter::once(my_str), Default::default());
/// ```
pub fn parse_to<
        Sink:TreeSink,
        It: IntoIterator<Item=tendril::StrTendril>
    >(
        sink: Sink,
        input: It,
        opts: XmlTokenizerOpts) -> Sink {

    let tb = XmlTreeBuilder::new(sink);
    let mut tok = XmlTokenizer::new(tb, opts);
    for s in input {
        tok.feed(s);
    }
    tok.end();
    tok.unwrap().unwrap()
}


/// Parse into a type which implements `ParseResult`.
///
/// ## Example
///
/// ```ignore
/// let dom: RcDom = parse(iter::once(my_str), Default::default());
/// ```
pub fn parse<Output, It>(input: It, opts: XmlTokenizerOpts) -> Output
    where Output: ParseResult,
          It: IntoIterator<Item=tendril::StrTendril>,
{
    let sink = parse_to(Default::default(), input, opts);
    ParseResult::get_result(sink)
}

/// Results which can be extracted from a `TreeSink`.
///
/// Implement this for your parse tree data type so that it
/// can be returned by `parse()`.
pub trait ParseResult {
    /// Type of consumer of tree modifications.
    /// It also extends `Default` for convenience.
    type Sink: TreeSink + Default;
    /// Returns parsed tree data type
    fn get_result(sink: Self::Sink) -> Self;
}

/// Tokenize and send results to a `XTokenSink`.
///
/// ## Example
///
/// ```ignore
/// let mut sink = MySink;
/// tokenize_to(&mut sink, iter::once(my_str), Default::default());
/// ```
pub fn tokenize_to<
        Sink: TokenSink,
        It: IntoIterator<Item=tendril::StrTendril>
    >(
        sink: Sink,
        input: It,
        opts: XmlTokenizerOpts) -> Sink {

    let mut tok = XmlTokenizer::new(sink, opts);
    for s in input {
        tok.feed(s);
    }
    tok.end();
    tok.unwrap()
}
