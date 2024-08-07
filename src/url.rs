use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

use crate::address::Address;

// -------------------Url-------------------------------//
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Url {
    schema: UrlSchema,
    address: Address,
    url: String,
}

impl Url {
    pub fn new(schema: UrlSchema, address: &Address, url: &str) -> Url {
        Url {
            schema: schema.clone(),
            address: address.clone(),
            url: url.to_string(),
        }
    }

    pub fn get(&self) -> String {
        format!(
            "{}://{}/{}",
            &self.schema.to_string(),
            &self.address.to_string(),
            &self.url
        )
    }

    pub fn push(&mut self, str: &str) {
        self.url = format!("{}/{}", self.url, str)
    }

    pub fn join(&self, str: &str) -> Url {
        Url {
            schema: self.schema.clone(),
            address: self.address.clone(),
            url: format!("{}/{}", self.url, str),
        }
    }

    pub async fn post<T: Serialize>(&self, body: &T) -> Result<Response, Error> {
        let client = Client::new();
        client.post(&self.get()).json(&body).send().await
    }
}

// -------------------Url-Schema------------------------//
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum UrlSchema {
    Http,
    Https,
}

impl UrlSchema {
    pub fn to_string(&self) -> String {
        match self {
            UrlSchema::Http => String::from("http"),
            UrlSchema::Https => String::from("https"),
        }
    }
}
