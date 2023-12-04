use crate::utils::extract_split_from_role_name;

#[test]
pub fn test_extract_split_from_role_name() {
    assert_eq!(
        extract_split_from_role_name("PMBFirstStructureSub9:40"),
        ("FirstStructure".to_string(), 9, 40)
    );
    assert_eq!(
        extract_split_from_role_name("PMBFirstStructure10:40"),
        ("FirstStructure".to_string(), 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("PMBEyeSpySub10:40"),
        ("EyeSpy".to_string(), 10, 40)
    );
}
