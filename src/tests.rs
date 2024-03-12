use crate::{
    types::{PlayerSplitsData, Split},
    utils::{
        extract_name_and_splits_from_line, extract_split_from_pb_role_name,
        extract_split_from_role_name,
    },
};

#[test]
pub fn test_extract_split_from_role_name() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        extract_split_from_role_name("*SS9:4")?,
        (Split::SecondStructure, 9, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*SS10:4")?,
        (Split::SecondStructure, 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*E10:4")?,
        (Split::EyeSpy, 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*EE10:4")?,
        (Split::EndEnter, 10, 40)
    );
    Ok(())
}

#[test]
pub fn test_extract_split_from_pb_role_name() {
    assert_eq!(
        extract_split_from_pb_role_name("*FSPB"),
        Some(Split::FirstStructure)
    );
    assert_eq!(
        extract_split_from_pb_role_name("*SSPB"),
        Some(Split::SecondStructure)
    );
    assert_eq!(extract_split_from_pb_role_name("*BPB"), Some(Split::Blind));
    assert_eq!(extract_split_from_pb_role_name("*EPB"), Some(Split::EyeSpy));
    assert_eq!(
        extract_split_from_pb_role_name("*EEPB"),
        Some(Split::EndEnter)
    );
}

#[test]
pub fn test_extract_name_and_splits_from_line() -> Result<(), Box<dyn std::error::Error>> {
    let split_data = PlayerSplitsData {
        first_structure: 10,
        second_structure: 20,
        blind: 30,
        eye_spy: 40,
        end_enter: 50,
    };
    assert_eq!(
        extract_name_and_splits_from_line("SathyaPramodh: 10/20/30/40/50")?,
        ("SathyaPramodh".to_string(), split_data)
    );
    assert_eq!(
        extract_name_and_splits_from_line("name_name_: 10/20/30/40/50")?,
        ("name_name_".to_string(), split_data)
    );
    Ok(())
}
