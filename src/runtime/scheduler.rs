use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use log::{info, warn, error};
use std::collections::HashMap;
use chrono::{DateTime, Utc, Duration as ChronoDuration};
use std::time::Duration;
use tokio::time;
use cursor_codes::core::error::CursorError;
use cursor_codes::monitoring::logger::LoggerSystem;
use cursor_codes::monitoring::alert::AlertSystem;
use cursor_codes::monitoring::metrics::MetricsSystem;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskConfig {
    pub id: String,
    pub name: String,
    pub description: String,
    pub task_type: String,
    pub schedule: String,
    pub max_retries: u32,
    pub retry_delay: Duration,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskStats {
    pub total_runs: u64,
    pub successful_runs: u64,
    pub failed_runs: u64,
    pub last_run_time: Option<DateTime<Utc>>,
    pub next_run_time: Option<DateTime<Utc>>,
    pub retry_count: u32,
    pub average_duration: Duration,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetrics {
    pub config: TaskConfig,
    pub stats: TaskStats,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskRun {
    pub id: String,
    pub task_id: String,
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub status: String,
    pub error: Option<String>,
}

pub struct SchedulerSystem {
    tasks: Arc<Mutex<HashMap<String, TaskMetrics>>>,
    runs: Arc<Mutex<HashMap<String, TaskRun>>>,
}

impl SchedulerSystem {
    pub fn new() -> Self {
        Self {
            tasks: Arc::new(Mutex::new(HashMap::new())),
            runs: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn add_task(&self, config: TaskConfig) -> Result<(), String> {
        let mut tasks = self.tasks.lock().await;
        
        if tasks.contains_key(&config.id) {
            return Err(format!("Task '{}' already exists", config.id));
        }

        let metrics = TaskMetrics {
            config,
            stats: TaskStats {
                total_runs: 0,
                successful_runs: 0,
                failed_runs: 0,
                last_run_time: None,
                next_run_time: None,
                retry_count: 0,
                average_duration: Duration::from_secs(0),
                last_error: None,
            },
        };

        tasks.insert(metrics.config.id.clone(), metrics);
        info!("Added new task: {}", metrics.config.id);
        Ok(())
    }

    pub async fn remove_task(&self, id: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock().await;
        let mut runs = self.runs.lock().await;
        
        if !tasks.contains_key(id) {
            return Err(format!("Task '{}' not found", id));
        }

        // Remove associated runs
        runs.retain(|_, r| r.task_id != id);
        
        tasks.remove(id);
        info!("Removed task: {}", id);
        Ok(())
    }

    pub async fn schedule_task(&self, task_id: &str) -> Result<(), String> {
        let mut tasks = self.tasks.lock().await;
        
        let task = tasks
            .get_mut(task_id)
            .ok_or_else(|| format!("Task '{}' not found", task_id))?;

        if !task.config.active {
            return Err("Task is not active".to_string());
        }

        let next_run = self.calculate_next_run(&task.config.schedule)?;
        task.stats.next_run_time = Some(next_run);

        info!(
            "Scheduled task: {} for next run at: {}",
            task_id, next_run
        );
        Ok(())
    }

    pub async fn run_task(&self, task_id: &str) -> Result<String, String> {
        let mut tasks = self.tasks.lock().await;
        let mut runs = self.runs.lock().await;
        
        let task = tasks
            .get_mut(task_id)
            .ok_or_else(|| format!("Task '{}' not found", task_id))?;

        if !task.config.active {
            return Err("Task is not active".to_string());
        }

        let run = TaskRun {
            id: uuid::Uuid::new_v4().to_string(),
            task_id: task_id.to_string(),
            start_time: Utc::now(),
            end_time: None,
            status: "running".to_string(),
            error: None,
        };

        runs.insert(run.id.clone(), run.clone());
        task.stats.total_runs += 1;
        task.stats.last_run_time = Some(run.start_time);

        info!("Started task run: {} for task: {}", run.id, task_id);
        Ok(run.id)
    }

    pub async fn complete_task_run(&self, run_id: &str, success: bool, error: Option<String>) -> Result<(), String> {
        let mut tasks = self.tasks.lock().await;
        let mut runs = self.runs.lock().await;
        
        let run = runs
            .get_mut(run_id)
            .ok_or_else(|| format!("Run '{}' not found", run_id))?;

        let task = tasks
            .get_mut(&run.task_id)
            .ok_or_else(|| format!("Task '{}' not found", run.task_id))?;

        let end_time = Utc::now();
        run.end_time = Some(end_time);

        if success {
            run.status = "completed".to_string();
            task.stats.successful_runs += 1;
        } else {
            run.status = "failed".to_string();
            run.error = error.clone();
            task.stats.failed_runs += 1;
            task.stats.last_error = error;

            if task.stats.retry_count < task.config.max_retries {
                task.stats.retry_count += 1;
                self.schedule_retry(task).await?;
            }
        }

        let duration = end_time.signed_duration_since(run.start_time);
        task.stats.average_duration = Duration::from_secs(
            (task.stats.average_duration.as_secs() + duration.num_seconds() as u64) / 2,
        );

        info!("Completed task run: {} with status: {}", run_id, run.status);
        Ok(())
    }

    async fn schedule_retry(&self, task: &mut TaskMetrics) -> Result<(), String> {
        let retry_delay = ChronoDuration::from_std(task.config.retry_delay)
            .map_err(|e| format!("Invalid retry delay: {}", e))?;
        
        let next_run = Utc::now() + retry_delay;
        task.stats.next_run_time = Some(next_run);

        info!(
            "Scheduled retry for task: {} at: {} (attempt: {})",
            task.config.id, next_run, task.stats.retry_count
        );
        Ok(())
    }

    fn calculate_next_run(&self, schedule: &str) -> Result<DateTime<Utc>, String> {
        // Simple schedule format: "HH:MM" for daily runs
        let parts: Vec<&str> = schedule.split(':').collect();
        if parts.len() != 2 {
            return Err("Invalid schedule format".to_string());
        }

        let hour: u32 = parts[0]
            .parse()
            .map_err(|_| "Invalid hour in schedule".to_string())?;
        let minute: u32 = parts[1]
            .parse()
            .map_err(|_| "Invalid minute in schedule".to_string())?;

        if hour > 23 || minute > 59 {
            return Err("Invalid time in schedule".to_string());
        }

        let mut next_run = Utc::now();
        next_run = next_run
            .with_hour(hour)
            .unwrap()
            .with_minute(minute)
            .unwrap()
            .with_second(0)
            .unwrap()
            .with_nanosecond(0)
            .unwrap();

        if next_run <= Utc::now() {
            next_run = next_run + ChronoDuration::days(1);
        }

        Ok(next_run)
    }

    pub async fn get_task(&self, id: &str) -> Result<TaskMetrics, String> {
        let tasks = self.tasks.lock().await;
        
        tasks
            .get(id)
            .cloned()
            .ok_or_else(|| format!("Task '{}' not found", id))
    }

    pub async fn get_all_tasks(&self) -> Vec<TaskMetrics> {
        let tasks = self.tasks.lock().await;
        tasks.values().cloned().collect()
    }

    pub async fn get_active_tasks(&self) -> Vec<TaskMetrics> {
        let tasks = self.tasks.lock().await;
        tasks
            .values()
            .filter(|t| t.config.active)
            .cloned()
            .collect()
    }

    pub async fn get_runs(&self, task_id: &str) -> Vec<TaskRun> {
        let runs = self.runs.lock().await;
        runs
            .values()
            .filter(|r| r.task_id == task_id)
            .cloned()
            .collect()
    }

    pub async fn set_task_active(&self, id: &str, active: bool) -> Result<(), String> {
        let mut tasks = self.tasks.lock().await;
        
        let task = tasks
            .get_mut(id)
            .ok_or_else(|| format!("Task '{}' not found", id))?;

        task.config.active = active;
        info!(
            "Task '{}' {}",
            id,
            if active { "activated" } else { "deactivated" }
        );
        Ok(())
    }

    pub async fn update_task_config(&self, id: &str, new_config: TaskConfig) -> Result<(), String> {
        let mut tasks = self.tasks.lock().await;
        
        let task = tasks
            .get_mut(id)
            .ok_or_else(|| format!("Task '{}' not found", id))?;

        task.config = new_config;
        info!("Updated task configuration: {}", id);
        Ok(())
    }
} 