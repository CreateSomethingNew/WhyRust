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
struct Env {
	env : HashMap<String, ValueV>
}

#[derive(Clone)]
enum ValueV {
    NumV { n : u32 },
    BoolV { b : bool },
    StringV { s : String },
    PrimV { s : String },
//    PrimV { p : fn(ValueV, ValueV) -> ValueV },
    CloV { params : Vec<String>, body : Box<ExprC>, env : Env }
}

fn interp(e: ExprC, env: &mut HashMap<String, ValueV>) -> ValueV {
    match e {
        ExprC::NumC { n } => ValueV::NumV { n : n },
        ExprC::IdC { i } => {
        	match env.get(&i) {
        		Some(v) => (*v).clone(),
        		None => panic!("Unbound identifier")
        	}
        }
        ExprC::LamC { params, body } => ValueV::CloV { params : params, body: body, env: Env {env: (*env).clone()} },
        ExprC::BoolC { b } => ValueV::BoolV { b : b },
        ExprC::StringC { s } => ValueV::StringV { s : s },
        ExprC::IfC { i, t, e } => if_helper(i, t, e, env),
        ExprC::AppC { fun_def, args } => {
        	let lambda_interp_args = |arg: &Box<ExprC>| interp((**arg).clone(), &mut(*env).clone());
        	let interp_args: Vec<ValueV> = args.iter().map(lambda_interp_args).collect();
        	match interp((*fun_def).clone(), &mut(*env).clone()) {
        		ValueV::PrimV { s } => {
        			if interp_args.len() != 2 {
        				panic!("Arg lengths must be 2 for primop")
        			}
                    let temp = &interp_args[0];
                    let temp2 = &interp_args[1];
                    match s.as_ref() {
                        "equal?" => {
                            match (temp, temp2) {
                                (ValueV::NumV { n: first }, ValueV::NumV { n: second }) => { 
                                    if first == second {
                                        return ValueV::BoolV{ b : true }
                                    }
                                    else {
                                        return ValueV::BoolV{ b : false }
                                    } 
                                },
                                (ValueV::BoolV { b: first }, ValueV::BoolV { b: second }) => { 
                                    if first == second {
                                        return ValueV::BoolV{ b : true }
                                    }
                                    else {
                                        return ValueV::BoolV{ b : false }
                                    } 
                                },
                                (ValueV::StringV { s: first }, ValueV::StringV { s: second }) => { 
                                    if first == second {
                                        return ValueV::BoolV{ b : true }
                                    }
                                    else {
                                        return ValueV::BoolV{ b : false }
                                    } 
                                },
                                _ => return  ValueV::BoolV{ b : false }
                            }
                        }
                        _ => ()
                    }
                    let argVal: Vec<u32> = prim_helper_num(temp.clone(), temp2.clone());
                    match s.as_ref() {
                        "+" => ValueV::NumV{ n: (argVal[0] + argVal[1]) }, //fix this to call helper
                        "-" => ValueV::NumV{ n: (argVal[0] - argVal[1]) },
                        "*" => ValueV::NumV{ n: (argVal[0] * argVal[1]) },
                        "<=" => ValueV::BoolV { b: (argVal[0] <= argVal[1]) },
                        "/" => { 
                                            if argVal[1] == 0 {
                                                panic!("Division by zero")
                                            } else {
                                                ValueV::NumV {n : (argVal[0] / argVal[1])}
                                            }
                        },   
                        _ => panic!("not caught!")
                }
            },
        	ValueV::CloV { params, body, env } => ValueV::NumV { n : 4 },
            _ => panic!("not caught")
            }
        }
    }
}    


// Does the type checking required for primitives
fn prim_helper_num(arg1: ValueV , arg2: ValueV) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    match arg1 {
        ValueV::NumV { n } => {
            v.push(n);
            match arg2 {
                ValueV::NumV { n } => { 
                                        v.push(n);
                                        v
                                        },
                _ => panic!("second arguement is not a NumV") 
           }

        },
        _ => panic!("invalid arguements")
     }   
}

// fn prim_helper_bool(arg1: ValueV, arg2: ValueV) -> Vec<bool> {
//     let mut v: Vec<bool> = Vec::new();
//     match arg1 {
//         ValueV::BoolV { b } => {
//             match arg2 {
//                 ValueV::BoolV { b } => { v.push(arg1.b);
//                                          v.push(arg2.b);
//                                          v
//                                          },
//                 _ => { v.push(false);
//                        v
//                      },                            
//             }
//         },
//         _ => panic!("invalid arguements")
//     }
// }


