use crate::{cache::players::PlayerSplitsData, Result};

pub fn extract_name_or_uuid_and_splits_from_line(line: &str) -> Result<(String, PlayerSplitsData)> {
    let line = line.trim();
    let line = line.replace(" ", "");
    let line_splits = line.split(':').collect::<Vec<&str>>();
    if line_splits.len() != 2 {
        return Err(format!("ExtractError: parse line contents: '{}'.", line).into());
    }
    let (player_name_or_uuid, splits_string) = (line_splits[0], line_splits[1]);
    let splits = splits_string.split('/').collect::<Vec<&str>>();
    if splits.len() != 5 && splits.len() != 6 {
        return Err(format!("ExtractError: parse line contents: '{}'.", line).into());
    }
    let mut idx = 0;
    let mut split_data = PlayerSplitsData::default();
    for split in splits {
        let split_u8 = match split.parse::<u8>() {
            Ok(split) => split,
            Err(err) => {
                return Err(format!("ExtractError: parse to u8 due to: {}", err).into());
            }
        };
        match idx {
            0 => split_data.first_structure = split_u8,
            1 => split_data.second_structure = split_u8,
            2 => split_data.blind = split_u8,
            3 => split_data.eye_spy = split_u8,
            4 => split_data.end_enter = split_u8,
            5 => split_data.finish = Some(split_u8),
            _ => (),
        };
        idx += 1;
    }
    Ok((player_name_or_uuid.to_string(), split_data))
}
