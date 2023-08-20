
#[derive(Debug, PartialEq, Eq, Clone, serde::Deserialize, serde::Serialize, derive_builder::Builder)]
#[builder(setter(into))]
pub struct CreatedBook {
    pub id: i32,
    pub title: String,
    pub isbn: String,
}
