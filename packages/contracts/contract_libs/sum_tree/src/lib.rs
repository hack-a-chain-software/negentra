pub mod bits;
pub mod impl_btree;
pub mod impl_near;

#[derive(Debug)]
pub struct SumTree<Tree, Vector, Map, InverseMap> {
    pub index: u64,
    pub dead_leaves: Vector,

    pub tree: Tree,
    pub leaf_map: Map,
    pub index_map: InverseMap,
}

#[derive(PartialEq)]
pub enum Operation {
    Sum,
    Subtraction,
}
