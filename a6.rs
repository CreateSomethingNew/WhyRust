
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
    NumV { n : u32 },
    BoolV { b : bool },
    StringV { s : String },
    PrimV { p : String },
    CloV { params : Vec<String>, body : Box<ExprC> }
}

fn interp(e: ExprC) -> ValueV {
    match e {
        ExprC::NumC { n } => ValueV::NumV { n : n },
        ExprC::BoolC { b } => ValueV::BoolV { b : b },
        ExprC::StringC { s } => ValueV::StringV { s : s },
        ExprC::IfC { i, t, e } => if_helper(i, t, e),
        _ => ValueV::NumV { n : 4 }
    }
}

fn if_helper(i: Box<ExprC>, t: Box<ExprC>, e: Box<ExprC>) -> ValueV {
    match interp(*i) {
        ValueV::BoolV { b } => if b { interp(*t) } else { interp(*e) }
        _ => panic!("invalid in if")
    }
}

fn serialize(v: ValueV) -> String {
    match v {
        ValueV::NumV { n } => n.to_string(),
        ValueV::BoolV { b } => b.to_string(),
        ValueV::StringV { s } => s,
        ValueV::CloV { params, body } => String::from("#<procedure>"),
        ValueV::PrimV { p } => String::from("#<primop>"),
        _ => panic!("invalid in serialize")
    }
}

fn main() {
    // primative types
    let bool_true = ExprC::BoolC { b: true };
    let bool_false = ExprC::BoolC { b: false };
    let num_45 = ExprC::NumC { n: 45 };
    let string_test = ExprC::StringC { s: String::from("just a test") };

    // true if case
    let test_if_i = ExprC::BoolC { b: true };
    let test_if_t = ExprC::NumC { n: 5 };
    let test_if_e = ExprC::NumC { n: 3 };
    let test_if = ExprC::IfC { i: Box::new(test_if_i), t: Box::new(test_if_t), e: Box::new(test_if_e) };

    // false if case
    let test_if_i_2 = ExprC::BoolC { b: false };
    let test_if_t_2 = ExprC::NumC { n: 5 };
    let test_if_e_2 = ExprC::NumC { n: 3 };
    let test_if_2 = ExprC::IfC { i: Box::new(test_if_i_2), t: Box::new(test_if_t_2), e: Box::new(test_if_e_2) };

    // tests
    assert_eq!("45", serialize(interp(num_45)));
    assert_eq!("false", serialize(interp(bool_false)));
    assert_eq!("true", serialize(interp(bool_true)));
    assert_eq!("just a test", serialize(interp(string_test)));
    assert_eq!("5", serialize(interp(test_if)));
    assert_eq!("3", serialize(interp(test_if_2)));
}
