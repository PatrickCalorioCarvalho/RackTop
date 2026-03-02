use std::process::Command;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ContainerInfo {
    pub id: String,
    pub name: String,
    pub status: String,
    pub cpu: String,
    pub mem: String,
}

pub fn list_containers() -> Vec<ContainerInfo> {
    // ===== docker ps =====
    let ps_output = match Command::new("docker")
        .args([
            "ps",
            "--format",
            "{{.ID}};{{.Names}};{{.Status}}",
        ])
        .output()
    {
        Ok(output) => output,
        Err(_) => return vec![],
    };

    let ps_stdout = String::from_utf8_lossy(&ps_output.stdout);

    let mut containers: Vec<ContainerInfo> = ps_stdout
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split(';').collect();

            ContainerInfo {
                id: parts.get(0).unwrap_or(&"").to_string(),
                name: parts.get(1).unwrap_or(&"").to_string(),
                status: parts.get(2).unwrap_or(&"").to_string(),
                cpu: "-".to_string(),
                mem: "-".to_string(),
            }
        })
        .collect();

    // ===== docker stats =====
    let stats_output = Command::new("docker")
        .args([
            "stats",
            "--no-stream",
            "--format",
            "{{.ID}};{{.CPUPerc}};{{.MemPerc}}",
        ])
        .output();

    if let Ok(stats_output) = stats_output {
        let stats_stdout = String::from_utf8_lossy(&stats_output.stdout);

        let mut stats_map: HashMap<String, (String, String)> = HashMap::new();

        for line in stats_stdout.lines() {
            let parts: Vec<&str> = line.split(';').collect();
            if parts.len() >= 3 {
                stats_map.insert(
                    parts[0].to_string(),
                    (parts[1].to_string(), parts[2].to_string()),
                );
            }
        }

        for container in &mut containers {
            if let Some((cpu, mem)) = stats_map.get(&container.id) {
                container.cpu = cpu.clone();
                container.mem = mem.clone();
            }
        }
    }

    containers
}