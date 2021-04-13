#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::BufReader;
    use xml::EventReader;
    use xml::reader::{XmlEvent, Error};

    #[test]
    fn test_load_xml() {
        let file = File::open("example/example.xml").unwrap();
        let file = BufReader::new(file);
        let parser = EventReader::new(file);
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                    let mut attrs = String::new();
                    for attr in attributes {
                        attrs.push_str(&format!(" {} = \"{}\" ", attr.name, attr.value));
                    }
                    println!("{}<{} {} >", depth_to_space(depth), name, attrs);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{}</{}>", depth_to_space(depth), name);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                Ok(XmlEvent::Characters(s)) => {
                    println!("{}{}", depth_to_space(depth), s.trim());
                }
                Ok(XmlEvent::Comment(s)) => {
                    println!("{}{}", depth_to_space(depth), s.trim());
                }
                Ok(XmlEvent::CData(s)) => {
                    println!("{}{}", depth_to_space(depth), s.trim());
                }
                _ => {
                    // println!("DefaultStr");
                }
            }
        }
    }

    fn depth_to_space(depth: i32) -> String {
        let mut s = String::new();
        for _ in 0..depth {
            s.push_str(" ");
        }
        return s;
    }
}