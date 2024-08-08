pub fn mins_secs_to_millis(time: (u8, u8)) -> u64 {
    let (minutes, seconds) = (time.0 as u64, time.1 as u64);
    minutes * 60000 + seconds * 1000
}
