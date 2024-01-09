use std::fmt::Display;

use derive_builder::Builder;

use crate::api::endpoint_prelude::*;

use super::params::Key;

#[derive(Debug, Clone)]
pub enum ResourceType {
    Posts,
    Comments,
    Albums,
    Photos,
    Todos,
    Users,
}

impl Display for ResourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ResourceType::Posts => write!(f, "posts"),
            ResourceType::Comments => write!(f, "comments"),
            ResourceType::Albums => write!(f, "albums"),
            ResourceType::Photos => write!(f, "photos"),
            ResourceType::Todos => write!(f, "todos"),
            ResourceType::Users => write!(f, "users"),
        }
    }
}

/// Query for a specific resource
#[derive(Debug, Clone, Builder)]
#[builder(setter(strip_option))]
pub struct Resource {
    #[builder(setter(into))]
    resource: ResourceType,
    #[builder(default)]
    format: Option<Key>,
    #[builder(default)]
    offset: Option<Key>,
    #[builder(default)]
    limit: Option<Key>,
    #[builder(default)]
    from_date_time: Option<Key>,
    #[builder(default)]
    to_date_time: Option<Key>,
    #[builder(default)]
    sort: Option<Key>,
}

impl Resource {
    pub fn builder() -> ResourceBuilder {
        ResourceBuilder::default()
    }
}

impl Endpoint for Resource {
    fn method(&self) -> http::Method {
        Method::GET
    }

    fn endpoint(&self) -> Cow<'static, str> {
        format!("{}/", self.resource).into()
    }

    fn parameters(&self) -> QueryParams {
        let mut params = QueryParams::default();

        params
            .push_opt("format", self.format)
            .push_opt("sort", self.sort)
            .push_opt("limit", self.limit)
            .push_opt("offset", self.offset)
            .push_opt("toDateTime", self.to_date_time)
            .push_opt("fromDateTime", self.from_date_time);
        params
    }
}

#[cfg(test)]
mod tests {
    use crate::api::params::Key;

    use super::{Resource, ResourceType};

    #[test]
    fn endpoint() {
        let endpoint = Resource::builder()
            .resource(ResourceType::Posts)
            .format(Key::Format)
            .build()
            .unwrap();
        println!("{:?}", endpoint);
    }
}
