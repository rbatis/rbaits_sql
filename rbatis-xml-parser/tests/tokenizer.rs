#![cfg_attr(feature = "unstable", feature(start, test))]

extern crate rustc_serialize;
#[cfg(feature = "unstable")] extern crate test;

extern crate rbatis_xml_parser;

use std::borrow::Cow::Borrowed;
use std::env;
use std::ffi::OsStr;
use std::mem::replace;
use std::path::Path;
use std::collections::BTreeMap;
use rustc_serialize::json::Json;


#[cfg(feature = "unstable")] use test::{TestDesc, TestDescAndFn, DynTestName, DynTestFn};
#[cfg(feature = "unstable")] use test::ShouldPanic::No;
use util::find_tests::foreach_xml5lib_test;

use rbatis_xml_parser::LocalName;
use rbatis_xml_parser::tendril::{StrTendril, SliceExt};
use rbatis_xml_parser::tokenizer::{Attribute, QName};
use rbatis_xml_parser::tokenizer::{Tag, StartTag, EndTag, CommentToken, EmptyTag, ShortTag};
use rbatis_xml_parser::tokenizer::{Token, CharacterTokens, TokenSink};
use rbatis_xml_parser::tokenizer::{NullCharacterToken, ParseError, TagToken};
use rbatis_xml_parser::tokenizer::{PIToken, Pi, DoctypeToken, Doctype};
use rbatis_xml_parser::tokenizer::{EOFToken, XmlTokenizer, XmlTokenizerOpts};

mod util {
    pub mod find_tests;
}

// Return all ways of splitting the string into at most n
// possibly-empty pieces.
fn splits(s: &str, n: usize) -> Vec<Vec<StrTendril>> {
    if n == 1 {
        return vec!(vec!(s.to_tendril()));
    }

    let mut points: Vec<usize> = s.char_indices().map(|(n,_)| n).collect();
    points.push(s.len());

    // do this with iterators?
    let mut out = vec!();
    for p in points.into_iter() {
        let y = &s[p..];
        for mut x in splits(&s[..p], n-1).into_iter() {
            x.push(y.to_tendril());
            out.push(x);
        }
    }

    out.extend(splits(s, n-1).into_iter());
    out
}

struct TokenLogger {
    tokens: Vec<Token>,
    current_str: StrTendril,
    exact_errors: bool,
}


impl TokenLogger {
    fn new(exact_errors: bool) -> TokenLogger {
        TokenLogger {
            tokens: vec!(),
            current_str: StrTendril::new(),
            exact_errors: exact_errors,
        }
    }

    // Push anything other than character tokens
    fn push(&mut self, token: Token) {
        self.finish_str();
        self.tokens.push(token);
    }

    fn finish_str(&mut self) {
        if self.current_str.len() > 0 {
            let s = replace(&mut self.current_str, StrTendril::new());
            self.tokens.push(CharacterTokens(s));
        }
    }

    fn get_tokens(mut self) -> Vec<Token> {
        self.finish_str();
        self.tokens
    }
}

impl TokenSink for TokenLogger {
    fn process_token(&mut self, token: Token) {
        match token {
            CharacterTokens(b) => {
                self.current_str.push_slice(&b);
            }

            NullCharacterToken => {
                self.current_str.push_char('\0');
            }

            ParseError(_) => if self.exact_errors {
                self.push(ParseError(Borrowed("")));
            },

            TagToken(mut t) => {
                // The spec seems to indicate that one can emit
                // erroneous end tags with attrs, but the test
                // cases don't contain them.
                match t.kind {
                    EndTag => {
                        t.attrs = vec!();
                    }
                    _ => t.attrs.sort_by(|a1, a2| a1.name.cmp(&a2.name)),
                }
                self.push(TagToken(t));
            }

            EOFToken => (),

            _ => self.push(token),
        }
    }
}

