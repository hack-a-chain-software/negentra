pub const fn next_index(index: u64) -> u64 {
    if lsz(index) == !index {
        (index % lsz(index)) >> 1
    } else {
        index + breadth_step(index)
    }
}

// least significant 0
pub const fn lsz(v: u64) -> u64 {
    !v & (1 << v.trailing_ones())
}

pub const fn parent(v: u64) -> u64 {
    if v & (lsz(v) << 1) == 0 {
        v + lsz(v)
    } else {
        v - lsz(v)
    }
}

// the difference d = (right_child - left_child) = (next_child - right_child)
pub const fn breadth_step(v: u64) -> u64 {
    lsz(v) << 1
}

#[cfg(tests)]
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
}
