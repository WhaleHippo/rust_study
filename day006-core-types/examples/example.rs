use day006_core_types::{CORE_SCALARS, array_and_tuple, range_totals, safe_array_value};

fn main() {
    let (array, tuple) = array_and_tuple();
    let totals = range_totals();

    println!("스칼라={CORE_SCALARS:?}");
    println!("배열={array:?}, 튜플={tuple:?}");
    println!("1..4 합={}, 1..=4 합={}", totals.0, totals.1);
    println!(
        "안전한 경계 접근={:?}",
        safe_array_value(&array, array.len())
    );
}
