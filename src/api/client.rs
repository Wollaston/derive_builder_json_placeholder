use anyhow::Result;
use url::Url;

pub trait RestClient {
    fn rest(&self, url: Url) -> Result<String>;
}
