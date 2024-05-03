use clap::{arg, crate_authors, crate_description, crate_version, value_parser, Command};

fn main() {
    let matches = Command::new("The Game of Life")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(
            arg!(-r --rows <VALUE> "Sets the number of rows")
                .required(true)
                .value_parser(value_parser!(u16)),
        )
        .arg(
            arg!(-c --columns <VALUE> "Sets the number of columns")
                .required(true)
                .value_parser(value_parser!(u16)),
        )
        .get_matches();

    let rows = matches.get_one::<u16>("rows").expect("required argument");
    let columns = matches
        .get_one::<u16>("columns")
        .expect("required argument");

    println!("Number of rows: {}", rows);
    println!("Number of columns: {}", columns);
}
