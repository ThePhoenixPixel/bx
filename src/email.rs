use serde::{Deserialize, Serialize};

use crate::network::domain::Domain;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Email {
    local: String,
    domain: Domain,
}
impl Email {
    pub fn new(email: &str) -> Result<Self, &str> {
        let parts: Vec<&str> = email.split('@').collect();

        if parts.len() != 2 {
            return Err("Invalid email format. Expected one '@' symbol.");
        }

        let local = parts[0].to_string();
        let domain_str = parts[1];

        // Versuche, die Domain aus dem String zu erstellen
        let domain = match Domain::new(domain_str) {
            Some(domain) => domain,
            None => return Err("Error by create a Domain obj. from a string"),
        };

        Ok(Email {
            local,
            domain,
        })
    }

    pub fn get_local(&self) -> &str {
        &self.local
    }

    pub fn set_local(&mut self, local: &str) {
        self.local = local.to_string()
    }

    pub fn get_domain(&self) -> Domain {
        self.domain.clone()
    }
    pub fn to_string(&self) -> String {
        format!("{}@{}", self.local, self.domain.to_string())
    }
}


