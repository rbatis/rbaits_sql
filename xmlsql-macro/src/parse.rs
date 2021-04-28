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
use crate::xml_loader::{load_xml, Element};

// fn fff(arg: &serde_json::Value) -> xml_sql::error::Result<(String, Vec<serde_json::Value>)> {
//     let mut sql = String::new();
//     let mut args: Vec<serde_json::Value> = vec![];
//
//     return Ok((sql, args));
// }

const  example_data:&'static str=include_str!("../../example/example.xml");

fn parse_str(arg: &str) -> TokenStream {
    let datas = load_xml(arg);
    let mut methods = quote!();
    let fn_impl = parse(&datas, &mut methods);
    let token = quote! {
        #methods
        #fn_impl
    };
    token.into()
}

/// gen rust code
fn parse(arg: &Vec<Element>, methods: &mut proc_macro2::TokenStream) -> proc_macro2::TokenStream {
    let mut body = quote! {};
    for x in arg {
        if x.tag.eq("mapper") {
            return parse(&x.childs, methods);
        }
        if x.tag.eq("") {
            let s = x.data.to_owned();
            body = quote!(
                        #body
                         sql=sql+#s;
                       );
        }
        if x.tag.eq("if") {
            let test_value = x.attributes.get("test").expect("if element must be have test field!");
            let method_name_string = encode(&test_value).replace("_", "__").replace("=", "_");
            let method_name = Ident::new(&method_name_string, Span::call_site());
            let test_value = test_value.replace(" and ", " && ");
            let test_value = test_value.replace(" or ", " && ");
            let method_impl = crate::func::impl_fn(&method_name.to_string(), &format!("\"{}\"", test_value));
            if !methods.to_token_stream().to_string().contains(&method_name_string) {
                *methods = quote! {
                                             #methods
                                             #method_impl
                                          };
            }
            if x.childs.len() != 0 {
                let if_tag_body = parse(&x.childs, methods);
                body = quote! {
                              #body
                              if #method_name(arg).as_bool().unwrap_or(false) {
                                   #if_tag_body
                                }
                              };
            }
        }
        if x.tag.eq("select") {
            let id = x.attributes.get("id").expect("select element must be have id!");
            let method_name = Ident::new(id, Span::call_site());
            let child_body = parse(&x.childs, methods);
            let mut select = quote! {
                            pub fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>) {
                               let mut sql = String::with_capacity(1000);
                               let mut args = vec![];
                               #child_body
                               return (sql,args);
                            }
                        };
            body = quote! {
                            #body
                            #select
                        };
        }
    }

    println!("gen methods:\n{}", methods);
    println!("gen fn:----start----\n{}\n----end----\n", body);
    return body.into();
}

fn depth_to_space(depth: i32) -> String {
    let mut s = String::new();
    for _ in 0..depth {
        s.push_str(" ");
    }
    return s;
}

pub(crate) fn impl_fn(f: &ItemFn, args: crate::proc_macro::TokenStream) -> TokenStream {
    let t = parse_str(example_data);
    return t.into();
}