use futures::executor::block_on;
use std::{thread, time};

async fn do_something(number: i8) -> i8 {
    println!("number {} is running", number);
    let two_seconds = time::Duration::new(2, 0);
    thread::sleep(two_seconds);
    return 2;
}

fn main() {
    let now = time::Instant::now();
    let future_three = async {
        let outcome_one = do_something(2).await;
        let outcome_two = do_something(3).await;
        return outcome_one + outcome_two;
    };
    let outcome = block_on(future_three);
    println!("time elapsed: {:?}", now.elapsed());
    println!("Here is the outcome: {}", outcome);
}
