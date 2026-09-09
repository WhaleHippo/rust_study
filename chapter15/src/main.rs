use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

// 재귀 열거형은 `Cons` 안에 다시 `List`를 직접 넣으면 크기를 계산할 수 없다.
// `Box<List>`는 힙의 다음 노드를 가리키는 고정 크기 포인터이므로, 각 노드의 크기가 확정된다.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    fn sum(&self) -> i32 {
        // `Box`는 소유한 다음 노드를 역참조해 재귀 호출하며, 최상위 `List` 소유자가 사라지면 연결된 노드도 차례로 해제된다.
        match self {
            Self::Cons(value, next) => value + next.sum(),
            Self::Nil => 0,
        }
    }
}

// 튜플 구조체가 값을 소유하지만, `Deref` 구현으로 포인터처럼 값을 빌려 줄 수 있다.
struct SmallBox<T>(T);

impl<T> SmallBox<T> {
    const fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> Deref for SmallBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // `*wrapped`는 이 메서드를 호출해 얻은 참조를 다시 역참조한다.
        &self.0
    }
}

// `Drop`은 값이 스코프를 벗어날 때 자동 정리를 관찰하는 RAII 훅이다.
struct DropRecorder {
    label: &'static str,
    events: Rc<RefCell<Vec<&'static str>>>,
}

impl Drop for DropRecorder {
    fn drop(&mut self) {
        // `drop`을 직접 호출하지 않는다. 컴파일러가 스코프 종료 때 정확히 한 번 호출한다.
        self.events.borrow_mut().push(self.label);
    }
}

fn drop_sequence() -> Vec<&'static str> {
    let events = Rc::new(RefCell::new(Vec::new()));
    {
        // 지역 변수는 선언의 역순으로 파기되므로 `_second`가 먼저, `_first`가 나중에 Drop 된다.
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
    // `Rc` 복제는 문자열 데이터를 복제하지 않고 같은 할당의 강한 소유자 수만 늘린다.
    // 단일 스레드 참조 계수이므로 여러 스레드에서는 `Arc`를 사용해야 한다.
    let first = Rc::new("shared");
    let initial = Rc::strong_count(&first);
    let second = Rc::clone(&first);
    let after_clone = Rc::strong_count(&first);
    let nested = {
        // 중첩 스코프가 끝나면 `third`가 Drop 되어 강한 계수도 다시 감소한다.
        let third = Rc::clone(&first);
        Rc::strong_count(&third)
    };
    let after_drop = Rc::strong_count(&second);
    [initial, after_clone, nested, after_drop]
}

fn refcell_value() -> i32 {
    // `RefCell`은 불변 참조만 있어도 내부 가변성을 허용하고, 빌림 규칙은 컴파일 시간이 아니라 실행 시간에 검사한다.
    let value = RefCell::new(5);
    *value.borrow_mut() += 10;
    value.into_inner()
}

struct Node {
    // 부모는 소유하지 않는 `Weak` 링크로 둬서 부모-자식 순환 참조가 메모리를 영구히 붙잡지 않게 한다.
    parent: RefCell<Weak<Node>>,
}

fn weak_parent_lifecycle() -> [bool; 2] {
    let child = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
    });
    let parent = Rc::new(Node {
        parent: RefCell::new(Weak::new()),
    });
    // `downgrade`는 강한 소유자 수를 늘리지 않으며, `upgrade`는 대상 생존 여부를 `Option<Rc<_>>`로 알려 준다.
    *child.parent.borrow_mut() = Rc::downgrade(&parent);
    let alive_before_drop = child.parent.borrow().upgrade().is_some();
    // 마지막 강한 `Rc`인 부모를 버린 뒤에는 약한 참조만 남아 `upgrade`가 `None`이 된다.
    drop(parent);
    let alive_after_drop = child.parent.borrow().upgrade().is_some();
    [alive_before_drop, alive_after_drop]
}

fn main() {
    println!("Chapter 15: Smart Pointers");
    // `Box`의 간접 참조가 유한한 크기의 재귀 리스트를 가능하게 한다.
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
    fn recursive_list_sums_three_nodes() {
        // Given: a three-node recursive list whose links require Box indirection.
        let list = List::Cons(
            1,
            Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
        );

        // When: every node is traversed.
        let total = list.sum();

        // Then: all three values contribute to the sum.
        assert_eq!(total, 6);
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