fn tokenize_xml(input: Vec<StrTendril>, opts: XmlTokenizerOpts) -> Vec<Token> {
    let sink = TokenLogger::new(opts.exact_errors);
    let mut tok = XmlTokenizer::new(sink, opts);
    for chunk in input.into_iter() {
        tok.feed(chunk);
    }
    tok.end();
    tok.unwrap().get_tokens()
}

trait JsonExt: Sized {
    fn get_str(&self) -> String;
    fn get_tendril(&self) -> StrTendril;
    fn get_nullable_tendril(&self) -> Option<StrTendril>;
    fn get_bool(&self) -> bool;
    fn get_obj<'t>(&'t self) -> &'t BTreeMap<String, Self>;
    fn get_list<'t>(&'t self) -> &'t Vec<Self>;
    fn find<'t>(&'t self, key: &str) -> &'t Self;
}

impl JsonExt for Json {
    fn get_str(&self) -> String {
        match *self {
            Json::String(ref s) => s.to_string(),
            _ => panic!("Json::get_str: not a String"),
        }
    }

    fn get_tendril(&self) -> StrTendril {
        match *self {
            Json::String(ref s) => s.to_tendril(),
            _ => panic!("Json::get_tendril: not a String"),
        }
    }

    fn get_nullable_tendril(&self) -> Option<StrTendril> {
        match *self {
            Json::Null => None,
            Json::String(ref s) => Some(s.to_tendril()),
            _ => panic!("Json::get_nullable_tendril: not a String"),
        }
    }

    fn get_bool(&self) -> bool {
        match *self {
            Json::Boolean(b) => b,
            _ => panic!("Json::get_bool: not a Boolean"),
        }
    }

    fn get_obj<'t>(&'t self) -> &'t BTreeMap<String, Json> {
        match *self {
            Json::Object(ref m) => &*m,
            _ => panic!("Json::get_obj: not an Object"),
        }
    }

    fn get_list<'t>(&'t self) -> &'t Vec<Json> {
        match *self {
            Json::Array(ref m) => m,
            _ => panic!("Json::get_list: not an Array"),
        }
    }

    fn find<'t>(&'t self, key: &str) -> &'t Json {
        self.get_obj().get(&key.to_string()).unwrap()
    }
}

// Parse a JSON object (other than "ParseError") to a token.
fn json_to_token(js: &Json) -> Token {
    let parts = js.as_array().unwrap();
    // Collect refs here so we don't have to use "ref" in all the patterns below.
    let args: Vec<&Json> = parts[1..].iter().collect();
    match &*parts[0].get_str() {

        "StartTag" => TagToken(Tag {
            kind: StartTag,
            name: QName::new_empty(LocalName::from(args[0].get_str())),
            attrs: args[1].get_obj().iter().map(|(k,v)| {
                Attribute {
                    name: QName::new_empty(LocalName::from(&**k)),
                    value: v.get_tendril()
                }
            }).collect(),
        }),

        "EndTag" => TagToken(Tag {
            kind: EndTag,
            name: QName::new_empty(LocalName::from(args[0].get_str())),
            attrs: vec!(),
        }),

        "ShortTag" => TagToken(Tag {
            kind: ShortTag,
            name: QName::new_empty(LocalName::from(args[0].get_str())),
            attrs: vec!(),
        }),

        "EmptyTag" => TagToken(Tag {
            kind: EmptyTag,
            name: QName::new_empty(LocalName::from(args[0].get_str())),
            attrs: args[1].get_obj().iter().map(|(k,v)| {
                Attribute {
                    name: QName::new_empty(LocalName::from(&**k)),
                    value: v.get_tendril()
                }
            }).collect(),
        }),

        "Comment" => CommentToken(args[0].get_tendril()),

        "Character" => CharacterTokens(args[0].get_tendril()),

        "PI" => PIToken(Pi {
            target: args[0].get_tendril(),
            data: args[1].get_tendril(),
        }),

        "DOCTYPE" => DoctypeToken (Doctype{
            name: args[0].get_nullable_tendril(),
            public_id: args[1].get_nullable_tendril(),
            system_id: args[2].get_nullable_tendril(),
        }),

        // We don't need to produce NullCharacterToken because
        // the TokenLogger will convert them to CharacterTokens.

        _ => panic!("don't understand token {:?}", parts),
    }
}


