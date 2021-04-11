use quote::quote;
use quote::ToTokens;
use syn;
use syn::{Expr, ItemFn, Member};
use crate::proc_macro::TokenStream;

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

pub(crate) fn impl_fn(f: &ItemFn, args: crate::proc_macro::TokenStream) -> TokenStream {
    let mut string_data = args.to_string();
    string_data = string_data[1..string_data.len() - 1].to_string();

    string_data = string_data.replace(".as_i32()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".as_i64()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".as_f64()", ".as_f64().unwrap_or(0.0)");
    string_data = string_data.replace(".as_str()", ".as_str().unwrap_or(\"\")");
    string_data = string_data.replace(".as_bool()", ".as_bool().unwrap_or(false)");


    string_data = string_data.replace(".i32()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".i64()", ".as_i64().unwrap_or(0)");
    string_data = string_data.replace(".f64()", ".as_f64().unwrap_or(0.0)");
    string_data = string_data.replace(".str()", ".as_str().unwrap_or(\"\")");
    string_data = string_data.replace(".bool()", ".as_bool().unwrap_or(false)");
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
        panic!("[rexpr]syn::parse_str fail for: {}", t.err().unwrap().to_string())
    } else {
        println!("[rexpr]parse expr:{} success!", string_data);
    }
    let mut t = t.unwrap();
    t = convert_to_arg_access(t);
    string_data = t.to_token_stream().to_string();
    string_data = string_data.replace(" . ", ".");

    //let s = syn::parse_str::<syn::LitStr>(&string_data).unwrap();
    println!("[rexpr]expr:{}", string_data);
    let t = syn::parse_str::<Expr>(&string_data);
    if t.is_err() {
        panic!("[rexpr]syn::parse_str fail for: {}", t.err().unwrap().to_string())
    } else {
        println!("[rexpr]parse expr:{} success!", string_data);
    }
    let t = t.unwrap();
    //t = convert_to_arg_access(t);

    println!("[rexpr]gen expr: {}", t.to_token_stream());
    let func_args = f.sig.inputs.to_token_stream();
    let func_name_ident = f.sig.ident.to_token_stream();
    let return_ty = f.sig.output.to_token_stream();
    return quote!(pub fn #func_name_ident(#func_args)  #return_ty {
                     use xmlsql::ops::AsProxy;
                     let result={#t};
                     return Ok(serde_json::json!(result));
                  })
        .to_token_stream().into();
}

fn convert_to_arg_access(arg: Expr) -> Expr {
    // println!("tk:{},expr:{}",expr_type(arg.clone()),arg.to_token_stream());
    match arg {
        Expr::Path(b) => {
            if b.to_token_stream().to_string().trim() == "null" {
                return syn::parse_str::<Expr>("&serde_json::Value::Null.as_proxy()").unwrap();
            }
            //println!("Path:{}", b.to_token_stream());
            return syn::parse_str::<Expr>(&format!("&arg[\"{}\"].as_proxy_clone()", b.to_token_stream().to_string().trim())).unwrap();
        }
        Expr::MethodCall(mut b) => {
            let ex = *(b.receiver.clone());
            let s = ex.to_token_stream().to_string();
            for x in s.chars() {
                if is_param_char(x) {
                    //println!("re:{},Type:{}", s, expr_type_box(&b.receiver));
                    b.receiver = Box::new(convert_to_arg_access(*b.receiver.clone()));
                    return Expr::MethodCall(b);
                }
                break;
            }
            return Expr::MethodCall(b);
        }
        Expr::Binary(mut b) => {
            b.left = Box::new(convert_to_arg_access(*b.left.clone()));
            b.right = Box::new(convert_to_arg_access(*b.right.clone()));
            return Expr::Binary(b);
        }
        Expr::Unary(mut b) => {
            b.expr = Box::new(convert_to_arg_access(*b.expr.clone()));
            if b.op.to_token_stream().to_string().trim() == "-" && b.expr.to_token_stream().to_string().trim().ends_with("as_proxy_clone()") {
                return syn::parse_str::<Expr>(&format!(" (0 {})", b.to_token_stream().to_string().trim())).unwrap();
            }
            return Expr::Unary(b);
        }
        Expr::Paren(mut b) => {
            b.expr = Box::new(convert_to_arg_access(*b.expr.clone()));
            return Expr::Paren(b);
        }
        Expr::Field(b) => {
            //println!("field:{}",b.to_token_stream());
            return match b.member.clone() {
                Member::Named(_) => {
                    let s = b.to_token_stream().to_string();
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
                    return syn::parse_str::<Expr>(&format!("&arg{}.as_proxy_clone()", token)).unwrap();
                }
                Member::Unnamed(unamed) => {
                    return Expr::Field(b);
                }
            };
        }
        Expr::Reference(mut b) => {
            b.expr = Box::new(convert_to_arg_access(*b.expr.clone()));
            let result = Expr::Reference(b);
            return result;
        }
        Expr::Index(mut b) => {
            b.expr = Box::new(convert_to_arg_access(*b.expr.clone()));
            let result = Expr::Index(b);
            //return result;
            //remove inner . as_proxy_clone(),keep  out . as_proxy_clone()
            return syn::parse_str::<Expr>(&format!("{}.as_proxy_clone()", result.to_token_stream().to_string().replace(". as_proxy_clone()", ""))).unwrap();
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