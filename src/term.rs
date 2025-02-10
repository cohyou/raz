use std::{collections::HashMap, rc::Rc};

use crate::{Attribute, Context, Edge, Function, RazType, Sort, Variable};

pub struct Term {
    context: Context,
    inner: TermInner,
}

impl Term {
    pub fn new(context: Context, inner: TermInner) -> Self {
        Term { context, inner }
    }
}
pub enum TermInner {
    Var(Variable),
    Fun(Function, Vec<TermInner>),
    Edg(Edge, Vec<TermInner>),
    Att(Attribute, Vec<TermInner>),
}

impl From<Variable> for TermInner {
    fn from(value: Variable) -> Self {
        TermInner::Var(value)
    }
}

#[test]
fn test() {
    let mut ctxt = HashMap::<Variable, RazType>::new();
    let sort_m = Sort::new(0, "m".into());
    let sort_s = Sort::new(1, "s".into());

    let eta = Function::new(0, "eta".into(), vec![], sort_m.clone());
    let mju = Function::new(1, "mju".into(), vec![sort_m.clone(), sort_m.clone()], sort_m.clone());
    let alpha = Function::new(2, "alpha".into(), vec![sort_m.clone(), sort_s.clone()], sort_s.clone());

    let x1 = Variable::new(1, "x1".into());
    let x2 = Variable::new(2, "x2".into());
    let p = Variable::new(3, "p".into());
    ctxt.insert(x1.clone(), sort_m.clone().into());
    ctxt.insert(x2.clone(), sort_m.clone().into());
    ctxt.insert(p.clone(), sort_s.clone().into());
    let ctxt = Context::new(Rc::new(ctxt));

    let inner = TermInner::Fun(alpha, vec![
        TermInner::Fun(mju.clone(), vec![
            x1.into(),
            x2.into(),
        ]),
        p.into(),
    ]);
    let _term1 = Term { context: ctxt, inner };
    let _term_ground = Term {
        context: Context::default(),
        inner: TermInner::Fun(mju.clone(), vec![
            TermInner::Fun(eta.clone(), vec![]),
            TermInner::Fun(mju.clone(), vec![
                TermInner::Fun(eta.clone(), vec![]),
                TermInner::Fun(eta.clone(), vec![]),
            ]),
        ])
    };
}