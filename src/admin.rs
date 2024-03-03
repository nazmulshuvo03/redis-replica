#[derive(Debug, Clone)]
pub struct Admin {
    port: u16,
    role: String,
}

impl Admin {
    pub fn new() -> Self {
        Admin {
            port: 6379,
            role: String::from("master"),
        }
    }

    pub fn set_port(&mut self, port: u16) {
        self.port = port;
    }

    pub fn get_port(&mut self) -> u16 {
        self.port.clone()
    }

    pub fn set_role(&mut self, role: String) {
        self.role = role;
    }

    pub fn get_role(&mut self) -> String {
        self.role.clone()
    }
}
