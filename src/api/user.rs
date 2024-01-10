use super::{client::RestClient, endpoint::UrlBase};
use anyhow::Result;
use reqwest::blocking::Client;

pub struct User {
    pub client: Client,
    base_url: UrlBase,
}

impl User {
    pub fn new() -> Self {
        let client = Client::new();
        User {
            client,
            base_url: UrlBase::Typicode,
        }
    }
}

impl RestClient for User {
    fn rest(&self, url: url::Url) -> Result<String> {
        let res = self.client.get(url).send()?.text()?;
        Ok(res)
    }
}
