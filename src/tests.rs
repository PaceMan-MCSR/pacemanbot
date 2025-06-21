use crate::{
    cache::{consts::ROLE_PREFIX, players::PlayerSplitsData, split::Split},
    utils::{
        extract_name_and_splits_from_line::extract_name_and_splits_from_line,
        extract_split_from_pb_role_name::extract_split_from_pb_role_name,
        extract_split_from_role_name::extract_split_from_role_name,
    },
    Result,
};

#[test]
pub fn test_extract_split_from_role_name() -> Result<()> {
    assert_eq!(
        extract_split_from_role_name(format!("{}AT9:0", ROLE_PREFIX).as_str())?,
        (Split::AdventuringTime, 9, 0)
    );
    assert_eq!(
        extract_split_from_role_name(format!("{}B10:4", ROLE_PREFIX).as_str())?,
        (Split::Beaconator, 10, 4)
    );
    assert_eq!(
        extract_split_from_role_name(format!("{}H10:4", ROLE_PREFIX).as_str())?,
        (Split::HDWGH, 10, 4)
    );
    Ok(())
}

#[test]
pub fn test_extract_split_from_pb_role_name() {
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}ATPB", ROLE_PREFIX).as_str()),
        Some(Split::AdventuringTime)
    );
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}BPB", ROLE_PREFIX).as_str()),
        Some(Split::Beaconator)
    );
    assert_eq!(
        extract_split_from_pb_role_name(format!("{}HPB", ROLE_PREFIX).as_str()),
        Some(Split::HDWGH)
    );
}

#[test]
pub fn test_extract_name_and_splits_from_line() -> Result<()> {
    let mut split_data = PlayerSplitsData {
        adventuring_time: 10,
        beaconator: 20,
        hdwgh: 30,
        finish: None,
    };
    assert_eq!(
        extract_name_and_splits_from_line("Its_Saanvi: 0;10/0;20/0;30")?,
        ("Its_Saanvi".to_string(), split_data)
    );
    assert_eq!(
        extract_name_and_splits_from_line("name_name_: 0;10/0;20/0;30")?,
        ("name_name_".to_string(), split_data)
    );

    split_data.finish = Some(60);
    assert_eq!(
        extract_name_and_splits_from_line("Its_Saanvi: 0;10/0;20/0;30/0;60")?,
        ("Its_Saanvi".to_string(), split_data)
    );
    assert_eq!(
        extract_name_and_splits_from_line("name_name_: 0;10/0;20/0;30/0;60")?,
        ("name_name_".to_string(), split_data)
    );
    Ok(())
}
