use std::collections::HashMap;
use html_parser::{Dom, Node, Result};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Element {
    pub tag: String,
    pub data: String,
    pub attributes: HashMap<String, String>,
    pub childs: Vec<Element>,
}


pub fn as_element(args: &Vec<Node>) -> Vec<Element> {
    let mut els = vec![];
    for x in args {
        let mut el = Element {
            tag: "".to_string(),
            data: "".to_string(),
            attributes: HashMap::new(),
            childs: vec![],
        };
        match x {
            Node::Text(txt) => {
                el.data = txt.to_string();
            }
            Node::Element(element) => {
                el.tag = element.name.to_string();
                if element.id.is_some(){
                    el.attributes.insert("id".to_string(),element.id.as_ref().unwrap_or(&String::new()).clone());
                }
                for (k, v) in &element.attributes {
                    el.attributes.insert(k.clone(), v.as_ref().unwrap_or(&String::new()).clone());
                }
                if !element.children.is_empty() {
                    let childs = as_element(&element.children);
                    el.childs = childs;
                }
            }
            Node::Comment(comment) => {
                println!("comment:{}",comment);
            }
        }
        els.push(el);
    }
    els
}

pub fn load_html(html: &str) -> Result<Vec<Element>> {
    let dom = Dom::parse(html)?;
    let els = as_element(&dom.children);
    return Ok(els);
}
