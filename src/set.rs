use std::collections::BTreeSet;

type Element = usize;

#[derive(Debug)]
struct Set<T>(BTreeSet<T>);
impl<T: Default> Set<T> {
    fn new() -> Self {
        Set(BTreeSet::default())
    }
}

use std::ops::Mul;

// impl<T: Default + Clone + Ord> Mul for Set<T> {
//     type Output = Self;
//     fn mul(self, rhs: Self) -> Self {
//         let mut s = Set::new();
//         for e0 in &self.0 {
//             for e1 in &rhs.0 {
//                 s.0.insert((e0.clone(), e1.clone()));
//             } 
//         }
//         s
//     }
// }

#[test]
fn test() {
    let set0 = Set::<Element>::new();
    let set1 = Set::<Element>::new(); 
    // let set = set0 * set1;
    // println!("{:?}", set);
}