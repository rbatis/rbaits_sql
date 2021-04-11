#[macro_use]
extern crate xmlsql;

use serde_json::json;


#[expr("(a.d+1)")]
pub fn gen(arg: &serde_json::Value) -> xmlsql::error::Result<serde_json::Value> {}


fn main() {
    let arg = serde_json::json!({
          "a":{
            "arr": [1,2,3],
            "b": "8",
            "c": true,
            "d": 1,
        },
        "e":1
    });
    let v = gen(&arg);
    println!("{}", v.unwrap());
}


#[test]
fn bench() {
    let arg = serde_json::json!({
        "a":{
            "arr":[1,2,3],
            "b":"8",
            "c":true
        }
    });
    gen(&arg);
    bench!(100000,{
       gen(&arg);
    });
}


// #[test]
// fn test_node_run() {
//     let runtime = RExprRuntime::new();
//     let arg = json!({
//         "a":1,
//         "b":2,
//         "c":"c",
//         "d":null,
//         "e":[1],
//         "f":[{"field":1}]
//          });
//     #[expr("-1 == -a.i32()")]
//     pub fn fff(arg: &serde_json::Value) -> rexpr::error::Result<serde_json::Value> {}
//     assert_eq!(fff(&arg).unwrap(), json!(true));
//     // assert_eq!(exec_expr(&arg, "d.a == null"), json!(true));
//     #[expr("d.a == null")]
//     pub fn fff2(arg: &serde_json::Value) -> rexpr::error::Result<serde_json::Value> {}
//     assert_eq!(fff2(&arg).unwrap(), json!(true));
//     // assert_eq!(exec_expr(&arg, "1 == 1.0"), json!(true));
//     #[expr("1.0 == 1.0")]
//     pub fn fff3(arg: &serde_json::Value) -> rexpr::error::Result<serde_json::Value> {}
//     assert_eq!(fff3(&arg).unwrap(), json!(true));
//
//     // assert_eq!(exec_expr(&arg, "'2019-02-26' == '2019-02-26'"), json!(true));
//     // assert_eq!(exec_expr(&arg, "`f`+`s`"), json!("fs"));
//     // assert_eq!(exec_expr(&arg, "a +1 > b * 8"), json!(false));
//     // assert_eq!(exec_expr(&arg, "a >= 0"), json!(true));
//     // assert_eq!(exec_expr(&arg, "'a'+c"), json!("ac"));
//     // assert_eq!(exec_expr(&arg, "b"), json!(2));
//     // assert_eq!(exec_expr(&arg, "a < 1"), json!(false));
//     // assert_eq!(exec_expr(&arg, "a +1 > b*8"), json!(false));
//     // assert_eq!(exec_expr(&arg, "a * b == 2"), json!(true));
//     // assert_eq!(exec_expr(&arg, "a - b == 0"), json!(false));
//     // assert_eq!(exec_expr(&arg, "a >= 0 && a != 0"), json!(true));
//     // assert_eq!(exec_expr(&arg, "a == 1 && a != 0"), json!(true));
//     // assert_eq!(exec_expr(&arg, "1 > 3 "), json!(false));
//     // assert_eq!(exec_expr(&arg, "1 + 2 != null"), json!(true));
//     // assert_eq!(exec_expr(&arg, "1 != null"), json!(true));
//     // assert_eq!(exec_expr(&arg, "1 + 2 != null && 1 > 0 "), json!(true));
//     // assert_eq!(exec_expr(&arg, "1 + 2 != null && 2 < b*8 "), json!(true));
//     // assert_eq!(exec_expr(&arg, "-1 != null"), json!(true));
//     // assert_eq!(exec_expr(&arg, "-1 != -2 && -1 == 2-3 "), json!(true));
//     // assert_eq!(exec_expr(&arg, "-3 == b*-1-1 "), json!(true));
//     // assert_eq!(exec_expr(&arg, "0-1 + a*0-1 "), json!(-2));
//     // assert_eq!(exec_expr(&arg, "2 ** 3"), json!(8.0));
//     // assert_eq!(exec_expr(&arg, "0-1 + -1*0-1 "), json!(-2));
//     // assert_eq!(exec_expr(&arg, "1-"), json!(1));
//     // assert_eq!(exec_expr(&arg, "-1"), json!(-1));
//     // assert_eq!(exec_expr(&arg, "1- -1"), json!(1 - -1));
//     // assert_eq!(exec_expr(&arg, "1-2 -1+"), json!(1 - 2 - 1));
//     // assert_eq!(exec_expr(&arg, "e[1]"), json!(null));
//     // assert_eq!(exec_expr(&arg, "e[0]"), json!(1));
//     // assert_eq!(exec_expr(&arg, "f[0].field"), json!(1));
//     // assert_eq!(exec_expr(&arg, "f.0.field"), json!(1));
//     // assert_eq!(exec_expr(&arg, "0.1"), json!(0.1));
//     // assert_eq!(exec_expr(&arg, "1"), json!(1));
//     // assert_eq!(exec_expr(&arg, "(1+1)"), json!(2));
//     // assert_eq!(exec_expr(&arg, "(1+5)>5"), json!((1 + 5) > 5));
//     // assert_eq!(exec_expr(&arg, "(18*19)<19*19"), json!((18 * 19) < 19 * 19));
//     // assert_eq!(exec_expr(&arg, "2*(1+1)"), json!(2 * (1 + 1)));
//     // assert_eq!(
//     //     exec_expr(&arg, "2*(1+(1+1)+1)"),
//     //     json!(2 * (1 + (1 + 1) + 1))
//     // );
//     // assert_eq!(
//     //     exec_expr(&arg, "(((34 + 21) / 5) - 12) * 348"),
//     //     json!((((34 + 21) / 5) - 12) * 348)
//     // );
//     // assert_eq!(exec_expr(&arg, "11 ^ 1"), json!(11 ^ 1));
//     // assert_eq!(exec_expr(&arg, "null ^ null"), json!(0 ^ 0));
//     // assert_eq!(exec_expr(&arg, "null >= 0"), json!(true));
//     // assert_eq!(exec_expr(&arg, "null <= a"), json!(true));
// }