use crate::cache::split::Split;

pub fn extract_split_from_pb_role_name(role_name: &str) -> Option<Split> {
    let role_name = role_name.replace("*", "");
    let role_name = role_name.replace(" ", "");
    let role_name = role_name.replace("PB", "");
    Split::from_str(role_name.as_str())
}
