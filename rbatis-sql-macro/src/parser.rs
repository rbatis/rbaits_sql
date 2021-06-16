use quote::{quote, ToTokens};
use syn::{AttributeArgs, ItemFn, Expr, ItemMod, Path, ItemStruct};
use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

use xml::EventReader;
use xml::reader::XmlEvent;
use base64::{encode, decode};
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;
use crate::string_util::find_convert_string;
use crate::html_loader::{load_html, Element};
use crate::py_sql::{NodeType, ParsePySql};
use url::Url;

fn parse_html_str(html: &str, format_char: char, fn_name: &str) -> proc_macro2::TokenStream {
    let datas = load_html(html).expect("load_html() fail!");
    let mut sql_map = HashMap::new();
    let datas = include_replace(datas, &mut sql_map);
    drop(sql_map);
    for x in datas {
        if x.tag.eq("mapper") {
            for x in x.childs {
                match x.attributes.get("id") {
                    Some(id) => {
                        if id.eq(fn_name) {
                            return parse_html_node(vec![x], format_char);
                        }
                    }
                    _ => {}
                }
            }

            break;
        }
    }
    panic!("html not find fn:{}", fn_name);
}


fn find_element(id: &str, htmls: &Vec<Element>) -> Option<Element> {
    for x in htmls {
        if x.childs.len() != 0 {
            let find = find_element(id, &x.childs);
            if find.is_some() {
                return find;
            }
        }
        match x.attributes.get("id") {
            None => {}
            Some(id) => {
                if id.eq(id) {
                    return Some(x.clone());
                }
            }
        }
    }
    return None;
}

fn include_replace(htmls: Vec<Element>, sql_map: &mut HashMap<String, Vec<Element>>) -> Vec<Element> {
    let mut results = vec![];
    for mut x in htmls {
        match x.tag.as_str() {
            "sql" => {
                sql_map.insert(x.attributes.get("id").expect("[rbatis] <sql> element must have id!").clone(), x.childs.clone());
            }
            "include" => {
                let refid = x.attributes.get("refid").expect("[rbatis] <include> element must have refid!").clone();
                let data_url = Url::parse(format!("url://{}", refid).as_str()).expect("[rbatis] parse include url fail!");
                let mut find_file = false;
                for (k, v) in data_url.query_pairs() {
                    let v = v.to_string();
                    match k.to_string().as_str() {
                        "file" => {
                            if v.is_empty() {
                                panic!("[rbatis] <include> element file must be have an value!");
                            }
                            let mut html_string = String::new();
                            let mut f = File::open(v.as_str()).expect(&format!("File:\"{}\" does not exist", v));
                            f.read_to_string(&mut html_string);
                            let datas = load_html(html_string.as_str()).expect("load_html() fail!");
                            let find = find_element(refid.as_str(), &datas).expect(&format!("[rbatis] not find html:{} , element id={}", v, refid));
                            x.childs.push(find);
                            find_file = true;
                            break;
                        }
                        &_ => {}
                    }
                }
                if !find_file {
                    let refid_path = data_url.host().unwrap().to_string();
                    let element = sql_map.get(refid_path.as_str()).expect(&format!("[rbatis] can not find element {} <include refid='{}'> !", refid, refid_path)).clone();
                    for el in element {
                        x.childs.push(el.clone());
                    }
                }
            }
            _ => {
                match x.attributes.get("id") {
                    None => {}
                    Some(id) => {
                        if !id.is_empty() {
                            sql_map.insert(id.clone(), x.childs.clone());
                        }
                    }
                }
            }
        }
        if x.childs.len() != 0 {
            x.childs = include_replace(x.childs.clone(), sql_map);
        }

        match x.tag.as_str() {
            "include" | "sql" => {
                for c in x.childs {
                    results.push(c);
                }
            }
            _ => {
                results.push(x);
            }
        }
    }
    return results;
}

