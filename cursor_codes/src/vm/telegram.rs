use crate::vm::{VmManager, VmConfig, VmStatus, Device, UsbPassthrough, PciePassthrough};
use teloxide::{prelude::*, utils::command::BotCommands};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Available commands:")]
pub enum Command {
    #[command(description = "Start the bot")]
    Start,
    #[command(description = "Show this help message")]
    Help,
    #[command(description = "List all VMs")]
    ListVms,
    #[command(description = "Create a new VM")]
    CreateVm { name: String, memory: u64, cpus: u32 },
    #[command(description = "Start a VM")]
    StartVm { name: String },
    #[command(description = "Stop a VM")]
    StopVm { name: String },
    #[command(description = "Delete a VM")]
    DeleteVm { name: String },
    #[command(description = "Get VM status")]
    VmStatus { name: String },
    #[command(description = "List USB devices")]
    ListUsb,
    #[command(description = "Attach USB device to VM")]
    AttachUsb { vm_name: String, device_id: String },
    #[command(description = "Detach USB device from VM")]
    DetachUsb { vm_name: String, device_id: String },
    #[command(description = "List PCIe devices")]
    ListPcie,
    #[command(description = "Attach PCIe device to VM")]
    AttachPcie { vm_name: String, device_id: String },
    #[command(description = "Detach PCIe device from VM")]
    DetachPcie { vm_name: String, device_id: String },
}

pub struct TelegramBot {
    bot: Bot,
    vm_manager: Arc<Mutex<Box<dyn VmManager>>>,
}

impl TelegramBot {
    pub fn new(token: String, vm_manager: Box<dyn VmManager>) -> Self {
        Self {
            bot: Bot::new(token),
            vm_manager: Arc::new(Mutex::new(vm_manager)),
        }
    }

    pub async fn run(&self) {
        let handler = Update::filter_message()
            .filter_command::<Command>()
            .endpoint(answer);

        Dispatcher::builder(self.bot.clone(), handler)
            .enable_ctrlc_handler()
            .build()
            .dispatch()
            .await;
    }
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Start => {
            bot.send_message(msg.chat.id, "Welcome to VM Manager Bot! Use /help to see available commands.").await?;
        }
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
        }
        Command::ListVms => {
            let vm_manager = get_vm_manager().await;
            match vm_manager.list_vms().await {
                Ok(vms) => {
                    let response = if vms.is_empty() {
                        "No VMs found".to_string()
                    } else {
                        vms.join("\n")
                    };
                    bot.send_message(msg.chat.id, response).await?;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error listing VMs: {}", e)).await?;
                }
            }
        }
        Command::CreateVm { name, memory, cpus } => {
            let vm_manager = get_vm_manager().await;
            let config = VmConfig {
                name: name.clone(),
                memory,
                cpus,
                devices: Vec::new(),
                usb_passthrough: Vec::new(),
                pcie_passthrough: Vec::new(),
            };
            match vm_manager.create_vm(config).await {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("VM {} created successfully", name)).await?;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error creating VM: {}", e)).await?;
                }
            }
        }
        Command::StartVm { name } => {
            let vm_manager = get_vm_manager().await;
            match vm_manager.start_vm(&name).await {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("VM {} started successfully", name)).await?;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error starting VM: {}", e)).await?;
                }
            }
        }
        Command::StopVm { name } => {
            let vm_manager = get_vm_manager().await;
            match vm_manager.stop_vm(&name).await {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("VM {} stopped successfully", name)).await?;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error stopping VM: {}", e)).await?;
                }
            }
        }
        Command::DeleteVm { name } => {
            let vm_manager = get_vm_manager().await;
            match vm_manager.delete_vm(&name).await {
                Ok(_) => {
                    bot.send_message(msg.chat.id, format!("VM {} deleted successfully", name)).await?;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error deleting VM: {}", e)).await?;
                }
            }
        }
        Command::VmStatus { name } => {
            let vm_manager = get_vm_manager().await;
            match vm_manager.get_vm_status(&name).await {
                Ok(status) => {
                    let response = format!(
                        "VM: {}\nState: {:?}\nMemory Usage: {} MB\nCPU Usage: {:.1}%",
                        status.name,
                        status.state,
                        status.memory_usage / 1024 / 1024,
                        status.cpu_usage
                    );
                    bot.send_message(msg.chat.id, response).await?;
                }
                Err(e) => {
                    bot.send_message(msg.chat.id, format!("Error getting VM status: {}", e)).await?;
                }
            }
        }
        Command::ListUsb => {
            let vm_manager = get_vm_manager().await;
            // TODO: Implement USB device listing
            bot.send_message(msg.chat.id, "USB device listing not implemented yet").await?;
        }
        Command::AttachUsb { vm_name, device_id } => {
            let vm_manager = get_vm_manager().await;
            // TODO: Implement USB device attachment
            bot.send_message(msg.chat.id, "USB device attachment not implemented yet").await?;
        }
        Command::DetachUsb { vm_name, device_id } => {
            let vm_manager = get_vm_manager().await;
            // TODO: Implement USB device detachment
            bot.send_message(msg.chat.id, "USB device detachment not implemented yet").await?;
        }
        Command::ListPcie => {
            let vm_manager = get_vm_manager().await;
            // TODO: Implement PCIe device listing
            bot.send_message(msg.chat.id, "PCIe device listing not implemented yet").await?;
        }
        Command::AttachPcie { vm_name, device_id } => {
            let vm_manager = get_vm_manager().await;
            // TODO: Implement PCIe device attachment
            bot.send_message(msg.chat.id, "PCIe device attachment not implemented yet").await?;
        }
        Command::DetachPcie { vm_name, device_id } => {
            let vm_manager = get_vm_manager().await;
            // TODO: Implement PCIe device detachment
            bot.send_message(msg.chat.id, "PCIe device detachment not implemented yet").await?;
        }
    }
    Ok(())
}

async fn get_vm_manager() -> Box<dyn VmManager> {
    // TODO: Implement proper VM manager access
    crate::vm::create_vm_manager()
} 