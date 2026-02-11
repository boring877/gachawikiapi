// there were our types for our gacha wiki !
// the data !

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub id: u32,
    pub name: String,
    pub rarity: u8,
    pub element: String,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    pub id: u32,
    pub name: String,
    pub item_type: String,
}
