#![allow(unreachable_patterns)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_must_use)]
#![allow(dead_code)]
#![allow(unused_attributes)]
#![allow(unused_imports)]

#[macro_use]
extern crate rbatis_sql;

use rbatis_sql::ops::{AsProxy, OpsIndex};
use std::sync::Arc;

pub struct BizActivity {
    pub id: Option<String>,
    pub name: Option<String>,
    pub pc_link: Option<String>,
    pub h5_link: Option<String>,
    pub remark: Option<String>,
    pub sort: Option<i32>,
    pub status: Option<i32>,
    pub version: Option<i32>,
    pub create_time: Option<String>,
    pub delete_flag: Option<i32>,
}
//
//
// #[rb_html("example/example.html",'$')]
// pub fn select_by_condition(arg: &mut bson::Bson) {}

#[rb_py("SELECT * FROM biz_activity
    SELECT * FROM biz_activity
    if  name != null:
      AND delete_flag = #{del}
      AND version = 1
      if  age!=1:
        AND version = 1
      AND version = 1
    AND a = 0
      yes
    for item in ids:
      #{item}
    for index,item in ids:
      #{item}
    trim 'AND':
      AND delete_flag = #{del2}
    choose:
        when age==27:
          AND age = 27
        otherwise:
          AND age = 0
    WHERE id  = '2';",'$')]
pub fn py_select_by_condition(arg: &mut bson::Bson) {}



// #[expr("a+b*(e[0]+b)/2")]
// pub fn gen(arg: &serde_json::Value) -> serde_json::Value {}
fn main() {
    let mut arg = bson2::bson!({
        "id":1,
        "order_by":["id","name"],
        "ids":[1,2,3],
        "name":"asdf",
        "map":{"a":1},
        "create_time":"2020-23-23"
    });
    let act = BizActivity {
        id: None,
        name: None,
        pc_link: None,
        h5_link: None,
        remark: None,
        sort: None,
        status: None,
        version: None,
        create_time: None,
        delete_flag: None,
    };
    //
    let (sql, args) = py_select_by_condition(&mut arg);
    println!("py->sql: {}", sql);
    println!("py->args: {}", serde_json::to_string(&args).unwrap());
    // let (sql, args) = select_by_condition(&mut arg);
    // println!("sql: {}", sql);
    // println!("args: {}", serde_json::to_string(&args).unwrap());
}

#[test]
fn bench() {
    let mut arg = bson::bson!({
        "id":1,
        "order_by":["id","name"],
        "ids":[1,2,3],
        "name":"asdf",
        "map":{"a":1},
        "create_time":"2020-23-23"
    });
    let act = BizActivity {
        id: None,
        name: None,
        pc_link: None,
        h5_link: None,
        remark: None,
        sort: None,
        status: None,
        version: None,
        create_time: None,
        delete_flag: None,
    };
    let (sql, args) = select_by_condition(&mut arg);
    println!("sql: {}", sql);
    println!("args: {}", serde_json::to_string(&args).unwrap());
    bench!(1000000,{
        select_by_condition(&mut arg);
    });
}


#[cfg(test)]
mod test {
    #[macro_use]
    use rbatis_sql;
    use rbatis_sql::error::Error;
    use serde::de::DeserializeOwned;

    use rbatis_sql::ops::*;


    // #[rb_html("example/example.html")]
    // pub fn select_by_condition(arg: &serde_json::Value) {}
    //
    // #[rb_html("example/example.html")]
    // pub fn insert(arg: &serde_json::Value) {}

    // pub struct B {}
    //
    // use async_trait::async_trait;
    //
    // impl B {
    //     async fn exec_prepare(&self, context_id: &str, sql: &str, args: &Vec<Value>) -> Result<(), Error> {
    //         println!("sql:{}", sql);
    //         println!("args:{:?}", args);
    //         Ok(())
    //     }
    // }
    //
    // #[test]
    // fn test_backend() {
    //     let b = B {};
    //     let mut arg = Value::Boolean({
    //     "id":1,
    //     "order_by":["id","name"],
    //     "ids":[1,2,3],
    //     "name":"asdf",
    //     "map":{"a":1},
    //     "create_time":"2020-23-23"
    //     });
    //
    //     // let v = gen(&arg);
    //     // println!("{}", v);
    //     // xml(&arg);
    //     let (sql, args) = select_by_condition(&mut arg);
    //     async_std::task::block_on(async {
    //         b.exec_prepare("", &sql, &args).await.unwrap();
    //     });
    // }


