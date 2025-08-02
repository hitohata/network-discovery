use shared::scan::usage::SystemInfo;

fn main() {
    let sys_info = SystemInfo::new();
    loop {
        println!("Scanning machine usage...");
        println!("---------------------------------");
        println!("Machine Info: {:?}", sys_info.get_machine_info());
        println!("Machine usage: {:?}", sys_info.get_usage());
        println!("---------------------------------");
        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
