use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateCategoryRequestDto {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EditCreateCategoryRequestDto {
    pub id: i64,
    pub name: Option<String>,
    pub description: Option<String>,
}
