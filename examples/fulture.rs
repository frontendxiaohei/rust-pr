// An example of using futures in Rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;
use tokio::time::sleep;

// A simple future that completes after a delay
struct Delay {
    duration: Duration,
}

impl Future for Delay {
    type Output = String;

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Simulate asynchronous work
        std::thread::sleep(self.duration);
        Poll::Ready(format!("Completed after {:?}", self.duration))
    }
}

#[tokio::main]
async fn main() {
    // Create a delay future
    let delay_future = Delay { duration: Duration::from_secs(2) };
    
    // Run the future and get the output
    let result = delay_future.await;
    println!("{}", result);
    
    // Using Tokio's sleep function which returns a future
    let sleep_future = sleep(Duration::from_secs(1));
    println!("Sleeping...");
    sleep_future.await;
    println!("Woke up!");
}