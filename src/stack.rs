use std::sync::Arc;

#[derive(Debug)]
enum StackError { NoSuchElementException, IndexOutOfRange }

trait Stack<T: Clone> {
	fn empty() -> Self;
	fn is_empty(&self) -> bool;
	fn cons(&self, value: T) -> Self;
	fn head(&self) -> Result<T, StackError>;
	fn tail(&self) -> Result<Arc<Self>, StackError>;
	fn update(&self, i: u32, value: T) -> Result<Self,StackError>; 
	fn size(&self) -> u32;
	fn get(&self, i: u32) -> Result<T,StackError>;
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
	fn cons(&self, value: T) -> Self {
		return CustomStack::Cons { value: value, tail: Arc::new( self.clone() ) };
	}
	fn head(&self) -> Result<T, StackError> {
		return match *self {
			CustomStack::Empty => Err(StackError::NoSuchElementException),
			CustomStack::Cons { ref value, .. } => Ok(value.clone())
		}
	}
	fn tail(&self) -> Result<Arc<Self>, StackError> {
		return match *self {
			CustomStack::Empty => Err(StackError::NoSuchElementException),
			CustomStack::Cons { ref tail, .. } => Ok(tail.clone())
		}
	}
	fn update(&self, i: u32, new_value: T) -> Result<Self,StackError> {
		return match *self {
			CustomStack::Empty => Err(StackError::IndexOutOfRange),
			CustomStack::Cons { ref value, ref tail } => match i {
				0 => Ok(tail.clone().cons(new_value)),
				_ => {
					let updated_tail = try!(tail.update(i - 1, new_value));
					Ok(updated_tail.cons(value.clone()))
				}
			}
		}
	}
	fn size(&self) -> u32 {
		return match *self {
			CustomStack::Empty => 0,
			CustomStack::Cons { ref tail, .. } => 1 + tail.size()
		}
	}
	fn get(&self, i: u32) -> Result<T, StackError> {
		return match *self {
			CustomStack::Empty => Err(StackError::IndexOutOfRange),
			CustomStack::Cons { ref value, ref tail } => match i {
				0 => Ok(value.clone()),
				_ => tail.get(i - 1)
			}
		}
	}
}

// Only compile this in tests to stop compiler whining.
#[cfg(test)]
fn suffixes<T: Clone>(stack: &Arc<CustomStack<T>>) -> CustomStack<Arc<CustomStack<T>>> {
	let tail_suffixes = match **stack {
		CustomStack::Empty => CustomStack::empty(),
		CustomStack::Cons { ref tail, .. } => suffixes(&tail)
	};

	return tail_suffixes.cons( stack.clone() );
}


#[test]
fn empty_is_empty() {
	let stack: CustomStack<()> = CustomStack::empty();

	assert!( stack.is_empty() );
	assert!( stack.size() == 0 );
}

#[test]
fn cons_is_not_empty() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(4);

	assert!( !stack.is_empty() );
	assert!( stack.size() == 1 );
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
fn size_multiple_items() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);

	assert!( stack.size() == 3 );
}

#[test]
fn get_valid() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);

	assert!( stack.get(1).unwrap() == 2 );
}

#[test]
fn get_out_of_range() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);

	assert!( stack.get(3).is_err() );
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

#[test]
fn update_valid() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
	let updated = stack.clone().update(1,10).unwrap();

	assert!(updated.head().unwrap() == 3);
	assert!(updated.tail().unwrap().head().unwrap() == 10);
	assert!(updated.tail().unwrap().tail().unwrap().head().unwrap() == 1);
}

#[test]
fn update_invalid() {
	let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
	let updated = stack.clone().update(4,10);

	assert!(updated.is_err());
}

#[test] 
fn suffixes_empty() {
	let stack: Arc<CustomStack<()>> = Arc::new( CustomStack::empty() );
	let suffixes = suffixes(&stack);

	// First suffix is empty list
	assert!( suffixes.head().unwrap().is_empty() );

	// No more suffixes
	assert!( suffixes.tail().unwrap().is_empty() );
}

#[test] 
fn suffixes_nonempty() {
	let stack: Arc<CustomStack<i32>> = Arc::new( CustomStack::empty().cons(1).cons(2) );
	let suffixes = suffixes(&stack);

	let suffix1 = suffixes.head().unwrap();
	assert!( suffix1.head().unwrap() == 2 );
	assert!( suffix1.tail().unwrap().head().unwrap() == 1 );
	assert!( suffix1.tail().unwrap().tail().unwrap().is_empty() );

	let suffix2 = suffixes.tail().unwrap().head().unwrap();
	assert!( suffix2.head().unwrap() == 1 );
	assert!( suffix2.tail().unwrap().is_empty() );

	let suffix3 = suffixes.tail().unwrap().tail().unwrap().head().unwrap();
	assert!( suffix3.is_empty() );

	assert!( suffixes.tail().unwrap().tail().unwrap().tail().unwrap().is_empty() );

}