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

    println!("datas:{:#?}", datas);

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
                let mut s = &x.data;
                if !s.trim().is_empty() {
                    body = quote!(
                        #body
                         sql.push_str(#s);
                       );
                }
            }
            "if" => {
                impl_if(x, &mut body, methods);
            }
            "trim" => {
                let mut empty_string = String::new();
                let prefix = x.attributes.get("prefix").unwrap_or(&empty_string).to_string();
                let suffix = x.attributes.get("suffix").unwrap_or(&empty_string).to_string();
                let mut prefixOverrides = x.attributes.get("prefixOverrides").unwrap_or(&empty_string).to_string();
                let mut suffixOverrides = x.attributes.get("suffixOverrides").unwrap_or(&empty_string).to_string();
                impl_trim(&prefix, &suffix, &prefixOverrides, &suffixOverrides, x, &mut body, arg, methods);
            }
            "bind" => {
                let name = x.attributes.get("name").expect("bind element must be have name!").to_string();
                let value = x.attributes.get("value").expect("bind element must be have value!").to_string();

                let s = syn::parse::<syn::LitStr>(name.to_token_stream().into()).unwrap();
                let name_expr = syn::parse_str::<Expr>(&s.value()).unwrap();

                let method_impl = crate::func::impl_fn(&body.to_string(), "this_is_gen", &format!("\"{}\"", value), false,true);

                let method_string = method_impl.to_string();
                let method_impl = &method_string[method_string.find("{").unwrap()..method_string.len()];

                let s = syn::parse::<syn::LitStr>(method_impl.to_token_stream().into()).unwrap();
                let method_impl = syn::parse_str::<Expr>(&s.value()).unwrap();

                body = quote! {
                            #body
                            let #name_expr=#method_impl;
                        };
            }

            "where" => {
                impl_trim("", "", "and |or ", " and| or", x, &mut body, arg, methods);
            }

            "choose" => {
                let mut inner_body = quote! {};
                for x in &x.childs {
                    if x.tag.ne("when") && x.tag.ne("otherwise") {
                        panic!("choose node's childs must be when node and otherwise node!");
                    }
                    if x.tag.eq("when") {
                        impl_if(x, &mut inner_body, methods);
                    }
                    if x.tag.eq("otherwise") {
                        impl_otherwise(x, &mut inner_body, methods);
                    }
                }
                body = quote! {
                #body
                let do_choose=||->String{
                   let mut sql = String::new();
                   #inner_body
                   return sql;
                };
                let choose_strings = do_choose();
                sql.push_str(&choose_strings);
              }
            }

            "foreach" => {
                let empty_string = String::new();
                let collection = x.attributes.get("collection").unwrap_or(&empty_string).to_string();
                let item = x.attributes.get("item").unwrap_or(&empty_string).to_string();
                let index = x.attributes.get("index").unwrap_or(&empty_string).to_string();
                let open = x.attributes.get("open").unwrap_or(&empty_string).to_string();
                let close = x.attributes.get("close").unwrap_or(&empty_string).to_string();
                let separator = x.attributes.get("separator").unwrap_or(&empty_string).to_string();

                let impl_body = parse(&x.childs, methods);



                let method_name_string = encode(&collection).replace("_", "__").replace("=", "_");
                let method_name = Ident::new(&method_name_string, Span::call_site());
                let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", collection), false,false);
                let mut method_string = method_impl.to_string();
                let mut method_impl = method_string[method_string.find("{").unwrap()..method_string.len()].to_string();
                //method_impl = method_impl.replace("as_proxy()",".as_array().unwrap_or(&vec![])");
                let s = syn::parse::<syn::LitStr>(method_impl.to_token_stream().into()).unwrap();
                let method_impl = syn::parse_str::<Expr>(&s.value()).unwrap();
                //check append value
                if !body.to_string().contains(&method_name.to_string()) {
                    body = quote! {
                              #body
                              let #method_name = #method_impl;
                          };
                }

                body = quote! {
                    #body
                    for item in #method_name.as_array().unwrap() {
                        use xmlsql::ops::AsProxy;
                        let item=item.as_proxy();
                        #impl_body
                    }
                }

            }

            "set" => {
                impl_trim(" set ", "", ",", ",", x, &mut body, arg, methods);
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
            "update" => {}
            "insert" => {}
            "delete" => {}
            _ => {}
        }
    }


    return body.into();
}

fn impl_if(x: &Element, body: &mut proc_macro2::TokenStream, methods: &mut proc_macro2::TokenStream) {
    let test_value = x.attributes.get("test").expect(&format!("{} element must be have test field!", x.tag));
    let method_name_string = encode(&test_value).replace("_", "__").replace("=", "_");
    let method_name = Ident::new(&method_name_string, Span::call_site());
    let test_value = test_value.replace(" and ", " && ");
    let test_value = test_value.replace(" or ", " && ");
    let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", test_value), false,true);
    let mut method_string = method_impl.to_string();

    let method_impl = &method_string[method_string.find("{").unwrap()..method_string.len()];

    let s = syn::parse::<syn::LitStr>(method_impl.to_token_stream().into()).unwrap();
    let method_impl = syn::parse_str::<Expr>(&s.value()).unwrap();


    //check append value
    if !body.to_string().contains(&method_name.to_string()) {
        *body = quote! {
                              #body
                              let #method_name = #method_impl;
                          };
    }
    if x.childs.len() != 0 {
        let if_tag_body = parse(&x.childs, methods);
        *body = quote! {
                              #body
                              if #method_name {
                                   #if_tag_body
                              }
                          };
    }
}

fn impl_otherwise(x: &Element, body: &mut proc_macro2::TokenStream, methods: &mut proc_macro2::TokenStream) {
    let child_body = parse(&x.childs, methods);
    *body = quote!(
                        #body
                        #child_body
                       );
}


fn impl_trim(prefix: &str, suffix: &str, prefixOverrides: &str, suffixOverrides: &str, x: &Element, body: &mut proc_macro2::TokenStream, arg: &Vec<Element>, methods: &mut proc_macro2::TokenStream) {
    let mut trim_body = parse(&x.childs, methods);
    let prefixs: Vec<&str> = prefixOverrides.split("|").collect();
    let suffixs: Vec<&str> = suffixOverrides.split("|").collect();
    let have_trim = prefixs.len() != 0 && suffixs.len() != 0;
    let mut trims = quote! {
                     let mut sql=String::new();
                     #trim_body
                     sql=sql
                };
    for x in prefixs {
        trims = quote! {
                            #trims
                            .trim_start_matches(#x)
                        }
    }
    for x in suffixs {
        trims = quote! {
                            #trims
                            .trim_end_matches(#x)
                        }
    }

    *body = quote! {
                   #body
                    sql.push_str(#prefix);
                };
    if have_trim {
        *body = quote! {
                   #body
                   sql.push_str(&{#trims.to_string(); sql });
                };
    }
    *body = quote! {
                   #body
                   sql.push_str(#suffix);
                };
}

pub(crate) fn impl_fn(f: &ItemMod, args: crate::proc_macro::TokenStream) -> TokenStream {
    let t = parse_str(example_data);
    return t.into();
}