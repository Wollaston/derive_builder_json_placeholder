use std::borrow::Cow;

use crate::api::params::QueryParams;

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

pub trait Endpoint {
    fn method(&self) -> http::Method;

    fn endpoint(&self) -> Cow<'static, str>;

    fn url_base(&self) -> UrlBase {
        UrlBase::Typicode
    }

    fn parameters(&self) -> QueryParams {
        QueryParams::default()
    }
}
