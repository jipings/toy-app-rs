use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize)]
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Page {
    #[prost(int32, tag = "1")]
    pub book_id: i32,
    #[prost(string, optional, tag = "2")]
    pub content: ::core::option::Option<::prost::alloc::string::String>,
    #[prost(uint32, tag = "3")]
    pub page_number: u32,
    #[prost(int32, tag = "4")]
    pub id: i32,
}
