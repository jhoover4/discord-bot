use serenity::framework::standard::macros::{check, command};
use serenity::framework::standard::{Args, CommandOptions, CommandResult, Reason};
use serenity::model::prelude::*;
use serenity::prelude::*;

use crate::EnvData;

#[command("map")]
#[description(
    "Provide the name of a person, place, or thing and receive a google map to the address."
)]
#[checks(Channel)]
#[sub_commands(add_command, list_command)]
async fn map_command(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "This is the main function!").await?;

    Ok(())
}

#[command]
#[aliases("add")]
#[checks(Channel)]
#[description("Use to add a new entry to use.")]
async fn add_command(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "Nice! Your entry was added.").await?;

    Ok(())
}

#[command]
#[aliases("list")]
#[checks(Channel)]
#[description("Show all currently available addresses.")]
async fn list_command(ctx: &Context, msg: &Message, _args: Args) -> CommandResult {
    msg.reply(&ctx.http, "Will show addresses in the future.")
        .await?;

    Ok(())
}

#[check]
#[name = "Channel"]
async fn channel_check(
    ctx: &Context,
    msg: &Message,
    _: &mut Args,
    _: &CommandOptions,
) -> Result<(), Reason> {
    let data = ctx.data.read().await;
    let env_data = data.get::<EnvData>().unwrap();

    if msg.channel_id.to_string() != env_data["dnd_general_channel"] {
        return Err(Reason::User(
            "This command can only be used in the #general channel".to_string(),
        ));
    }

    Ok(())
}