fn if_helper(i: Box<ExprC>, t: Box<ExprC>, e: Box<ExprC>, 
	env: &mut HashMap<String, ValueV>) -> ValueV {
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
        ValueV::PrimV { s: _ } => String::from("#<primop>")
    }
}

fn make_top_map() -> HashMap<String, ValueV> {
	let mut h: HashMap<String, ValueV> = HashMap::new();
	h.insert("true".to_string(), ValueV::BoolV{ b: true });
	h.insert("false".to_string(), ValueV::BoolV{ b: false });
    h.insert("+".to_string(), ValueV::PrimV{ s: "+".to_string() });
    h.insert("-".to_string(), ValueV::PrimV{ s: "-".to_string() });
    h.insert("*".to_string(), ValueV::PrimV{ s: "*".to_string() });
    h.insert("/".to_string(), ValueV::PrimV{ s: "/".to_string() });
    h.insert("<=".to_string(), ValueV::PrimV{ s: "<=".to_string() });
    h.insert("equal?".to_string(), ValueV::PrimV{ s: "equal?".to_string() });
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
    let plus_func = ExprC::IdC { i: "+".to_string() };
    let plus_arg1 = ExprC::NumC { n: 1};
    let plus_arg2 = ExprC::NumC { n: 2};

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

    assert_eq!("true", serialize(interp(ExprC::AppC { fun_def : Box::new(ExprC::IdC { i: "equal?".to_string() }), args : vec![Box::new(ExprC::StringC{ s : "hi".to_string()}), 
        Box::new(ExprC::StringC{ s : "hi".to_string()})]}, &mut(make_top_map()))));
    assert_eq!("false", serialize(interp(ExprC::AppC { fun_def : Box::new(ExprC::IdC { i: "equal?".to_string() }), args : vec![Box::new(ExprC::StringC{ s : "hi".to_string()}), 
        Box::new(ExprC::StringC{ s : "hello".to_string()})]}, &mut(make_top_map()))));
    assert_eq!("true", serialize(interp(ExprC::AppC { fun_def : Box::new(ExprC::IdC { i: "equal?".to_string() }), args : vec![Box::new(ExprC::BoolC{ b : true}), 
        Box::new(ExprC::BoolC{ b: true})]}, &mut(make_top_map()))));
    assert_eq!("false", serialize(interp(ExprC::AppC { fun_def : Box::new(ExprC::IdC { i: "equal?".to_string() }), args : vec![Box::new(ExprC::BoolC{ b : false}), 
        Box::new(ExprC::BoolC{ b: true})]}, &mut(make_top_map()))));
    assert_eq!("true", serialize(interp(ExprC::AppC { fun_def : Box::new(ExprC::IdC { i: "equal?".to_string() }), args : vec![Box::new(ExprC::NumC{ n : 10}), 
        Box::new(ExprC::NumC{ n : 10})]}, &mut(make_top_map()))));
    assert_eq!("false", serialize(interp(ExprC::AppC { fun_def : Box::new(ExprC::IdC { i: "equal?".to_string() }), args : vec![Box::new(ExprC::NumC{ n : 10}), 
        Box::new(ExprC::NumC{ n : 11})]}, &mut(make_top_map()))));
    assert_eq!("false", serialize(interp(ExprC::AppC { fun_def : Box::new(ExprC::IdC { i: "equal?".to_string() }), args : vec![Box::new(ExprC::BoolC{ b : true}), 
        Box::new(ExprC::NumC{ n : 11})]}, &mut(make_top_map()))));

    assert_eq!("3", serialize(interp(ExprC::AppC { fun_def : Box::new(plus_func), args : vec![Box::new(plus_arg1), Box::new(plus_arg2)]},
        &mut(make_top_map()))));

    assert_eq!(serialize(interp(ExprC::AppC {fun_def : Box::new(ExprC::IdC{ i: "*".to_string() }), args : vec![Box::new(ExprC::NumC { n: 4 }) , Box::new(ExprC::NumC { n : 6 })]},
    &mut(make_top_map()))), "24");
  //    interp(ExprC::AppC { fun_def : Box::new(bool_true_2), args : vec![Box::new(bool_true_3)]}, 
  //  	&mut(make_top_map()));
    /*
    println!("Panic test");
    let idc_unbound = ExprC::IdC { i: "hehe".to_string() };
    let result = panic::catch_unwind(|| {
    	interp(idc_unbound, &mut(make_top_map()))
		});
		assert!(result.is_err());
    */
}