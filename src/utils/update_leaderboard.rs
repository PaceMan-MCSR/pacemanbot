use std::collections::HashMap;

use serenity::{client::Context, model::id::ChannelId};

use crate::Result;

use super::{format_time::format_time, mins_secs_to_millis::mins_secs_to_millis};

pub async fn update_leaderboard(
    ctx: &Context,
    leaderboard_channel: ChannelId,
    nickname: String,
    time: (u8, u8),
) -> Result<()> {
    let messages = leaderboard_channel.messages(&ctx, |m| m.limit(100)).await?;
    if messages.is_empty() {
        let leaderboard_content = format!(
            "## Runner Leaderboard\n\n`{}`\t\t{}",
            format_time(mins_secs_to_millis(time)),
            nickname
        );
        leaderboard_channel
            .send_message(&ctx.http, |m| m.content(leaderboard_content))
            .await?;
    } else {
        let first_message_id = messages.last().unwrap().id;
        let first_message = messages.last().unwrap().content.to_owned();
        let leaderboard_lines = first_message
            .split("\n")
            .filter(|l| l != &"## Runner Leaderboard" && l != &"")
            .collect::<Vec<&str>>();
        let mut player_names_with_time: HashMap<String, u64> = HashMap::new();
        for l in leaderboard_lines {
            let splits = l.split("\t\t").collect::<Vec<&str>>();
            if splits.len() != 2 {
                return Err("LeaderboardError: parse leaderboard message.".into());
            }
            let player_name = splits[1];
            let time = splits[0].replace("`", "");
            let time_splits = time
                .split(':')
                .map(|sp| sp.parse::<u8>().unwrap())
                .collect::<Vec<u8>>();
            let (minutes, seconds) = (time_splits[0], time_splits[1]);
            let time_millis: u64 = mins_secs_to_millis((minutes, seconds));
            player_names_with_time.insert(player_name.to_owned(), time_millis);
        }
        let current_finish_time = mins_secs_to_millis(time);
        if player_names_with_time.get(&nickname).is_some() {
            let time = player_names_with_time.get(&nickname).unwrap();
            if time > &current_finish_time {
                player_names_with_time.insert(nickname.to_owned(), current_finish_time);
            }
        } else {
            player_names_with_time.insert(nickname, mins_secs_to_millis(time));
        }
        let mut entry_vector: Vec<(&String, &u64)> = player_names_with_time
            .iter()
            .collect::<Vec<(&String, &u64)>>();
        entry_vector.sort_by(|a, b| a.1.cmp(b.1));
        let mut updated_contents: Vec<String> = vec![];
        for entry in entry_vector {
            let name = entry.0;
            let time = format_time(entry.1.to_owned());
            updated_contents.push(format!("`{}`\t\t{}", time, name));
        }
        let leaderboard_content =
            format!("## Runner Leaderboard\n\n{}", updated_contents.join("\n"));
        leaderboard_channel
            .edit_message(&ctx, first_message_id, |m| m.content(leaderboard_content))
            .await?;
    }
    Ok(())
}
