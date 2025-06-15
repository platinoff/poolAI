use std::sync::mpsc;
use solana_sdk::pubkey::Pubkey;
use crate::state::AppState;
use std::time::Duration;
use tokio::time::sleep;
use std::sync::Arc;

#[derive(Debug)]
pub enum Task {
    // AI Generation Tasks
    TextGeneration {
        prompt: String,
        max_tokens: u32,
        temperature: f32,
        model: String,
    },
    ImageGeneration {
        prompt: String,
        width: u32,
        height: u32,
        steps: u32,
        model: String,
    },
    CodeGeneration {
        prompt: String,
        language: String,
        max_tokens: u32,
        model: String,
    },
    ModelTraining {
        dataset_path: String,
        epochs: u32,
        batch_size: u32,
        model_type: String,
    },
    // System Tasks
    DataProcessing,
    SyncSeeds,
    Stop,
}

pub struct Worker {
    pub id: String,
    pub solana_address: Pubkey,
    pub mining_power: f64,
    pub task_sender: mpsc::Sender<Task>,
    pub gpu_memory: u64,
    pub cpu_cores: u32,
    pub available_models: Vec<String>,
}

impl Worker {
    pub fn new(id: String, solana_address: Pubkey, mining_power: f64, gpu_memory: u64, cpu_cores: u32) -> Self {
        let (tx, rx) = mpsc::channel();
        
        // Start worker task
        let worker_id = id.clone();
        tokio::spawn(async move {
            Worker::run(worker_id, rx).await;
        });

        Self {
            id,
            solana_address,
            mining_power,
            task_sender: tx,
            gpu_memory,
            cpu_cores,
            available_models: Vec::new(),
        }
    }

    async fn run(id: String, receiver: mpsc::Receiver<Task>) {
        log::info!("Worker {} started", id);
        
        while let Ok(task) = receiver.recv() {
            match task {
                Task::TextGeneration { prompt, max_tokens, temperature, model } => {
                    log::info!("Worker {} starting text generation with model {}", id, model);
                    // Implement text generation logic here
                }
                Task::ImageGeneration { prompt, width, height, steps, model } => {
                    log::info!("Worker {} starting image generation with model {}", id, model);
                    // Implement image generation logic here
                }
                Task::CodeGeneration { prompt, language, max_tokens, model } => {
                    log::info!("Worker {} starting code generation with model {}", id, model);
                    // Implement code generation logic here
                }
                Task::ModelTraining { dataset_path, epochs, batch_size, model_type } => {
                    log::info!("Worker {} starting model training for {}", id, model_type);
                    // Implement model training logic here
                }
                Task::DataProcessing => {
                    log::info!("Worker {} starting data processing task", id);
                    // Implement data processing logic here
                }
                Task::SyncSeeds => {
                    log::info!("Worker {} syncing seeds", id);
                    // Implement seed synchronization logic here
                }
                Task::Stop => {
                    log::info!("Worker {} received stop command", id);
                    break;
                }
            }
        }
        
        log::info!("Worker {} stopped", id);
    }

    pub fn stop(&self) {
        let _ = self.task_sender.send(Task::Stop);
    }
}

pub async fn monitor_workers(app_state: Arc<AppState>) {
    loop {
        let workers = app_state.workers.read();
        for (id, worker) in workers.iter() {
            log::info!("Worker {} status: power={}", id, worker.mining_power);
        }
        sleep(Duration::from_secs(10)).await;
    }
} 