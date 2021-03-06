use std::sync::Arc;

#[derive(Debug)]
pub enum StackError {
    NoSuchElementException,
    IndexOutOfRange,
}

/// A trait representing an immutable Stack type.
pub trait Stack<T: Clone> {
    /// Returns an empty stack.
    fn empty() -> Self;
    /// Tests whether a stack is empty.
    fn is_empty(&self) -> bool;
    /// Returns a new stack with `value` as its head.
    fn cons(&self, value: T) -> Self;
    /// Returns the head item of the stack.
    ///
    /// # Failures
    ///
    /// Returns `StackError::NoSuchElementException` if this is an empty stack.
    fn head(&self) -> Result<T, StackError>;
    /// Returns the tail of the stack (everything but the head).
    ///
    /// # Failures
    /// 
    /// Returns `StackError::NoSuchElementException` if this is an empty stack.
    fn tail(&self) -> Result<Arc<Self>, StackError>;
    /// Returns a stack identical to this one except that the value at index `i`
    /// is replaced by `value`.
    ///
    /// # Failures
    ///
    /// Returns `StackError::IndexOutOfRange` if `i` is greater than the greatest 
    /// index currently in this stack (size - 1).
    fn update(&self, i: u32, value: T) -> Result<Self, StackError>;
    /// Returns the number of items in this stack.
    fn size(&self) -> u32;
    /// Returns the item currently at index `i` in the stack.
    ///
    /// # Failures
    ///
    /// Returns `StackError::IndexOutOfRange` if `i` is greater than the greatest 
    /// index currently in this stack (size - 1).
    fn get(&self, i: u32) -> Result<T, StackError>;
}

/// An immutable Stack implemented as a singly-linked list.
///
/// This is the `CustomStack` type described in chapter 2 of PFDL.
#[derive(Debug, Clone)]
pub enum CustomStack<T> {
    Empty,
    Cons {
        value: T,
        tail: Arc<CustomStack<T>>,
    },
}

impl<T: Clone> Stack<T> for CustomStack<T> {
    fn empty() -> CustomStack<T> {
        return CustomStack::Empty;
    }
    fn is_empty(&self) -> bool {
        return match *self {
            CustomStack::Empty => true,
            _ => false,
        };
    }
    fn cons(&self, value: T) -> Self {
        return CustomStack::Cons {
            value: value,
            tail: Arc::new(self.clone()),
        };
    }
    fn head(&self) -> Result<T, StackError> {
        return match *self {
            CustomStack::Empty => Err(StackError::NoSuchElementException),
            CustomStack::Cons { ref value, .. } => Ok(value.clone()),
        };
    }
    fn tail(&self) -> Result<Arc<Self>, StackError> {
        return match *self {
            CustomStack::Empty => Err(StackError::NoSuchElementException),
            CustomStack::Cons { ref tail, .. } => Ok(tail.clone()),
        };
    }
    fn update(&self, i: u32, new_value: T) -> Result<Self, StackError> {
        return match *self {
            CustomStack::Empty => Err(StackError::IndexOutOfRange),
            CustomStack::Cons { ref value, ref tail } => match i {
                0 => Ok(tail.clone().cons(new_value)),
                _ => {
                    let updated_tail = try!(tail.update(i - 1, new_value));
                    Ok(updated_tail.cons(value.clone()))
                }
            },
        };
    }
    fn size(&self) -> u32 {
        return match *self {
            CustomStack::Empty => 0,
            CustomStack::Cons { ref tail, .. } => 1 + tail.size(),
        };
    }
    fn get(&self, i: u32) -> Result<T, StackError> {
        return match *self {
            CustomStack::Empty => Err(StackError::IndexOutOfRange),
            CustomStack::Cons { ref value, ref tail } => match i {
                0 => Ok(value.clone()),
                _ => tail.get(i - 1),
            },
        };
    }
}

// Only compile this in tests to stop compiler whining.
#[cfg(test)]
fn suffixes<T: Clone>(stack: &Arc<CustomStack<T>>) -> CustomStack<Arc<CustomStack<T>>> {
    let tail_suffixes = match **stack {
        CustomStack::Empty => CustomStack::empty(),
        CustomStack::Cons { ref tail, .. } => suffixes(&tail),
    };

    return tail_suffixes.cons(stack.clone());
}


#[test]
fn empty_is_empty() {
    let stack: CustomStack<()> = CustomStack::empty();

    assert!(stack.is_empty());
    assert!(stack.size() == 0);
}

#[test]
fn cons_is_not_empty() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(4);

    assert!(!stack.is_empty());
    assert!(stack.size() == 1);
}

#[test]
fn head_empty_error() {
    let stack: CustomStack<()> = CustomStack::empty();

    assert!(stack.head().is_err());
}

#[test]
fn head_last_item() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(5).cons(6);
    let head = stack.head();

    assert!(head.is_ok());
    assert!(head.unwrap() == 6);
}


#[test]
fn tail_empty_is_error() {
    let stack: CustomStack<()> = CustomStack::empty();

    assert!(stack.tail().is_err());
}


#[test]
fn head_after_tail() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
    let tailtail = stack.tail().unwrap().tail().unwrap();

    assert!(tailtail.head().unwrap() == 1);
}

#[test]
fn size_multiple_items() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);

    assert!(stack.size() == 3);
}

#[test]
fn get_valid() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);

    assert!(stack.get(1).unwrap() == 2);
}

#[test]
fn get_out_of_range() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);

    assert!(stack.get(3).is_err());
}

#[test]
fn cloneable() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
    let stack2 = stack.clone();

    let tailtail = stack.tail().unwrap().tail().unwrap();
    let tail = stack2.tail().unwrap();

    assert!(tailtail.head().unwrap() == 1);
    assert!(tail.head().unwrap() == 2);
}

#[test]
fn update_valid() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
    let updated = stack.clone().update(1, 10).unwrap();

    assert!(updated.size() == 3);
    assert!(updated.get(0).unwrap() == 3);
    assert!(updated.get(1).unwrap() == 10);
    assert!(updated.get(2).unwrap() == 1);

    // And stack is unchanged (I think the typesystem ensures this?)
    assert!(stack.size() == 3);
    assert!(stack.get(0).unwrap() == 3);
    assert!(stack.get(1).unwrap() == 2);
    assert!(stack.get(2).unwrap() == 1);
}

#[test]
fn update_invalid() {
    let stack: CustomStack<i32> = CustomStack::empty().cons(1).cons(2).cons(3);
    let updated = stack.clone().update(4, 10);

    assert!(updated.is_err());
}

#[test]
fn suffixes_empty() {
    let stack: Arc<CustomStack<()>> = Arc::new(CustomStack::empty());
    let suffixes = suffixes(&stack);

    assert!(suffixes.size() == 1);
    assert!(suffixes.get(0).unwrap().is_empty());
}

#[test]
fn suffixes_nonempty() {
    let stack: Arc<CustomStack<i32>> = Arc::new(CustomStack::empty().cons(1).cons(2));
    let suffixes = suffixes(&stack);

    assert!(suffixes.size() == 3);

    let suffix1 = suffixes.get(0).unwrap();
    assert!(suffix1.size() == 2);
    assert!(suffix1.get(0).unwrap() == 2);
    assert!(suffix1.get(1).unwrap() == 1);

    let suffix2 = suffixes.get(1).unwrap();
    assert!(suffix2.size() == 1);
    assert!(suffix2.get(0).unwrap() == 1);

    let suffix3 = suffixes.get(2).unwrap();
    assert!(suffix3.is_empty());

}
