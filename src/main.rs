use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

const HELP_MESSAGE: &str = "
Hello there, Human!
You have summoned me. Let's see about getting you what you need.
❓ Need technical help?
➡️ Post in the <#CHANNEL_ID> channel and other humans will assist you.
❓ Looking for the Code of Conduct?
➡️ Here it is: <https://opensource.facebook.com/code-of-conduct>
❓ Something wrong?
➡️ You can flag an admin with @admin
I hope that resolves your issue!
— HelpBot 🤖
";

const HELP_COMMAND: &str = "!help";
const TEST_COMMAND: &str = "!test";

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match &msg.content[..] {
            HELP_COMMAND => {
                if let Err(why) = msg.channel_id.say(&ctx.http, HELP_MESSAGE).await {
                    println!("Error sending message: {:?}", why);
                }
            }
            TEST_COMMAND => println!("{}", msg.content),
            _ => println!("no command"),
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().expect(".env file not found");
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
