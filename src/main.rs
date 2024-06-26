mod game;
use game::GameOfLife;

use clap::{arg, crate_authors, crate_description, crate_version, value_parser, Command};

fn main() {
    let matches = Command::new("The Game of Life")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            arg!(-r --rows <VALUE> "Sets the number of rows")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(-c --columns <VALUE> "Sets the number of columns")
                .required(true)
                .value_parser(value_parser!(usize)),
        )
        .arg(
            arg!(-m --ms <VALUE> "Sets the delay in milliseconds")
                .required(true)
                .value_parser(value_parser!(u64)),
        )
        .get_matches();

    let rows = matches.get_one::<usize>("rows").expect("required argument");
    let columns = matches
        .get_one::<usize>("columns")
        .expect("required argument");
    let ms = matches.get_one::<u64>("ms").expect("required argument");

    let mut game = GameOfLife::new(*rows, *columns);
    loop {
        game.print();
        game.generate_next_states();
        std::thread::sleep(std::time::Duration::from_millis(*ms));
    }
}
