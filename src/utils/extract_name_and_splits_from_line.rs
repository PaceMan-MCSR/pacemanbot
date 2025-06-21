use crate::{cache::players::PlayerSplitsData, Result};

pub fn extract_name_and_splits_from_line(line: &str) -> Result<(String, PlayerSplitsData)> {
    let line = line.trim();
    let line = line.replace(" ", "");
    let line_splits = line.split(':').collect::<Vec<&str>>();
    if line_splits.len() != 2 {
        return Err(format!("ExtractError: parse line contents: '{}'.", line).into());
    }
    let (player_name, splits_string) = (line_splits[0], line_splits[1]);
    let splits = splits_string.split('/').collect::<Vec<&str>>();
    if splits.len() != 3 && splits.len() != 4 {
        return Err(format!("ExtractError: parse line contents: '{}'.", line).into());
    }
    let mut idx = 0;
    let mut split_data = PlayerSplitsData::default();
    for split in splits {
        let time_splits = split.split(';').collect::<Vec<&str>>();
        if time_splits.len() != 2 {
            return Err(format!("ExtractError: parse time: '{}'.", split).into());
        }
        let (split_hours_u32, split_minutes_u32) = (
            match time_splits[0].parse::<u32>() {
                Ok(split) => split,
                Err(err) => {
                    return Err(format!("ExtractError: parse to u8 due to: {}", err).into());
                }
            },
            match time_splits[1].parse::<u32>() {
                Ok(split) => split,
                Err(err) => {
                    return Err(format!("ExtractError: parse to u8 due to: {}", err).into());
                }
            },
        );
        let split_u32 = split_hours_u32 * 60 + split_minutes_u32;
        match idx {
            0 => split_data.adventuring_time = split_u32,
            1 => split_data.beaconator = split_u32,
            2 => split_data.hdwgh = split_u32,
            3 => split_data.finish = Some(split_u32),
            _ => (),
        };
        idx += 1;
    }
    Ok((player_name.to_string(), split_data))
}
