#[path = "remove_pb_roles.rs"]
mod remove_pb_roles;
#[path = "remove_roles.rs"]
mod remove_roles;
#[path = "send_message.rs"]
mod send_message;
#[path = "setup_default_roles.rs"]
mod setup_default_roles;
#[path = "setup_pb_roles.rs"]
mod setup_pb_roles;
#[path = "setup_pings.rs"]
mod setup_pings;
#[path = "setup_roles.rs"]
mod setup_roles;
#[path = "validate_config.rs"]
mod validate_config;
#[path = "whitelist.rs"]
mod whitelist;
#[path = "whitelist_uuid.rs"]
mod whitelist_uuid;

use crate::command::Command;

use remove_pb_roles::REMOVE_PB_ROLES;
use remove_roles::REMOVE_ROLES;
use send_message::SEND_MESSAGE;
use setup_default_roles::SETUP_DEFAULT_ROLES;
use setup_pb_roles::SETUP_PB_ROLES;
use setup_pings::SETUP_PINGS;
use setup_roles::SETUP_ROLES;
use validate_config::VALIDATE_CONFIG;
use whitelist::WHITELIST;
use whitelist_uuid::WHITELIST_UUID;

pub fn get_default_commands() -> Vec<&'static dyn Command> {
    return vec![
        &SEND_MESSAGE,
        &SETUP_DEFAULT_ROLES,
        &SETUP_PINGS,
        &SETUP_ROLES,
        &SETUP_PB_ROLES,
        &WHITELIST,
        &WHITELIST_UUID,
        &VALIDATE_CONFIG,
        &REMOVE_ROLES,
        &REMOVE_PB_ROLES,
    ];
}
