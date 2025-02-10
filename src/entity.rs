type EntityId = usize;
pub struct Entity {
    pub id: EntityId
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Entity { id }
    }
}