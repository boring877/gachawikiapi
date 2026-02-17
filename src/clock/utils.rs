// Time utilities for clock calculations

use crate::types::TimeRemaining;
use js_sys::Date;
use wasm_bindgen::JsValue;

/// Calculate time remaining until target timestamp
pub fn calculate_time_remaining(now_ms: f64, target_ms: f64) -> TimeRemaining {
    let diff = if target_ms > now_ms {
        (target_ms - now_ms) as u64
    } else {
        0
    };

    let total_seconds = diff / 1000;
    let days = total_seconds / 86400;
    let hours = (total_seconds % 86400) / 3600;
    let minutes = (total_seconds % 3600) / 60;
    let seconds = total_seconds % 60;

    let display = if days > 0 {
        format!("{}d {}h {}m", days, hours, minutes)
    } else if hours > 0 {
        format!("{}h {}m {}s", hours, minutes, seconds)
    } else if minutes > 0 {
        format!("{}m {}s", minutes, seconds)
    } else {
        format!("{}s", seconds)
    };

    TimeRemaining {
        days,
        hours,
        minutes,
        seconds,
        total_seconds,
        display,
    }
}

/// Get current date components
pub fn get_current_date() -> (u32, u32, u32, u32, u32, u32, u32, f64) {
    let now = Date::now();
    let date = Date::new(&JsValue::from_f64(now));

    (
        date.get_full_year() as u32, // u32
        date.get_month(),            // u32 (0-11)
        date.get_date(),             // u32 (1-31)
        date.get_day(),              // u32 (0-6, Sunday = 0)
        date.get_hours(),            // u32 (0-23)
        date.get_minutes(),          // u32 (0-59)
        date.get_seconds(),          // u32 (0-59)
        now,                         // f64 milliseconds
    )
}

/// Create a date timestamp from components (returns milliseconds)
pub fn create_timestamp(year: u32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> f64 {
    // Date::new_with_year_month_day_hr_min_sec signature:
    // (year: u32, month: i32, day: i32, hour: i32, minute: i32, second: i32)
    let date = Date::new_with_year_month_day_hr_min_sec(
        year,
        month as i32,   // JS months are 0-indexed
        day as i32,
        hour as i32,
        minute as i32,
        second as i32,
    );
    date.get_time()
}

/// Get next daily reset time in milliseconds
pub fn get_next_daily_reset(now_ms: f64, reset_hour_utc: u8, reset_minute_utc: u8) -> f64 {
    let (year, month, date, _, _, _, _, _) = get_current_date();

    let ms_per_day = 86400000.0;

    // Create target time for today
    let today_reset = create_timestamp(
        year,
        month,
        date,
        reset_hour_utc as u32,
        reset_minute_utc as u32,
        0,
    );

    // If reset has passed today, use tomorrow
    if today_reset <= now_ms {
        today_reset + ms_per_day
    } else {
        today_reset
    }
}

/// Get next weekly reset time in milliseconds
pub fn get_next_weekly_reset(now_ms: f64, target_day: u8, reset_hour_utc: u8, reset_minute_utc: u8) -> f64 {
    let (year, month, date, current_day, _, _, _, _) = get_current_date();
    let ms_per_day = 86400000.0;

    // 0 = Sunday, 1 = Monday, etc.
    let days_until_target = if target_day as u32 >= current_day {
        target_day as u32 - current_day
    } else {
        7 - (current_day - target_day as u32)
    };

    // Create target time
    let target_ms = create_timestamp(
        year,
        month,
        date + days_until_target,
        reset_hour_utc as u32,
        reset_minute_utc as u32,
        0,
    );

    // If target is today but already passed, move to next week
    if target_ms <= now_ms {
        target_ms + (7.0 * ms_per_day)
    } else {
        target_ms
    }
}

/// Parse ISO date string to milliseconds
pub fn parse_iso_date(iso_str: &str) -> Option<f64> {
    // Simple parser for format: 2026-03-01T13:00:00Z
    let parts: Vec<&str> = iso_str.split('T').collect();
    if parts.len() != 2 {
        return None;
    }

    let date_parts: Vec<u32> = parts[0].split('-').filter_map(|s| s.parse().ok()).collect();
    let time_parts: Vec<u32> = parts[1].trim_end_matches('Z').split(':').filter_map(|s| s.parse().ok()).collect();

    if date_parts.len() != 3 || time_parts.len() != 3 {
        return None;
    }

    // date_parts: [year, month, day] - month needs to be converted to 0-indexed
    let timestamp = create_timestamp(
        date_parts[0],
        date_parts[1] - 1, // Convert to 0-indexed month
        date_parts[2],
        time_parts[0],
        time_parts[1],
        time_parts[2],
    );

    Some(timestamp)
}

/// Format date as ISO 8601 string
pub fn format_iso_date(ms: f64) -> String {
    let date = Date::new(&JsValue::from_f64(ms));
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        date.get_full_year(),
        date.get_month() + 1,
        date.get_date(),
        date.get_hours(),
        date.get_minutes(),
        date.get_seconds()
    )
}
