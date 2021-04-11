#![allow(unused_assignments)]
extern crate proc_macro;

use syn::{ItemFn};
use crate::proc_macro::TokenStream;

mod func;

#[proc_macro_attribute]
pub fn expr(args: TokenStream, func: TokenStream) -> TokenStream {
    //let args = parse_macro_input!(args as AttributeArgs);
    let target_fn: ItemFn = syn::parse(func).unwrap();
    let stream = func::impl_fn(&target_fn, args);
    #[cfg(feature = "debug_mode")]
        {
            println!("............gen macro rexpr:\n {}", stream);
            println!("............gen macro rexpr end............");
        }
    stream
}
