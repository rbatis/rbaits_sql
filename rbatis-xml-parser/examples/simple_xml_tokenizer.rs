#!/usr/bin/env run-cargo-script
//! This is a regular crate doc comment, but it also contains a partial
//! Cargo manifest.  Note the use of a *fenced* code block, and the
//! `cargo` "language".
//!
//! ```cargo
//! [dependencies]
//! rbatis_xml_parser = "0.1.1"
//! tendril = "0.1.3"
//! ```
extern crate rbatis_xml_parser;

use std::io;
use std::default::Default;

use rbatis_xml_parser::tendril::{ByteTendril, ReadExt};
use rbatis_xml_parser::tokenizer::{TokenSink, Token, ParseError};
use rbatis_xml_parser::tokenizer::{CharacterTokens, NullCharacterToken, TagToken};
use rbatis_xml_parser::tokenizer::{PIToken, Pi, CommentToken};
use rbatis_xml_parser::tokenizer::{EOFToken, DoctypeToken, Doctype};
use rbatis_xml_parser::tokenize_to;

struct SimpleTokenPrinter;

impl TokenSink for SimpleTokenPrinter {
    fn process_token(&mut self, token: Token) {
        match token {
            CharacterTokens(b) => {
                println!("TEXT: {}", &*b);
            },
            NullCharacterToken => print!("NULL"),
            TagToken(tag) => {
                println!("{:?} {} ", tag.kind, &*tag.name.local);
            },
            ParseError(err) => {
                println!("ERROR: {}", err);
            },
            PIToken(Pi{ref target, ref data}) => {
                println!("PI : <?{} {}?>", &*target, &*data);
            },
            CommentToken(ref comment) => {
                println!("<!--{:?}-->", &*comment);
            },
            EOFToken => {
                println!("EOF");
            },
            DoctypeToken(Doctype{ref name, ref public_id, ..}) => {
                println!("<!DOCTYPE {:?} {:?}>", &*name, &*public_id);
            }
        }
    }
}

fn main() {
    // Our implementation of TokenSink
    let sink = SimpleTokenPrinter;

    // We need a ByteTendril to read a file
    let mut input = ByteTendril::new();
    // Using SliceExt.read_to_tendril we can read stdin
    io::stdin().read_to_tendril(&mut input).unwrap();
    // For rbatis_xml_parser we need StrTendril, so we reinterpret it
    // into StrTendril.
    let input = input.try_reinterpret().unwrap();
    // Here we execute tokenizer
    tokenize_to(sink, Some(input), Default::default());
}
