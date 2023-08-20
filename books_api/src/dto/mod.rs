use derive_builder::Builder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Builder, Debug)]
pub struct Book {
    pub id: i32,
    pub title: String,
    pub isbn: String,
}

#[derive(Serialize, Deserialize, Builder, Debug)]
pub struct Page {
    pub id: i32,
    pub book_id: i32,
    pub page_number: u32,
    pub content: String,
}
