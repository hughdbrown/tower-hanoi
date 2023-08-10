use std::error::Error;

use clap::{
    Command,
    Arg,
};

mod tower;

use tower::{
    Tower,
};

pub type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug, Clone)]
pub struct AppArgs {
    /// Count of disks to simulate
    pub disks: usize,
}

fn get_args() -> MyResult<AppArgs> {
    let filetype_arg = Arg::new("disks")
        .short('d')
        .long("disks")
        .default_value("3")
        .value_parser(clap::value_parser!(usize))
        .required(false);

    let command = Command::new("Tower")
        .version("0.1.0")
        .author("Hugh Brown <hughdbrown@gmail.com>")
        .about("Tower of Hanoi")
        .arg(filetype_arg);
    let matches = command.get_matches();

    let disks = *matches.get_one::<usize>("disks").unwrap();

    Ok(
        AppArgs { disks, }
    )
}

fn main() {
    let args = get_args();
    match args {
        Ok(app) => {
            let disks = app.disks;
            let mut tower: Tower = Tower::new(disks as i8);
            tower.run();
            tower.visualize();
        },
        Err(err) => eprintln!("{}", err),
    }
}
