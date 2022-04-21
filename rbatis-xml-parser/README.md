# rbatis_xml_parser

[![Build Status](https://travis-ci.org/Ygg01/rbatis_xml_parser.svg?branch=master)](https://travis-ci.org/Ygg01/rbatis_xml_parser) ![http://www.apache.org/licenses/LICENSE-2.0](https://img.shields.io/badge/license-Apache-blue.svg) ![https://opensource.org/licenses/MIT](https://img.shields.io/badge/license-MIT-blue.svg)
[![Clippy Linting Result](http://clippy.bashy.io/github/Ygg01/rbatis_xml_parser/master/badge.svg)](http://clippy.bashy.io/github/Ygg01/rbatis_xml_parser/master/log)
[![](http://meritbadge.herokuapp.com/rbatis_xml_parser)](https://crates.io/crates/rbatis_xml_parser)

[API documentation](https://Ygg01.github.io/docs/rbatis_xml_parser/rbatis_xml_parser/index.html)

**Warning:** This library is alpha quality, so no guarantees are given.

This crate provides a push based XML parser library that trades well-formedness for error recovery.

rbatis_xml_parser is based largely on [html5ever](https://github.com/servo/html5ever) parser, so if you have experience with html5ever you will be familiar with rbatis_xml_parser.

The library is dual licensed under MIT and Apache license.

#Why you should use rbatis_xml_parser

Main use case for this library is when XML is badly formatted, usually from bad XML
templates. XML5 tries to handle most common errors, in a manner similar to HTML5.

## When you should use it?

  - You aren't interested in well-formed documents.
  - You need to get some info from your data even if it has errors (although not all possible errors are handled).
  - You want to features like character references or xml namespaces.

## When you shouldn't use it

  - You need to have your document validated.
  - You require DTD support.
  - You require an easy to use parser, with lots of extensions (e.g. XPath, XQuery).
  - You require a battle tested, industry proven solution.

#Installation

Add rbatis_xml_parser as a dependency in your project manifest.

```toml
    [dependencies]
    rbatis_xml_parser = "0.1.3"
```

And add crate declaration in your lib.rs

```rust
    extern crate rbatis_xml_parser
```

#Getting started

Here is a very simple RcDom backed parser:

```rust

    let input = "<xml></xml>".to_tendril();

    // To parse XML into a tree form, we need a TreeSink
    // luckily rbatis_xml_parser comes with a static RC backed tree represetation.
    let dom: RcDom = parse(std::iter::once(input), Default::default());

    // Do something with dom

```
The thing that does actual parsing is the `parse` function. It expects an iterator that can be converted into `StrTendril`, so you can use `std::iter::once(input)` or  `Some(input).into_iter()` (where `input` is `StrTendril` like structure).
