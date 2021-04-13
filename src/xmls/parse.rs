



#[cfg(test)]
mod test{
    use std::fs::File;
    use std::io::BufReader;
    use xml::EventReader;
    use xml::reader::XmlEvent;

    #[test]
    fn test_load_xml(){
        let file = File::open("example/example.xml").unwrap();
        let file = BufReader::new(file);
        let parser = EventReader::new(file);
        let mut depth = 0;
        for e in parser {
            match e {
                Ok(XmlEvent::StartElement { name, .. }) => {
                    println!("{}+{}", depth, name);
                    depth += 1;
                }
                Ok(XmlEvent::EndElement { name }) => {
                    depth -= 1;
                    println!("{}-{}", depth, name);
                }
                Err(e) => {
                    println!("Error: {}", e);
                    break;
                }
                _ => {}
            }
        }
    }
}