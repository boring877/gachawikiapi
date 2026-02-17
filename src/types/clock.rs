// Clock types for Discord bot integration

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum TimerType {
    Daily,
    Weekly,
    Launch,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct TimeRemaining {
    pub days: u64,
    pub hours: u64,
    pub minutes: u64,
    pub seconds: u64,
    pub total_seconds: u64,
    pub display: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct GameClock {
    pub id: String,
    pub name: String,
    pub timer_type: TimerType,
    pub timer_name: String,
    pub reset_hour_utc: u8,
    pub reset_minute_utc: u8,
    pub timezone: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub launch_date: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weekly_reset_day: Option<u8>,
    pub next_reset_utc: String,
    pub time_remaining: TimeRemaining,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ClockResponse {
    pub current_time_utc: String,
    pub games: Vec<GameClock>,
}
