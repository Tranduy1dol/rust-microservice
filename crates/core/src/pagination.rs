use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Debug, Deserialize, Validate, Copy, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pagination {
    #[serde(default = "default_page")]
    #[validate(range(min = 1))]
    pub page: u64,
    #[serde(default = "default_page_size")]
    #[validate(range(min = 1, max = 100))]
    pub page_size: u64,
}

fn default_page() -> u64 {
    1
}
fn default_page_size() -> u64 {
    10
}

#[derive(Serialize, Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationResponseDto<T> {
    pub data: Vec<T>,
    pub page: u64,
    pub page_size: u64,
}

impl<T> PaginationResponseDto<T> {
    pub fn new(data: Vec<T>, page: u64, page_size: u64) -> Self {
        Self {
            data,
            page,
            page_size,
        }
    }
}
