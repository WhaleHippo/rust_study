fn doubled_evens(values: &[i32]) -> Vec<i32> {
    // 어댑터는 지연 평가된다. 아래 filter와 map은 `collect`가 실제로 반복자를 소비할 때 비로소 실행된다.
    values
        // `iter()`는 원소를 빌려 `&i32`를 만들므로, filter 안의 `&&i32`에서 값을 읽기 위해 두 번 역참조한다.
        .iter()
        .filter(|value| **value % 2 == 0)
        .map(|value| value * 2)
        // `collect`가 지연된 파이프라인을 끝까지 소비하여 소유한 `Vec<i32>` 결과를 만든다.
        .collect()
}

fn call_fn<F>(operation: F) -> i32
where
    // 불변으로만 캡처한 클로저는 여러 번 호출할 수 있는 Fn 요구사항을 만족한다.
    F: Fn(i32) -> i32,
{
    operation(40)
}

fn call_fn_mut<F>(mut operation: F)
where
    // 캡처한 환경을 변경하는 클로저는 FnMut이며, 호출하려면 매개변수 바인딩도 가변이어야 한다.
    F: FnMut(i32),
{
    operation(1);
    operation(2);
}

fn call_fn_once<F>(operation: F) -> String
where
    // 소유한 캡처를 이동해 소비할 수 있는 클로저는 한 번만 호출 가능한 FnOnce다.
    F: FnOnce() -> String,
{
    operation()
}

fn closure_summary() -> (i32, Vec<i32>, String) {
    let offset = 2;
    // 이 클로저는 offset을 불변으로 빌리므로 Fn으로 전달할 수 있다.
    let add_offset = |value| value + offset;
    let mut observed = Vec::new();
    // observed에 push하려면 가변 빌림이 필요하므로 이 클로저는 FnMut로 호출된다.
    call_fn_mut(|value| observed.push(value));
    let owned_label = String::from("moved into FnOnce");
    // `move`는 owned_label의 소유권을 클로저로 옮긴다. 반환할 때 값을 꺼내므로 FnOnce가 된다.
    let take_label = move || owned_label;

    (call_fn(add_offset), observed, call_fn_once(take_label))
}

struct Counter {
    current: u8,
    end: u8,
}

impl Counter {
    fn new(end: u8) -> Self {
        Self { current: 0, end }
    }
}

impl Iterator for Counter {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // 이 Counter는 end에 도달하면 current가 더 변하지 않으므로 이후 호출도 계속 None인 영구 소진 상태가 된다.
        // Iterator 계약에서 첫 None은 끝을 알리는 값이며, 다음 호출에서 다시 Some을 반환할 수 없다.
        if self.current == self.end {
            None
        } else {
            self.current += 1;
            Some(self.current)
        }
    }
}

fn main() {
    println!("Chapter 13: Functional Language Features: Iterators and Closures");
    let (fn_value, fn_mut_values, fn_once_label) = closure_summary();
    println!("closure traits: Fn={fn_value}, FnMut={fn_mut_values:?}, FnOnce={fn_once_label}");
    println!("filter + map adapters: {:?}", doubled_evens(&[1, 2, 3, 4]));
    println!(
        "custom Iterator yields then exhausts: {:?}",
        // `collect`는 Counter를 소유해 끝까지 next를 호출하므로, 수집 뒤에는 같은 반복자를 다시 사용할 수 없다.
        Counter::new(3).collect::<Vec<_>>()
    );
}

#[cfg(test)]
mod tests {
    use super::doubled_evens;

    #[test]
    fn doubled_evens_when_values_include_even_numbers() {
        // Given: a mixture of odd and even values.
        let values = [1, 2, 3, 4];
        // When: even values are filtered and mapped.
        // `collect`가 없으면 이 어댑터 조합은 결과를 계산하지 않는다는 점을 이 예제와 함께 확인한다.
        let result = doubled_evens(&values);
        // Then: only doubled even values remain.
        assert_eq!(result, vec![4, 8]);
    }

    #[test]
    fn closure_summary_when_captures_have_different_traits() {
        // Given: closures that borrow, mutate, and move captured values.
        // When: each closure is called through its matching Fn trait.
        let summary = super::closure_summary();
        // Then: Fn, FnMut, and FnOnce each produce their expected value.
        assert_eq!(summary, (42, vec![1, 2], String::from("moved into FnOnce")));
    }

    #[test]
    fn counter_when_end_is_three() {
        // Given: a custom iterator with three values to yield.
        // When: it is consumed into a vector.
        let values = super::Counter::new(3).collect::<Vec<_>>();
        // Then: next yields consecutive values and then ends.
        assert_eq!(values, vec![1, 2, 3]);
    }

    #[test]
    fn counter_when_end_is_zero() {
        // Given: a custom iterator with no values to yield.
        // When: next is requested immediately.
        let next_value = super::Counter::new(0).next();
        // Then: the iterator reports completion.
        assert_eq!(next_value, None);
    }

    // [학습 실습: C13-01]
    // 목표: Iterator가 소진된 뒤에는 계속 None을 반환하는 계약을 확인한다.
    // 학습자 행동: 소진 분기를 잠시 `Some(self.current)`로 바꾸고 아래 테스트만 실행한다.
    // RED: 워크스페이스 루트에서 `cargo test -p chapter13-functional-language-features-iterators-and-closures counter_when_end_is_one_stays_exhausted`를 실행하면 `[Some(1), None, None]` 기대값과 달라 실패한다.
    // GREEN: 소진 분기를 None으로 복원해 이후 호출도 영구적으로 None이 되게 한다.
    // 힌트: `current == end`일 때 값을 다시 내보내면 끝 상태가 아니라 같은 값을 반복하는 상태가 된다.
    #[test]
    fn counter_when_end_is_one_stays_exhausted() {
        // 준비: 하나의 값을 내보내는 사용자 정의 반복자를 준비한다.
        let mut counter = super::Counter::new(1);
        // 실행: 끝을 지난 호출까지 세 번 next를 요청한다.
        let values = [counter.next(), counter.next(), counter.next()];
        // 검증: 한 번만 Some(1)을 내보내고 계속 소진 상태를 유지한다.
        // 이 구현에서는 None 뒤에 Some으로 되돌아가는 상태 전이가 없음을 세 번째 호출까지 확인한다.
        assert_eq!(values, [Some(1), None, None]);
    }
}
