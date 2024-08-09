
pub struct Domain {
    tld: String,
    sld: String,
    sub: String,
}

impl Domain {
    // create a Domain obj. from a string
    pub fn new(domain: &str) -> Option<Self> {
        let parts: Vec<&str> = domain.split('.').collect();

        match parts.len() {
            // if domain has no sub
            2 => Some(Domain {
                sub: String::new(),
                sld: parts[0].to_string(),
                tld: parts[1].to_string(),
            }),
            // if domain has a sub
            3.. => Some(Domain {
                sub: parts[..parts.len()-2].join("."),
                sld: parts[parts.len()-2].to_string(),
                tld: parts[parts.len()-1].to_string(),
            }),

            _ => None,
        }
    }

    // getter and setter
    pub fn get_tld(&self) -> &str {
        &self.tld
    }

    pub fn set_tld(&mut self, tld: String) {
        self.tld = tld;
    }

    pub fn get_sld(&self) -> &str {
        &self.sld
    }

    pub fn set_sld(&mut self, sld: String) {
        self.sld = sld;
    }

    pub fn get_sub(&self) -> &str {
        &self.sub
    }

    pub fn set_sub(&mut self, sub: String) {
        self.sub = sub;
    }

    // get the full Domain as string
    pub fn to_string(&self) -> String {
        if self.sub.is_empty() {
            format!("{}.{}", self.sld, self.tld)
        } else {
            format!("{}.{}.{}", self.sub, self.sld, self.tld)
        }
    }

    pub fn clone(&self) -> Self {
        Self {
            tld: self.tld.clone(),
            sld: self.sld.clone(),
            sub: self.sub.clone(),
        }
    }
}