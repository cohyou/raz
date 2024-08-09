use std::collections::BTreeSet;
type Set = BTreeSet<usize>;

struct Function;
struct Equation;

trait Schema {
    fn functions(&self) -> Vec<Function>;
    fn equations(&self) -> Vec<Equation>;
}
trait Instance {
    fn schema(&self) -> impl Schema;
}
trait Action {
    fn apply(&self, instance: impl Instance) -> impl Instance;
}

#[derive(Debug)]
struct Singleton;
struct Identity;

impl Schema for Singleton {
    fn functions(&self) -> Vec<Function> {
        unimplemented!()
    }
    fn equations(&self) -> Vec<Equation> {
        unimplemented!()
    }
}
impl Instance for Singleton {
    fn schema(&self) -> Self {
        unimplemented!()
    }
}
impl Action for Identity {
    fn apply(&self, instance: impl Instance) -> impl Instance {
        instance
    }
}
fn main() {
    let singleton = Singleton;
    let schema = singleton.schema();
    schema.functions();
    schema.equations();
    println!("{:?}", &singleton);
    let identity = Identity;
    identity.apply(singleton);
}
