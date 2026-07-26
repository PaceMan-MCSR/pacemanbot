pub fn hrs_mins_secs_to_millis(time: (u8, u8)) -> u64 {
    let (hours, minutes) = (time.0 as u64, time.1 as u64);
    hours * 3600000 + minutes * 60000
}

pub fn millis_to_hrs_mins(milliseconds: u64) -> (u8, u8) {
    let seconds_total = milliseconds / 1000;
    let hours = seconds_total / (60 * 60);
    let minutes = (seconds_total % 60) / 60;
    (hours as u8, minutes as u8)
}

pub fn format_time(milliseconds: u64) -> String {
    let seconds_total = milliseconds / 1000;
    let hours = seconds_total / (60 * 60);
    let minutes = (seconds_total % 60) / 60;
    let seconds = (seconds_total % 60) % 60;
    format!("{}:{:02}:{:02}", hours, minutes, seconds)
}
