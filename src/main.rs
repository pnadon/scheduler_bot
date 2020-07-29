mod day;
mod parse;
mod process;
mod schedules;
mod user;

use serde_json;
use std::fs;

use parse::{filter_query, parse_query};
use schedules::ScheduleCollection;

use std::env;

use serenity::{
    model::{
        channel::Message,
        gateway::{Activity, Ready},
    },
    prelude::*,
};

static DATA_FNAME: &str = "./data.json";

/// Wrapper for persistent data.
struct PersistentData;

impl TypeMapKey for PersistentData {
    type Value = ScheduleCollection;
}

/// Observes and handles events.
struct Handler;

impl EventHandler for Handler {
    /// Reads incoming messages and parses them if they begin with "?".
    /// Messages are parsed into tokens, which are then processed accordingly.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("?") {
            let id = *msg.author.id.as_u64();
            let name = &msg.author.name;

            // Data safely retrieved from persistent context.
            // Take note that Discord bots may be multi-threaded.
            let mut data = ctx.data.write();
            let schedule = data.get_mut::<PersistentData>().unwrap();

            // If the message contains valid tokens, processs them.
            if let (Some(p_type), Some(vals)) = parse_query(filter_query(&msg.content)) {
                // If the user is interacting with the bot for the first time,
                // they must be registered first.
                if schedule.get_id(name).is_none() {
                    if !schedule.id_exists(id) {
                        schedule.insert_user(id, name);
                    }
                    if let Err(why) = schedule.add_name_id(name, id) {
                        println!("Error adding user: {:?}", why);
                    }
                }

                match process::process(schedule, name, p_type, vals) {
                    Ok(res) => {
                        if let Some(res_msg) = res {
                            if let Err(why) = msg.channel_id.say(&ctx.http, res_msg) {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    }
                    Err(why) => {
                        println!("Error parsing message: {:?}", why);
                    }
                }
            } else if let Err(why) = msg.channel_id.say(&ctx.http, "Failed to parse message") {
                println!("Error sending message: {:?}", why);
            }
            fs::write(DATA_FNAME, serde_json::to_string(&schedule).unwrap()).expect("failed to write file");
        }
    }

    /// Executes when the bot first starts.
    fn ready(&self, ctx: Context, ready: Ready) {
        ctx.set_activity(Activity::playing("Type \"?help\" to get started!"));
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    run_bot();
}

/// Retrieve's the token as well as set the persistent data,
/// before starting the bot.
fn run_bot() {
    let mut client = Client::new(
        &env::var("DISCORD_TOKEN").expect("Could not find token."),
        Handler,
    )
    .expect("Could not create client.");

    {
        let mut data = client.data.write();
        if let Ok(serialized_schedule) = fs::read_to_string(DATA_FNAME) {
            if let Ok(schedule) = serde_json::from_str(&serialized_schedule) {
                data.insert::<PersistentData>(schedule);
            } else {
                println!("Error parsing data file. Creating new...");
                data.insert::<PersistentData>(ScheduleCollection::new());
            }
        } else {
            println!("Could not find data file. Creating new...");
            data.insert::<PersistentData>(ScheduleCollection::new());
        }
    }

    client.start().expect("Could not start client.");
}
