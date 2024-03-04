use rand::{thread_rng, Rng};

#[derive(Debug, Clone)]
pub struct Admin {
    port: u16,
    role: String,
    id: String,
    offset: u16,
}

impl Admin {
    pub fn new() -> Self {
        Admin {
            port: 6379,
            role: String::from("master"),
            id: generate_random_id(),
            offset: 0,
        }
    }

    pub fn get_port(&mut self) -> u16 {
        self.port
    }

    pub fn get_role(&mut self) -> String {
        self.role.clone()
    }

    pub fn get_id(&mut self) -> String {
        self.id.clone()
    }

    pub fn get_offset(&mut self) -> u16 {
        self.offset
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn set_role(&mut self, role: String) {
        self.role = role;
    }
}

fn generate_random_id() -> String {
    let mut rng = thread_rng();
    let id: String = (0..40).map(|_| rng.gen_range(0..=9).to_string()).collect();
    id
}
