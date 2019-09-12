use backend::db::{create_mood, establish_connection, query_mood};
use std::env;

fn help() {
    println!("subcommands:");
    println!("    new<title>: create a new mood");
    println!("    show      : show all moods");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        help();
        return;
    }

    let subcommand = &args[1];
    match subcommand.as_ref() {
        "new" => new_mood(&args[2..]),
        "show" => show_moods(&args[2..]),
        _ => help(),
    }
}

fn new_mood(args: &[String]) {
    if args.len() < 1 {
        println!("new: missing <title>");
        help();
        return;
    }

    let conn = establish_connection();
    create_mood(&conn, &args[0]);
}

fn show_moods(args: &[String]) {
    if args.len() > 0 {
        println!("show: unexpected argument");
        help();
        return;
    }

    let conn = establish_connection();
    println!("MOODS\n-----");
    for mood in query_mood(&conn) {
        println!("{}", mood.title);
    }
}
