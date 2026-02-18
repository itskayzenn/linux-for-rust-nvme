use std::process::Command;

fn main() {
    println!("Checking NVMe devices...\n");

    let output = Command::new("ls")
        .arg("/dev")
        .output()
        .expect("Failed to execute command");

    let devices = String::from_utf8_lossy(&output.stdout);

    for line in devices.lines() {
        if line.starts_with("nvme") {
            println!("Found NVMe device: {}", line);
        }
    }
}
