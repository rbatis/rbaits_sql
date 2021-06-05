use quote::quote;
use quote::ToTokens;
use syn;
use syn::{BinOp, Expr, ItemFn, Lit, Member};

use crate::proc_macro::TokenStream;
use proc_macro2::{Ident, Span};

// fn is_name_char(arg: char) -> bool {
//     match arg {
//         '.' |
//         '_' |
//         '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' |
//         'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
//         'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z'
//         => {
//             return true;
//         }
//         _ => {}
//     }
//     return false;
// }

fn is_param_char(arg: char) -> bool {
    match arg {
        'a' | 'b' | 'c' | 'd' | 'e' | 'f' | 'g' | 'h' | 'i' | 'j' | 'k' | 'l' | 'm' | 'n' | 'o' | 'p' | 'q' | 'r' | 's' | 't' | 'u' | 'v' | 'w' | 'x' | 'y' | 'z' |
        'A' | 'B' | 'C' | 'D' | 'E' | 'F' | 'G' | 'H' | 'I' | 'J' | 'K' | 'L' | 'M' | 'N' | 'O' | 'P' | 'Q' | 'R' | 'S' | 'T' | 'U' | 'V' | 'W' | 'X' | 'Y' | 'Z'
        => {
            return true;
        }
        _ => {}
    }
    return false;
}


fn token_steam_string(arg: proc_macro2::TokenStream) -> String {
    arg.to_token_stream().to_string().trim().to_string()
}


