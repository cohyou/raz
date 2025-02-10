use std::{fmt::Display, hash::Hash};

use crate::{Category, Entity, Equation, Label, Sort, Theory};

type AttributeId = usize;

pub struct Attribute {
    id: AttributeId,
    name: Label,
    entity: Entity,
    sort: Sort,
}

struct Profunctor {
    attributes: Vec<Attribute>,
    observables: Vec<Equation>,
}

pub struct Schema {
    category: Category,
    profunctor: Profunctor,
    theory: Theory,
}