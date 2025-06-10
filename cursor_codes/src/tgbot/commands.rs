use teloxide::utils::command::BotCommands;

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