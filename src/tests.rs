use crate::utils::extract_split_from_role_name;

#[test]
pub fn test_extract_split_from_role_name() {
    assert_eq!(
        extract_split_from_role_name("*SS9:4"),
        ("SS".to_string(), 9, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*SS10:4"),
        ("SS".to_string(), 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*E10:4"),
        ("E".to_string(), 10, 40)
    );
    assert_eq!(
        extract_split_from_role_name("*EE10:4"),
        ("EE".to_string(), 10, 40)
    );
}
