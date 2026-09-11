use day005_bindings_and_constants::{
    COURSE_NAME, MINUTES_PER_HOUR, binding_snapshot, hours_to_minutes,
};

fn main() {
    let (immutable, mutable, shadowed_length) = binding_snapshot(3);

    println!("불변={immutable}, mut 결과={mutable}, shadowing 뒤 길이={shadowed_length}");
    println!(
        "과정={COURSE_NAME}, 시간당 {MINUTES_PER_HOUR}분, 2시간={}분",
        hours_to_minutes(2)
    );
}
