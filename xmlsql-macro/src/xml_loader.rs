use std::fs::File;

use xml::reader::{EventReader, XmlEvent};
use std::io::{Read, BufReader};
use std::fs;
use std::thread::park;
use std::fmt::Error;
use core::borrow::Borrow;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Element {
    pub tag: String,
    pub data: String,
    pub attributes: HashMap<String, String>,
    pub childs: Vec<Element>,
}

impl Element {
    pub fn reset(&mut self) {
        self.tag.clear();
        self.data.clear();
        self.attributes.clear();
        self.childs.clear();
    }
}


pub fn load_xml(file_content: &str) -> Vec<Element> {
    let parser = EventReader::from_str(file_content);
    return parser_func(parser);
}

fn parser_func(parser: EventReader<&[u8]>) -> Vec<Element> {
    let mut depth = 0;

    let mut fathers = vec![];

    let mut temp_element = &mut Element {
        tag: "".to_string(),
        data: "".to_string(),
        attributes: HashMap::new(),
        childs: vec![],
    };

    for item in parser {
        match item {
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                *temp_element = Element {
                    tag: "".to_string(),
                    data: "".to_string(),
                    attributes: HashMap::new(),
                    childs: vec![],
                };
                //load attr
                temp_element.tag = name.to_string();
                for x in attributes {
                    temp_element.attributes.insert(x.name.to_string(), x.value.to_string());
                }
                &fathers.push(temp_element.clone());
                depth += 1;
            }
            Ok(XmlEvent::Characters(data)) | Ok(XmlEvent::Comment(data)) | Ok(XmlEvent::CData(data)) => {
                let mut data = data.replace("\r\n", "").to_string();
                data=data.trim().to_string();
                let last = fathers.last_mut().unwrap();
                (*last).childs.push(Element {
                    tag: "".to_string(),
                    data: data,
                    attributes: HashMap::new(),
                    childs: vec![],
                })
            }
            Ok(XmlEvent::EndElement { name }) => {
                let pop = fathers.pop().unwrap();
                let last = fathers.last_mut();
                if last.is_some() {
                    last.unwrap().childs.push(pop);
                } else {
                    fathers.push(pop)
                }
                temp_element.reset();

                depth -= 1;
            }
            Err(e) => {
                println!("Error: {},{}", e, temp_element.tag);
                break;
            }
            _ => {}
        }
    }
    return fathers;
}

//load a xml file
#[test]
fn test_load_file() {
    // --snip--
    let file_path = "example/example.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", file_path);
    let content = fs::read_to_string(file_path).unwrap();
    println!("With text:/n{}", content);
}

//load xml
#[test]
fn test_load_xml() {
    let file_path = "example/example.xml";
    println!(">>>>>>>>>>>>>>>>>>>>>>start load {} >>>>>>>>>>>>>>>>>>>>>>>", file_path);
    let content = fs::read_to_string(file_path).unwrap();
    //println!("With text:/n{}", content);

    load_xml(content.as_str());
}