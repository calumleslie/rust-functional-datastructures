use std::sync::Arc;
use std::fmt::Debug;

#[cfg(test)]
use std::cmp;

/// A trait representing an immutable Set type.
pub trait Set<T: Eq> {
    /// Returns a set containing nothing.
    fn empty() -> Self;
    /// Creates a copy of this set with `value` added to it.
    fn insert(&self, value: T) -> Self;
    /// Tests whether `value` is a member of this set.
    fn member(&self, value: T) -> bool;
}

/// A trait representing an immutable Map type.
pub trait Map<K: Eq, V> {
    /// Creates a map containing nothing.
    fn empty_map() -> Self;
    /// Returns a copy of this map with `key` bound to `value`
    fn bind(&self, key: K, value: V) -> Self;
    /// Returns a `Some` of the value bound to `key`, or `None`
    /// if `key` is not a key in this map.
    fn lookup(&self, key: K) -> Option<V>;
}

/// An unbalanced binary tree that can be used as both an immutable `Map`
/// and an immutable `Set`. In the case of a `Set` the value type must
/// be `()`.
///
/// This is the type called `UnbalancedSet` in Chapter 2 of PFDL.
///
/// # Examples
///
/// Using the type as a map:
/// 
/// ```
/// use functional_datastructures::set::Map;
/// use functional_datastructures::set::Tree;
///
/// let empty_map: Tree<u32,String> = Tree::empty_map();
/// let map_with_values = empty_map.bind(1, "Harold".to_string())
///                                .bind(2, "27".to_string());
///
/// let map_with_altered_values = map_with_values.bind(1, "Raymond".to_string())
///                                              .bind(3, "Something else".to_string());
///
/// assert!(empty_map.lookup(1).is_none());
/// assert!(empty_map.lookup(2).is_none());
/// assert!(empty_map.lookup(3).is_none());
///
/// assert!(map_with_values.lookup(1).unwrap() == "Harold");
/// assert!(map_with_values.lookup(2).unwrap() == "27");
/// assert!(map_with_values.lookup(3).is_none());
///
/// assert!(map_with_altered_values.lookup(1).unwrap() == "Raymond");
/// assert!(map_with_altered_values.lookup(2).unwrap() == "27");
/// assert!(map_with_altered_values.lookup(3).unwrap() == "Something else");
/// ```
///
/// Using the type as a set:
/// 
/// 
/// ```
/// use functional_datastructures::set::Set;
/// use functional_datastructures::set::Tree;
///
/// let empty_set: Tree<u32,()> = Tree::empty();
/// let set_with_contents = empty_set.insert(1).insert(2);
///
/// assert!(!empty_set.member(1));
/// assert!(!empty_set.member(2));

/// assert!(set_with_contents.member(1));
/// assert!(set_with_contents.member(2));
/// ```
#[derive(Debug, Clone)]
pub enum Tree<K: Ord + Clone, V: Clone> {
    #[doc(hidden)]
    Empty,
    #[doc(hidden)]
    Node {
        left: Arc<Tree<K, V>>,
        key: K,
        value: V,
        right: Arc<Tree<K, V>>,
    },
}

impl <K: Ord + Clone + Debug, V: Clone + Debug> Map<K, V> for Tree<K,V> {
    fn empty_map() -> Self {
        return Tree::Empty;
    }
    fn bind(&self, new_key: K, new_value: V) -> Self {
        match *self {
            Tree::Empty => Tree::singleton(new_key, new_value),
            Tree::Node { ref left, ref key, ref right, ref value } => {
                if new_key < *key {
                    Tree::Node {
                        left: Arc::new(left.bind(new_key, new_value)),
                        key: key.clone(),
                        value: value.clone(),
                        right: right.clone(),
                    }
                } else if new_key > *key {
                    Tree::Node {
                        left: left.clone(),
                        key: key.clone(),
                        value: value.clone(),
                        right: Arc::new(right.bind(new_key, new_value)),
                    }
                } else {
                    // Update "this" node.
                    Tree::Node {
                        left: left.clone(),
                        key: new_key,
                        value: new_value,
                        right: right.clone(),
                    }
                }
            }
        }
    }
    fn lookup(&self, search_key: K) -> Option<V> {
        match *self {
            Tree::Empty => None,
            Tree::Node { ref key, ref value, .. } =>
                self.lookup_with_candidate(search_key, &key, &value),
        }
    }
}

