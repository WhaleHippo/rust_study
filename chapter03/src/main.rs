const HOURS_IN_SECONDS: u32 = 60 * 60;

const fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    celsius * 9.0 / 5.0 + 32.0
}

const fn plus_one(value: i32) -> i32 {
    value + 1
}

fn main() {
    println!("Chapter 03: Common Programming Concepts");

    let language = "Rust";
    let mut lessons = 3;
    lessons += 1;
    let language = language.len();
    let signed: i8 = -8;
    let decimal = 2.5_f32;
    let initial = 'R';
    let truth = true;
    let pair = (language, signed);
    let chapters = [3, 4, 5, 6];

    let conditional = if truth { plus_one(4) } else { 0 };
    let mut loop_count = 0;
    let loop_result = loop {
        loop_count += 1;
        if loop_count == 2 {
            break loop_count * 10;
        }
    };
    let mut while_index = 0;
    while while_index < chapters.len() {
        while_index += 1;
    }
    let mut for_total = 0;
    for chapter in chapters {
        for_total += chapter;
    }

    let cold_celsius = -10.0;
    let temperature_description = if cold_celsius < 0.0 {
        "below freezing"
    } else {
        "at or above freezing"
    };

    println!("bindings: lessons={lessons}, shadowed_language_length={language}");
    println!("scalars: {signed}, {decimal}, {initial}, {truth}");
    println!("tuple_first={}, array_count={while_index}", pair.0);
    println!("control_flow: if={conditional}, loop={loop_result}, for={for_total}");
    println!("constant_seconds_per_hour={HOURS_IN_SECONDS}");
    println!("0C is {}F", celsius_to_fahrenheit(0.0));
    println!(
        "negative temperature: {cold_celsius}C is {}F ({temperature_description})",
        celsius_to_fahrenheit(cold_celsius)
    );
}

#[cfg(test)]
mod tests {
    use super::celsius_to_fahrenheit;

    #[test]
    fn converts_freezing_point_when_celsius_is_zero() {
        // Given: water at its freezing point in Celsius.
        let celsius = 0.0;
        // When: the temperature is converted to Fahrenheit.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // Then: it is the Fahrenheit freezing point.
        assert_eq!(fahrenheit, 32.0);
    }

    #[test]
    fn converts_boiling_point_when_celsius_is_one_hundred() {
        // Given: water at its boiling point in Celsius.
        let celsius = 100.0;
        // When: the temperature is converted to Fahrenheit.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // Then: it is the Fahrenheit boiling point.
        assert_eq!(fahrenheit, 212.0);
    }

    #[test]
    fn preserves_shared_scale_point_when_celsius_is_negative_forty() {
        // Given: the temperature where both scales have the same value.
        let celsius = -40.0;
        // When: the temperature is converted to Fahrenheit.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // Then: the value remains negative forty.
        assert_eq!(fahrenheit, -40.0);
    }

    #[test]
    fn converts_negative_temperature_when_celsius_is_below_zero() {
        // Given: a negative Celsius temperature outside the shared scale point.
        let celsius = -10.0;
        // When: the temperature is converted to Fahrenheit.
        let fahrenheit = celsius_to_fahrenheit(celsius);
        // Then: the conversion follows the same formula below zero.
        assert_eq!(fahrenheit, 14.0);
    }
}
