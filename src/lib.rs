pub mod xmls;
pub mod error;
#[macro_use]
pub mod bencher;
pub mod ops;
pub use ops::Value;


pub mod ops_eq;
pub mod ops_cmp;



pub mod ops_add;
pub mod ops_div;
pub mod ops_mul;
pub mod ops_sub;
pub mod ops_rem;


#[macro_use]
extern crate xmlsql_macro;

pub use xmlsql_macro::{expr};