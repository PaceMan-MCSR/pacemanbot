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
            command
                .name("setup_default_roles")
                .description("Setup default pace-roles for sub 10.")
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
                    .add_string_choice("First Structure", Split::FirstStructure.to_str())
                    .add_string_choice("Second Structure", Split::SecondStructure.to_str())
                    .add_string_choice("Blind", Split::Blind.to_str())
                    .add_string_choice("Eye Spy", Split::EyeSpy.to_str())
                    .add_string_choice("End Enter", Split::EndEnter.to_str())
            })
            .create_option(|option| {
                option
                    .name("time")
                    .description("The time of the split that you want for the runner.")
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
                    .name("first_structure")
                    .description("The time for first structure that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("second_structure")
                    .description("The time for second structure that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("blind")
                    .description("The time for blind that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("eye_spy")
                    .description("The time for eye spy that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("end_enter")
                    .description("The time for end enter that you want to setup for the runner.")
                    .kind(CommandOptionType::Integer)
            })
            .create_option(|option| {
                option
                    .name("finish")
                    .description("The time for completion that you want to setup for the runner(optional).")
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
                "Setup pace-roles based on split, start time and end time in increments of 30s.",
            )
            .create_option(|option| {
                option
                    .name("split_name")
                    .description("The name of the split.")
                    .kind(CommandOptionType::String)
                    .required(true)
                    .add_string_choice("First Structure", "first_structure")
                    .add_string_choice("Second Structure", "second_structure")
                    .add_string_choice("Blind", "blind")
                    .add_string_choice("Eye Spy", "eye_spy")
                    .add_string_choice("End Enter", "end_enter")
            })
            .create_option(|option| {
                option
                    .name("split_start")
                    .description("The lower bound for the split in minutes.")
                    .kind(CommandOptionType::Integer)
                    .required(true)
            })
            .create_option(|option| {
                option
                    .name("split_end")
                    .description("The upper bound for the split in minutes.")
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
