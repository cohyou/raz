use std::collections::BTreeSet;
#[derive(PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Object(usize);
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Arrow(usize);
struct Category {
    objects: BTreeSet<Object>,
    arrows: BTreeSet<Arrow>,
    dom: Box<dyn Fn(Arrow) -> Object>,
    cod: Box<dyn Fn(Arrow) -> Object>,
}
/*
射の結合をどのように表現するか
射f 射gが存在し、cod(f) = dom(g)であれば、f;gが存在する。
*/
#[test]
fn test() {
    let object = Object(0);
    let arrow = Arrow(0);
    let mut category = Category {
        objects: BTreeSet::new(),
        arrows: BTreeSet::new(),
        dom: Box::new(|_arrow| { unimplemented!() }),
        cod: Box::new(|_arrow| { unimplemented!() }),
    };
    category.objects.insert(object.clone());
    category.arrows.insert(arrow);
    category.dom = Box::new(|_arrow| { Object(0) });
    category.cod = Box::new(|_arrow| { Object(0) });
}