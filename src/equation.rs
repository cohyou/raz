use std::{collections::HashMap, rc::Rc};

use crate::{Context, Function, RazType, Signature, Sort, Term, TermInner, Theory, Variable};

pub struct Equation(Term, Term);

impl Equation {
    pub fn new(left: Term, right: Term) -> Self {
        Equation(left, right)
    }
}

#[test]
fn test() {
    
    let sort_m = Sort::new(0, "m".into());
    let sort_s = Sort::new(1, "s".into());

    let eta = Function::new(0, "eta".into(), vec![], sort_m.clone());
    let mju = Function::new(1, "mju".into(), vec![sort_m.clone(), sort_m.clone()], sort_m.clone());
    let alpha = Function::new(2, "alpha".into(), vec![sort_m.clone(), sort_s.clone()], sort_s.clone());

    let x = Variable::new(1, "x".into());
    let y = Variable::new(2, "y".into());
    let z = Variable::new(3, "z".into());
    let p = Variable::new(4, "p".into());


    let mut ctxt1 = HashMap::<Variable, RazType>::new();
    ctxt1.insert(x.clone(), sort_m.clone().into());
    let ctxt1 = Context::new(Rc::new(ctxt1));

    let mut ctxt2 = HashMap::<Variable, RazType>::new();
    ctxt2.insert(x.clone(), sort_m.clone().into());
    ctxt2.insert(y.clone(), sort_m.clone().into());
    ctxt2.insert(z.clone(), sort_m.clone().into());
    let ctxt2 = Context::new(Rc::new(ctxt2));

    let mut ctxt3 = HashMap::<Variable, RazType>::new();
    ctxt3.insert(x.clone(), sort_m.clone().into());
    ctxt3.insert(y.clone(), sort_m.clone().into());
    ctxt3.insert(p.clone(), sort_s.clone().into());
    let ctxt3 = Context::new(Rc::new(ctxt3));



    let eq1 = Equation::new(
        Term::new(ctxt1.clone(), TermInner::Fun(alpha.clone(), vec![
        TermInner::Fun(mju.clone(), vec![
            x.clone().into(),
            TermInner::Fun(eta.clone(), vec![]),
        ]),
        p.clone().into(),
    ])),
    Term::new(ctxt1.clone(), TermInner::Var(x.clone())));
    
    let eq2 = Equation::new(
        Term::new(ctxt1.clone(), TermInner::Fun(alpha.clone(), vec![
        TermInner::Fun(mju.clone(), vec![            
            TermInner::Fun(eta.clone(), vec![]),
            x.clone().into(),
        ]),
        p.clone().into(),
    ])),
    Term::new(ctxt1.clone(), TermInner::Var(x.clone())));

    let eq3 = Equation::new(
        Term::new(ctxt2.clone(), 
        TermInner::Fun(mju.clone(), vec![
            x.clone().into(),
            TermInner::Fun(mju.clone(), vec![
                y.clone().into(),
                z.clone().into(),
            ]
        )
    ])),
    Term::new(ctxt2.clone(), 
        TermInner::Fun(mju.clone(), vec![
            TermInner::Fun(mju.clone(), vec![
                x.clone().into(),
                y.clone().into(),
            ]),
            z.clone().into(),
        ]))
    );

    let left = 
    TermInner::Fun(alpha.clone(), vec![
        x.clone().into(),
        TermInner::Fun(alpha.clone(), vec![
            y.clone().into(),
            p.clone().into(),
        ]),
    ]);
    let right = 
    TermInner::Fun(alpha.clone(), vec![
        TermInner::Fun(mju.clone(), vec![
            x.clone().into(),
            y.clone().into(),
        ]),
        p.clone().into(),
    ]);

    let eq4 = Equation::new(
        Term::new(ctxt3.clone(), left),
        Term::new(ctxt3.clone(), right),
    );

    let sorts = vec![sort_m.clone(), sort_s.clone()];
    let functions = vec![eta.clone(), mju.clone(), alpha.clone()];
    let sig = Signature::new(sorts, functions);

    let _theory = Theory::new(sig, 
    vec![eq1, eq2, eq3, eq4]);
}