    #[test]
    fn test_node_run() {
        let arg = bson::bson!({
        "a":1,
        "b":2,
        "c":"c",
        "d":null,
        "e":[1],
        "f":[{"field":1}],
        "g":true
        });
        macro_rules! call {
            ($func_name:ident,$s:expr,$value:expr) => {
                #[expr($s)]
                pub fn $func_name(arg: &bson::Bson) -> bson::Bson{}
                assert_eq!($func_name(&arg), $value);
                };
        }
        call!(fn1,"-1 == -a", Value::Boolean(true));
        call!(fn2,"d.a.is_null()", Value::Boolean(true));
        call!(fn3,"1.0 == 1.0", Value::Boolean(true));
        call!(fn4,"'2019-02-26' == '2019-02-26'", Value::Boolean(true));
        call!(fn5,"'f\'uc'+'k'", Value::String("f'uck".to_string()));
        call!(fn6,"'f'+'s'",Value::String("fs".to_string()));
        call!(fn7,"a +1 > b * 8",Value::Boolean(false));
        call!(fn8,"a >= 0",Value::Boolean(true));
        call!(fn9,"'a'+c",Value::String("ac".to_string()));
        call!(fn10,"'a'+c", Value::String("ac".to_string()));
        call!(fn11,"b", bson::bson!(2));
        call!(fn12,"a < 1", Value::Boolean(false));
        call!(fn13,"a +1 > b*8", Value::Boolean(false));
        call!(fn14,"a * b == 2", Value::Boolean(true));
        call!(fn15,"a - b == 0", Value::Boolean(false));
        call!(fn16,"a >= 0 && a != 0", Value::Boolean(true));
        call!(fn17,"a == 1 && a != 0", Value::Boolean(true));
        call!(fn18,"1 > 3 ", Value::Boolean(false));
        call!(fn19,"1 + 2 != null", Value::Boolean(true));
        call!(fn20,"1 != null", Value::Boolean(true));
        call!(fn21,"1 + 2 != null && 1 > 0 ", Value::Boolean(true));
        call!(fn22,"1 + 2 != null && 2 < b*8 ", Value::Boolean(true));
        call!(fn23,"-1 != null", Value::Boolean(true));
        call!(fn24,"-1 != -2 && -1 == 2-3 ", Value::Boolean(true));
        call!(fn25,"-3 == b*-1-1 ", Value::Boolean(true));
        call!(fn26,"0-1 + a*0-1 ", Value::Int64(-2));
        call!(fn28,"0-1 + -1*0-1 ", Value::Int64(-2));
        call!(fn29,"1-0", Value::Int64(1));
        call!(fn30,"-1", Value::Int64(-1));
        call!(fn31,"1- -1", Value::Int64(1 - -1));
        call!(fn32,"1-2 -1+0", Value::Int64(1 - 2 - 1));
        call!(fn33,"e[1]", Value::Null);
        call!(fn34,"e[0]", bson::bson!(1));
        call!(fn35,"f[0].field", bson::bson!(1));
        call!(fn37,"0.1", Value::Double(0.1));
        call!(fn38,"1", Value::Int64(1));
        call!(fn39,"(1+1)", Value::Int64(2));
        call!(fn40,"(1+5)>5", Value::Boolean((1 + 5) > 5));
        call!(fn41,"(18*19)<19*19", Value::Boolean((18 * 19) < 19 * 19));
        call!(fn42,"2*(1+1)", Value::Int64(2 * (1 + 1)));
        call!(fn43, "2*(1+(1+1)+1)",Value::Int64(2 * (1 + (1 + 1) + 1)));
        call!(fn44, "(((34 + 21) / 5) - 12) * 348",Value::Int64((((34 + 21) / 5) - 12) * 348));
        call!(fn45,"11 ^ 1", Value::Int64(11 ^ 1));
        call!(fn46,"e[0] != null", Value::Boolean(true));
        call!(fn47,"null >= 0", Value::Boolean(true));
        call!(fn48,"null <= a", Value::Boolean(true));
        call!(fn49,"null >= 0", Value::Boolean(true));
        call!(fn50,"null <= a", Value::Boolean(true));
        call!(fn51,"a == 1 && g", Value::Boolean(true));
        call!(fn52,"1+2", Value::Int64(3));
        call!(fn53,"1+a", Value::Int64(2));
        call!(fn54,"'c'+c", Value::String("cc".to_string()));
        call!(fn55,"c+'c'", Value::String("cc".to_string()));
    }

    #[expr("a+b*(e[0]+b)/2")]
    pub fn gen(arg: &bson::Bson) -> bson::Bson {}

    #[test]
    fn bench() {
        let arg = bson::bson!({
        "a":1,
        "b":2,
        "c":"c",
        "d":null,
        "e":[1],
        "f":[{"field":1}],
        "g":true
        });
        gen(&arg);
        bench!(100000,{
          gen(&arg);
        });
    }

    // #[rb_html("example/example.html",'$')]
    // pub fn test_include(arg: &serde_json::Value) {}
    //
    // #[test]
    // fn test_include_file(){
    //     let mut arg = Value::Boolean({
    //     });
    //     let (sql, args) = test_include(&mut arg);
    //     println!("py->sql: {}", sql);
    //     println!("py->args: {}", serde_json::to_string(&args).unwrap());
    // }
}