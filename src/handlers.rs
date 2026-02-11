use std::fs;
use axum::Json;
use crate::types::Character;

pub async fn get_characters() -> Json<Vec<Character>> {
    // read the json file
    let json = fs::read_to_string("data/characters.json").unwrap();
    // parse the json file
    let characters: Vec<Character> = serde_json::from_str(&json).unwrap();
    // return the characters
    Json(characters)
}