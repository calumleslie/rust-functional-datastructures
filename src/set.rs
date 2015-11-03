use std::sync::Arc;
use std::fmt::Debug;

#[cfg(test)]
use std::cmp;

trait Set<T: Ord> {
    fn empty() -> Self;
    fn insert(&self, value: T) -> Self;
    fn member(&self, value: T) -> bool;
}

#[derive(Debug, Clone)]
enum Tree<T: Ord + Clone> {
    Empty,
    Node {
        left: Arc<Tree<T>>,
        value: T,
        right: Arc<Tree<T>>,
    },
}

impl <T: Ord + Clone + Debug> Set<T> for Tree<T> {
    fn empty() -> Self {
        return Tree::Empty;
    }
    fn insert(&self, new_value: T) -> Self {
        match *self {
            Tree::Empty => Tree::singleton(new_value),
            Tree::Node { ref value, .. } =>
                self.try_insert_with_candidate(new_value, value.clone())
                    .unwrap_or_else(|| self.clone()),
        }
    }
    fn member(&self, search_value: T) -> bool {
        return match *self {
            Tree::Empty => false,
            Tree::Node { ref value, .. } => self.member_with_candidate(search_value, value.clone()),
        };
    }
}

impl<T: Ord + Clone + Debug> Tree<T> {
    fn member_with_candidate(&self, search_value: T, best_candidate: T) -> bool {
        return match *self {
            Tree::Empty => search_value == best_candidate,
            Tree::Node { ref left, ref value, ref right } => if search_value < *value {
                left.member_with_candidate(search_value, best_candidate)
            } else {
                right.member_with_candidate(search_value, value.clone())
            },
        };
    }
    fn try_insert_with_candidate(&self, new_value: T, candidate: T) -> Option<Self> {
        match *self {
            Tree::Empty => {
                if new_value == candidate {
                    None
                } else {
                    Some(Tree::singleton(new_value))
                }
            }
            Tree::Node { ref left, ref value, ref right } => {
                if new_value < *value {
                    left.try_insert_with_candidate(new_value, candidate).map(|new_left| {
                        Tree::Node {
                            left: Arc::new(new_left),
                            value: value.clone(),
                            right: right.clone(),
                        }
                    })
                } else if new_value > *value {
                    right.try_insert_with_candidate(new_value, value.clone()).map(|new_right| {
                        Tree::Node {
                            left: left.clone(),
                            value: value.clone(),
                            right: Arc::new(new_right),
                        }
                    })
                } else {
                    None
                }
            }
        }
    }
    fn singleton(value: T) -> Self {
        let empty: Arc<Tree<T>> = Arc::new(Tree::empty());
        Tree::Node {
            left: empty.clone(),
            right: empty,
            value: value,
        }
    }
    #[cfg(test)]
    fn complete(value: T, depth: u32) -> Self {
        let mut tree: Arc<Tree<T>> = Arc::new(Tree::empty());
        for _ in 0..depth {
            tree = Arc::new(Tree::Node {
                left: tree.clone(),
                value: value.clone(),
                right: tree,
            })
        }
        // TODO: This is pretty untidy
        return (*tree).clone();
    }
    #[cfg(test)]
    fn depth(&self) -> u32 {
        match *self {
            Tree::Empty => 0,
            Tree::Node { ref left, ref right, .. } => 1 + cmp::max(left.depth(), right.depth()),
        }
    }
}



#[test]
fn empty_contains_nothing() {
    let empty_tree: Tree<()> = Tree::empty();

    // There is only one value of this type so this is exhaustive.
    assert!(!empty_tree.member(()));
}

#[test]
fn inserted_values_are_contained() {
    let tree = Tree::empty().insert(3).insert(5);

    assert!(tree.member(3));
    assert!(tree.member(5));
    assert!(!tree.member(42));
}

#[test]
fn complete_test() {
    let complete_tree = Tree::complete(12, 14);
    assert!(complete_tree.depth() == 14);
}
