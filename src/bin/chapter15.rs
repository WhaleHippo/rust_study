use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn sum(&self) -> i32 {
        match self {
            Self::Cons(value, next) => value + next.sum(),
            Self::Nil => 0,
        }
    }
}

struct SmallBox<T>(T);

impl<T> SmallBox<T> {
    const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for SmallBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

struct DropRecorder {
    label: &'static str,
    events: Rc<RefCell<Vec<&'static str>>>,
}

impl Drop for DropRecorder {
    fn drop(&mut self) {
        self.events.borrow_mut().push(self.label);
    }
}

fn drop_sequence() -> Vec<&'static str> {
    let events = Rc::new(RefCell::new(Vec::new()));
    {
        let _first = DropRecorder {
            label: "first",
            events: Rc::clone(&events),
        };
        let _second = DropRecorder {
            label: "second",
            events: Rc::clone(&events),
        };
    }
    events.take()
}

fn rc_counts() -> [usize; 4] {
    let first = Rc::new("shared");
    let initial = Rc::strong_count(&first);
    let second = Rc::clone(&first);
    let after_clone = Rc::strong_count(&first);
    let nested = {
        let third = Rc::clone(&first);
        Rc::strong_count(&third)
    };
    let after_drop = Rc::strong_count(&second);
    [initial, after_clone, nested, after_drop]
}

fn refcell_value() -> i32 {
    let value = RefCell::new(5);
    *value.borrow_mut() += 10;
    value.into_inner()
}

struct Node {
    parent: RefCell<Weak<Node>>,
}

fn weak_parent_lifecycle() -> [bool; 2] {
    let child = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
    });
    let parent = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
    });
    *child.parent.borrow_mut() = Rc::downgrade(&parent);
    let alive_before_drop = child.parent.borrow().upgrade().is_some();
    drop(parent);
    let alive_after_drop = child.parent.borrow().upgrade().is_some();
    [alive_before_drop, alive_after_drop]
}

fn main() {
    println!("Chapter 15: Smart Pointers");
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    let wrapped = SmallBox::new(15);
    println!("Box recursive list sum: {}", list.sum());
    println!("Deref custom wrapper: {}", *wrapped);
    println!("Drop order: {:?}", drop_sequence());
    println!("Rc strong counts: {:?}", rc_counts());
    println!("RefCell interior value: {}", refcell_value());
    println!(
        "Weak parent alive before/after drop: {:?}",
        weak_parent_lifecycle()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn recursive_list_sums_values_when_populated() {
        // Given: a recursive list whose links require Box indirection.
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));

        // When: every node is traversed.
        let total = list.sum();

        // Then: all values contribute to the sum.
        assert_eq!(total, 3);
    }

    #[test]
    fn recursive_list_returns_zero_when_empty() {
        // Given: the edge case of an empty recursive list.
        let list = List::Nil;

        // When: it is summed.
        let total = list.sum();

        // Then: the additive identity is returned.
        assert_eq!(total, 0);
    }

    #[test]
    fn custom_wrapper_dereferences_to_inner_value() {
        // Given: a small custom smart-pointer wrapper.
        let value = SmallBox::new(15);

        // When: the wrapper is dereferenced.
        let inner = *value;

        // Then: Deref exposes the wrapped value.
        assert_eq!(inner, 15);
    }

    #[test]
    fn drop_records_reverse_scope_order() {
        // Given: two values that record their own destruction.

        // When: their shared scope ends.
        let events = drop_sequence();

        // Then: RAII drops them in reverse declaration order.
        assert_eq!(events, ["second", "first"]);
    }

    #[test]
    fn shared_ownership_counts_regressions() {
        // Given: one Rc owner that is cloned in a nested scope.

        // When: strong counts are observed across clone and drop boundaries.
        let counts = rc_counts();

        // Then: ownership changes are exact and deterministic.
        assert_eq!(counts, [1, 2, 3, 2]);
    }

    #[test]
    fn interior_mutation_and_weak_links_do_not_leak() {
        // Given: RefCell mutation and a child holding a Weak parent link.

        // When: the value is mutated and the parent owner is dropped.
        let value = refcell_value();
        let parent_states = weak_parent_lifecycle();

        // Then: mutation is visible and the weak link cannot keep the parent alive.
        assert_eq!(value, 15);
        assert_eq!(parent_states, [true, false]);
    }
}
