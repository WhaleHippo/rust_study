use day003_expressions_and_places::{block_result, update_second};

fn main() {
    let answer = block_result(40);
    let mut pair = (answer, 0);
    update_second(&mut pair);

    println!("block value: {answer}");
    println!("mutated tuple: {pair:?}");
}