fn convert_to_arg_access(context: &str, arg: Expr, as_proxy: bool) -> Expr {
    match arg {
        Expr::Path(b) => {
            let token = b.to_token_stream().to_string();
            if token == "null" {
                return syn::parse_str::<Expr>("serde_json::Value::Null.into_proxy()").unwrap();
            }
            if token == "sql" {
                return Expr::Path(b);
            }
            if token == "arg" {
                return Expr::Path(b);
            }
            if token == "args" {
                return Expr::Path(b);
            }

            let param = token_steam_string(b.to_token_stream());

            if as_proxy {
                if context.contains(&format!("let {} =", param)) {
                    return syn::parse_str::<Expr>(&format!("{}.as_proxy()", param)).unwrap();
                }
                return syn::parse_str::<Expr>(&format!("(&arg)[\"{}\"].as_proxy()", param)).unwrap();
            } else {
                if context.contains(&format!("let {} =", param)) {
                    return syn::parse_str::<Expr>(&format!("{}", param)).unwrap();
                }
                return syn::parse_str::<Expr>(&format!("(&arg)[\"{}\"].as_proxy()", param)).unwrap();
            }
        }
        Expr::MethodCall(mut b) => {
            let ex = *(b.receiver.clone());
            let s = ex.to_token_stream().to_string();
            for x in s.chars() {
                if is_param_char(x) {
                    b.receiver = Box::new(convert_to_arg_access(context, *b.receiver.clone(), as_proxy));
                    return Expr::MethodCall(b);
                }
                break;
            }
            return Expr::MethodCall(b);
        }
        Expr::Binary(mut b) => {
            b.left = Box::new(convert_to_arg_access(context, *b.left.clone(), as_proxy));
            b.right = Box::new(convert_to_arg_access(context, *b.right.clone(), as_proxy));
            match b.op {
                BinOp::And(_) => {
                    b.left = Box::new(syn::parse_str::<Expr>(&format!("bool::from({})", b.left.to_token_stream().to_string().trim())).unwrap());
                    b.right = Box::new(syn::parse_str::<Expr>(&format!("bool::from({})", b.right.to_token_stream().to_string().trim())).unwrap());
                }
                BinOp::Or(_) => {
                    b.left = Box::new(syn::parse_str::<Expr>(&format!("bool::from({})", b.left.to_token_stream().to_string().trim())).unwrap());
                    b.right = Box::new(syn::parse_str::<Expr>(&format!("bool::from({})", b.right.to_token_stream().to_string().trim())).unwrap());
                }
                BinOp::Add(_) => {
                    let left_token = b.left.to_token_stream().to_string();
                    if left_token.trim().ends_with("\"") && left_token.trim().starts_with("\"") {
                        b.left = Box::new(syn::parse_str::<Expr>(&format!("String::from({})", b.left.to_token_stream().to_string().trim())).unwrap());
                    }
                }
                _ => {}
            }
            return Expr::Binary(b);
        }
        Expr::Unary(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr.clone(), as_proxy));
            if b.op.to_token_stream().to_string().trim() == "-" {
                return syn::parse_str::<Expr>(&format!(" (0 {})", b.to_token_stream().to_string().trim())).unwrap();
            }
            return Expr::Unary(b);
        }
        Expr::Paren(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr.clone(), as_proxy));
            return Expr::Paren(b);
        }
        Expr::Field(mut b) => {
            b.base = Box::new(convert_to_arg_access(context, *b.base.clone(), as_proxy));
            b.base = Box::new(syn::parse_str::<Expr>(&b.base.to_token_stream().to_string().trim().trim_end_matches(" . as_proxy()")).unwrap());
            return match b.member.clone() {
                Member::Named(n) => {
                    let s = b.member.to_token_stream().to_string();
                    let vs: Vec<&str> = s.split(".").collect();
                    let mut token = String::new();
                    for x in vs {
                        if x.ends_with("()") {
                            token.push_str(".");
                            token.push_str(x.trim());
                        } else {
                            let x = x.trim();
                            //format index
                            let xs: Vec<&str> = x.split("[").collect();
                            if xs.len() > 1 {
                                let mut is_first = true;
                                for x in xs {
                                    if is_first {
                                        token.push_str("[\"");
                                        token.push_str(x.trim());
                                        token.push_str("\"]");
                                    } else {
                                        token.push_str("[");
                                        token.push_str(x.trim());
                                    }
                                    is_first = false;
                                }
                            } else {
                                token.push_str("[\"");
                                token.push_str(x.trim());
                                token.push_str("\"]");
                            }
                        }
                    }
                    if as_proxy {
                        syn::parse_str::<Expr>(&format!("{}{}.as_proxy()", b.base.to_token_stream(), token)).unwrap()
                    } else {
                        syn::parse_str::<Expr>(&format!("{}{}", b.base.to_token_stream(), token)).unwrap()
                    }
                }
                Member::Unnamed(unamed) => {
                    Expr::Field(b)
                }
            };
        }
        Expr::Reference(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr.clone(), as_proxy));
            let result = Expr::Reference(b);
            return result;
        }
        Expr::Index(mut b) => {
            b.expr = Box::new(convert_to_arg_access(context, *b.expr.clone(), as_proxy));
            let result = Expr::Index(b);
            //return result;
            //remove inner . as_proxy(),keep  out . as_proxy()

            if as_proxy {
                return syn::parse_str::<Expr>(&format!("{}.as_proxy()", result.to_token_stream().to_string().replace(". as_proxy()", ""))).unwrap();
            } else {
                return syn::parse_str::<Expr>(&format!("{}", result.to_token_stream().to_string().replace(". as_proxy()", ""))).unwrap();
            }
        }
        Expr::Lit(mut b) => {
            match b.lit.clone() {
                Lit::Str(_) => {}
                Lit::ByteStr(_) => {}
                Lit::Byte(_) => {}
                Lit::Char(_) => {}
                Lit::Int(i) => {
                    //cast int to i64
                    return syn::parse_str::<Expr>(&format!("{}i64", i)).unwrap();
                }
                Lit::Float(f) => {
                    //cast int to f64
                    return syn::parse_str::<Expr>(&format!("{}f64", f)).unwrap();
                }
                Lit::Bool(_) => {}
                Lit::Verbatim(_) => {}
            }
            return Expr::Lit(b);
        }
        _ => {
            println!("_def:{:?}", expr_type(arg.clone()));
            return arg;
        }
    }
}

fn expr_type_box(expr: &Box<Expr>) -> String {
    expr_type(*expr.clone())
}

