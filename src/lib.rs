mod commands;

use anyhow::anyhow;
use serenity::async_trait;
use serenity::framework::standard::macros::group;
use serenity::framework::StandardFramework;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use shuttle_secrets::SecretStore;
use std::collections::HashMap;
use tracing::{error, info};

use crate::commands::map::*;

pub struct EnvData;
struct Handler;

impl serenity::prelude::TypeMapKey for EnvData {
    type Value = HashMap<String, String>;
}

#[group]
#[commands(map_command)]
struct General;

pub async fn get_client(discord_token: &str, discord_guild_id: u64) -> Client {
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("/"))
        .group(&GENERAL_GROUP);

    Client::builder(discord_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Err creating client")
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        const CHANNEL: u64 = 1042184998121377932;
        if msg.content == "!fight" && msg.channel_id == CHANNEL {
            if let Err(e) = msg.channel_id.say(&ctx.http, "world!").await {
                error!("Error sending message: {:?}", e);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        info!("{} is connected!", ready.user.name);
    }
}

#[shuttle_service::main]
async fn serenity(
    #[shuttle_secrets::Secrets] secret_store: SecretStore,
) -> shuttle_service::ShuttleSerenity {
    let token = if let Some(token) = secret_store.get("DISCORD_TOKEN") {
        token
    } else {
        return Err(anyhow!("'DISCORD_TOKEN' was not found").into());
    };

    let dnd_guild_id = if let Some(dnd_guild_id) = secret_store.get("DND_DISCORD_ID") {
        dnd_guild_id
            .parse::<u64>()
            .expect("Could not parse DND_DISCORD_ID to integer.")
    } else {
        return Err(anyhow!("'DND_DISCORD_ID' was not found").into());
    };

    let allowed_channel_id = if cfg!(debug_assertions) {
        let allowed_channel_id = if let Some(allowed_channel_id) = secret_store.get("DEV_CHANNEL_ID") {
            allowed_channel_id
        } else {
            return Err(anyhow!("'DEV_CHANNEL_ID' was not found").into());
        };
        allowed_channel_id
    } else {
        let allowed_channel_id = if let Some(allowed_channel_id) = secret_store.get("DND_GENERAL_CHANNEL_ID") {
            allowed_channel_id
        } else {
            return Err(anyhow!("'DND_GENERAL_CHANNEL_ID' was not found").into());
        };
        allowed_channel_id
    };

    let client = get_client(&token, dnd_guild_id).await;
    {
        let mut env_data = HashMap::new();
        env_data.insert(
            "dnd_general_channel".to_string(),
            allowed_channel_id,
        );

        let mut data = client.data.write().await;
        data.insert::<EnvData>(env_data);
    }

    Ok(client)
}
