//! Task Queue for Stage 4.1 Runtime (Stub)

pub struct TaskQueue {
    #[allow(dead_code)] // Will be used for capacity checks in future
    capacity: usize,
}

impl TaskQueue {
    pub fn new(capacity: usize) -> Self {
        Self { capacity }
    }

    pub async fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn start(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub async fn shutdown(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        Ok(())
    }

    pub fn get_length(&self) -> usize {
        0
    }
}
