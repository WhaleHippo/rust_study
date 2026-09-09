fn add_two(value: i32) -> i32 {
    value + 2
}

struct Guess {
    value: u32,
}

impl Guess {
    fn new(value: u32) -> Self {
        assert!(
            value <= 100,
            "Guess value must be less than or equal to 100, got {value}."
        );
        Self { value }
    }

    fn value(&self) -> u32 {
        self.value
    }
}

fn main() {
    println!("Chapter 11: Writing Automated Tests");
    let guess = Guess::new(42);
    println!("assert_eq!: add_two(40) = {}", add_two(40));
    println!(
        "boundary invariant: Guess::new({}) accepts values through 100.",
        guess.value()
    );
}

#[cfg(test)]
mod tests {
    use super::{Guess, add_two};

    #[test]
    fn add_two_when_given_two() {
        // Given: the number two.
        // When: two is added.
        let result = add_two(2);
        // Then: the result is four.
        assert_eq!(result, 4, "add_two should increase the input by two");
        assert_ne!(result, 2, "the result must differ from the original input");
    }

    #[test]
    fn add_two_when_given_a_negative_value() {
        // Given: a negative integer.
        // When: two is added.
        let result = add_two(-4);
        // Then: the arithmetic remains correct across zero.
        assert_eq!(result, -2);
    }

    #[test]
    #[should_panic(expected = "Guess value must be less than or equal to 100")]
    fn guess_new_when_value_is_above_the_allowed_range() {
        // Given: a value outside the lesson's allowed range.
        // When: a Guess is constructed.
        let _guess = Guess::new(101);
        // Then: the constructor panics with its documented message.
    }

    #[test]
    fn result_returning_test_when_guess_and_add_two_are_valid() -> Result<(), String> {
        // Given: a valid guess and an input near the expected sum.
        let guess = Guess::new(42);
        // When: the lesson values are read and calculated.
        let total = add_two(40);
        // Then: both outcomes can be checked while returning Result.
        if guess.value() != 42 {
            return Err("Guess constructor did not preserve its valid value".to_owned());
        }
        if total != 42 {
            return Err("add_two did not produce the expected total".to_owned());
        }
        Ok(())
    }

    #[test]
    fn guess_new_when_value_is_upper_boundary() {
        // Given: the largest permitted guess.
        // When: the boundary value is constructed.
        let guess = Guess::new(100);
        // Then: the constructor preserves the valid boundary.
        assert_eq!(guess.value(), 100);
    }
}
