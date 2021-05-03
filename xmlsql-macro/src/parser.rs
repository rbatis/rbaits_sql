use quote::{quote, ToTokens};
use syn::{ItemFn, Expr, ItemMod, Path};
use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use xml::EventReader;
use xml::reader::XmlEvent;
use base64::{encode, decode};
use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use crate::xml_loader::{load_xml, Element};
use crate::string_util::find_convert_string;

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
    let empty_string = String::new();
    let mut body = quote! {};
    for x in arg {
        match x.tag.as_str() {
            "mapper" => {
                return parse(&x.childs, methods);
            }
            "table" => {
                let table_name = x.attributes.get("name").expect("<table> mut have name attr!");
                if table_name.is_empty() {
                    panic!("<table> mut have name attr!");
                }
                let table_ident = Ident::new(&table_name, Span::call_site());

                let mut table_fields = quote! {};
                for x in &x.childs {
                    match x.tag.as_ref() {
                        "id" => {
                            let column = x.attributes.get("column").unwrap_or(&empty_string);
                            let mut type_lang = x.attributes.get("type_lang").unwrap_or(&empty_string).to_string();
                            let type_option = x.attributes.get("type_option").unwrap_or(&empty_string);
                            if !type_option.is_empty() {
                                type_lang = format!("Option<{}>", type_option);
                            }
                            if column.is_empty() {
                                panic!("<id> column can not be empty!")
                            }
                            if type_lang.is_empty() {
                                panic!("<id> type_lang can not be empty!")
                            }
                            let column_ident = Ident::new(&column, Span::call_site());
                            let type_lang_ident = parse_path(&type_lang);
                            table_fields = quote! {
                                #table_fields
                                pub #column_ident:#type_lang_ident,
                            };
                        }
                        "result" => {
                            let column = x.attributes.get("column").unwrap_or(&empty_string);
                            let mut type_lang = x.attributes.get("type_lang").unwrap_or(&empty_string).to_string();
                            let type_option = x.attributes.get("type_option").unwrap_or(&empty_string);
                            if !type_option.is_empty() {
                                type_lang = format!("Option<{}>", type_option);
                            }
                            if column.is_empty() {
                                panic!("<id> column can not be empty!")
                            }
                            if type_lang.is_empty() {
                                panic!("<id> type_lang can not be empty!")
                            }
                            let column_ident = Ident::new(&column, Span::call_site());
                            let type_lang_ident = parse_path(&type_lang);
                            table_fields = quote! {
                                #table_fields
                                pub #column_ident:#type_lang_ident,
                            };
                        }
                        _ => {}
                    }
                }

                body = quote! {
                    #body
                    #[derive(Clone, Debug)]
                    #[derive(serde::Serialize, serde::Deserialize)]
                    pub struct #table_ident{
                         #table_fields
                    }
                }
            }
            "" => {
                let mut string_data = x.data.trim().to_string();
                let convert_list = find_convert_string(&string_data);
                let mut replaces = quote! {};

                let mut replaced = HashMap::<String, bool>::new();
                for (k, v) in convert_list {
                    let method_name_string = encode(&k).replace("_", "__").replace("=", "_");
                    let method_name = Ident::new(&method_name_string, Span::call_site());
                    let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", k), false, true);
                    let mut method_string = method_impl.to_string();
                    method_string = method_string.replace("& arg", "arg");
                    let mut method_impl = method_string[method_string.find("{").unwrap()..method_string.len()].to_string();
                    let method_impl = parse_expr(&method_impl);
                    //check append value
                    if !body.to_string().contains(&format!("{} ", method_name)) {
                        body = quote! {
                              #body
                              let #method_name = #method_impl;
                          };
                    }
                    if v.starts_with("#") {
                        string_data = string_data.replace(&v, " ? ");
                        body = quote! {
                              #body
                              args.push(serde_json::json!(#method_name));
                          };
                    } else {
                        if replaced.get(&v).is_none() {
                            replaces = quote! {#replaces.replace(#v, &#method_name.to_string())};
                            replaced.insert(v.to_string(), true);
                        }
                    }
                }
                if !replaces.is_empty() {
                    replaces = quote! {
                        #replaces.as_str()
                    }
                }
                if !string_data.is_empty() {
                    body = quote!(
                        #body
                         sql.push_str(#string_data#replaces);
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
                let name = x.attributes.get("name").expect("<bind> must be have name!").to_string();
                let value = x.attributes.get("value").expect("<bind> element must be have value!").to_string();

                let name_expr = parse_expr(&name);

                let method_impl = crate::func::impl_fn(&body.to_string(), "this_is_gen", &format!("\"{}\"", value), false, true);

                let method_string = method_impl.to_string();
                let method_impl = &method_string[method_string.find("{").unwrap()..method_string.len()];

                let method_impl = parse_expr(&method_impl);

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

                let def_item = "item".to_string();
                let def_index = "index".to_string();

                let collection = x.attributes.get("collection").unwrap_or(&empty_string).to_string();
                let item = x.attributes.get("item").unwrap_or(&def_item).to_string();
                let index = x.attributes.get("index").unwrap_or(&def_index).to_string();
                let open = x.attributes.get("open").unwrap_or(&empty_string).to_string();
                let close = x.attributes.get("close").unwrap_or(&empty_string).to_string();
                let separator = x.attributes.get("separator").unwrap_or(&empty_string).to_string();

                let impl_body = parse(&x.childs, methods);
                //do replace arg get index and item
                let mut body_strings = impl_body.to_string().replace("  ", " ").replace("\n", "");
                // body_strings = body_strings.replace(&format!("arg [\"{}\"] . as_proxy()", index), &index);
                // body_strings = body_strings.replace(&format!("arg [\"{}\"] . as_proxy()", item), &item);
                body_strings = body_strings.replace(&format!("arg [\"{}\"]", index), &index);
                body_strings = body_strings.replace(&format!("arg [\"{}\"]", item), &item);
                let s = syn::parse::<syn::LitStr>(body_strings.to_token_stream().into()).unwrap();
                let impl_body = syn::parse_str::<proc_macro2::TokenStream>(&s.value()).unwrap();


                let method_name_string = encode(&collection).replace("_", "__").replace("=", "_");
                let method_name = Ident::new(&method_name_string, Span::call_site());
                let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", collection), false, false);
                let mut method_string = method_impl.to_string();
                let mut method_impl = method_string[method_string.find("{").unwrap()..method_string.len()].to_string();

                let method_impl = parse_expr(&method_impl);
                //check append value
                if !body.to_string().contains(&format!("{} ", method_name)) {
                    body = quote! {
                              #body
                              let #method_name = #method_impl;
                          };
                }

                let mut open_impl = quote! {};
                if !open.is_empty() {
                    open_impl = quote! {
                    sql.push_str(#open);
                    };
                }
                let mut close_impl = quote! {};
                if !close.is_empty() {
                    close_impl = quote! {
                    sql.push_str(#close);
                    };
                }


                let item_ident = Ident::new(&item, Span::call_site());
                let index_ident = Ident::new(&index, Span::call_site());

                let mut index_create = quote! {};
                let mut index_add = quote! {};
                index_create = quote! {
                        let mut #index_ident=0;
                    };
                index_add = quote! {
                        #index_ident=#index_ident+1;
                    };

                body = quote! {
                    #body
                    if #method_name.is_array(){
                        #open_impl
                        #index_create
                        for #item_ident in #method_name.as_array().unwrap() {
                          use xmlsql::ops::AsProxy;
                          let item=#item_ident.as_proxy();
                          #impl_body
                          #index_add
                        }
                        #close_impl
                    }else if #method_name.is_object(){
                        #open_impl
                        for (#index_ident,#item_ident) in #method_name.as_object().unwrap() {
                          use xmlsql::ops::AsProxy;
                          let item=#item_ident.as_proxy();
                          #impl_body
                        }
                        #close_impl
                    }
                };
            }

            "set" => {
                impl_trim(" set ", "", ",", ",", x, &mut body, arg, methods);
            }

            "select" => {
                let id = x.attributes.get("id").expect("<select> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods);
                let mut select = quote! {
                            pub fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>) {
                               let mut sql = String::with_capacity(1000);
                               let mut args = Vec::with_capacity(20);
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
    let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", test_value), false, true);
    let mut method_string = method_impl.to_string();

    let method_impl = &method_string[method_string.find("{").unwrap()..method_string.len()];

    let s = syn::parse::<syn::LitStr>(method_impl.to_token_stream().into()).unwrap();
    let method_impl = syn::parse_str::<Expr>(&s.value()).unwrap();


    //check append value
    if !body.to_string().contains(&format!("{} ", method_name)) {
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

/// parse to expr
fn parse_expr(lit_str: &str) -> Expr {
    let s = syn::parse::<syn::LitStr>(lit_str.to_token_stream().into()).expect(&format!("parse::<syn::LitStr> fail: {}", lit_str));
    return syn::parse_str::<Expr>(&s.value()).expect(&format!("parse_str::<Expr> fail: {}", lit_str));
}

/// parse to expr
fn parse_path(lit_str: &str) -> Path {
    let s = syn::parse::<syn::LitStr>(lit_str.to_token_stream().into()).expect(&format!("parse::<syn::LitStr> fail: {}", lit_str));
    return syn::parse_str::<Path>(&s.value()).expect(&format!("parse_str::<Path> fail: {}", lit_str));
}