// Parse the "output" field of the test case into a vector of tokens.
fn json_to_tokens(js: &Json, exact_errors: bool) -> Vec<Token> {
    // Use a TokenLogger so that we combine character tokens separated
    // by an ignored error.
    let mut sink = TokenLogger::new(exact_errors);
    for tok in js.as_array().unwrap().iter() {
        match *tok {
            Json::String(ref s)
                if &s[..] == "ParseError" => sink.process_token(ParseError(Borrowed(""))),
            _ => sink.process_token(json_to_token(tok)),
        }
    }
    sink.get_tokens()
}

#[cfg(feature = "unstable")]
fn mk_xml_test(desc: String, input: String, expect: Json, opts: XmlTokenizerOpts)
        -> TestDescAndFn {
    TestDescAndFn {
        desc: TestDesc {
            name: DynTestName(desc),
            ignore: false,
            should_panic: No,
        },
        testfn: DynTestFn(Box::new(move |()| {
            // Split up the input at different points to test incremental tokenization.
            let insplits = splits(&input, 3);
            for input in insplits.into_iter() {
                // Clone 'input' so we have it for the failure message.
                // Also clone opts.  If we don't, we get the wrong
                // result but the compiler doesn't catch it!
                // Possibly mozilla/rust#12223.
                let output = tokenize_xml(input.clone(), opts.clone());
                let expect = json_to_tokens(&expect, opts.exact_errors);
                if output != expect {
                    panic!("\ninput: {:?}\ngot: {:?}\nexpected: {:?}",
                        input, output, expect);
                }
            }
        })),
    }
}
#[cfg(feature = "unstable")]
fn mk_xml_tests(tests: &mut Vec<TestDescAndFn>, filename: &str, js: &Json) {
    let input = js.find("input").unwrap().as_string().unwrap();
    let expect = js.find("output").unwrap().clone();
    let desc = format!("tok: {}: {}",
        filename, js.find("description").unwrap().as_string().unwrap());

    // Some tests want to start in a state other than Data.
    let state_overrides = vec!(None);


    // Build the tests.
    for state in state_overrides.into_iter() {
        for &exact_errors in [false, true].iter() {
            let mut newdesc = desc.clone();
            match state {
                Some(s) => newdesc = format!("{} (in state {:?})", newdesc, s),
                None  => (),
            };
            if exact_errors {
                newdesc = format!("{} (exact errors)", newdesc);
            }

            tests.push(mk_xml_test(newdesc, String::from(input), expect.clone(), XmlTokenizerOpts {
                exact_errors: exact_errors,
                initial_state: state,

                // Not discarding a BOM is what the test suite expects; see
                // https://github.com/html5lib/html5lib-tests/issues/2
                discard_bom: false,

                .. Default::default()
            }));
        }
    }
}

#[cfg(feature = "unstable")]
fn tests(src_dir: &Path) -> Vec<TestDescAndFn> {
    let mut tests = vec!();
    foreach_xml5lib_test(src_dir, "tokenizer",
                         OsStr::new("test"), |path, mut file| {
        let js = Json::from_reader(&mut file).ok().expect("json parse error");

        match js["tests"] {
            Json::Array(ref lst) => {
                for test in lst.iter() {
                    mk_xml_tests(&mut tests, path.file_name().unwrap().to_str().unwrap(), test);
                }
            }

            _ => (),
        }

    });

    tests
}

#[cfg(feature = "unstable")]
#[test]
fn run() {
    let args: Vec<_> = env::args().collect();
    test::test_main(&args, tests(Path::new(env!("CARGO_MANIFEST_DIR"))));
}
