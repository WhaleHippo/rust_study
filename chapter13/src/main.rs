fn doubled_evens(values: &[i32]) -> Vec<i32> {
    values
        .iter()
        .filter(|value| **value % 2 == 0)
        .map(|value| value * 2)
        .collect()
}

fn call_fn<F>(operation: F) -> i32
where
    F: Fn(i32) -> i32,
{
    operation(40)
}

fn call_fn_mut<F>(mut operation: F)
where
    F: FnMut(i32),
{
    operation(1);
    operation(2);
}

fn call_fn_once<F>(operation: F) -> String
where
    F: FnOnce() -> String,
{
    operation()
}

fn closure_summary() -> (i32, Vec<i32>, String) {
    let offset = 2;
    let add_offset = |value| value + offset;
    let mut observed = Vec::new();
    call_fn_mut(|value| observed.push(value));
    let owned_label = String::from("moved into FnOnce");
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

    #[test]
    fn counter_when_end_is_one_stays_exhausted() {
        // Given: a custom iterator containing one value.
        let mut counter = super::Counter::new(1);
        // When: all positions including one beyond the end are requested.
        let values = [counter.next(), counter.next(), counter.next()];
        // Then: it yields one once and remains exhausted.
        assert_eq!(values, [Some(1), None, None]);
    }
}
