use crate::utils::current_time_millis;

#[derive(Debug, Clone)]
pub struct Assets {
    value: String,
    expiry: Option<u64>,
    created: u64,
}

impl Assets {
    pub fn new(value: String) -> Self {
        let current = current_time_millis();
        Assets {
            value,
            expiry: None,
            created: current,
        }
    }

    pub fn get_value(&mut self) -> String {
        self.value.clone()
    }

    pub fn get_value_len(&mut self) -> usize {
        self.value.len()
    }

    pub fn update_expiry(&mut self, time_string: &str) {
        match time_string.parse::<u64>() {
            Ok(parsed_number) => {
                println!("Parsed u64 value: {}", parsed_number);
                self.expiry = Some(parsed_number);
            }
            Err(_) => {
                println!("Failed to parse as u64");
                self.expiry = None;
            }
        }
    }

    pub fn is_value_expired(&mut self) -> bool {
        match self.expiry {
            Some(exp) => self.created + exp < current_time_millis(),
            None => false,
        }
    }
}
