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
    post_id: Option<Key>,
    #[builder(default)]
    user_id: Option<Key>,
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
            .push_opt("post_id", self.post_id)
            .push_opt("user_id", self.user_id);
        params
    }
}

#[cfg(test)]
mod tests {
    use super::{Resource, ResourceType};

    #[test]
    fn posts_endpoint() {
        let endpoint = Resource::builder()
            .resource(ResourceType::Posts)
            .build()
            .unwrap();
        println!("{:?}", endpoint);
    }
}
