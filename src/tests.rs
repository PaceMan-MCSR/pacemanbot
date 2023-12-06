use crate::utils::extract_split_from_role_name;

#[test]
pub fn test_extract_split_from_role_name() {
    assert_eq!(
        extract_split_from_role_name("*FS9:4"),
        ("FS".to_string(), 9, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*FS10:4"),
        ("FS".to_string(), 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*E10:4"),
        ("E".to_string(), 10, 40)
    );
}
