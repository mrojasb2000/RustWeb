use async_std;
use futures::executor::block_on;
use futures::future::join_all;
use std::vec::Vec;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let now = time::Instant::now();
    let async_outcome = async {
        // 1. Define futures
        let mut futures_vec = Vec::new();
        let outcome_one = do_something(2);
        let outcome_two = do_something(3);
        // 2. Push futures into the vector
        futures_vec.push(outcome_one);
        futures_vec.push(outcome_two);
        // 3. Spawn tasks and collect handles
        let handles = futures_vec
            .into_iter()
            .map(async_std::task::spawn)
            .collect::<Vec<_>>();
        // 4. Wait for all tasks to complete
        let results = join_all(handles).await;
        return results.into_iter().sum::<i8>();
    };
    let outcome = block_on(async_outcome);
    println!("time elapsed: {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
}
