use futures::{
    executor::block_on,
    future::{join, ready},
    stream::{self, StreamExt},
};

async fn joined_total(left: u32, right: u32) -> u32 {
    // `async fn`을 호출해도 즉시 계산되지 않고, 계산 절차를 담은 Future가 만들어진다. 실행은 그 Future가 poll될 때 시작된다.
    // `ready`는 첫 poll에서 준비된 값을 내놓는 Future이며, `join`은 두 Future를 함께 진행해 입력 순서의 결과 튜플을 만든다.
    let (left, right) = join(ready(left * 2), ready(right * 2)).await;
    // `.await`는 아직 준비되지 않았다면 현재 Future를 양보하고, 준비되면 중단했던 지점에서 값을 받아 계속 실행한다.
    async { left + right + 1 }.await
}

async fn collect_even(values: &[u32]) -> Vec<u32> {
    // 슬라이스의 복사 가능한 값을 순서대로 내보내는 Stream을 구성한다. 이 시점에도 스트림의 항목은 아직 소비되지 않는다.
    let mut values = stream::iter(values.iter().copied());
    let mut collected = Vec::new();

    // `next().await`는 다음 항목을 poll한다. 끝에 도달해 `None`이 되면 `while let` 조건이 맞지 않아 반복이 종료된다.
    while let Some(value) = values.next().await {
        // 짝수 조건을 통과한 값만 모아 stream 필터링 결과를 유지한다.
        if value.is_multiple_of(2) {
            collected.push(value);
        }
    }

    collected
}

fn main() {
    println!(
        "Chapter 17: Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams"
    );

    // 이 async 블록도 Future를 만든다. `block_on`이 간단한 실행기로서 완료될 때까지 poll해 동기 `main`에서 결과를 얻는다.
    let (total, even_values) = block_on(async {
        let total = joined_total(2, 3).await;
        let even_values = collect_even(&[1, 2, 3, 4]).await;
        (total, even_values)
    });

    println!("joined total: {total}");
    println!("streamed even values: {even_values:?}");
}

#[cfg(test)]
mod tests {
    use super::{collect_even, joined_total};
    use futures::executor::block_on;

    #[test]
    fn joins_ready_futures_when_values_differ() {
        // Given: two distinct values whose doubled sum is known.
        let values = (2, 3);
        // When: both ready futures are joined.
        let total = block_on(joined_total(values.0, values.1));
        // Then: both results and the composed bonus are included.
        assert_eq!(total, 11);
    }

    #[test]
    fn collects_even_values_when_stream_is_mixed() {
        // Given: a stream source containing odd and even values.
        let values = [1, 2, 3, 4];
        // When: the asynchronous stream is consumed.
        let collected = block_on(collect_even(&values));
        // Then: only the even values remain in source order.
        assert_eq!(collected, vec![2, 4]);
    }

    #[test]
    fn keeps_empty_stream_empty_when_no_values_exist() {
        // Given: an empty stream source.
        let values = [];
        // When: the asynchronous stream is consumed.
        let collected = block_on(collect_even(&values));
        // Then: no placeholder value is introduced.
        assert_eq!(collected, Vec::<u32>::new());
    }

    #[test]
    fn includes_composed_bonus_when_joined_values_are_zero() {
        // Given: two zero-valued ready futures.
        let values = (0, 0);
        // When: both futures are joined and composed.
        let total = block_on(joined_total(values.0, values.1));
        // Then: the asynchronous bonus still contributes one.
        assert_eq!(total, 1);
    }

    // [학습 실습: C17-01]
    // 목표: 홀수만 있는 비동기 stream에서 짝수 결과를 하나도 수집하지 않는 계약을 고정한다.
    // 학습자 행동: 짝수 조건을 로컬에서 제거해 모든 stream 값을 수집한 뒤, 짝수만 선택하는 비동기 소비 로직을 직접 다시 구현한다.
    // RED: 워크스페이스 루트에서 `cargo test -p chapter17-fundamentals-of-asynchronous-programming all_odd_stream_collects_no_even_values`를 실행하면 빈 결과 기대값 대신 홀수 값이 들어와 실패한다.
    // GREEN: `value.is_multiple_of(2)`를 복원하면 홀수 stream에서 빈 결과를 유지한다.
    // 힌트: stream의 각 값을 `.await`로 받은 뒤 `collected`에 넣기 전 조건을 확인한다.
    #[test]
    fn all_odd_stream_collects_no_even_values() {
        // 준비: 홀수만 포함한 stream 원본을 준비한다.
        let values = [1, 3, 5];

        // 실행: 비동기 stream을 끝까지 소비한다.
        let collected = block_on(collect_even(&values));

        // 검증: 짝수 필터를 통과한 값이 없어 빈 결과가 된다.
        assert_eq!(collected, Vec::<u32>::new());
    }
}
