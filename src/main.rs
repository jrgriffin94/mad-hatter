use std::error::Error;
// use std::fmt;
use std::io;
use std::process;

use serde::Deserialize;

// #[derive(Debug, Deserialize)]
// enum Sex {
//     Male,
//     Female,
// }

#[derive(Debug, Deserialize)]
struct Player {
    name: String,
    sex: String,
}

fn example() -> Result<(), Box<dyn Error>> {
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::Reader::from_reader(io::stdin());
    // for result in rdr.deserialize() {
    //     // The iterator yields Result<StringRecord, Error>, so we check the
    //     // error here.
    //     let record: Player = result?;
    //     println!("{:?}", record);
    // }
    let players: Vec<Player> = rdr.deserialize()
        .filter_map(|s| s.ok())
        .collect();

    println!("{:?}", players);
    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
