use std::sync::{Mutex, OnceLock};
use sysinfo::System;

#[derive(Clone, Debug)]
pub struct SystemInfo {
    system: &'static Mutex<sysinfo::System>,
    network: &'static Mutex<sysinfo::Networks>,
    machine_info: MachineInfo,
}

impl SystemInfo {
    pub fn new() -> SystemInfo {
        let system = sys_info();
        let network = network_info();

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        let s = sysinfo::System::new_all();

        let machine_info = MachineInfo {
            os: System::name().unwrap_or(String::from("OS  name not found")),
            os_version: System::os_version().unwrap_or(String::from("OS version not found")),
            host_name: System::host_name().unwrap_or(String::from("Host name not found")),
            kernel_version: System::kernel_version()
                .unwrap_or(String::from("Kernel version not found")),
            number_of_cpu: System::physical_core_count().unwrap_or(0),
            arch: System::cpu_arch(),
            brand: s.cpus()[0].brand().to_string(),
        };

        Self {
            system,
            network,
            machine_info,
        }
    }

    pub fn get_machine_info(&self) -> &MachineInfo {
        &self.machine_info
    }

    pub fn get_usage(&self) -> MachineUsage {
        let mut sys_guard = self.system.lock().unwrap();
        let mut network_guard = self.network.lock().unwrap();

        let mut cpu_usage = Vec::with_capacity(self.machine_info.number_of_cpu);
        let mut cpu_frequency = Vec::with_capacity(self.machine_info.number_of_cpu);
        let mut network_down = 0;
        let mut network_up = 0;

        sys_guard.refresh_all();
        network_guard.refresh(true);

        for cpu in sys_guard.cpus().iter() {
            cpu_usage.push(cpu.cpu_usage());
            cpu_frequency.push(cpu.frequency());
        }

        for data in network_guard.list().values() {
            network_down += data.received();
            network_up += data.transmitted();
        }

        MachineUsage {
            total_memory: sys_guard.total_memory(),
            used_memory: sys_guard.used_memory(),
            total_swap: sys_guard.total_swap(),
            used_swap: sys_guard.used_swap(),
            cpu_usage,
            cpu_frequency,
            network_up,
            network_down,
        }
    }
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct MachineInfo {
    os: String,
    os_version: String,
    host_name: String,
    kernel_version: String,
    number_of_cpu: usize,
    arch: String,
    brand: String,
}

#[derive(Clone, Debug)]
pub struct MachineUsage {
    total_memory: u64,
    used_memory: u64,
    total_swap: u64,
    used_swap: u64,
    cpu_usage: Vec<f32>,
    cpu_frequency: Vec<u64>,
    network_down: u64,
    network_up: u64,
}

/// scan machine usage
pub fn machine_usage() {
    let sys = sys_info();
    {
        let mut sys_guard = sys.lock().unwrap();
        sys_guard.refresh_all();

        println!("Total memory: {:?}", sys_guard.total_memory());
    }
}

/// Instance of sysinfo::System wrapped in a Mutex for thread safety
fn sys_info() -> &'static Mutex<sysinfo::System> {
    static SYS_INFO: OnceLock<Mutex<sysinfo::System>> = OnceLock::new();
    SYS_INFO.get_or_init(|| Mutex::new(sysinfo::System::new_all()))
}

fn network_info() -> &'static Mutex<sysinfo::Networks> {
    static NETWORK_INFO: OnceLock<Mutex<sysinfo::Networks>> = OnceLock::new();
    NETWORK_INFO.get_or_init(|| Mutex::new(sysinfo::Networks::new_with_refreshed_list()))
}
