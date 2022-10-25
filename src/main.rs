use std::env;
use serenity::{prelude::GatewayIntents, model::{gateway::Ready, prelude::Member}, prelude::*, framework::StandardFramework};
mod mongo;
mod events;

struct Handler {
    pub db: mongo::Database,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn guild_member_update(&self, _: Context, old: Option<Member>, new: Member) {
        self.guild_member_update(old, new).await;
    }

    async fn guild_member_addition(&self, ctx: Context, member_join: Member) {
        self.guild_member_join(ctx, member_join).await;
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::non_privileged() | GatewayIntents::GUILD_MEMBERS;
    let db = mongo::connect().await;

    let mut client = Client::builder(&token, intents).event_handler(Handler { db }).framework(StandardFramework::new().configure(|c| c.with_whitespace(true).prefix('~'))).await.expect("Error creating client");
    client.start().await.expect("Error starting client");
}