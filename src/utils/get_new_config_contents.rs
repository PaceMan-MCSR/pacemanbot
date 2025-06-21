use crate::cache::players::Players;

pub fn get_new_config_contents(players: Players) -> String {
    let mut new_config = String::new();
    for (name, splits) in players {
        let finish_config = if splits.finish.is_some() {
            format!(
                "/{};{}",
                splits.finish.unwrap() / 60,
                splits.finish.unwrap() % 60
            )
        } else {
            "".to_string()
        };
        let line = format!(
            "{}:{};{}/{};{}/{};{}{}",
            name,
            splits.adventuring_time / 60,
            splits.adventuring_time % 60,
            splits.beaconator / 60,
            splits.beaconator % 60,
            splits.hdwgh / 60,
            splits.hdwgh % 60,
            finish_config
        );
        new_config = format!("{}\n{}", new_config, line);
    }
    new_config
}
