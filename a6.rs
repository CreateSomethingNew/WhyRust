use std::collections::HashMap;

enum ExprC {
    NumC { n : u32 },
    BoolC { b : bool },
    StringC { s : String },
    IfC { i : Box<ExprC>, t : Box<ExprC>, e : Box<ExprC> },
    IdC { i : String },
    LamC { params : Vec<String>, body : Box<ExprC> },
    AppC { fun_def : Box<ExprC>, params: Vec<Box<ExprC>> }
}

enum ValueV {
    NumV { n : u32 }
}

fn interp(e: ExprC) -> ValueV {
    match e {
        ExprC::NumC { n } => ValueV::NumV { n : n },
        _ => ValueV::NumV { n : 4 }
    }
}

fn serialize(v: ValueV) -> String {
    match v {
        ValueV::NumV { n } => n.to_string(),
        _ => String::from("didn't catch")
    }
}

fn main() {
    let mut scores: HashMap<String, Box<ValueV>> = HashMap::new();
    let test_num = ExprC::BoolC { b: false };
    let test_num_2 = ExprC::NumC { n: 45 };
    let test_string = ExprC::StringC { s: String::from("just a test") };
    let test_bool = ExprC::BoolC { b: false };
    let test_if = ExprC::IfC { i: Box::new(test_bool), t: Box::new(test_string), e: Box::new(test_num) };

    print!("{}", serialize(interp(test_num_2)));
}

