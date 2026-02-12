// Zone Nova character types for the API

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct CharacterStats {
    pub hp: u32,
    pub attack: u32,
    pub defense: u32,
    pub energyRecovery: f32,
    pub critRate: f32,
    pub critDmg: f32,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Skill {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub cooldown: Option<String>,
    #[serde(default)]
    pub energyCost: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Skills {
    pub normal: Skill,
    pub auto: Skill,
    pub ultimate: Skill,
    pub passive: Skill,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TeamSkillRequirements {
    pub faction: String,
    pub element: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct TeamSkill {
    pub name: String,
    pub description: String,
    pub requirements: TeamSkillRequirements,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct Awakening {
    pub level: u8,
    pub effect: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MemoryCardStats {
    pub hp: String,
    pub attack: String,
    pub defense: String,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct MemoryCard {
    pub name: String,
    pub image: String,
    pub stats: MemoryCardStats,
    pub effects: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[allow(non_snake_case)]
pub struct ZoneNovaCharacter {
    pub id: u32,
    pub name: String,
    pub slug: String,
    pub rarity: String,
    pub element: String,
    #[serde(rename = "class")]
    pub char_class: String,
    pub role: String,
    pub faction: String,
    pub tags: Vec<String>,
    pub image: String,
    pub detailUrl: String,
    pub stats: CharacterStats,
    pub skills: Skills,
    pub teamSkill: TeamSkill,
    pub awakenings: Vec<Awakening>,
    pub memoryCard: MemoryCard,
}
