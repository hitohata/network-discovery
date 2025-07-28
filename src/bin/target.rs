use discovery_network::scan::usage;

fn main() {
    
    loop {
        usage::machine_usage();
        std::thread::sleep(std::time::Duration::from_secs(5));
    }

}
