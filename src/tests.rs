use crate::{
    cache::{players::PlayerSplitsData, split::Split},
    utils::{
        extract_name_and_splits_from_line::extract_name_and_splits_from_line,
        extract_split_from_pb_role_name::extract_split_from_pb_role_name,
        extract_split_from_role_name::extract_split_from_role_name,
    },
};

#[test]
pub fn test_extract_split_from_role_name() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        extract_split_from_role_name("*1717T9:4")?,
        (Split::TowerStart, 9, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*17T10:4")?,
        (Split::TowerStart, 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*17EE10:4")?,
        (Split::EndEnter, 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*17EE10:4")?,
        (Split::EndEnter, 10, 40)
    );
    Ok(())
}

#[test]
pub fn test_extract_split_from_pb_role_name() {
    assert_eq!(
        extract_split_from_pb_role_name("*17TPB"),
        Some(Split::TowerStart)
    );
    assert_eq!(
        extract_split_from_pb_role_name("*17EEPB"),
        Some(Split::EndEnter)
    );
}

#[test]
pub fn test_extract_name_and_splits_from_line() -> Result<(), Box<dyn std::error::Error>> {
    let mut split_data = PlayerSplitsData {
        tower_start: 40,
        end_enter: 50,
        finish: None,
    };
    assert_eq!(
        extract_name_and_splits_from_line("SathyaPramodh: 40/50")?,
        ("SathyaPramodh".to_string(), split_data)
    );
    assert_eq!(
        extract_name_and_splits_from_line("name_name_: 40/50")?,
        ("name_name_".to_string(), split_data)
    );

    split_data.finish = Some(60);
    assert_eq!(
        extract_name_and_splits_from_line("SathyaPramodh: 40/50/60")?,
        ("SathyaPramodh".to_string(), split_data)
    );
    assert_eq!(
        extract_name_and_splits_from_line("name_name_: 40/50/60")?,
        ("name_name_".to_string(), split_data)
    );
    Ok(())
}
