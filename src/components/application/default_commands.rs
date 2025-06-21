use serenity::{
    client::Context,
    model::{id::GuildId, prelude::command::CommandOptionType},
};

use crate::{
    cache::{consts::PACEMANBOT_RUNNER_NAMES_CHANNEL, split::Split},
    eprintln,
};

pub async fn setup_default_commands(ctx: &Context, guild_id: GuildId) {
    match GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        commands.create_application_command(|command| {
            command
                .name("send_message")
                .description("Send role message to the current channel.")
        });
        commands.create_application_command(|command| {
            command.name("setup_pb_roles").description(
                format!("Setup split PB pace-roles(as specified per runner in #{}).", PACEMANBOT_RUNNER_NAMES_CHANNEL),
            )
        });
        commands.create_application_command(|command| {
            command.name("validate_config").description(
                "Check if the current server configuration is valid and if the bot will work properly or not.",
            )
        });
        commands.create_application_command(|command| {
            command
            .name("setup_pings")
            .description(
                "Setup pings for specific runners.",
            )
            .create_option(|option| {
                option
                    .name("action")
                    .description("Action to perform out of 'add_or_update' or 'remove'.")
                    .required(true)
                    .kind(CommandOptionType::String)
                    .add_string_choice("Add or Update", "add_or_update")
                    .add_string_choice("Remove", "remove")
            })
            .create_option(|option| {
                option
                    .name("ign")
                    .description("In-game name of the runner you want to setup pings for.")
                    .required(true)
                    .kind(CommandOptionType::String)
            })
            .create_option(|option| {
                option
                    .name("split")
                    .description("Split name for the runner that you want to change.")
                    .required(true)
                    .kind(CommandOptionType::String)
                    .add_string_choice("Adventuring Time", Split::AdventuringTime.to_str())
                    .add_string_choice("Beaconator", Split::Beaconator.to_str())
                    .add_string_choice("HDWGH", Split::HDWGH.to_str())
            })
            .create_option(|option| {
                option
                    .name("time_hours")
                    .description("The time in hours of the split that you want for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("time_minutes")
                    .description("The time in minutes of the split that you want for the runner.")
                    .kind(CommandOptionType::Integer)
            })
        });
        commands.create_application_command(|command| {
            command
            .name("whitelist")
            .description(
                "Whitelist new players or edit old players' configurations in the server based on ign.",
            )
            .create_option(|option| {
                option
                    .name("action")
                    .description("Action to perform out of 'add_or_update' or 'remove'.")
                    .required(true)
                    .kind(CommandOptionType::String)
                    .add_string_choice("Add or Update", "add_or_update")
                    .add_string_choice("Remove", "remove")
            })
            .create_option(|option| {
                option
                    .name("ign")
                    .description("In-game name of the runner that you want to add.")
                    .required(true)
                    .kind(CommandOptionType::String)
            })
            .create_option(|option| {
                option
                    .name("adventuring_time_hours")
                    .description("The time for adventuring time in hours that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("adventuring_time_minutes")
                    .description("The time for adventuring time in minutes that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("beaconator_hours")
                    .description("The time for beaconator in hours that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("beaconator_minutes")
                    .description("The time for beaconator in minutes that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("hdwgh_hours")
                    .description("The time for hdwgh in hours that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("hdwgh_minutes")
                    .description("The time for hdwgh in minutes that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("finish_hours")
                    .description("The time for completion in hours that you want to setup for the runner(optional).")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("finish_minutes")
                    .description("The time for completion in minutes that you want to setup for the runner(optional).")
                    .kind(CommandOptionType::Integer)
            })
        });
        commands.create_application_command(|command| {
            command
            .name("migrate")
            .description(
                format!("Migrate the old configuration from first message in #{}.", PACEMANBOT_RUNNER_NAMES_CHANNEL)
            )
        });
        commands.create_application_command(|command| {
            command
            .name("setup_roles")
            .description(
                "Setup pace-roles based on split, start time and end time in increments of 30m.",
            )
            .create_option(|option| {
                option
                    .name("split_name")
                    .description("The name of the split.")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .add_string_choice("Adventuring Time", "adventuring_time")
                    .add_string_choice("Beaconator", "beaconator")
                    .add_string_choice("HDWGH", "hdwgh")
            })
            .create_option(|option| {
                option
                    .name("split_start")
                    .description("The lower bound for the split in hours.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("split_end")
                    .description("The upper bound for the split in hours.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
        })
    })
    .await
    {
        Ok(_) => (),
        Err(err) => eprintln!("Error creating command: {}", err),
    };
}
