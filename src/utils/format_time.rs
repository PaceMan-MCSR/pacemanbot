pub fn format_time(milliseconds: u64) -> String {
    let seconds_total = milliseconds / 1000;
    let hours = seconds_total / (60 * 60);
    let minutes = seconds_total / 60;
    let seconds = seconds_total % 60;
    format!("{}:{:02}:{:02}", hours, minutes, seconds)
}
