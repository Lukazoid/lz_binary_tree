use lz_eytzinger_tree::EytzingerTree;

const LEFT: usize = 0;
const RIGHT: usize = 1;

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum ChildDirection {
    Left,
    Right,
}

pub struct BinaryTree<N> {
    inner: EytzingerTree<N>,
}

impl<N> BinaryTree<N> {
    pub fn new() -> Self {
        Self {
            inner: EytzingerTree::new(2),
        }
    }
}

pub struct NodeMut<'a, N>(lz_eytzinger_tree::NodeMut<'a, N>);

impl<N: Ord> BinaryTree<N> {
    pub fn insert(&mut self, value: N) -> NodeMut<N> {
        use lz_eytzinger_tree::Entry::*;
        use std::cmp::Ordering::*;

        match self.inner.root_entry() {
            Occupied(root) => {
                let mut current = root;
                loop {
                    let direction = match value.cmp(current.value()) {
                        Equal => break NodeMut(current),
                        Less => LEFT,
                        Greater => RIGHT,
                    };

                    match current.to_child_entry(direction) {
                        Occupied(child) => {
                            current = child;
                            continue;
                        }
                        Vacant(vacant) => break NodeMut(vacant.insert(value)),
                    }
                }
            }
            Vacant(vacant) => NodeMut(vacant.insert(value)),
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
