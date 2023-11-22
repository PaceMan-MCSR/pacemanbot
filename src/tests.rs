use dotenv::dotenv;
use serde_json::json;
use std::env;

use crate::{
    types::{Event, Response, User},
    utils::{extract_split_from_role_name, get_response_from_api},
};

#[tokio::test]
async fn test_api_data() {
    // Test with some dummy data. Call this with `cargo test`.
    dotenv().ok();
    let access_key = env::var("ACCESS_KEY").expect("Expected an access key in the environment");
    let client = reqwest::Client::new();
    let body = json!({
        "accessKey": access_key,
        "eventList": [
            "rsg.enter_nether 1000 1000",
            "rsg.enter_bastion 123000 123000",
            "rsg.enter_fortress 303000 303000",
        ],
        "gameData": {
            "worldId": "real-id",
            "gameVersion": "1.16.1",
            "modList":[
                "world-preview",
            ],
            "category": "Any%",
        }
    });
    match client
        .post("https://paceman.gg/api/sendevent")
        .json(&body)
        .send()
        .await
    {
        Ok(_) => println!("Request recieved!"),
        Err(err) => panic!("Error processing request: {}", err),
    };
    let response = get_response_from_api().await;
    let expected_response = Response {
        event_list: vec![
            Event {
                event_id: "rsg.enter_nether".to_string(),
                igt: 1000,
                rta: 1000,
            },
            Event {
                event_id: "rsg.enter_bastion".to_string(),
                igt: 123000,
                rta: 123000,
            },
            Event {
                event_id: "rsg.enter_fortress".to_string(),
                igt: 303000,
                rta: 303000,
            },
        ],
        world_id: "real-id".to_string(),
        is_hidden: false,
        is_cheated: false,
        last_updated: 0,
        user: User {
            uuid: "13415023-cc13-4451-8d83-46593be7352d".to_string(),
            live_account: None,
        },
    };
    assert_eq!(expected_response, response[0]);
}

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
