use serenity::client::{Client, Context};
use serenity::model::channel::Reaction;
use serenity::prelude::EventHandler;
use std::env;

const VOTING_CHANNEL: u64 = 588447667038978068;
const RUST_SERVER: u64 = 273534239310479360;
const COUNCIL_ROLE: u64 = 585200700024422431;

struct Handler;

impl EventHandler for Handler {
    fn reaction_add(&self, context: Context, reaction: Reaction) {
        if reaction.channel_id != VOTING_CHANNEL {
            return;
        }

        match reaction.user_id.to_user(&context) {
            Ok(user) => {
                let is_council = user
                    .has_role(&context, RUST_SERVER, COUNCIL_ROLE)
                    .unwrap_or_else(|err| {
                        println!(
                            "Couldn't check whether user {} has Council role because {}",
                            user.id, err
                        );
                        true
                    });
                if !is_council {
                    reaction.delete(&context).unwrap_or_else(|err| {
                        println!("Couldn't delete user {} reaction because {}", user.id, err);
                    });;
                }
            }
            Err(error) => println!(
                "An error occurred while querying user {}: {:?}",
                reaction.user_id, error
            ),
        }
    }
}

fn main() {
    // Login with a bot token from the environment
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("Couldn't find token, whoops!"),
        Handler,
    )
    .expect("Couldn't create a client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start() {
        println!("An error occurred while running the client: {:?}", why);
    }
}
