use std::sync::{Mutex, OnceLock};

/// scan machine usage
pub fn machine_usage() {
    let sys = sys_info();
    {
        let mut sys_guard = sys.lock().unwrap();
        sys_guard.refresh_all();
        
        println!("Total memory: {:?}", sys_guard.total_memory());
        
        println!("Hostname: {:?}", sysinfo::System::host_name().unwrap_or("Unknown".to_string()));
    }
}

/// Instance of sysinfo::System wrapped in a Mutex for thread safety
fn sys_info() -> &'static Mutex<sysinfo::System> {
    static SYS_INFO: OnceLock<Mutex<sysinfo::System>> = OnceLock::new();
    SYS_INFO.get_or_init(|| {
        Mutex::new(sysinfo::System::new_all())
    })
}
