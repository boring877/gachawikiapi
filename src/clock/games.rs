// Game configurations for clock timers
// Data extracted from gacha-wiki/src/data/clock/

use crate::types::TimerType;

/// Game configuration for clock timers
pub struct GameConfig {
    pub id: &'static str,
    pub name: &'static str,
    pub timer_type: TimerType,
    pub timer_name: &'static str,
    pub reset_hour_utc: u8,
    pub reset_minute_utc: u8,
    pub timezone: &'static str,
    pub launch_date: Option<&'static str>,
    pub weekly_reset_day: Option<u8>,
}

/// All supported games with their clock configurations
pub const GAMES: &[GameConfig] = &[
    // Zone Nova - Daily reset at 20:00 UTC (04:00 UTC+8)
    GameConfig {
        id: "zone-nova",
        name: "Zone Nova",
        timer_type: TimerType::Daily,
        timer_name: "Daily Reset",
        reset_hour_utc: 20,
        reset_minute_utc: 0,
        timezone: "UTC+8",
        launch_date: None,
        weekly_reset_day: None,
    },
    // Silver & Blood - Daily reset at 20:00 UTC
    GameConfig {
        id: "silver-and-blood",
        name: "Silver & Blood",
        timer_type: TimerType::Daily,
        timer_name: "Daily Reset",
        reset_hour_utc: 20,
        reset_minute_utc: 0,
        timezone: "UTC",
        launch_date: None,
        weekly_reset_day: None,
    },
    // Horizon Walker - Daily reset at 15:00 UTC (00:00 UTC+9)
    GameConfig {
        id: "horizon-walker",
        name: "Horizon Walker",
        timer_type: TimerType::Daily,
        timer_name: "Daily Reset",
        reset_hour_utc: 15,
        reset_minute_utc: 0,
        timezone: "UTC+9",
        launch_date: None,
        weekly_reset_day: None,
    },
    // Stella Sora - Daily reset at 20:00 UTC (13:00 UTC-7)
    GameConfig {
        id: "stella-sora",
        name: "Stella Sora",
        timer_type: TimerType::Daily,
        timer_name: "Daily Reset",
        reset_hour_utc: 20,
        reset_minute_utc: 0,
        timezone: "UTC-7",
        launch_date: None,
        weekly_reset_day: Some(1), // Monday
    },
    // Busty Burst Fantasy - Daily reset at 03:00 UTC
    GameConfig {
        id: "busty-burst",
        name: "Busty Burst Fantasy",
        timer_type: TimerType::Daily,
        timer_name: "Daily Reset",
        reset_hour_utc: 3,
        reset_minute_utc: 0,
        timezone: "UTC",
        launch_date: None,
        weekly_reset_day: None,
    },
    // Taimanin Squad - Launch countdown March 1, 2026 13:00 UTC
    GameConfig {
        id: "taimanin-squad",
        name: "Taimanin Squad",
        timer_type: TimerType::Launch,
        timer_name: "Game Launch",
        reset_hour_utc: 13,
        reset_minute_utc: 0,
        timezone: "UTC+9",
        launch_date: Some("2026-03-01T13:00:00Z"),
        weekly_reset_day: None,
    },
];
