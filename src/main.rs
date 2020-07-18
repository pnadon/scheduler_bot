
//Sure, you'd just also need to add the ScheduleCollection into ctx.data to access it.

mod day;
mod parse;
mod schedules;

use schedules::{ScheduleCollection, User};
use day::Day;

fn main() {
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
