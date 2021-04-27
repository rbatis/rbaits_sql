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
use crate::xml_loader::load_xml;

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


    let datas = load_xml(&arg);
    for x in datas {
        if x.tag.eq("mapper") {
            for x in x.childs {
                let id = x.attributes.get("id");
                match id {
                    None => {}
                    Some(id) => {
                        let method_name = Ident::new(id, Span::call_site());


                        let mut body=quote! {};
                        for x in x.childs {
                            if x.tag.eq("if") {
                                let test = x.attributes.get("test");
                                match test {
                                    Some(test_value) => {
                                        let method_name = encode(&test_value).replace("_", "__").replace("=", "_");
                                        let method_name = Ident::new(&method_name, Span::call_site());
                                        let test_value = test_value.replace(" and ", " && ");
                                        let test_value = test_value.replace(" or ", " && ");
                                        let method_impl = crate::func::impl_fn(&method_name.to_string(), &format!("\"{}\"", test_value));
                                        methods = quote! {
                                             #methods
                                             #method_impl
                                        };
                                        body = quote!(if #method_name() {
                                            sql=sql+"1";
                                        } );

                                    }
                                    _ => {}
                                }
                            }
                        }

                        let mut body = quote!(fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>) {
                               let mut  sql=String::new();
                               let mut args=vec![];
                               #body
                               return (sql,args)
                        } );
                        fn_impl=quote! {
                            #fn_impl
                            #body
                        }
                    }
                }
            }
        }
    }


    println!("gen methods:\n{}", methods);
    println!("gen fn:----start----\n{}\n----end----\n", fn_impl);


    let token=quote! {
        #methods
        #fn_impl
    };
    token.into()
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
    return t.into();
}