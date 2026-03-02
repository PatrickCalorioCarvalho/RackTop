use sysinfo::{System, Disks};

pub struct DiskInfo {
    pub name: String,
    pub used: u64,
    pub total: u64,
}

#[derive(Clone)]
pub struct ProcessInfo {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub memory: u64,
}

pub struct Metrics {
    pub cpu_usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub swap_used: u64,
    pub swap_total: u64,
    pub cpu_count: usize,
    pub disks: Vec<DiskInfo>,
    pub processes: Vec<ProcessInfo>,
}

impl Metrics {
    pub fn collect() -> Self {
        let mut sys = System::new_all();
        sys.refresh_all();

        let cpu_usage = sys.global_cpu_info().cpu_usage();
        let memory_used = sys.used_memory();
        let memory_total = sys.total_memory();
        let swap_used = sys.used_swap();
        let swap_total = sys.total_swap();

        // ===== DISKS NOVO MODELO =====
        let disks_struct = Disks::new_with_refreshed_list();

        let disks = disks_struct
            .list()
            .iter()
            .map(|d| {
                let total = d.total_space();
                let available = d.available_space();
                let used = total - available;

                DiskInfo {
                    name: d.mount_point().to_string_lossy().to_string(),
                    used,
                    total,
                }
            })
            .collect();

        let cpu_count = sys.cpus().len();
        
        let processes = sys
            .processes()
            .values()
            .map(|p| ProcessInfo {
                pid: p.pid().as_u32() as i32,
                name: p.name().to_string(),
                cpu: p.cpu_usage(),
                memory: p.memory(),
            })
            .collect();

        Self {
            cpu_usage,
            memory_used,
            memory_total,
            swap_used,
            swap_total,
            disks,
            processes,
            cpu_count,
        }
    }
}