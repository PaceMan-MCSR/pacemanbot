pub fn mins_secs_to_millis(time: (u8, u8)) -> u64 {
    let (minutes, seconds) = (time.0 as u64, time.1 as u64);
    minutes * 60000 + seconds * 1000
}

pub fn millis_to_mins_secs(milliseconds: u64) -> (u8, u8) {
    let seconds_total = milliseconds / 1000;
    let minutes = seconds_total / 60;
    let seconds = seconds_total % 60;
    (minutes as u8, seconds as u8)
}

pub fn format_time(milliseconds: u64) -> String {
    let seconds_total = milliseconds / 1000;
    let minutes = seconds_total / 60;
    let seconds = seconds_total % 60;
    format!("{}:{:02}", minutes, seconds)
}
