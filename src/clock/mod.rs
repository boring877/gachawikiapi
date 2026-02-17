// Clock module for Discord bot integration
// Provides game reset timers and countdown calculations

mod games;
mod utils;

use crate::types::{ClockResponse, GameClock, TimerType};
use games::{GameConfig, GAMES};
use utils::{
    calculate_time_remaining, format_iso_date, get_next_daily_reset, get_next_weekly_reset,
    parse_iso_date,
};
use js_sys::Date;

/// Build a GameClock from config
fn build_game_clock(config: &GameConfig, now_ms: f64) -> GameClock {
    let (next_reset_ms, is_active) = match &config.timer_type {
        TimerType::Daily => {
            let reset = get_next_daily_reset(now_ms, config.reset_hour_utc, config.reset_minute_utc);
            (reset, true)
        }
        TimerType::Weekly => {
            let day = config.weekly_reset_day.unwrap_or(1);
            let reset = get_next_weekly_reset(now_ms, day, config.reset_hour_utc, config.reset_minute_utc);
            (reset, true)
        }
        TimerType::Launch => {
            if let Some(launch_date) = config.launch_date {
                if let Some(target_ms) = parse_iso_date(launch_date) {
                    if target_ms > now_ms {
                        (target_ms, true)
                    } else {
                        // Already launched
                        (now_ms, false)
                    }
                } else {
                    (now_ms, false)
                }
            } else {
                (now_ms, false)
            }
        }
    };

    let next_reset_utc = format_iso_date(next_reset_ms);
    let time_remaining = calculate_time_remaining(now_ms, next_reset_ms);

    GameClock {
        id: config.id.to_string(),
        name: config.name.to_string(),
        timer_type: config.timer_type.clone(),
        timer_name: config.timer_name.to_string(),
        reset_hour_utc: config.reset_hour_utc,
        reset_minute_utc: config.reset_minute_utc,
        timezone: config.timezone.to_string(),
        launch_date: config.launch_date.map(|s| s.to_string()),
        weekly_reset_day: config.weekly_reset_day,
        next_reset_utc,
        time_remaining,
        is_active,
    }
}

/// Get all game clocks with current time calculations
pub fn get_all_clocks() -> ClockResponse {
    let now_ms = Date::now();
    let current_time_utc = format_iso_date(now_ms);

    let games: Vec<GameClock> = GAMES
        .iter()
        .map(|config| build_game_clock(config, now_ms))
        .collect();

    ClockResponse {
        current_time_utc,
        games,
    }
}

/// Get clock for a specific game by ID
pub fn get_game_clock(game_id: &str) -> Option<GameClock> {
    let now_ms = Date::now();

    GAMES
        .iter()
        .find(|config| config.id == game_id)
        .map(|config| build_game_clock(config, now_ms))
}

/// Get list of supported game IDs
pub fn get_game_ids() -> Vec<&'static str> {
    GAMES.iter().map(|g| g.id).collect()
}
