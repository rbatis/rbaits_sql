use std::collections::HashMap;
use std::fmt::{Debug, Formatter};
use std::iter;

use xml5ever::tendril::{SliceExt};
use xml5ever::{parse};
use xml5ever::tree_builder::{TreeSink};
use xml5ever::rcdom::{Handle, NodeEnum, RcDom, Text};


#[derive(Clone, Eq, PartialEq)]
pub struct Element {
    pub tag: String,
    pub data: String,
    pub attrs: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl Debug for Element {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut s = f.debug_struct("");
        match self.tag.as_str() {
            "" => {
                s.field("data", &self.data);
            }
            _ => {
                s.field("tag", &self.tag);
                if !self.attrs.is_empty() {
                    s.field("attributes", &self.attrs);
                }
                if !self.childs.is_empty() {
                    s.field("childs", &self.childs);
                }
            }
        }
        return s.finish();
    }
}


pub fn as_element(args: &Vec<Handle>) -> Vec<Element> {
    let mut els = vec![];
    for x in args {
        let mut el = Element {
            tag: "".to_string(),
            data: "".to_string(),
            attrs: HashMap::new(),
            childs: vec![],
        };
        let b=&x.borrow();
        let n= &b.node;
        match n{
            NodeEnum::Document => {
            }
            NodeEnum::Doctype(_, _, _) => {
            }
            Text(txt) => {el.data = txt.to_string();}
            NodeEnum::Comment(comment) => {
                println!("comment:{}", comment);
            }
            NodeEnum::Element(n, attrs) => {
                el.tag = n.local.to_string();
                for attr in attrs {
                    el.attrs.insert(attr.name.local.to_string(), attr.value.to_string());
                }
                //         if !element.children.is_empty() {
                //             let childs = as_element(&element.children);
                //             el.childs = childs;
                //         }
                if !b.children.is_empty(){
                    let childs = as_element(&b.children);
                    el.childs = childs;
                }
            }
            NodeEnum::PI(_, _) => {}
        }
        els.push(el);
    }
    els
}


pub fn load_html(html: &str) -> Result<Vec<Element>,String> {
    // Using SliceExt.to_tendril functions we can read stdin
    let input = html.to_tendril();
    // To parse XML into a tree form, we need a TreeSink
    // luckily xml5ever comes with a static RC backed tree represetation.
    let dom: RcDom = parse(iter::once(input), Default::default());
    let b= dom.document.borrow();
    let els = as_element(&b.children);
    return Ok(els);
}
