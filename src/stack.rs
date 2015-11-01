use std::sync::Arc;

#[derive(Debug)]
enum StackError { NoSuchElementException }

trait Stack<T: Clone> {
	fn empty() -> Self;
	fn is_empty(&self) -> bool;
	fn cons(self, value: T) -> Self;
	fn head(&self) -> Result<T, StackError>;
	fn tail(&self) -> Result<Arc<Self>, StackError>;
}

#[derive(Debug, Clone)]
enum CustomStack<T> {
	Empty,
	Cons { value: T, tail: Arc<CustomStack<T>> }
}

impl<T: Clone> Stack<T> for CustomStack<T> {
	fn empty() -> CustomStack<T> {
		return CustomStack::Empty;
	}
	fn is_empty(&self) -> bool {
		return match *self {
			CustomStack::Empty => true,
			_ => false,
		}
	}
	fn cons(self, value: T) -> Self {
		return CustomStack::Cons { value: value, tail: Arc::new( self ) };
	}
	fn head(&self) -> Result<T, StackError> {
		return match *self {
			CustomStack::Empty => Err(StackError::NoSuchElementException),
			CustomStack::Cons { ref value, tail: _ } => Ok(value.clone())
		}
	}
	fn tail(&self) -> Result<Arc<Self>, StackError> {
		return match *self {
			CustomStack::Empty => Err(StackError::NoSuchElementException),
			CustomStack::Cons { value: _, ref tail } => Ok(tail.clone())
		}
	}
}

#[test]
fn empty_is_empty() {
	let stack: CustomStack<()> = CustomStack::empty();

	assert!( stack.is_empty() );
}

#[test]
fn cons_is_not_empty() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(4);

	assert!( !stack.is_empty() );
}

#[test]
fn head_empty_error() {
	let stack: CustomStack<()> = CustomStack::empty();

	assert!( stack.head().is_err() );
}

#[test]
fn head_last_item() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(5).cons(6);
	let head = stack.head();

	assert!( head.is_ok() );
	assert!( head.unwrap() == 6 );
}


#[test]
fn tail_empty_is_error() {
	let stack: CustomStack<()> = CustomStack::empty();

	assert!( stack.tail().is_err() );
}


#[test]
fn head_after_tail() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
	let tailtail = stack.tail().unwrap().tail().unwrap();

	assert!( tailtail.head().unwrap() == 1 );
}

#[test]
fn cloneable() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
	let stack2 = stack.clone();

	let tailtail = stack.tail().unwrap().tail().unwrap();
	let tail = stack2.tail().unwrap();

	assert!( tailtail.head().unwrap() == 1 );
	assert!( tail.head().unwrap() == 2 );
}