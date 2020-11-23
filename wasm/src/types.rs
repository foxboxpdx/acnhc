use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct CatalogData {
    item_id: i32,
    user_id: i32,
    owned_id: i32,
    name_en: String,
    name_jp: String,
    thumb: String,
    owned: bool,
    extra: i32
}