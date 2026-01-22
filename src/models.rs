use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Product {
    pub id: u32,
    pub name: String,
    pub price: f64,
}
