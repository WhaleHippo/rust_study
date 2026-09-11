#![forbid(unsafe_code)]

use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "learner-practice")]
pub mod practice;

type Events = Rc<RefCell<Vec<&'static str>>>;

struct DropRecorder {
    name: &'static str,
    events: Events,
}

impl DropRecorder {
    fn new(name: &'static str, events: &Events) -> Self {
        Self {
            name,
            events: Rc::clone(events),
        }
    }
}

impl Drop for DropRecorder {
    fn drop(&mut self) {
        self.events.borrow_mut().push(self.name);
    }
}

fn recorded(events: &Events) -> Vec<&'static str> {
    events.borrow().clone()
}

/// 한 필드를 이동한 뒤 이동되지 않은 필드는 계속 사용할 수 있음을 보여 준다.
#[must_use]
pub fn partial_move(name: String, skills: Vec<String>) -> (String, usize) {
    struct Profile {
        name: String,
        skills: Vec<String>,
    }

    let profile = Profile { name, skills };
    let moved_name = profile.name;
    let skill_count = profile.skills.len();
    (moved_name, skill_count)
}

/// 같은 블록의 지역 값이 선언의 역순으로 소멸하는 모습을 기록한다.
#[must_use]
pub fn local_drop_order() -> Vec<&'static str> {
    let events = Rc::new(RefCell::new(Vec::new()));
    {
        let _first = DropRecorder::new("first", &events);
        let _second = DropRecorder::new("second", &events);
    }
    recorded(&events)
}

/// 구조체 필드가 선언 순서대로 소멸하는 모습을 기록한다.
#[must_use]
pub fn field_drop_order() -> Vec<&'static str> {
    struct Pair {
        _first: DropRecorder,
        _second: DropRecorder,
    }

    let events = Rc::new(RefCell::new(Vec::new()));
    {
        let _pair = Pair {
            _first: DropRecorder::new("first", &events),
            _second: DropRecorder::new("second", &events),
        };
    }
    recorded(&events)
}

/// `std::mem::drop`이 블록 끝보다 먼저 소유 값을 소비하는 모습을 기록한다.
#[must_use]
pub fn early_drop_order() -> Vec<&'static str> {
    let events = Rc::new(RefCell::new(Vec::new()));
    {
        let _later = DropRecorder::new("later", &events);
        let early = DropRecorder::new("early", &events);
        std::mem::drop(early);
    }
    recorded(&events)
}

#[cfg(test)]
mod tests {
    #[test]
    fn partial_move_preserves_access_to_unmoved_field() {
        // Given
        let name = String::from("Ferris");
        let skills = vec![String::from("ownership"), String::from("borrowing")];

        // When
        let summary = super::partial_move(name, skills);

        // Then
        assert_eq!(summary, (String::from("Ferris"), 2));
    }

    #[test]
    fn local_values_drop_in_reverse_declaration_order() {
        // Given / When
        let events = super::local_drop_order();

        // Then
        assert_eq!(events, ["second", "first"]);
    }

    #[test]
    fn struct_fields_drop_in_declaration_order() {
        // Given / When
        let events = super::field_drop_order();

        // Then
        assert_eq!(events, ["first", "second"]);
    }

    #[test]
    fn mem_drop_releases_a_value_before_scope_end() {
        // Given / When
        let events = super::early_drop_order();

        // Then
        assert_eq!(events, ["early", "later"]);
    }
}
