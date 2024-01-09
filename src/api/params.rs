use std::{borrow::Cow, fmt::Display};

/// impl this trait on the type of any return value
pub trait ParamValue<'a> {
    fn as_value(&self) -> Cow<'a, str>;
}

impl<'a> ParamValue<'a> for Key {
    fn as_value(&self) -> Cow<'a, str> {
        match self {
            Self::Sort => "sort".into(),
            _ => "Ok".into(),
        }
    }
}

/// Keys for the query parameters available on the Congress.gov and GovInfo APIs.
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Format,
    Offset,
    Limit,
    FromDateTime,
    ToDateTime,
    Sort,
}

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::Format => write!(f, "format"),
            Key::Offset => write!(f, "offset"),
            Key::Limit => write!(f, "limit"),
            Key::FromDateTime => write!(f, "fromDateTime"),
            Key::ToDateTime => write!(f, "toDateTime"),
            Key::Sort => write!(f, "sort"),
        }
    }
}

impl From<Key> for &str {
    fn from(key: Key) -> &'static str {
        match key {
            Key::Format => "format",
            Key::Offset => "offset",
            Key::Limit => "limit",
            Key::FromDateTime => "fromDateTime",
            Key::ToDateTime => "toDateTime",
            Key::Sort => "sort",
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
}
