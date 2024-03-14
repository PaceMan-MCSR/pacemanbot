use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub event_id: String,
    pub rta: i64,
    pub igt: i64,
}

impl PartialEq for Event {
    fn eq(&self, other: &Self) -> bool {
        let event_id_check = self.event_id == other.event_id;
        let rta_check = self.rta == other.rta;
        let igt_check = self.igt == other.rta;
        event_id_check && rta_check && igt_check
    }
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uuid: String,
    pub live_account: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub world_id: String,
    pub event_list: Vec<Event>,
    pub context_event_list: Vec<Event>,
    pub user: User,
    pub is_cheated: bool,
    pub is_hidden: bool,
    pub last_updated: i64,
    pub nickname: String,
}
