use std::{collections::HashMap, rc::Rc};

use crate::{Entity, Label, Sort};


type VariableId = usize;
#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Variable {
    pub id: VariableId,
    name: Label,
}

impl Variable {
    pub fn new(id: VariableId, name: Label) -> Self {
        Variable { id, name }
    }
}
pub enum RazType {
    Signature(Sort),
    Entity(Entity),
}

impl From<Sort> for RazType {
    fn from(value: Sort) -> Self {
        RazType::Signature(value)
    }
}

#[derive(Default, Clone)]
pub struct Context(Rc<HashMap<Variable, RazType>>);

impl Context {
    pub fn new(map: Rc<HashMap<Variable, RazType>>) -> Self {
        Context(map)
    }
}

#[test]
fn test() {
    let mut ctxt = HashMap::<Variable, RazType>::new();
    let sort_m = Sort::new(0, "m".into());
    let sort_s = Sort::new(1, "s".into());

    ctxt.insert(Variable::new(1, "x1".into()), sort_m.clone().into());
    ctxt.insert(Variable::new(2, "x2".into()), sort_m.clone().into());
    ctxt.insert(Variable::new(3, "p".into()), sort_s.clone().into());
    let _ctxt = Context::new(Rc::new(ctxt));
}