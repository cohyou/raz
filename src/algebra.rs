use crate::{equation, Context, Equation};

pub struct Algebra {
    context: Context,
    equations: Vec<Equation>,
}

impl Algebra {
    pub fn new(context: Context, equations: Vec<Equation>) -> Self {
        Algebra { context, equations }
    }
}