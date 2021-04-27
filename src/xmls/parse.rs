use xml::EventReader;
use xml::reader::XmlEvent;
use base64::{encode, decode};

/// gen rust code
fn parse(arg: &str) {
    let mut methods = "".to_string();

    let mut fn_impl = "let mut sql=String::new();\n".to_string();

    let parser = EventReader::from_str(arg);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                let mut attrs = String::new();
                for attr in attributes {
                    attrs.push_str(&format!(" {} = \"{}\" ", attr.name, attr.value));
                    //if node
                    if name.local_name.eq("if") {
                        if attr.name.local_name.eq("test") {
                            let method_name = encode(attr.value).replace("_", "__").replace("=", "_");
                            let method_impl = format!("fn {}() -> bool {}\n", method_name, "{ true }");
                            methods.push_str(&method_impl);
                            fn_impl.push_str(&format!("if {}(){}", method_name, "{\n"));
                        }
                    }
                }
                println!("{}<{} {} >", depth_to_space(depth), name, attrs);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                println!("{}</{}>", depth_to_space(depth), name);

                fn_impl.push_str("\n}\n");
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            Ok(XmlEvent::Characters(s)) => {
                println!("{}{}", depth_to_space(depth), s.trim());
                fn_impl.push_str(&format!("sql=sql+\"{}\";\n", s.trim()));
            }
            Ok(XmlEvent::Comment(s)) => {
                println!("{}{}", depth_to_space(depth), s.trim());
                fn_impl.push_str(&format!("sql=sql+\"{}\";\n", s.trim()));
            }
            Ok(XmlEvent::CData(s)) => {
                println!("{}{}", depth_to_space(depth), s.trim());
                fn_impl.push_str(&format!("sql=sql+\"{}\";\n", s.trim()));
            }
            _ => {
                // println!("DefaultStr");
            }
        }
    }
    println!("gen methods:\n{}", methods);
    println!("gen fn:\n{}", fn_impl);
}

fn depth_to_space(depth: i32) -> String {
    let mut s = String::new();
    for _ in 0..depth {
        s.push_str(" ");
    }
    return s;
}


#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::{BufReader, Read};
    use xml::EventReader;
    use xml::reader::{XmlEvent, Error};
    use crate::xmls::parse::parse;

    #[test]
    fn test_load_xml() {
        let mut file = File::open("example/example.xml").unwrap();
        let mut s = String::new();
        file.read_to_string(&mut s);
        parse(&s);
    }
}