use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::time::Duration;
use reqwest::{Client, Error, Response};
use serde::{Deserialize, Serialize};

use crate::network::address::Address;

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

    pub async fn download_file(url: &str, file_path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        // url = http://domain.com/test.download
        // file_path = folder/file.test
        if file_path.exists() {
            return Ok(());
        }

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(30))
            .no_brotli()
            .build()?;
        let response = client.get(url).send().await?;
        // check ob response 2xx
        if !response.status().is_success() {
            return Err("Error response from Server".into());
        }

        let mut file = File::create(&file_path)?;

        file.write_all(&response.bytes().await?)?;

        Ok(())
    }

    pub fn extract_extension_from_url(url: &String) -> Option<String> {
        if let Ok(url) = reqwest::Url::parse(url) {
            if let Some(file_name) = url.path_segments().and_then(|segments| segments.last()) {
                if let Some(extension) = Path::new(file_name).extension() {
                    return Some(extension.to_string_lossy().to_string());
                }
            }
        }
        None
    }

    pub fn extract_filename_from_url(url: &str) -> Option<String> {
        Option::from(
            match url.rsplit('/').next() {
                Some(file_name) => file_name,
                None => return None,
            }
                .to_string(),
        )
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
