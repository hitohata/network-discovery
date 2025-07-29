struct Machine {
    pub name: String,
    pub ip: String,
}

pub fn scan() -> Vec<Machine> {
    let mut machines = Vec::new();
    machines.push(Machine {
        name: "Machine1".to_string(),
        ip: "".to_string(),
    });
    machines.push(Machine {
        name: "Machine2".to_string(),
        ip: "".to_string(),
    });
    machines
}
