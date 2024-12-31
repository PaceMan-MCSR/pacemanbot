use crate::cache::{consts::ROLE_PREFIX, split::Split};

pub fn extract_split_from_pb_role_name(role_name: &str) -> Option<Split> {
    let role_name = role_name.replace(ROLE_PREFIX, "");
    let role_name = role_name.replace(" ", "");
    let role_name = role_name.replace("PB", "");
    Split::from_str(role_name.as_str())
}
