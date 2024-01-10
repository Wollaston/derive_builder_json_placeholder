use anyhow::Result;
use std::{borrow::Cow, error::Error, fmt::Display};

use url::Url;

use crate::api::params::QueryParams;

use super::{client::RestClient, query::Query};

pub enum UrlBase {
    /// The base endpoint for the JSON Placeholder API.
    ///
    /// This pattern can be extended if there are multiple bases
    /// for a REST API.
    Typicode,
}

impl UrlBase {
    pub fn base(&self) -> &str {
        match self {
            UrlBase::Typicode => "https://jsonplaceholder.typicode.com",
        }
    }
}

impl Display for UrlBase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UrlBase::Typicode => write!(f, "https://jsonplaceholder.typicode.com"),
        }
    }
}

pub trait Endpoint {
    fn method(&self) -> http::Method;

    fn endpoint(&self) -> Cow<'static, str>;

    fn url_base(&self) -> UrlBase {
        UrlBase::Typicode
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }

    fn url(&self) -> Result<String> {
        let mut url = Url::parse(self.url_base().to_string().as_str())?;
        self.parameters().add_to_url(&mut url);
        let url = url.to_string();
        Ok(url)
    }

    fn query(&self, url: String, client: &reqwest::blocking::Client) -> anyhow::Result<String> {
        let res = client.get(url).send()?.text()?;
        Ok(res)
    }
}
