use std::cmp::{PartialEq, PartialOrd};
use std::collections::BTreeMap;
use std::ops::{Add, Sub};

#[derive(Debug)]
pub struct SumTree<Tree, Vector, Map, InverseMap> {
    index: u64,
    dead_leaves: Vector,

    tree: Tree,
    leaf_map: Map,
    index_map: InverseMap,
}

const ROOT_INDEX: u64 = u64::MAX >> 1;

#[derive(PartialEq)]
enum Operation {
    Sum,
    Subtraction,
}

impl<Value, Id> SumTree<BTreeMap<u64, Value>, Vec<u64>, BTreeMap<u64, Id>, BTreeMap<Id, u64>>
where
    Value: PartialOrd + PartialEq + Copy + Add<Output = Value> + Sub<Output = Value>,
    Id: Ord + Copy,
{
    pub fn new() -> Self {
        Self {
            index: ROOT_INDEX,
            dead_leaves: vec![],
            tree: BTreeMap::new(),
            leaf_map: BTreeMap::new(),
            index_map: BTreeMap::new(),
        }
    }

    fn update_branch(&mut self, from_index: u64, value: Value, op: Operation) {
        let mut parent_index = from_index;

        while parent_index != u64::MAX {
            let parent_value = match self.tree.get(&parent_index) {
                None => return,
                Some(&p) => p,
            };

            let new_value = match op {
                Operation::Sum => parent_value + value,
                Operation::Subtraction => parent_value - value,
            };

            self.tree.insert(parent_index, new_value);

            parent_index = parent(parent_index);
        }
    }

    fn shift_index(&mut self) -> u64 {
        let index = self.index;
        self.index = next_index(index);

        index
    }

    pub fn insert(&mut self, id: Id, value: Value) {
        let index = if self.dead_leaves.len() == 0 {
            self.shift_index()
        } else {
            self.dead_leaves.pop().unwrap()
        };

        self.tree.insert(index, value);
        self.leaf_map.insert(index, id);
        self.index_map.insert(id, index);

        let parent_index = parent(index);

        match self.tree.get(&parent_index) {
            None => (),
            Some(&parent_value) => match self.leaf_map.get(&parent_index) {
                None => (),
                Some(&parent_id) => {
                    let sibling_index = self.shift_index();

                    self.tree.insert(sibling_index, parent_value);
                    self.leaf_map.remove(&parent_index);
                    self.leaf_map.insert(sibling_index, parent_id);
                    self.index_map.insert(parent_id, sibling_index);
                }
            },
        }

        self.update_branch(parent_index, value, Operation::Sum);
    }

    pub fn remove(&mut self, id: Id) {
        let index = match self.index_map.get(&id) {
            None => return,
            Some(&i) => i,
        };

        self.leaf_map.remove(&index);
        self.index_map.remove(&id);

        let value = match self.tree.get(&index) {
            None => return,
            Some(&v) => v,
        };

        self.update_branch(index, value, Operation::Subtraction);

        self.dead_leaves.push(index);
    }

    pub fn get(&self, index: &u64) -> Option<Value> {
        match self.tree.get(&index) {
            None => None,
            Some(&v) => Some(v),
        }
    }

    pub fn root(&self) -> Option<Value> {
        self.get(&ROOT_INDEX)
    }
}

const fn next_index(index: u64) -> u64 {
    if lsz(index) == !index {
        (index % lsz(index)) >> 1
    } else {
        index + breadth_step(index)
    }
}

/*
const fn last_n_bits(v: u64, n: u64) -> u64 {
    v & !(!0u64 << n)
}

// least significant bit
const fn lsb(v: u64) -> u64 {
    v & (1 << v.trailing_zeros())
}

const fn log_2(value: u64) -> u64 {
    let mut v = value;

    v |= v >> 1;
    v |= v >> 2;
    v |= v >> 4;
    v |= v >> 8;
    v |= v >> 16;
    v |= v >> 32;

    v & !(v >> 1)
}
*/

// least significant 0
const fn lsz(v: u64) -> u64 {
    !v & (1 << v.trailing_ones())
}

