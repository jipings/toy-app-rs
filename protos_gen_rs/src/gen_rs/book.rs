use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BookRequest {
    #[prost(int32, optional, tag = "1")]
    pub id: ::core::option::Option<i32>,
    #[prost(string, tag = "2")]
    pub isbn: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub title: ::prost::alloc::string::String,
}
