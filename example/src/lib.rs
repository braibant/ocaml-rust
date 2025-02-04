use ocaml_rust::closure::{Fn0, Fn1};

fn option_result(v: Option<isize>, e: String) -> Result<isize, String> {
    match v {
        Some(v) => Ok(v),
        None => Err(e),
    }
}

fn add_one(x: isize) -> isize {
    x + 1
}

fn add_i64(x: i64, y: i64) -> i64 {
    x + y
}

fn str_format(x: (isize, isize), y: String) -> String {
    format!("foo<{}|{}>: {}", x.0, x.1, y)
}

fn pair(xy: (String, f64, (isize, isize))) -> String {
    let (x, y, (a, b)) = xy;
    format!("{x}:{y}:{a}:{b}")
}
fn vec_add(x: Vec<isize>, y: isize) -> Vec<isize> {
    x.iter().map(|x| x + y).collect()
}

#[ocaml_rust::bridge]
mod ffi {
    extern "Rust" {
        fn add_one(x: isize) -> isize;
        fn add_i64(x: i64, y: i64) -> i64;
        fn str_format(x: (isize, isize), y: String) -> String;
        fn pair(xy: (String, f64, (isize, isize))) -> String;
        fn option_result(v: Option<isize>, e: String) -> Result<isize, String>;
        fn vec_add(x: Vec<isize>, y: isize) -> Vec<isize>;
    }
}

fn vec_new() -> MyVec {
    Vec::new()
}

fn vec_push(v: &mut MyVec, x: isize) {
    v.push(x as i64);
}

fn vec_content(v: &MyVec) -> Vec<i64> {
    v.clone()
}

#[ocaml_rust::bridge]
mod ffi2 {
    type MyVec = Vec<i64>;

    extern "Rust" {
        fn vec_new() -> MyVec;
        fn vec_push(vec: &mut MyVec, v: isize);
        fn vec_content(vec: &MyVec) -> Vec<i64>;
    }
}

#[ocaml_rust::bridge]
mod ffi3 {
    #[derive(Debug, Clone)]
    enum MyEnum {
        NoArg,
        OneArg(isize),
        TwoArgs(isize, String),
        StructArgs { x: isize, y: String },
        // Rec(Box<MyEnum>),
    }

    #[derive(Debug, Clone)]
    struct MyStruct {
        x: isize,
        y: String,
        z: (isize, Option<String>, f64),
        zs: Vec<f64>,
    }

    extern "Rust" {
        fn mystruct_to_string(v: &MyStruct) -> String;
        fn mystruct_add_x(v: &MyStruct, x: isize) -> MyStruct;
        fn myenum_to_string(v: &MyEnum) -> String;
        fn myenum_add_x(m: &MyEnum, v: isize) -> MyEnum;
    }
}

fn mystruct_to_string(v: &MyStruct) -> String {
    format!("{:?}", v)
}

fn mystruct_add_x(v: &MyStruct, x: isize) -> MyStruct {
    let mut v = v.clone();
    v.x += x;
    v
}

fn myenum_to_string(v: &MyEnum) -> String {
    format!("{:?}", v)
}

fn myenum_add_x(m: &MyEnum, v: isize) -> MyEnum {
    match m {
        MyEnum::NoArg => MyEnum::NoArg,
        MyEnum::OneArg(x) => MyEnum::OneArg(x + v),
        MyEnum::TwoArgs(x, s) => MyEnum::TwoArgs(x + v, s.to_string()),
        MyEnum::StructArgs { x, y } => MyEnum::StructArgs { x: x + v, y: y.to_string() },
    }
}

#[ocaml_rust::bridge]
mod ffi4 {
    extern "Rust" {
        fn map_callback(vs: &Vec<isize>, f: &mut Fn1<isize, String>) -> Vec<String>;

        fn sum_n(n: isize, f: &mut Fn0<isize>) -> isize;
    }
}

fn map_callback(vs: &Vec<isize>, f: &mut Fn1<isize, String>) -> Vec<String> {
    vs.iter().map(|x| f.call1(*x)).collect()
}

fn sum_n(n: isize, f: &mut Fn0<isize>) -> isize {
    (0..n).map(|_x| f.call0()).sum()
}
