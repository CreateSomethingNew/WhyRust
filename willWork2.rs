use std::collections::HashMap;
use std::panic;

#[derive(Clone)]
enum ExprC {
    NumC { n : u32 },
    BoolC { b : bool },
    StringC { s : String },
    IfC { i : Box<ExprC>, t : Box<ExprC>, e : Box<ExprC> },
    IdC { i : String },
    LamC { params : Vec<String>, body : Box<ExprC> },
    AppC { fun_def : Box<ExprC>, args: Vec<Box<ExprC>> }
}

#[derive(Clone)]
struct Env<'a> {
	env : &'a mut HashMap<String, ValueV<'a>>
}

#[derive(Clone)]
enum ValueV<'a> {
    NumV { n : u32 },
    BoolV { b : bool },
    StringV { s : String },
    PrimV { p : fn(ValueV, ValueV) -> ValueV<'a> },
    CloV { params : Vec<String>, body : Box<ExprC>, env : Env<'a> }
}

fn interp<'a>(e: ExprC, env: &mut HashMap<String, ValueV<'a>>) -> ValueV<'a> {
    match e {
        ExprC::NumC { n } => ValueV::NumV { n : n },
        ExprC::IdC { i } => {
        	match env.get(&i) {
        		Some(v) => (*v).clone(),
        		None => panic!("Unbound identifier")
        	}
        }
        ExprC::BoolC { b } => ValueV::BoolV { b : b },
        ExprC::StringC { s } => ValueV::StringV { s : s },
        ExprC::IfC { i, t, e } => if_helper(i, t, e, env),
        ExprC::AppC { fun_def, args } => {
        	let lambda_interp_args = |arg: &Box<ExprC>| interp((**arg).clone(), &mut(*env).clone());
        	let interp_args: Vec<ValueV> = args.iter().map(lambda_interp_args).collect();
        	match interp((*fun_def).clone(), &mut(*env).clone()) {
        		ValueV::PrimV { p } => {
        			if interp_args.len() != 2 {
        				panic!("Arg lengths must be 2 for primop")
        			}
        			p(interp_args[0].clone(), interp_args[1].clone())		
        		}
        		ValueV::CloV { params, body, env } => ValueV::NumV { n : 4 } 
        	}
        }
        _ => ValueV::NumV { n : 4 }
    }
}

fn if_helper<'a>(i: Box<ExprC>, t: Box<ExprC>, e: Box<ExprC>, 
	env: &mut HashMap<String, ValueV<'a>>) -> ValueV<'a> {
    match interp(*i, env) {
        ValueV::BoolV { b } => if b { interp(*t, env) } else { interp(*e, env) }
        _ => panic!("invalid in if")
    }
}

fn serialize(v: ValueV) -> String {
    match v {
        ValueV::NumV { n } => n.to_string(),
        ValueV::BoolV { b } => b.to_string(),
        ValueV::StringV { s } => s,
        ValueV::CloV { params, body, env } => String::from("#<procedure>"),
        ValueV::PrimV { p: _ } => String::from("#<primop>")
    }
}

fn make_top_map() -> HashMap<String, ValueV<'static>> {
	let mut h: HashMap<String, ValueV> = HashMap::new();
	h.insert("true".to_string(), ValueV::BoolV{ b: true });
	h.insert("false".to_string(), ValueV::BoolV{ b: false });
	h
}

fn main() {
    // primative types
    let bool_true = ExprC::BoolC { b: true };
    let bool_true_2 = ExprC::BoolC { b: true };
    let bool_true_3 = ExprC::BoolC { b: true };
    let idc_true = ExprC::IdC { i: "true".to_string() };
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
    assert_eq!("45", serialize(interp(num_45, &mut(make_top_map()))));
    assert_eq!("false", serialize(interp(bool_false, &mut(make_top_map()))));
    assert_eq!("true", serialize(interp(bool_true, &mut(make_top_map()))));
    assert_eq!("just a test", serialize(interp(string_test, &mut(make_top_map()))));
    assert_eq!("5", serialize(interp(test_if, &mut(make_top_map()))));
    assert_eq!("3", serialize(interp(test_if_2, &mut(make_top_map()))));
    assert_eq!("true", serialize(interp(idc_true, &mut(make_top_map()))));

    interp(ExprC::AppC { fun_def : Box::new(bool_true_2), args : vec![Box::new(bool_true_3)]}, 
    	&mut(make_top_map()));

    println!("Panic test");
    let idc_unbound = ExprC::IdC { i: "hehe".to_string() };
    let result = panic::catch_unwind(|| {
    	interp(idc_unbound, &mut(make_top_map()))
		});
		assert!(result.is_err());
}