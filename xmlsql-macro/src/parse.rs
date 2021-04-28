use quote::{quote, ToTokens};
use syn::{ItemFn, Expr, ItemMod};
use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use xml::EventReader;
use xml::reader::XmlEvent;
use base64::{encode, decode};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use crate::xml_loader::{load_xml, Element};

const example_data: &'static str = include_str!("../../example/example.xml");

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
        match x.tag.as_str() {
            "mapper" => {
                return parse(&x.childs, methods);
            }
            "" => {
                let mut s = x.data.to_owned();
                s = format!(" {} ", s.trim());
                body = quote!(
                        #body
                         sql=sql+#s;
                       );
            }
            "if" => {
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
            "trim" => {
                let mut empty_string = String::new();
                let prefix = x.attributes.get("prefix").unwrap_or(&empty_string).to_string();
                let suffix = x.attributes.get("suffix").unwrap_or(&empty_string).to_string();
                let mut prefixOverrides = x.attributes.get("prefixOverrides").unwrap_or(&empty_string).to_string();
                let mut suffixOverrides = x.attributes.get("suffixOverrides").unwrap_or(&empty_string).to_string();
                let mut trim_body = parse(&x.childs, methods);
                body = quote! {
                   #body
                   sql = format!("{}{}",sql,#prefix);
                };
                if !prefixOverrides.is_empty() {
                    let prefixs: Vec<&str> = prefixOverrides.split("|").collect();
                    let mut trims = quote! {sql};
                    for x in prefixs {
                        trims = quote! {
                            #trims
                            .trim_start_matches(#x)
                        }
                    }
                    body = quote! {
                        #body
                         let trim_string_prefix={
                            let mut sql = String::new();
                            #trim_body
                            sql = sql.trim_start().to_string();
                            if sql.starts_with(#prefixOverrides){
                               sql = #trims.to_string();
                            }
                            sql};
                        sql = sql + trim_string_prefix.as_str();
                    };
                    trim_body = quote! {};
                }
                body = quote! {
                        #body
                        #trim_body
                    };
                if !suffixOverrides.is_empty() {
                    let suffixs: Vec<&str> = suffixOverrides.split("|").collect();
                    let mut trims = quote! {sql_trim};
                    for x in suffixs {
                        trims = quote! {
                            #trims
                            .trim_end_matches(#x)
                        }
                    }
                    body = quote! {
                        #body
                        let sql_trim=sql.trim();
                        if sql_trim.ends_with(#suffixOverrides){
                            sql = #trims.to_string();
                        }
                    }
                }
                body = quote! {
                  #body
                  sql = sql+#suffix;
                };
            }
            "select" => {
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
            _ => {}
        }
    }

    println!("gen methods:\n{}", methods);
    println!("gen fn:----start----\n{}\n----end----\n", body);
    return body.into();
}

pub(crate) fn impl_fn(f: &ItemMod, args: crate::proc_macro::TokenStream) -> TokenStream {
    let t = parse_str(example_data);
    return t.into();
}