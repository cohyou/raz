use crate::{Entity, Equation, Label};

// これはひとまず、単数引数の場合だけを書きました
// 積など、型の演算も本来は許したい。
// Entityの中身だけを考える。
type EdgeId = usize;

pub struct Edge {
    id: EdgeId,
    name: Label,
    dom: Entity,
    cod: Entity,
}

struct Graph {
    nodes: Vec<Entity>,
    edges: Vec<Edge>,
}

pub struct Category {
    graph: Graph,
    equations: Vec<Equation>,
}