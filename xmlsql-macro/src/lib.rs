#![allow(unused_assignments)]
extern crate proc_macro;

use syn::{ItemFn, DataStruct};
use crate::proc_macro::TokenStream;

mod func;
mod parser;
mod xml_loader;

#[proc_macro_attribute]
pub fn expr(args: TokenStream, func: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = func::impl_fn("",&target_fn.sig.ident.to_string(), &args.to_string()).into();
    #[cfg(feature = "debug_mode")]
        {
            println!("............gen macro rexpr:\n {}", stream);
            println!("............gen macro rexpr end............");
        }
    stream
}


#[proc_macro_attribute]
pub fn xml(args: TokenStream, func: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(args as AttributeArgs);
    let target_fn = syn::parse(func).unwrap();
    let stream = parser::impl_fn(&target_fn, args);
    #[cfg(feature = "debug_mode")]
        {
            println!("............gen macro xml:\n {}", stream);
            println!("............gen macro xml end............");
        }
    stream
}