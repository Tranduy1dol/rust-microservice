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

/// Default page number for pagination.
///
/// # Returns
///
/// The default page number: `1`.
///
/// # Examples
///
/// ```ignore
/// let p = default_page();
/// assert_eq!(p, 1);
/// ```
fn default_page() -> u64 {
    1
}
/// Provides the default page size for pagination.
///
/// # Returns
///
/// The default page size value: `10`.
///
/// # Examples
///
/// ```ignore
/// assert_eq!(default_page_size(), 10);
/// ```
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
    /// Creates a paginated response containing the provided items and pagination metadata.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// let dto = PaginationResponseDto::new(vec![1, 2, 3], 1, 10);
    /// assert_eq!(dto.data, vec![1, 2, 3]);
    /// assert_eq!(dto.page, 1);
    /// assert_eq!(dto.page_size, 10);
    /// ```
    pub fn new(data: Vec<T>, page: u64, page_size: u64) -> Self {
        Self {
            data,
            page,
            page_size,
        }
    }
}
