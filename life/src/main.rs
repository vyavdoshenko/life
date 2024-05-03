use clap::{Command, arg, crate_version, crate_authors, crate_description};

fn main() {
    let matches = Command::new("The Game of Life")
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(arg!(-r --rows "Sets the number of rows")
             .required(true))
        .arg(arg!(-c --columns "Sets the number of columns")
             .required(true))
        .get_matches();

    let rows = matches.get_one::<String>("rows").expect("required argument");
    let columns = matches.get_one::<String>("columns").expect("required argument");

    println!("Number of rows: {}", rows);
    println!("Number of columns: {}", columns);
}