use serenity::utils::Color;

pub const SPECIAL_UNDERSCORE: &'static str = "ˍ";

pub const WS_FALLBACK_HOST: &'static str = "paceman.gg:8081";
pub const WS_FALLBACK_URL: &'static str = "wss://paceman.gg/ws";
pub const WS_UPGRADE_HEADER: &'static str = "upgrade";
pub const WS_CONNECTION_HEADER: &'static str = "upgrade";
pub const WS_SEC_VERSION_HEADER: u64 = 13;

pub const WS_TIMEOUT_FOR_RETRY: u64 = 5;

pub const ROLE_COLOR: u32 = Color::from_rgb(54, 57, 63).0;
