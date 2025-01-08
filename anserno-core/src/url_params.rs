#[derive(serde::Serialize, serde::Deserialize)]
#[serde(default)]
pub struct Pagination {
    pub page: u64,
    pub items: u64,
}

impl ::std::default::Default for Pagination {
    fn default() -> Self {
        Pagination { page: 1, items: 12 }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Search {
    pub query: String,
}
