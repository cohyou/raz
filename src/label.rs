use std::{fmt::Display, rc::Rc};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct Label {
    name: Rc<String>
}

impl Label {
    pub fn new(name: Rc<String>) -> Self {
        Label { name }
    }
}

impl From<&'static str> for Label {
    fn from(value: &'static str) -> Self {
        Label { name: Rc::new(value.to_string()) }
    }
}

impl Display for Label {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}