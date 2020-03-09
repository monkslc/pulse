use clap::{App, Arg, SubCommand};

mod db;
mod language_breakdown;
mod pulse;
mod watch;
use language_breakdown::{create_language_breakdown_chart, get_language_breakdown};

fn main() {
    let db = db::get_db().expect("can't create connection to the database");

    let matches = App::new("Pulse")
        .version("0.1.0")
        .about("Track your activity while coding!")
        .subcommand(
            SubCommand::with_name("watch").about("track your activity in the current directory"),
        )
        .subcommand(
            SubCommand::with_name("breakdown")
                .about("view which languages you have been working with")
                .arg(
                    Arg::with_name("output")
                        .help("Sets the output file for the language breakdown histogram")
                        .index(1),
                ),
        )
        .get_matches();

    match matches.subcommand() {
        ("watch", Some(_)) => watch::watch_activity(&db),
        ("breakdown", Some(matches)) => {
            let start_of_day: u32 = 1583625600;
            let end_of_day: u32 = 1583712000;

            let output_file = matches
                .value_of("output")
                .unwrap_or("language-breakdown.png");
            let language_breakdown = get_language_breakdown(&db, start_of_day, end_of_day)
                .expect("Error fetching the language breakdown from the database");

            create_language_breakdown_chart(language_breakdown, output_file.to_string())
                .expect("Couldn't create the language breakdown");

            println!(
            "Successfully crated the language breakdown. You can find your language breakdown at {} {}",
            output_file,
            '\u{1F642}'
        );
            db.close().expect("Error closing the database");
        }
        _ => println!(
            "I'm not quite sure what you're looking for. {}",
            '\u{1f643}'
        ),
    }
}
