pub fn millis_to_hrs_mins(milliseconds: u64) -> (u8, u8) {
    let seconds_total = milliseconds / 1000;
    let hours = seconds_total / (60 * 60);
    let minutes = seconds_total / 60;
    (hours as u8, minutes as u8)
}
