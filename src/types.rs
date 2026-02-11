// there were our types for our gacha wiki !
// the data !

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub id: u32,
    pub name: String,
    pub rarity: u8,
    pub element: String,
}
