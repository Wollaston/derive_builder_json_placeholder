use anyhow::Result;

pub trait Query {
    fn query(&self, client: &reqwest::blocking::Client) -> Result<String>;
}