impl <T: Ord + Clone + Debug> Set<T> for Tree<T, ()> {
    fn empty() -> Self {
        return Tree::Empty;
    }
    fn insert(&self, new_value: T) -> Self {
        match *self {
            Tree::Empty => Tree::singleton(new_value, ()),
            Tree::Node { ref key, .. } => self.try_insert_with_candidate(new_value, key.clone())
                                              .unwrap_or_else(|| self.clone()),
        }
    }
    fn member(&self, search_value: T) -> bool {
        self.lookup(search_value).is_some()
    }
}


impl<K: Ord + Clone + Debug, V: Clone + Debug> Tree<K, V> {
    fn singleton(key: K, value: V) -> Self {
        let empty: Arc<Self> = Arc::new(Tree::empty_map());
        Tree::Node {
            left: empty.clone(),
            right: empty,
            key: key,
            value: value,
        }
    }
    fn lookup_with_candidate(&self,
                             search_key: K,
                             candidate_key: &K,
                             candidate_value: &V)
                             -> Option<V> {
        match *self {
            Tree::Empty => if search_key == *candidate_key {
                Some(candidate_value.clone())
            } else {
                None
            },
            Tree::Node { ref left, ref key, ref right, ref value } => if search_key < *key {
                left.lookup_with_candidate(search_key, candidate_key, candidate_value)
            } else {
                right.lookup_with_candidate(search_key, &key, &value)
            },
        }
    }
}

impl<T: Ord + Clone + Debug> Tree<T, ()> {
    fn try_insert_with_candidate(&self, new_value: T, candidate: T) -> Option<Self> {
        match *self {
            Tree::Empty => {
                if new_value == candidate {
                    None
                } else {
                    Some(Tree::singleton(new_value, ()))
                }
            }
            Tree::Node { ref left, ref key, ref right, .. } => {
                if new_value < *key {
                    left.try_insert_with_candidate(new_value, candidate).map(|new_left| {
                        Tree::Node {
                            left: Arc::new(new_left),
                            key: key.clone(),
                            right: right.clone(),
                            value: (),
                        }
                    })
                } else if new_value > *key {
                    right.try_insert_with_candidate(new_value, key.clone()).map(|new_right| {
                        Tree::Node {
                            left: left.clone(),
                            key: key.clone(),
                            right: Arc::new(new_right),
                            value: (),
                        }
                    })
                } else {
                    None
                }
            }
        }
    }
    #[cfg(test)]
    fn complete(value: T, depth: u32) -> Self {
        let mut tree: Arc<Self> = Arc::new(Tree::empty());
        for _ in 0..depth {
            tree = Arc::new(Tree::Node {
                left: tree.clone(),
                key: value.clone(),
                right: tree,
                value: (),
            })
        }
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
    let empty_tree: Tree<(), ()> = Tree::empty();

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

#[test]
fn map_missing_values_not_present() {
    let map = Tree::empty_map().bind(10, "hello".to_string());

    assert!(map.lookup(4).is_none());
}

#[test]
fn map_present_values_are_present() {
    let map = Tree::empty_map().bind(10, "hello".to_string());

    assert!(map.lookup(10).unwrap() == "hello");
}

#[test]
fn map_values_can_be_replaced() {
    let map1 = Tree::empty_map().bind(3, "three").bind(1, "one").bind(2, "two");


    let map2 = map1.bind(2, "not two");

    assert!(map1.lookup(2).unwrap() == "two");
    assert!(map2.lookup(2).unwrap() == "not two");
}