const fn parent(v: u64) -> u64 {
    if v & (lsz(v) << 1) == 0 {
        v + lsz(v)
    } else {
        v - lsz(v)
    }
}

// the difference d = (right_child - left_child) = (next_child - right_child)
const fn breadth_step(v: u64) -> u64 {
    lsz(v) << 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lsz() {
        const TABLE: [(u64, u64); 3] = [
            ((1 << 3) - 1, 0b1_000),
            ((1 << 4) - 1, 0b10_000),
            ((1 << 5) - 1 + (1 << 6), 0b100_000),
        ];

        for (input, output) in TABLE {
            assert_eq!(lsz(input), output);
        }
    }

    #[test]
    fn test_parent() {
        const TABLE: [(u64, u64); 4] = [
            (ROOT_INDEX >> 1, ROOT_INDEX),
            (ROOT_INDEX >> 2, ROOT_INDEX >> 1),
            (!(1 << 6), !(1 << 7)),
            (!(1 << 6) - breadth_step(!(1 << 6)), !(1 << 7)),
        ];

        for (input, output) in TABLE {
            assert_eq!(parent(input), output);
        }
    }

    #[test]
    fn test_next_index() {
        const TABLE: [(u64, u64); 1] = [(ROOT_INDEX, ROOT_INDEX >> 1)];

        for (input, output) in TABLE {
            assert_eq!(next_index(input), output);
        }
    }

    #[test]
    fn test_insert_and_remove() {
        let mut tree =
            SumTree::<BTreeMap<u64, u64>, Vec<u64>, BTreeMap<u64, u64>, BTreeMap<u64, u64>>::new();

        tree.insert(1, 15);
        tree.insert(2, 30);
        tree.insert(3, 45);
        assert_eq!(tree.root(), Some(90));

        tree.remove(1);
        assert_eq!(tree.dead_leaves.len(), 1);
        assert_eq!(tree.root(), Some(75));

        tree.insert(20, 12);
        assert_eq!(tree.dead_leaves.len(), 0);
        assert_eq!(tree.root(), Some(87));

        tree.insert(25, 3);
        assert_eq!(tree.root(), Some(90));
    }

    #[test]
    fn test_sum_property() {
        enum OP {
            Insert,
            Remove,
        }
        const OPS: [(OP, u64); 12] = [
            (OP::Insert, 3),
            (OP::Insert, 3),
            (OP::Insert, 63),
            (OP::Insert, 7),
            (OP::Insert, 3),
            (OP::Remove, 5),
            (OP::Insert, 27),
            (OP::Remove, 2),
            (OP::Remove, 1),
            (OP::Insert, 30),
            (OP::Insert, 78),
            (OP::Insert, 40),
        ];

        let mut tree =
            SumTree::<BTreeMap<u64, u64>, Vec<u64>, BTreeMap<u64, u64>, BTreeMap<u64, u64>>::new();
        let mut id = 1;

        for (op, input) in OPS {
            match op {
                OP::Insert => {
                    tree.insert(id, input);
                    id += 1;
                }
                OP::Remove => {
                    tree.remove(input);
                }
            }

            let mut queue: Vec<u64> = vec![ROOT_INDEX];
            while queue.len() > 0 {
                let index = queue.pop().unwrap();
                match tree.leaf_map.get(&index) {
                    None => {
                        let step = lsz(index) >> 1;
                        let parent = match tree.tree.get(&index) {
                            None => continue,
                            Some(&v) => {
                                if v == 0 {
                                    continue;
                                } else {
                                    v
                                }
                            }
                        };

                        let left_child = match tree.tree.get(&(index - step)) {
                            None => 0,
                            Some(&v) => v,
                        };
                        let right_child = match tree.tree.get(&(index + step)) {
                            None => 0,
                            Some(&v) => v,
                        };

                        assert_eq!(parent, left_child + right_child);
                        queue.push(index - step);
                        queue.push(index + step);
                    }
                    Some(_) => (),
                }
            }
        }
    }
}
