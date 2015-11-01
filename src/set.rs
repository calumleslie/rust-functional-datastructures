use std::sync::Arc;
use std::fmt::Debug;

trait Set<T: Ord> {
	fn empty() -> Self;
	fn insert(&self, value: T) -> Self;
	fn member(&self, value: T) -> bool;
}

#[derive(Debug, Clone)]
enum Tree<T: Ord + Clone> {
	Empty,
	Node { left: Arc<Tree<T>>, value: T, right: Arc<Tree<T>> }
}

impl <T: Ord + Clone + Debug> Set<T> for Tree<T> {
	fn empty() -> Self {
		return Tree::Empty;
	}
	fn insert(&self, new_value: T) -> Self {
		return match *self {
			Tree::Empty => {
				let empty = Arc::new( Tree::empty() );
				Tree::Node { 
					left: empty.clone(),
					right: empty.clone(),
					value: new_value
				}
			},
			Tree::Node { ref left, ref value, ref right } => {
				if new_value < *value {
					Tree::Node {
						left: Arc::new( left.insert( new_value ) ),
						value: value.clone(),
						right: right.clone()
					}
				} else if new_value > *value {
					Tree::Node {
						left: left.clone(),
						value: value.clone(),
						right: Arc::new( right.insert( new_value ) )
					}
				} else {
					// TODO: Would be good if this could return the original?
					return self.clone();
				}
			}
		}
	}
	fn member(&self, search_value: T) -> bool {
		return match *self {
			Tree::Empty => false,
			Tree::Node { ref left, ref value, ref right } => if search_value < *value {
				left.member(search_value)
			} else {
				right.member_with_candidate(search_value, value.clone())
			}
		}
	}
}

impl<T: Ord + Clone + Debug> Tree<T> {
	fn member_with_candidate(&self, search_value:T, best_candidate: T) -> bool {
		return match *self {
			Tree::Empty => search_value == best_candidate,
			Tree::Node { ref left, ref value, ref right } => if search_value < *value {
				left.member_with_candidate(search_value, best_candidate)
			} else {
				right.member_with_candidate(search_value, value.clone())
			}
		}
	}
}

#[test]
fn empty_contains_nothing() {
	let empty_tree: Tree<()> = Tree::empty();

	// There is only one value of this type so this is exhaustive.
	assert!( !empty_tree.member( () ) );
}

#[test]
fn inserted_values_are_contained() {
	let tree = Tree::empty().insert( 3 ).insert( 5 );

	assert!( tree.member(3) );
	assert!( tree.member(5) );
	assert!( !tree.member(42) );
}