fn parse_html_node(htmls: Vec<Element>, format_char: char) -> proc_macro2::TokenStream {
    #[cfg(feature = "debug_mode")]
        {
            println!("load html:{:#?}", htmls);
        }
    let mut methods = quote!();
    let fn_impl = parse(&htmls, &mut methods, "", format_char);
    let token = quote! {
        #methods
        #fn_impl
    };
    token
}

fn to_mod(m: &ItemMod, t: &proc_macro2::TokenStream) -> TokenStream {
    let ident = &m.ident;
    let mod_token = quote! {
        pub mod #ident{
            #t
        }
    };
    mod_token.into()
}


/// gen rust code
fn parse(arg: &Vec<Element>, methods: &mut proc_macro2::TokenStream, block_name: &str, format_char: char) -> proc_macro2::TokenStream {
    let mut body = quote! {};
    let fix_sql = quote! {
    macro_rules! push_index {
     ($n:expr,$new_sql:ident,$index:expr) => {
                  {
                     let mut num=$index/$n;
                     $new_sql.push((num+48) as u8 as char);
                     $index % $n
                  }
              };
    ($index:ident,$new_sql:ident) => {
                if  $index>=0 && $index<10{
                    $new_sql.push(($index+48)as u8 as char);
                }else if $index>=10 && $index<100 {
                    let $index = push_index!(10,$new_sql,$index);
                    let $index = push_index!(1,$new_sql,$index);
                }else if $index>=100 && $index<1000{
                    let $index = push_index!(100,$new_sql,$index);
                    let $index = push_index!(10,$new_sql,$index);
                    let $index = push_index!(1,$new_sql,$index);
                }else if $index>=1000 && $index<10000{
                    let $index = push_index!(1000,$new_sql,$index);
                    let $index = push_index!(100,$new_sql,$index);
                    let $index = push_index!(10,$new_sql,$index);
                    let $index = push_index!(1,$new_sql,$index);
                }else{
                     use std::fmt::Write;
                     $new_sql.write_fmt(format_args!("{}", $index))
                    .expect("a Display implementation returned an error unexpectedly");
               }
       };
    }
    let mut new_sql = String::with_capacity(sql.len()+20);
    let mut string_start = false;
    let mut index:i32 = 0;
    for x in sql.chars() {
        if x == '\'' || x == '"' {
            if string_start == true {
                string_start = false;
                new_sql.push(x);
                continue;
            }
            string_start = true;
            new_sql.push(x);
            continue;
        }
        if string_start {
            new_sql.push(x);
        } else {
            if x=='?' && #format_char != '?' {
                index+=1;
                new_sql.push(#format_char);
                push_index!(index,new_sql);
            }else{
                new_sql.push(x);
            }
        }
    }
       sql=new_sql;
    };
    for x in arg {
        match x.tag.as_str() {
            "mapper" => {
                return parse(&x.childs, methods, "mapper", format_char);
            }
            "sql" => {
                return parse(&x.childs, methods, "sql", format_char);
            }
            "include" => {
                return parse(&x.childs, methods, "include", format_char);
            }
            "println" => {
                impl_println(x, &mut body);
            }
            "" => {
                let mut string_data = x.data.trim().to_string();
                let convert_list = find_convert_string(&string_data);
                let mut replaces = quote! {};

                let mut replaced = HashMap::<String, bool>::new();
                for (k, v) in convert_list {
                    let (method_name_string, method_name) = gen_method_name(&format!("{}:{}", block_name, k));
                    let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", k), false, true);
                    let mut method_string = method_impl.to_string();
                    method_string = method_string.replace("& arg", "arg");
                    let method_impl = method_string[method_string.find("{").unwrap()..method_string.len()].to_string();
                    let method_impl = parse_expr(&method_impl);
                    //check append value
                    if !body.to_string().contains(&format!("{} ", method_name)) {
                        body = quote! {
                              #body
                              let #method_name = #method_impl;
                          };
                    }
                    if v.starts_with("#") {
                        string_data = string_data.replacen(&v, &" ? ", 1);
                        body = quote! {
                              #body
                              args.push(serde_json::json!(#method_name));
                          };
                    } else {
                        if replaced.get(&v).is_none() {
                            replaces = quote! {#replaces.replacen(#v, &#method_name.to_string(), 1)};
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
                impl_if(x, &mut body, methods, quote! {}, &format!("{}:{}", block_name, "if"), format_char);
            }
            "trim" => {
                let empty_string = String::new();
                let prefix = x.attributes.get("prefix").unwrap_or(&empty_string).to_string();
                let mut suffix = x.attributes.get("suffix").unwrap_or(&empty_string).to_string();
                if suffix.is_empty() {
                    suffix = " ".to_string();
                }
                let  prefixOverrides = x.attributes.get("prefixOverrides").unwrap_or(&empty_string).to_string();
                let  suffixOverrides = x.attributes.get("suffixOverrides").unwrap_or(&empty_string).to_string();
                impl_trim(&prefix, &suffix, &prefixOverrides, &suffixOverrides, x, &mut body, arg, methods, &format!("{}:{}", block_name, "trim"), format_char);
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
                impl_trim("", " ", "and |or ", " and| or", x, &mut body, arg, methods, &format!("{}:{}", block_name, "where:trim"), format_char);
            }

            "choose" => {
                let mut inner_body = quote! {};
                for x in &x.childs {
                    if x.tag.ne("when") && x.tag.ne("otherwise") {
                        panic!("choose node's childs must be when node and otherwise node!");
                    }
                    if x.tag.eq("when") {
                        impl_if(x, &mut inner_body, methods, quote! {return sql;}, &format!("{}:{}", block_name, "choose:when:if"), format_char);
                    }
                    if x.tag.eq("otherwise") {
                        impl_otherwise(x, &mut inner_body, methods, &format!("{}:{}", block_name, "choose:otherwise"), format_char);
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
                sql.push_str(" ");
                sql.push_str(&choose_strings);
                sql.push_str(" ");
              }
            }

            "foreach" => {
                let empty_string = String::new();

                let def_item = "item".to_string();
                let def_index = "index".to_string();

                let collection = x.attributes.get("collection").unwrap_or(&empty_string).to_string();
                let item = x.attributes.get("item").unwrap_or(&def_item).to_string();
                let findex = x.attributes.get("index").unwrap_or(&def_index).to_string();
                let open = x.attributes.get("open").unwrap_or(&empty_string).to_string();
                let close = x.attributes.get("close").unwrap_or(&empty_string).to_string();
                let separator = x.attributes.get("separator").unwrap_or(&empty_string).to_string();

                let impl_body = parse(&x.childs, methods, "foreach", format_char);
                //do replace arg get index and item
                let mut body_strings = impl_body.to_string().replace("\n", " ").replace("  ", " ");
                //TODO batter way do not replace
                body_strings = body_strings.replace(&format!("(arg) [\"{}\"]", findex), &findex);
                body_strings = body_strings.replace(&format!("(arg) [\"{}\"]", item), &item);

                let s = syn::parse::<syn::LitStr>(body_strings.to_token_stream().into()).unwrap();
                let impl_body = syn::parse_str::<proc_macro2::TokenStream>(&s.value()).unwrap();

                let (method_name_string, method_name) = gen_method_name(&format!("{}:{}", block_name, collection));
                let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", collection), false, false);
                let method_string = method_impl.to_string();
                let method_impl = method_string[method_string.find("{").unwrap()..method_string.len()].to_string();

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
                let index_ident = Ident::new(&findex, Span::call_site());

                let mut index_create = quote! {};
                let mut index_add = quote! {};
                let mut split_code = quote! {};
                let mut split_code_end = quote! {};
                index_create = quote! {
                        let mut #index_ident=0;
                    };
                index_add = quote! {
                        #index_ident=#index_ident+1;
                    };

                if !separator.is_empty() {
                    split_code = quote! {    sql.push_str(#separator);  };
                    split_code_end = quote! {
                        if foreach_arr.len() != 0 {
                             sql.pop();
                        }
                    };
                }

                body = quote! {
                    #body
                    if #method_name.is_array(){
                        #open_impl
                        #index_create
                        {
                          use rbatis_sql::ops::AsProxy;
                          let foreach_arr = #method_name.as_array().unwrap();
                          for #item_ident in foreach_arr {
                            let item=#item_ident.as_proxy();
                            #impl_body
                            #split_code
                            #index_add
                          }
                          #split_code_end
                        }
                        #close_impl
                    }else if #method_name.is_object(){
                        #open_impl
                        {
                          use rbatis_sql::ops::AsProxy;
                          let foreach_arr = #method_name.as_object().unwrap();
                          for (#index_ident,#item_ident) in foreach_arr {
                              let item=#item_ident.as_proxy();
                              #impl_body
                              #split_code
                          }
                          #split_code_end
                        }
                        #close_impl
                    }
                };
            }

            "set" => {
                impl_trim(" set ", " ", ",", ",", x, &mut body, arg, methods, &format!("{}:{}", block_name, "set:trim"), format_char);
            }

            "select" => {
                let id = x.attributes.get("id").expect("<select> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", format_char);
                let select = quote! {
                            pub fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>) {
                               let mut sql = String::with_capacity(1000);
                               let mut args = Vec::with_capacity(20);
                               #child_body
                               #fix_sql
                               return (sql,args);
                            }
                        };
                body = quote! {
                            #body
                            #select
                        };
            }
            "update" => {
                let id = x.attributes.get("id").expect("<update> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", format_char);
                let select = quote! {
                            pub fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>) {
                               let mut sql = String::with_capacity(1000);
                               let mut args = Vec::with_capacity(20);
                               #child_body
                               #fix_sql
                               return (sql,args);
                            }
                        };
                body = quote! {
                            #body
                            #select
                        };
            }
            "insert" => {
                let id = x.attributes.get("id").expect("<insert> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", format_char);
                let select = quote! {
                            pub fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>) {
                               let mut sql = String::with_capacity(1000);
                               let mut args = Vec::with_capacity(20);
                               #child_body
                               #fix_sql
                               return (sql,args);
                            }
                        };
                body = quote! {
                            #body
                            #select
                        };
            }
            "delete" => {
                let id = x.attributes.get("id").expect("<delete> element must be have id!");
                let method_name = Ident::new(id, Span::call_site());
                let child_body = parse(&x.childs, methods, "select", format_char);
                let select = quote! {
                            pub fn #method_name (arg:&serde_json::Value) -> (String,Vec<serde_json::Value>) {
                               let mut sql = String::with_capacity(1000);
                               let mut args = Vec::with_capacity(20);
                               #child_body
                               #fix_sql
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


    return body.into();
}


fn impl_println(x: &Element, body: &mut proc_macro2::TokenStream) {
    let value = x.attributes.get("value").expect(&format!("{} element must be have value field!", x.tag));
    let method_name = impl_method(value, body);
    let mut format = String::new();
    if let Some(s) = x.attributes.get("format") {
        format = s.to_string();
    }
    if format.is_empty() {
        *body = quote! {
                   #body
                   println!("{}",#method_name);
                  };
    } else {
        let format_expr = syn::parse_str::<syn::Lit>(&format!("\"{}\"", format)).expect(&format!("[rexpr]syn::parse_str: {}", format));
        *body = quote! {
                   #body
                   println!(#format_expr,#method_name);
                  };
    }
}

fn gen_method_name(test_value: &str) -> (String, Ident) {
    let method_name_string = encode(&test_value).replace("_", "__").replace("=", "_");
    (method_name_string.clone(), Ident::new(&method_name_string, Span::call_site()))
}


fn impl_method(test_value: &str, body: &mut proc_macro2::TokenStream) -> Ident {
    let (_, method_name) = gen_method_name(&test_value);
    let method_impl = crate::func::impl_fn(&body.to_string(), &method_name.to_string(), &format!("\"{}\"", test_value), false, true);
    let method_string = method_impl.to_string();
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
    return method_name;
}

fn impl_if(x: &Element, body: &mut proc_macro2::TokenStream, methods: &mut proc_macro2::TokenStream, appends: proc_macro2::TokenStream, block_name: &str, format_char: char) {
    let test_value = x.attributes.get("test").expect(&format!("{} element must be have test field!", x.tag));
    let method_name = impl_method(test_value, body);
    if x.childs.len() != 0 {
        let if_tag_body = parse(&x.childs, methods, block_name, format_char);
        *body = quote! {
                              #body
                              if #method_name {
                                   sql.push_str(" ");
                                   #if_tag_body
                                   sql.push_str(" ");
                                   #appends
                              }
                          };
    }
}

fn impl_otherwise(x: &Element, body: &mut proc_macro2::TokenStream, methods: &mut proc_macro2::TokenStream, block_name: &str, format_char: char) {
    let child_body = parse(&x.childs, methods, block_name, format_char);
    *body = quote!(
                        #body
                        sql.push_str(" ");
                        #child_body
                        sql.push_str(" ");
                       );
}


fn impl_trim(prefix: &str, suffix: &str, prefixOverrides: &str, suffixOverrides: &str, x: &Element, body: &mut proc_macro2::TokenStream, arg: &Vec<Element>, methods: &mut proc_macro2::TokenStream, block_name: &str, format_char: char) {
    let trim_body = parse(&x.childs, methods, block_name, format_char);
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

pub fn impl_fn_html(m: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let fn_name = m.sig.ident.to_string();
    let mut file_name = args.get(0).to_token_stream().to_string();
    if file_name.ne("\"\"") && file_name.starts_with("\"") && file_name.ends_with("\"") {
        file_name = file_name[1..file_name.len() - 1].to_string();
    }
    let t;
    #[cfg(feature = "debug_mode")]
        {
            println!("try open file:{}", file_name);
        }
    let mut data = String::new();
    let mut f = File::open(file_name.as_str()).expect(&format!("File:\"{}\" does not exist", file_name));
    f.read_to_string(&mut data);


    let mut format_char = '?';
    if args.len() > 1 {
        for x in args.get(1).to_token_stream().to_string().chars() {
            if x != '\'' && x != '"' {
                format_char = x;
                break;
            }
        };
    }
    t = parse_html_str(&data, format_char, &fn_name);
    return t.into();
}

pub fn impl_fn_py(m: &ItemFn, args: &AttributeArgs) -> TokenStream {
    let fn_name = m.sig.ident.to_string();
    let mut data = args.get(0).to_token_stream().to_string();
    if data.ne("\"\"") && data.starts_with("\"") && data.ends_with("\"") {
        data = data[1..data.len() - 1].to_string();
    }
    let t;
    #[cfg(feature = "debug_mode")]
        {
            println!("py_sql:{}", data);
        }
    let mut format_char = '?';
    if args.len() > 1 {
        for x in args.get(1).to_token_stream().to_string().chars() {
            if x != '\'' && x != '"' {
                format_char = x;
                break;
            }
        };
    }
    let nodes = NodeType::parse(&data).expect("[rbatis] parse py_sql fail!");
    let htmls = crate::py_sql::to_html(&nodes, data.starts_with("select") || data.starts_with(" select"), &fn_name);
    #[cfg(feature = "debug_mode")]
        {
            println!("html:{}", htmls);
        }
    t = parse_html_str(&htmls, format_char, &fn_name);
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