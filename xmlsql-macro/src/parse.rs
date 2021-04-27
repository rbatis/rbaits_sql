use quote::{quote, ToTokens};
use syn::{ItemFn, Expr};
use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use xml::EventReader;
use xml::reader::XmlEvent;
use base64::{encode, decode};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;

// fn fff(arg: &serde_json::Value) -> xml_sql::error::Result<(String, Vec<serde_json::Value>)> {
//     let mut sql = String::new();
//     let mut args: Vec<serde_json::Value> = vec![];
//
//     return Ok((sql, args));
// }

/// gen rust code
fn parse(arg: &str) -> TokenStream {
    let mut file = File::open("example/example.xml").unwrap();
    let mut arg = String::new();
    file.read_to_string(&mut arg);

    let mut methods = quote!();

    let mut fn_impl = quote!();

    let mut fn_body_vec = vec![];

    let parser = EventReader::from_str(&arg);
    let mut depth = 0;

    let mut current_tag = String::new();
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, namespace }) => {
                current_tag = name.local_name.to_string();

                let mut attr_map = HashMap::new();
                let mut attrs = String::new();
                for attr in &attributes {
                    attrs.push_str(&format!(" {} = \"{}\" ", attr.name, attr.value));
                    attr_map.insert(attr.name.to_string(), attr.value.clone());
                }

                //select node
                if name.local_name.eq("select") {
                    let id = attr_map.get("id");
                    match id {
                        None => {}
                        Some(id) => {
                            let method_name = Ident::new(id, Span::call_site());
                            //let method_impl = format!("fn {}(arg:&serde_json::Value) -> (String,Vec<serde_json::Value>,error) {}\n", method_name, "{ ");
                            let mut body = quote!(fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>,error)  );
                            fn_body_vec.push(body);
                        }
                    }
                }
                //if node
                if name.local_name.eq("if") {
                    let test = attr_map.get("test");
                    match test {
                        Some(test_value) => {
                            let method_name = encode(&test_value).replace("_", "__").replace("=", "_");
                            let method_name = Ident::new(&method_name, Span::call_site());
                            let test_value=test_value.replace(" and "," && ");
                            let test_value=test_value.replace(" or "," && ");
                            let method_impl= crate::func::impl_fn(&method_name.to_string(),&format!("\"{}\"",test_value));
                            methods = quote!{
                                #methods
                                #method_impl
                            };
                            let mut body = quote!(if #method_name() );
                            fn_body_vec.push(body);
                        }
                        _ => {}
                    }
                }

               // println!("{}<{} {} >", depth_to_space(depth), name, attrs);
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                //println!("{}</{}>", depth_to_space(depth), name);


                let f = fn_body_vec.pop();
                match f {
                    None => {}
                    Some(f) => {
                        if current_tag.eq("select") {
                            fn_impl = quote! { #f { #fn_impl Ok((sql,args)) } };
                        }else{
                            fn_impl = quote! { #f { #fn_impl } };
                        }
                    }
                }
            }
            Err(e) => {
               // println!("Error: {}", e);
                break;
            }
            Ok(XmlEvent::Characters(s)) => {
                //println!("{}{}", depth_to_space(depth), s.trim());
                let s = s.trim();
                fn_impl = quote!(#fn_impl sql=sql+ #s; );
            }
            Ok(XmlEvent::Comment(s)) => {
                //println!("{}{}", depth_to_space(depth), s.trim());
                let s = s.trim();
                fn_impl = quote!(#fn_impl sql=sql+ #s; );
            }
            Ok(XmlEvent::CData(s)) => {
               // println!("{}{}", depth_to_space(depth), s.trim());
                let s = s.trim();
                fn_impl = quote!(#fn_impl sql=sql+ #s; );
            }
            _ => {
                // println!("DefaultStr");
            }
        }
    }
    println!("gen methods:\n{}", methods);
    println!("gen fn:----start----\n{}\n----end----\n", fn_impl);
    fn_impl.into()
}

fn depth_to_space(depth: i32) -> String {
    let mut s = String::new();
    for _ in 0..depth {
        s.push_str(" ");
    }
    return s;
}

pub(crate) fn impl_fn(f: &ItemFn, args: crate::proc_macro::TokenStream) -> TokenStream {
    let t = parse(&args.to_string());
    return args.into();
}