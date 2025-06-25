use poolai::{PoolAI, AppError};
use std::process;
use tokio;

#[tokio::main]
async fn main() {
    // Настройка логирования
    env_logger::init();

    println!("🚀 Starting PoolAI - AI Mining Pool Management System");
    println!("Version: {}", env!("CARGO_PKG_VERSION"));
    println!("Build: {}", env!("CARGO_PKG_NAME"));

    // Создание экземпляра PoolAI
    let poolai = match PoolAI::new().await {
        Ok(poolai) => poolai,
        Err(e) => {
            eprintln!("❌ Failed to create PoolAI instance: {}", e);
            process::exit(1);
        }
    };

    // Инициализация системы
    println!("🔧 Initializing PoolAI system...");
    if let Err(e) = poolai.initialize().await {
        eprintln!("❌ Failed to initialize PoolAI: {}", e);
        process::exit(1);
    }

    println!("✅ PoolAI system initialized successfully!");
    println!("📊 System components:");
    println!("   - Pool Management: ✅");
    println!("   - Runtime Management: ✅");
    println!("   - Network API: ✅");
    println!("   - Platform Management: ✅");
    println!("   - UI Dashboard: ✅");
    println!("   - Library Management: ✅");
    println!("   - VM Management: ✅");
    println!("   - RAID Management: ✅");
    println!("   - Telegram Bot: ✅");
    println!("   - Monitoring: ✅");

    // Вывод информации о системе
    if let Ok(status) = poolai.get_system_status().await {
        println!("📈 System Status:");
        println!("   - Overall Health: {:.1}%", status.overall_health);
        println!("   - Active Workers: {}", status.active_workers);
        println!("   - GPU Utilization: {:.1}%", status.gpu_utilization);
        println!("   - Memory Usage: {:.1}GB", status.memory_usage_mb / 1024.0);
    }

    // Ожидание сигнала завершения
    println!("🔄 PoolAI is running. Press Ctrl+C to stop...");
    
    // Обработка сигналов завершения
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            println!("🛑 Received shutdown signal...");
        }
        _ = tokio::time::sleep(tokio::time::Duration::from_secs(3600)) => {
            println!("⏰ Shutdown after 1 hour...");
        }
    }

    // Корректное выключение системы
    println!("🔧 Shutting down PoolAI system...");
    if let Err(e) = poolai.shutdown().await {
        eprintln!("❌ Error during shutdown: {}", e);
        process::exit(1);
    }

    println!("✅ PoolAI system shutdown completed successfully!");
} 