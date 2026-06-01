//! Demonstrates tokio channels (mpsc, oneshot).

use tokio::sync::mpsc;

/// A message that workers process.
#[derive(Debug, Clone)]
pub enum WorkItem {
    Task(String),
    Shutdown,
}

/// Result of processing a work item.
#[derive(Debug, Clone, PartialEq)]
pub struct WorkResult {
    pub input: String,
    pub output: String,
}

/// Spawns `num_workers` worker tasks and dispatches items to them via mpsc.
/// Returns collected results.
pub async fn run_worker_pool(items: Vec<String>, num_workers: usize) -> Vec<WorkResult> {
    let (tx, rx) = mpsc::channel::<WorkItem>(32);
    let rx = std::sync::Arc::new(tokio::sync::Mutex::new(rx));
    let results = std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new()));

    let mut handles = Vec::new();

    for _id in 0..num_workers {
        let rx = rx.clone();
        let results = results.clone();

        let handle = tokio::spawn(async move {
            loop {
                let item = {
                    let mut guard = rx.lock().await;
                    guard.recv().await
                };
                match item {
                    Some(WorkItem::Task(input)) => {
                        let output = format!("processed: {}", input.to_uppercase());
                        let result = WorkResult { input, output };
                        results.lock().await.push(result);
                    }
                    Some(WorkItem::Shutdown) | None => break,
                }
            }
        });
        handles.push(handle);
    }

    // Send work items
    for item in items {
        tx.send(WorkItem::Task(item)).await.ok();
    }

    // Send shutdown signals
    for _ in 0..num_workers {
        tx.send(WorkItem::Shutdown).await.ok();
    }
    drop(tx);

    // Wait for workers to finish
    for handle in handles {
        handle.await.ok();
    }

    // All workers done — unwrap the sole remaining Arc
    std::sync::Arc::try_unwrap(results)
        .expect("workers completed")
        .into_inner()
}
