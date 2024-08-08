use crate::cache::players::Players;

pub fn get_new_config_contents(players: Players) -> String {
    let mut new_config = String::new();
    for (name, splits) in players {
        let finish_config = if splits.finish.is_some() {
            format!("/{}", splits.finish.unwrap())
        } else {
            "".to_string()
        };
        let line = format!(
            "{}:{}/{}/{}/{}/{}{}",
            name,
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
