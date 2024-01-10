use std::{borrow::Cow, fmt::Display};

use url::Url;

/// impl this trait on the type of any return value
pub trait ParamValue<'a> {
    fn as_value(&self) -> Cow<'a, str>;
}

impl<'a> ParamValue<'a> for Key {
    fn as_value(&self) -> Cow<'a, str> {
        match self {
            Self::PostId => "postId".into(),
            Self::UserId => "userId".into(),
        }
    }
}

/// Keys for the query parameters available on the Congress.gov and GovInfo APIs.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    PostId,
    UserId,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::PostId => write!(f, "postId"),
            Key::UserId => write!(f, "userId"),
        }
    }
}

impl From<Key> for &str {
    fn from(key: Key) -> &'static str {
        match key {
            Key::PostId => "postId",
            Key::UserId => "userId",
        }
    }
}

#[derive(Default)]
pub struct QueryParams<'a> {
    params: Vec<(Cow<'a, str>, Cow<'a, str>)>,
}

impl<'a> QueryParams<'a> {
    pub fn push<'b, K, V>(&mut self, key: K, value: V) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        self.params.push((key.into(), value.as_value()));
        self
    }

    pub fn push_opt<'b, K, V>(&mut self, key: K, value: Option<V>) -> &mut Self
    where
        K: Into<Cow<'a, str>>,
        V: ParamValue<'b>,
        'b: 'a,
    {
        if let Some(value) = value {
            self.params.push((key.into(), value.as_value()));
        }
        self
    }

    pub fn add_to_url(&self, url: &mut Url) {
        let mut pairs = url.query_pairs_mut();
        pairs.extend_pairs(self.params.iter());
    }
}
