// Copyright 2015 The rbatis_xml_parser Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

pub use self::XmlPhase::*;
pub use self::XmlProcessResult::*;
pub use self::Token::*;

use tendril::StrTendril;
use tokenizer::{Tag, Pi, Doctype};

#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub enum XmlPhase {
    StartPhase,
    MainPhase,
    EndPhase,
}

/// A subset/refinement of `tokenizer::XToken`.  Everything else is handled
/// specially at the beginning of `process_token`.
#[derive(PartialEq, Eq, Clone, Debug)]
pub enum Token {
    TagToken(Tag),
    DoctypeToken(Doctype),
    CommentToken(StrTendril),
    CharacterTokens(StrTendril),
    PIToken(Pi),
    NullCharacterToken,
    EOFToken,
}

pub enum XmlProcessResult {
    Done,
    Reprocess(XmlPhase, Token),
}
