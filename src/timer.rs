use crate::{rwlog_debug, rwlog_error};

use std::sync::Arc;
use tokio::time::{sleep, Duration};
use std::time;

pub fn spawn_rw_timer(timer_ms: u64, functions: Arc<[fn()]>) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        rw_timer(timer_ms, functions).await;
    })
}

async fn rw_timer(timer_ms: u64, functions: Arc<[fn()]>) {
    loop {
        let now = time::Instant::now();
        for function in functions.as_ref() {
            function();
        }

        let elapsed = now.elapsed().as_millis() as u64;
        rwlog_debug!("ProcessTime: {}ms", elapsed);

        if let Some(sleep_duration) = timer_ms.checked_sub(elapsed) {
            sleep(Duration::from_millis(sleep_duration)).await;
        } else {
            rwlog_error!("Warning: Execution time exceeded timer interval.");
        }
    }
}