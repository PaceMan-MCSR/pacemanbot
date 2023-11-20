use crate::components;
use serenity::{
    framework::standard::{
        macros::{command, group},
        Args, CommandResult,
    },
    model::channel::Message,
    prelude::Context,
};

#[group]
#[commands(send_message, setup_default_roles, remove_pmb_roles)]
pub struct General;

#[command]
pub async fn send_message(ctx: &Context, msg: &Message, mut args: Args) -> CommandResult {
    let roles = msg.guild_id.unwrap().roles(&ctx.http).await?;
    let channel_id = args.single::<u64>()?;
    if let Err(why) = components::send_role_selection_message(&ctx, &roles, channel_id).await {
        eprintln!("Error sending role selection message: {:?}", why);
    }
    Ok(())
}

#[command]
pub async fn setup_default_roles(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if !msg.author.id.to_user(&ctx).await?.bot {
        if let Err(why) = components::setup_default_roles(&ctx, msg.guild_id.unwrap()).await {
            eprintln!("Error setting up default roles: {:?}", why);
        } else {
            print!("Default roles set up successfully");
        }
    }
    Ok(())
}
#[command]
pub async fn remove_pmb_roles(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    if !msg.author.id.to_user(&ctx).await?.bot {
        if let Err(why) =
            components::remove_all_pmb_roles_from_guild(&ctx, msg.guild_id.unwrap()).await
        {
            eprintln!("Error setting up default roles: {:?}", why);
        } else {
            print!("Default roles set up successfully");
        }
    }
    Ok(())
}

// Additional command definitions go here
