use std::fmt;

use rand::{thread_rng, Rng};

#[derive(Debug, Clone, PartialEq)]
pub enum Role {
    Master,
    Slave,
}

impl fmt::Display for Role {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Role::Master => write!(f, "master"),
            Role::Slave => write!(f, "slave"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Admin {
    host: String,
    port: u16,
    replica: Replica,
}

#[derive(Debug, Clone)]
pub struct Replica {
    master_host: String,
    master_port: u16,
    role: Role,
    id: String,
    offset: u16,
}

impl Replica {
    pub fn new() -> Self {
        Replica {
            master_host: "127.0.0.1".to_string(),
            master_port: 6379,
            role: Role::Master,
            id: generate_random_id(),
            offset: 0,
        }
    }
}

impl Admin {
    pub fn new() -> Self {
        let replica = Replica::new();
        Admin {
            host: "127.0.0.1".to_string(),
            port: 6379,
            replica,
        }
    }

    pub fn get_host(&mut self) -> String {
        self.host.clone()
    }

    pub fn get_port(&mut self) -> u16 {
        self.port
    }

    pub fn get_replica_role(&mut self) -> Role {
        self.replica.role.clone()
    }

    pub fn get_replica_host(&mut self) -> String {
        self.replica.master_host.clone()
    }

    pub fn get_replica_port(&mut self) -> u16 {
        self.replica.master_port
    }

    pub fn get_replica_id(&mut self) -> String {
        self.replica.id.clone()
    }

    pub fn get_replica_offset(&mut self) -> u16 {
        self.replica.offset
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_replica(&mut self, master_host: String, master_port: u16, role: Role) {
        self.replica.master_host = master_host;
        self.replica.master_port = master_port;
        self.replica.role = role;
    }
}

fn generate_random_id() -> String {
    let mut rng = thread_rng();
    let id: String = (0..40).map(|_| rng.gen_range(0..=9).to_string()).collect();
    id
}
