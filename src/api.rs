//! Root-level API endpoints
//!     - /comments
pub mod resource;

/// Endpoint types
mod endpoint;

/// The client for calling the endpoints
mod client;

/// The base struct initialized for calling endpoints.
mod user;

/// Types for query parameters
mod params;

/// Query functionality
mod query;

/// Endpoint prelude to share functionality across different endpoints
pub mod endpoint_prelude;
