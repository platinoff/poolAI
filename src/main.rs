use poolai::{initialize_system, shutdown_system, health_check, get_system_info};
use std::process;
use tokio;

#[tokio::main]
async fn main() {
    // Настройка логирования
    env_logger::init();

    println!("🚀 Starting PoolAI - AI Mining Pool Management System");
    let info = get_system_info();
    println!("Version: {}", info.version);
    println!("Build: {}", info.name);
    println!("Description: {}", info.description);
    println!("Features: {:?}", info.features);
    println!("Modules: {:?}", info.modules);

    // Инициализация системы
    println!("🔧 Initializing PoolAI system...");
    if let Err(e) = initialize_system().await {
        eprintln!("❌ Failed to initialize PoolAI: {}", e);
        process::exit(1);
    }
    println!("✅ PoolAI system initialized successfully!");

    // Проверка здоровья системы
    if let Err(e) = health_check().await {
        eprintln!("❌ Health check failed: {}", e);
        process::exit(1);
    } else {
        println!("📈 System health check passed.");
    }

    // Ожидание сигнала завершения
    println!("🔄 PoolAI is running. Press Ctrl+C to stop...");
    tokio::signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
    println!("🛑 Received shutdown signal...");

    // Корректное выключение системы
    println!("🔧 Shutting down PoolAI system...");
    if let Err(e) = shutdown_system().await {
        eprintln!("❌ Error during shutdown: {}", e);
        process::exit(1);
    }
    println!("✅ PoolAI system shutdown completed successfully!");
} 