use super::{PlatformError, PlatformService, SystemInfo, MemoryInfo, CpuInfo, DiskInfo};
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicBool, Ordering};
use std::fs;
use std::io::Read;
use std::time::Duration;
use tokio::time;

pub struct UnixService {
    name: String,
    running: AtomicBool,
}

impl UnixService {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            running: AtomicBool::new(false),
        }
    }
}

#[async_trait::async_trait]
impl PlatformService for UnixService {
    async fn install(&self) -> Result<(), PlatformError> {
        // Unix service installation logic
        Ok(())
    }

    async fn uninstall(&self) -> Result<(), PlatformError> {
        // Unix service uninstallation logic
        Ok(())
    }

    async fn start(&self) -> Result<(), PlatformError> {
        self.running.store(true, Ordering::SeqCst);
        Ok(())
    }

    async fn stop(&self) -> Result<(), PlatformError> {
        self.running.store(false, Ordering::SeqCst);
        Ok(())
    }

    async fn status(&self) -> Result<String, PlatformError> {
        if self.running.load(Ordering::SeqCst) {
            Ok("Running".to_string())
        } else {
            Ok("Stopped".to_string())
        }
    }
}

pub struct UnixSystemInfo;

impl UnixSystemInfo {
    pub fn new() -> Self {
        Self
    }

    fn get_cpu_usage() -> f32 {
        let mut prev_idle = 0;
        let mut prev_total = 0;

        // Read CPU stats
        let mut cpu_stats = String::new();
        if let Ok(mut file) = fs::File::open("/proc/stat") {
            file.read_to_string(&mut cpu_stats).ok();
        }

        // Parse CPU stats
        if let Some(line) = cpu_stats.lines().next() {
            let values: Vec<u64> = line
                .split_whitespace()
                .skip(1)
                .filter_map(|s| s.parse().ok())
                .collect();

            if values.len() >= 7 {
                let idle = values[3] + values[4];
                let total = values.iter().sum();

                if prev_total > 0 {
                    let idle_diff = idle - prev_idle;
                    let total_diff = total - prev_total;
                    return 100.0 * (1.0 - (idle_diff as f32 / total_diff as f32));
                }

                prev_idle = idle;
                prev_total = total;
            }
        }

        0.0
    }

    fn get_cpu_temperature() -> Option<f32> {
        let mut temp = String::new();
        if let Ok(mut file) = fs::File::open("/sys/class/thermal/thermal_zone0/temp") {
            if file.read_to_string(&mut temp).is_ok() {
                if let Ok(temp) = temp.trim().parse::<f32>() {
                    return Some(temp / 1000.0);
                }
            }
        }
        None
    }
}

#[async_trait::async_trait]
impl SystemInfo for UnixSystemInfo {
    fn get_os_name(&self) -> String {
        let output = Command::new("uname")
            .arg("-s")
            .output()
            .unwrap_or_default();
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn get_os_version(&self) -> String {
        let output = Command::new("uname")
            .arg("-r")
            .output()
            .unwrap_or_default();
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    fn get_architecture(&self) -> String {
        let output = Command::new("uname")
            .arg("-m")
            .output()
            .unwrap_or_default();
        String::from_utf8_lossy(&output.stdout).trim().to_string()
    }

    async fn get_memory_info(&self) -> Result<MemoryInfo, PlatformError> {
        let mut mem_info = String::new();
        if let Ok(mut file) = fs::File::open("/proc/meminfo") {
            file.read_to_string(&mut mem_info).ok();
        }

        let mut total = 0;
        let mut free = 0;
        let mut cached = 0;

        for line in mem_info.lines() {
            if line.starts_with("MemTotal:") {
                total = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            } else if line.starts_with("MemFree:") {
                free = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            } else if line.starts_with("Cached:") {
                cached = line.split_whitespace().nth(1).and_then(|s| s.parse().ok()).unwrap_or(0);
            }
        }

        Ok(MemoryInfo {
            total: total * 1024,
            free: free * 1024,
            used: (total - free) * 1024,
            swap_total: 0,
            swap_free: 0,
            cache: Some(cached * 1024),
        })
    }

    async fn get_cpu_info(&self) -> Result<CpuInfo, PlatformError> {
        let mut cpu_info = String::new();
        if let Ok(mut file) = fs::File::open("/proc/cpuinfo") {
            file.read_to_string(&mut cpu_info).ok();
        }

        let mut model = String::new();
        let mut cores = 0;

        for line in cpu_info.lines() {
            if line.starts_with("model name") {
                model = line.split(':').nth(1).unwrap_or("").trim().to_string();
            } else if line.starts_with("processor") {
                cores += 1;
            }
        }

        Ok(CpuInfo {
            model,
            cores,
            threads: cores,
            frequency: 0,
            usage: Self::get_cpu_usage(),
            temperature: Self::get_cpu_temperature(),
        })
    }

    async fn get_disk_info(&self) -> Result<DiskInfo, PlatformError> {
        let output = Command::new("df")
            .arg("-B1")
            .arg("/")
            .output()
            .unwrap_or_default();

        let output_str = String::from_utf8_lossy(&output.stdout);
        let lines: Vec<&str> = output_str.lines().collect();

        if lines.len() >= 2 {
            let values: Vec<&str> = lines[1].split_whitespace().collect();
            if values.len() >= 4 {
                let total = values[1].parse().unwrap_or(0);
                let used = values[2].parse().unwrap_or(0);
                let free = values[3].parse().unwrap_or(0);

                return Ok(DiskInfo {
                    total,
                    free,
                    used,
                    mount_point: PathBuf::from("/"),
                    fs_type: Some(values[0].to_string()),
                });
            }
        }

        Ok(DiskInfo {
            total: 0,
            free: 0,
            used: 0,
            mount_point: PathBuf::from("/"),
            fs_type: None,
        })
    }
} 