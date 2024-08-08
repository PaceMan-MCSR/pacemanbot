use serenity::model::guild::Role;

use crate::{
    utils::{
        extract_split_from_pb_role_name::extract_split_from_pb_role_name,
        extract_split_from_role_name::extract_split_from_role_name,
        extract_splits_and_name_from_role_name::extract_splits_and_name_from_role_name,
    },
    Result,
};

use super::split::Split;

#[derive(Debug)]
pub struct RoleData {
    pub split: Split,
    pub minutes: u8,
    pub seconds: u8,
    pub runner: String,
    pub guild_role: Role,
}

impl RoleData {
    pub fn new(guild_role: Role) -> Result<Self> {
        let split: Split;
        let mut minutes: u8 = 0;
        let mut seconds: u8 = 0;
        let mut runner: String = String::new();
        if guild_role.name.contains("PB") {
            split = match extract_split_from_pb_role_name(guild_role.name.as_str()) {
                Some(tup) => tup,
                None => {
                    return Err(format!(
                        "RoleDataError: extract split from pb role name: {}.",
                        guild_role.name
                    )
                    .into())
                }
            };
        } else if guild_role.name.contains("+") {
            (split, minutes, seconds, runner) =
                match extract_splits_and_name_from_role_name(guild_role.name.as_str()) {
                    Ok(tup) => tup,
                    Err(err) => {
                        return Err(format!(
                            "RoleDataError: extract split from pb role name: {} due to: {}",
                            guild_role.name, err
                        )
                        .into())
                    }
                }
        } else {
            (split, minutes, seconds) = match extract_split_from_role_name(guild_role.name.as_str())
            {
                Ok(tup) => tup,
                Err(err) => {
                    return Err(format!(
                        "RoleDataError: extract split from role name: {} due to: {}",
                        guild_role.name, err
                    )
                    .into())
                }
            };
        }
        Ok(Self {
            guild_role,
            split,
            minutes,
            seconds,
            runner,
        })
    }
}
