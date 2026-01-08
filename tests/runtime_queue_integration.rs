//! Integration tests for Runtime Queue Module

use poolai::runtime::queue::TaskQueue;

#[tokio::test]
async fn test_task_queue_creation() {
    let queue = TaskQueue::new(100);
    assert_eq!(queue.get_capacity(), 100);
}

#[tokio::test]
async fn test_task_queue_initialization() -> Result<(), Box<dyn std::error::Error>> {
    let mut queue = TaskQueue::new(100);
    queue.initialize().await?;
    Ok(())
}

#[tokio::test]
async fn test_task_queue_lifecycle() -> Result<(), Box<dyn std::error::Error>> {
    let mut queue = TaskQueue::new(100);
    queue.initialize().await?;
    queue.start().await?;
    queue.shutdown().await?;
    Ok(())
}

#[tokio::test]
async fn test_task_queue_capacity() {
    let queue = TaskQueue::new(200);
    assert_eq!(queue.get_capacity(), 200);
}

#[tokio::test]
async fn test_task_queue_length() -> Result<(), Box<dyn std::error::Error>> {
    let mut queue = TaskQueue::new(100);
    queue.initialize().await?;
    let length = queue.get_length();
    assert_eq!(length, 0);
    Ok(())
}
