#[macro_use]
extern crate xmlsql;

use serde_json::json;

#[expr("a == 1")]
pub fn gen(arg: &serde_json::Value) -> serde_json::Value {}


fn main() {
    let arg = serde_json::json!({
        "a":1,
        "b":2,
        "c":"c",
        "d":null,
        "e":[1],
        "f":[{"field":1}]
    });
    let v = gen(&arg);
    println!("{}", v);
}


#[test]
fn bench() {
    let arg = serde_json::json!({
        "a":1,
        "b":2,
        "c":"c",
        "d":null,
        "e":[1],
        "f":[{"field":1}]
    });
    gen(&arg);
    bench!(100000,{
       gen(&arg);
    });
}


#[cfg(test)]
mod test {
    use serde_json::json;

    #[macro_use]
    use xmlsql;
    use xmlsql::ops::AsProxy;

    #[test]
    fn test_node_run() {
        let arg = json!({
        "a":1,
        "b":2,
        "c":"c",
        "d":null,
        "e":[1],
        "f":[{"field":1}]
         });
        macro_rules! call {
            ($func_name:ident,$s:expr,$value:expr) => {
                #[expr($s)]
                pub fn $func_name(arg: &serde_json::Value) -> serde_json::Value {}
                     assert_eq!($func_name(&arg), $value);
                };
        }
        call!(fn1,"-1 == -a", json!(true));
        call!(fn2,"d.a.is_null()", json!(true));
        call!(fn3,"1.0 == 1.0", json!(true));
        call!(fn4,"'2019-02-26' == '2019-02-26'", json!(true));
        call!(fn5,"'f\'uc'.string()+'k'", json!("f'uck"));
        call!(fn6,"'f'.string()+'s'",json!("fs"));
        call!(fn7,"a +1 > b * 8",json!(false));
        call!(fn8,"a >= 0",json!(true));
        call!(fn9,"'a'+c",json!("ac"));
        call!(fn10,"'a'+c", json!("ac"));
        call!(fn11,"b", json!(2));
        call!(fn12,"a < 1", json!(false));
        call!(fn13,"a +1 > b*8", json!(false));
        call!(fn14,"a * b == 2", json!(true));
        call!(fn15,"a - b == 0", json!(false));
        call!(fn16,"a >= 0 && a != 0", json!(true));
        call!(fn17,"a == 1 && a != 0", json!(true));
        call!(fn18,"1 > 3 ", json!(false));
        call!(fn19,"1 + 2 != null", json!(true));
        call!(fn20,"1 != null", json!(true));
        call!(fn21,"1 + 2 != null && 1 > 0 ", json!(true));
        call!(fn22,"1 + 2 != null && 2 < b*8 ", json!(true));
        call!(fn23,"-1 != null", json!(true));
        call!(fn24,"-1 != -2 && -1 == 2-3 ", json!(true));
        call!(fn25,"-3 == b*-1-1 ", json!(true));
        call!(fn26,"0-1 + a*0-1 ", json!(-2));
        call!(fn28,"0-1 + -1*0-1 ", json!(-2));
        call!(fn29,"1-0", json!(1));
        call!(fn30,"-1", json!(-1));
        call!(fn31,"1- -1", json!(1 - -1));
        call!(fn32,"1-2 -1+0", json!(1 - 2 - 1));
        call!(fn33,"e[1]", json!(null));
        call!(fn34,"e[0]", json!(1));
        call!(fn35,"f[0].field", json!(1));
        call!(fn37,"0.1", json!(0.1));
        call!(fn38,"1", json!(1));
        call!(fn39,"(1+1)", json!(2));
        call!(fn40,"(1+5)>5", json!((1 + 5) > 5));
        call!(fn41,"(18*19)<19*19", json!((18 * 19) < 19 * 19));
        call!(fn42,"2*(1+1)", json!(2 * (1 + 1)));
        call!(fn43, "2*(1+(1+1)+1)",json!(2 * (1 + (1 + 1) + 1)));
        call!(fn44, "(((34 + 21) / 5) - 12) * 348",json!((((34 + 21) / 5) - 12) * 348));
        call!(fn45,"11 ^ 1", json!(11 ^ 1));
        call!(fn46,"e[0] != nil", json!(true));
        call!(fn47,"null >= 0", json!(true));
        call!(fn48,"null <= a", json!(true));
        call!(fn49,"nil >= 0", json!(true));
        call!(fn50,"nil <= a", json!(true));
    }
}