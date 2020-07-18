
//Sure, you'd just also need to add the ScheduleCollection into ctx.data to access it.

mod day;
mod parse;
mod schedules;

use schedules::{ScheduleCollection, User};
use day::Day;

use std::env;

use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!") {
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

    // {
    //     let mut data = client.data.write();
    //     data.insert::<ScheduleCollection>(HashMap::default());
    // }

    client.start().expect("Could not start client.");
}


fn demo() {
    let mut schedule = ScheduleCollection::new();
    schedule.insert_user("uid345".to_string(), User::new("bob".to_string()));
    if let Some(usr) = schedule.get_mut_user("uid345".to_string()) {
        usr.set_time(Day::Monday, 4, true);
    }

    println!("{:?}", schedule.available_at(Day::Monday, 4, 0));
    
    if let Some(usr) = schedule.get_user("uid345".to_string()) {
        println!("{}", usr.disp_schedule(true, 0));
        println!("{}", usr.disp_schedule(true, 4));
    }

    println!("{:?}", parse::to_params("add, from 1 to 2"));
    println!("{:?}", parse::to_params("add mon tue wed from 11 to 2"));
    println!("{:?}", parse::to_params("add from sun to wed 11 12 15"));
    if let (Some(p_type), Some(vals)) = parse::to_params("add 11, 14, 15, 16") {
        println!("{:?} {:?}", p_type, vals);
        if parse::process(&mut schedule, "uid345".to_string(), p_type, vals).is_ok() {
            if let Some(usr) = schedule.get_user("uid345".to_string()) {
                println!("{}", usr.disp_schedule(true, 0));
            }
        }
    }
    if let (Some(p_type), Some(vals)) = parse::to_params("remove from mondi to wed from 13 to 14") {
        println!("{:?} {:?}", p_type, vals);
        if parse::process(&mut schedule, "uid345".to_string(), p_type, vals).is_ok() {
            if let Some(usr) = schedule.get_user("uid345".to_string()) {
                println!("{}", usr.disp_schedule(true, 0));
                println!("{}", usr.disp_schedule(true, 10));
            }
        }
    }
}