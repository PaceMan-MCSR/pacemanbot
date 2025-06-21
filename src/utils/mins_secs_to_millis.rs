pub fn hrs_mins_secs_to_millis(time: (u8, u8)) -> u64 {
    let (hours, minutes) = (time.0 as u64, time.1 as u64);
    hours * 3600000 + minutes * 60000
}
