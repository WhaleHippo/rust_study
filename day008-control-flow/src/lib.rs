#![forbid(unsafe_code)]

#[cfg(feature = "learner-practice")]
pub mod practice;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Signal {
    Red,
    Yellow,
    Green,
}

pub const fn countdown_steps(remaining: u8) -> u8 {
    if remaining == 0 {
        return 0;
    }

    let mut left = remaining;
    let mut steps = 0;
    while left > 0 {
        left -= 1;
        steps += 1;
    }
    steps
}

pub const fn signal_action(signal: Signal) -> &'static str {
    match signal {
        Signal::Red => "멈춤",
        Signal::Yellow => "주의",
        Signal::Green => "진행",
    }
}

pub const fn loop_result(target: u8) -> u8 {
    let mut current = 0;
    loop {
        if current == target {
            break current.saturating_mul(2);
        }
        current += 1;
    }
}

pub fn sum_for(values: &[u8]) -> u16 {
    let mut total = 0_u16;
    for value in values {
        total = total.saturating_add(u16::from(*value));
    }
    total
}

pub fn labeled_pair(limit: u8, target_sum: u8) -> Option<(u8, u8)> {
    let mut found = None;
    'outer: for row in 0..limit {
        for column in 0..limit {
            if row.saturating_add(column) == target_sum {
                found = Some((row, column));
                break 'outer;
            }
        }
    }
    found
}

#[cfg(test)]
mod tests {
    use super::{Signal, countdown_steps, labeled_pair, loop_result, signal_action, sum_for};

    #[test]
    fn if_selects_a_value_when_counting_down() {
        // Given
        let remaining = 3;

        // When
        let steps = countdown_steps(remaining);

        // Then
        assert_eq!(steps, 3);
    }

    #[test]
    fn match_handles_every_signal_variant() {
        // Given
        let signals = [Signal::Red, Signal::Yellow, Signal::Green];

        // When
        let actions = signals.map(signal_action);

        // Then
        assert_eq!(actions, ["멈춤", "주의", "진행"]);
    }

    #[test]
    fn loop_break_supplies_the_result_value() {
        // Given
        let target = 21;

        // When
        let result = loop_result(target);

        // Then
        assert_eq!(result, 42);
    }

    #[test]
    fn for_visits_each_array_element() {
        // Given
        let values = [10, 20, 12];

        // When
        let total = sum_for(&values);

        // Then
        assert_eq!(total, 42);
    }

    #[test]
    fn label_breaks_both_nested_loops_after_a_match() {
        // Given
        let target_sum = 3;

        // When
        let pair = labeled_pair(4, target_sum);

        // Then
        assert_eq!(pair, Some((0, 3)));
    }
}
