use crate::cache::players::Players;

pub fn get_new_config_contents(players: Players) -> String {
    let mut new_config = String::new();
    let mut keys: Vec<&String> = players.keys().collect();
    keys.sort_by_key(|name| name.to_lowercase());
    for key in keys {
        let players_unchecked = players.get(key);
        if players_unchecked.is_none() {
            continue;
        }

        let splits = players_unchecked.unwrap();
        let finish_config = if splits.finish.is_some() {
            format!("/{}", splits.finish.unwrap())
        } else {
            "".to_string()
        };
        let line = format!(
            "{}:{}/{}/{}/{}/{}{}",
            key,
            splits.first_structure,
            splits.second_structure,
            splits.blind,
            splits.eye_spy,
            splits.end_enter,
            finish_config
        );
        new_config = format!("{}\n{}", new_config, line);
    }
    new_config
}
