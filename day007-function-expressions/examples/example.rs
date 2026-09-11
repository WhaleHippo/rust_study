use day007_function_expressions::{
    apply, discarded_expression, double, explicit_return, tail_expression,
};

fn main() {
    let function_item = double;
    let function_pointer: fn(i32) -> i32 = function_item;
    let mut observed = 0;

    println!("꼬리 식={}", tail_expression(40));
    discarded_expression(40, &mut observed);
    println!("세미콜론 뒤 unit=(), 호출 중 기록한 값={observed}");
    println!("명시적 조기 return={}", explicit_return(-3));
    println!("함수 포인터 호출={}", apply(function_pointer, 21));
}
