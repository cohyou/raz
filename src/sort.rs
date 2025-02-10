use std::fmt::Display;

use crate::Label;

type SortId = usize;

#[derive(Clone)]
pub struct Sort {
    pub id: SortId,
    pub name: Label,
}

impl Sort {
    pub fn new(id: SortId, name: Label) -> Self {
        Sort { id, name }
    }
    pub fn zero() -> Self { Self::new(0, "empty".into()) }
    pub fn one() -> Self { Self::new(1, "unit".into()) }
    pub fn two() -> Self { Self::new(2, "bool".into()) }
}

// DisplayとDebugトレイトとの使い分けをどうするのか
impl Display for Sort {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
