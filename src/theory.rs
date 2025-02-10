use std::fmt::Display;

use crate::{Equation, Label, Sort};

type FunctionId = usize;

#[derive(Clone)]
pub struct Function {
    id: FunctionId,
    name: Label,
    dom: Vec<Sort>,
    cod: Sort,
}

impl Function {
    pub fn new(id: FunctionId, name: Label, dom: Vec<Sort>, cod: Sort) -> Self {
        Function { id, name, dom, cod }
    }
}

// 4.1
pub struct Signature {
    sorts: Vec<Sort>,
    functions: Vec<Function>,
}

impl Signature {
    pub fn new(sorts: Vec<Sort>, functions: Vec<Function>) -> Self {
        Signature { sorts, functions }
    }
}

pub struct Theory {
    signature: Signature,
    equations: Vec<Equation>,
}

impl Theory {
    pub fn new(signature: Signature, equations: Vec<Equation>) -> Self {
        Theory { signature, equations }
    }
}
impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let _ = write!(f, "{}: (", self.name);
        for (i, s) in self.dom.iter().enumerate() {
            let _ = write!(f, "{s}");
            if i < self.dom.len() - 1 {
                let _ = write!(f, ", ");
            }
        }
        write!(f, ") -> {}", self.cod)
    }
}

#[test]
fn test() {
    use crate::Sort;
    let sorts = vec![Sort::zero(), Sort::one()];
    let func_syb = Function { id: 0, name: "emp".into(), dom: sorts, cod: Sort::two() };
    println!("{}", func_syb);
}

#[test]
fn test2() {
    use crate::Sort;
    use std::collections::HashMap;
    let mut sort_env = HashMap::new();
    let empty = Sort::zero();
    let unit = Sort::one();
    let bool = Sort::two();
    sort_env.insert(empty.clone().name, empty.clone());
    sort_env.insert(unit.clone().name, unit.clone());
    sort_env.insert(bool.clone().name, bool.clone());
    
    let sorts = vec![empty.clone(), unit.clone()];
    let func = Function { id: 0, name: "true".into(), dom: sorts, cod: bool.clone() };
    println!("{}", func);
}

// 4.4 Exampleより
#[test]
fn test3() {
    use crate::Sort;
    use std::collections::HashMap;
    
    let sort_m = Sort::new(0, "m".into());
    let sort_s = Sort::new(1, "s".into());
    let mut sort_env = HashMap::new();
    sort_env.insert(sort_m.clone().name, sort_m.clone());
    sort_env.insert(sort_s.clone().name, sort_s.clone());

    let sorts = vec![sort_m.clone(), sort_s.clone()];

    
    let eta = Function::new(0, "0".into(), vec![], sort_m.clone());
    let mju = Function::new(1, "1".into(), vec![sort_m.clone(), sort_m.clone()], sort_m.clone());
    let alpha = Function::new(2, "2".into(), vec![sort_m.clone(), sort_s.clone()], sort_s.clone());
    let mut func_env = HashMap::new();
    func_env.insert(sort_m.clone().name, sort_m.clone());
    func_env.insert(sort_s.clone().name, sort_s.clone());
    
    let functions = vec![eta.clone(), mju.clone(), alpha.clone()];

    let _sig = Signature::new(sorts, functions);
}