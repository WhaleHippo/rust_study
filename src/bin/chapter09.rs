#[derive(Debug, PartialEq, Eq)]
enum ParsePositiveError {
    Malformed,
    OutOfRange,
    NotPositive,
}

fn parse_positive(input: &str) -> Result<u32, ParsePositiveError> {
    let number = match input {
        "" => return Err(ParsePositiveError::Malformed),
        text => text.parse::<u32>().map_err(|_| {
            if text.bytes().all(|byte| byte.is_ascii_digit()) {
                ParsePositiveError::OutOfRange
            } else {
                ParsePositiveError::Malformed
            }
        })?,
    };
    match number {
        0 => Err(ParsePositiveError::NotPositive),
        positive => Ok(positive),
    }
}

fn first_positive(values: &[&str]) -> Option<u32> {
    values.iter().find_map(|value| parse_positive(value).ok())
}

fn describe(value: Option<u32>) -> &'static str {
    match value {
        Some(_) => "a positive value",
        None => "no positive value",
    }
}

fn main() {
    println!("Chapter 09: Error Handling");
    println!("{}", describe(first_positive(&["0", "bad", "12"])));
}

#[cfg(test)]
mod tests {
    use super::parse_positive;

    #[test]
    fn parse_positive_when_input_is_a_positive_integer() {
        // Given: a positive integer written as text.
        // When: it is parsed.
        let value = parse_positive("7");
        // Then: parsing returns that integer.
        assert_eq!(value, Ok(7));
    }

    #[test]
    fn parse_positive_when_input_is_malformed() {
        // Given: text that is not an unsigned integer.
        // When: it is parsed.
        let value = parse_positive("seven");
        // Then: the typed malformed-input error is returned.
        assert_eq!(value, Err(super::ParsePositiveError::Malformed));
    }

    #[test]
    fn parse_positive_when_input_exceeds_u32() {
        // Given: decimal text larger than u32::MAX.
        // When: it is parsed.
        let value = parse_positive("4294967296");
        // Then: the typed out-of-range error is returned.
        assert_eq!(value, Err(super::ParsePositiveError::OutOfRange));
    }

    #[test]
    fn first_positive_when_prior_values_fail_to_parse() {
        // Given: zero and malformed values before a valid positive value.
        // When: the first positive value is selected through Option.
        let value = super::first_positive(&["0", "bad", "12"]);
        // Then: parsing continues without panicking on expected bad input.
        assert_eq!(value, Some(12));
    }

    #[test]
    fn describe_when_no_positive_value_exists() {
        // Given: the absence of a parsed positive value.
        // When: the Option is matched for display.
        let description = super::describe(None);
        // Then: the empty case has an explicit description.
        assert_eq!(description, "no positive value");
    }
}
