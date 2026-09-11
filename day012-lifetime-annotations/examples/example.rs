#![forbid(unsafe_code)]

use day012_lifetime_annotations::longer;

fn main() {
    let first = String::from("lifetime");
    let second = String::from("borrow");
    let selected = longer(&first, &second);

    println!("선택: {selected}");
}