fn expr_type(expr: Expr) -> String {
    match expr {
        Expr::Array(_) => { format!("Array") }
        Expr::Assign(_) => { format!("Assign") }
        Expr::AssignOp(_) => { format!("AssignOp") }
        Expr::Async(_) => { format!("Async") }
        Expr::Await(_) => { format!("Await") }
        Expr::Binary(_) => { format!("Binary") }
        Expr::Block(_) => { format!("Block") }
        Expr::Box(_) => { format!("Box") }
        Expr::Break(_) => { format!("Break") }
        Expr::Call(_) => { format!("Call") }
        Expr::Cast(_) => { format!("Cast") }
        Expr::Closure(_) => { format!("Closure") }
        Expr::Continue(_) => { format!("Continue") }
        Expr::Field(_) => { format!("Field") }
        Expr::ForLoop(_) => { format!("ForLoop") }
        Expr::Group(_) => { format!("Group") }
        Expr::If(_) => { format!("If") }
        Expr::Index(_) => { format!("Index") }
        Expr::Let(_) => { format!("Let") }
        Expr::Lit(_) => { format!("Lit") }
        Expr::Loop(_) => { format!("Loop") }
        Expr::Macro(_) => { format!("Macro") }
        Expr::Match(_) => { format!("Match") }
        Expr::MethodCall(_) => { format!("MethodCall") }
        Expr::Paren(_) => { format!("Paren") }
        Expr::Path(_) => { format!("Path") }
        Expr::Range(_) => { format!("Range") }
        Expr::Reference(_) => { format!("Reference") }
        Expr::Repeat(_) => { format!("Repeat") }
        Expr::Return(_) => { format!("Return") }
        Expr::Struct(_) => { format!("Struct") }
        Expr::Try(_) => { format!("Try") }
        Expr::TryBlock(_) => { format!("TryBlock") }
        Expr::Tuple(_) => { format!("Tuple") }
        Expr::Type(_) => { format!("Type") }
        Expr::Unary(_) => { format!("Unary") }
        Expr::Unsafe(_) => { format!("Unsafe") }
        Expr::Verbatim(_) => { format!("Verbatim") }
        Expr::While(_) => { format!("While") }
        Expr::Yield(_) => { format!("Yield") }
        Expr::__TestExhaustive(_) => { format!("__TestExhaustive") }
    }
}


pub(crate) fn impl_fn(context: &str, func_name_ident: &str, args: &str, serialize_result: bool, as_proxy: bool) -> proc_macro2::TokenStream {
    let mut string_data = args.to_string();
    string_data = string_data[1..string_data.len() - 1].to_string();
    string_data = string_data.replace(".string()", ".to_string()");
    //convert string define
    let mut last_char = '_';
    let mut string_data_new = String::new();
    for x in string_data.chars() {
        if x == '\'' && last_char != '\\' {
            string_data_new.push('\"');
        } else {
            string_data_new.push(x);
        }
        last_char = x;
    }
    string_data = string_data_new;
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str: {} fail for: {}", string_data, t.err().unwrap().to_string())
    }
    let mut t = t.unwrap();
    t = convert_to_arg_access(context, t, as_proxy);
    string_data = t.to_token_stream().to_string();
    string_data = string_data.replace(" . ", ".");
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str: {} fail for: {}", string_data, t.err().unwrap().to_string())
    }
    let t = t.unwrap();
    let mut result_impl = quote! { result };
    if serialize_result {
        result_impl = quote! {serde_json::json!(result)};
    }
    if func_name_ident.is_empty() || func_name_ident.eq("\"\""){
        return quote! {
         {
           use rbatis_sql::ops::AsProxy;
           let result={#t};
           #result_impl
        }
    }.to_token_stream();
    }else{
        let func_name_ident = Ident::new(&func_name_ident.to_string(), Span::call_site());
        return quote! {
        pub fn #func_name_ident(arg:&serde_json::Value) -> serde_json::Value {
           use rbatis_sql::ops::AsProxy;
           let result={#t};
           #result_impl
        }
    }.to_token_stream();
    }
}