use near_sdk::borsh::{BorshDeserialize, BorshSerialize};

pub mod bits;
pub mod impl_btree;
pub mod impl_near;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct SumTree<Tree, Map, InverseMap, Vector> {
    pub index: u64,
    pub tree: Tree,
    pub leaf_map: Map,
    pub index_map: InverseMap,
    pub dead_leaves: Vector,
}

#[derive(PartialEq)]
pub enum Operation {
    Sum,
    Subtraction,
}
