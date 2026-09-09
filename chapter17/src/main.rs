use futures::{
    executor::block_on,
    future::{join, ready},
    stream::{self, StreamExt},
};

async fn joined_total(left: u32, right: u32) -> u32 {
    let (left, right) = join(ready(left * 2), ready(right * 2)).await;
    async { left + right + 1 }.await
}

async fn collect_even(values: &[u32]) -> Vec<u32> {
    let mut values = stream::iter(values.iter().copied());
    let mut collected = Vec::new();

    while let Some(value) = values.next().await {
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

    #[test]
    fn all_odd_stream_collects_no_even_values() {
        // Given: a stream source containing only odd values.
        let values = [1, 3, 5];

        // When: the asynchronous stream is consumed.
        let collected = block_on(collect_even(&values));

        // Then: no values pass the even filter.
        assert_eq!(collected, Vec::<u32>::new());
    }
}
