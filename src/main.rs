
//Sure, you'd just also need to add the ScheduleCollection into ctx.data to access it.

mod day;
mod parse;
mod schedules;

use schedules::{ScheduleCollection, User};

use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct MessageEventParser;

impl TypeMapKey for MessageEventParser {
    type Value = ScheduleCollection;
}

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("?") {
            let id = *msg.author.id.as_u64();
            let name = &msg.author.name;
            let mut data = ctx.data.write();
            let schedule = data.get_mut::<MessageEventParser>().unwrap();
            if let (Some(p_type), Some(vals)) = parse::to_params(&msg.content) {
                if schedule.get_id(name).is_none() {
                    if !schedule.id_exists(id) {
                        schedule.insert_user(id, User::new(name.to_string()))
                    }
                    if let Err(why) = schedule.add_name_id(name, id) {
                        println!("Error adding user: {:?}", why);
                    }
                }

                match parse::process(schedule, name, p_type, vals) {
                    Ok(res) => {
                        if let Some(res_msg) = res {
                            if let Err(why) = msg.channel_id.say(&ctx.http, res_msg) {
                                println!("Error sending message: {:?}", why);
                            }
                        }
                    },
                    Err(why) => {
                        println!("Error parsing message: {:?}", why);
                    }
                }
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            } else if let Err(why) = msg.channel_id.say(&ctx.http, "Failed to parse message") {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

fn main() {
    run_bot();
}

fn run_bot() {
    let mut client = Client::new(&env::var("DISCORD_TOKEN").expect("Could not find token."), Handler)
        .expect("Could not create client.");

    {
        let mut data = client.data.write();
        data.insert::<MessageEventParser>(ScheduleCollection::new());
    }

    client.start().expect("Could not start client.");
}

fn test() {
    let mut usr = User::new("bob".to_string());
    usr.set_day_range(day::Day::Monday, day::Day::Friday, 2, true);
    usr.set_day_range(day::Day::Saturday, day::Day::Sunday, 2, true);
}