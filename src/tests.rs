use crate::utils::{
    extract_name_and_splits_from_line, extract_split_from_pb_role_name,
    extract_split_from_role_name,
};

#[test]
pub fn test_extract_split_from_role_name() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        extract_split_from_role_name("*SS9:4")?,
        ("SS".to_string(), 9, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*SS10:4")?,
        ("SS".to_string(), 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*E10:4")?,
        ("E".to_string(), 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*EE10:4")?,
        ("EE".to_string(), 10, 40)
    );
    Ok(())
}

#[test]
pub fn test_extract_split_from_pb_role_name() {
    assert_eq!(extract_split_from_pb_role_name("*FSPB"), "FS".to_string());
    assert_eq!(extract_split_from_pb_role_name("*SSPB"), "SS".to_string());
    assert_eq!(extract_split_from_pb_role_name("*BPB"), "B".to_string());
    assert_eq!(extract_split_from_pb_role_name("*EPB"), "E".to_string());
    assert_eq!(extract_split_from_pb_role_name("*EEPB"), "EE".to_string());
}

#[test]
pub fn test_extract_name_and_splits_from_line() -> Result<(), Box<dyn std::error::Error>> {
    assert_eq!(
        extract_name_and_splits_from_line("SathyaPramodh: 10/20/30/40/50")?,
        ("SathyaPramodh".to_string(), vec![10, 20, 30, 40, 50])
    );
    assert_eq!(
        extract_name_and_splits_from_line("name_name_: 10/20/30/40/50")?,
        ("name_name_".to_string(), vec![10, 20, 30, 40, 50])
    );
    Ok(())
}
