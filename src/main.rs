mod dice;

use clap::{Arg, App};
use colored::Colorize;
use crate::dice::Dice;

fn main() {
    let matches = App::new("dice-roller")
                        .version("1.0")
                        .author("Nicolas Marzagão")
                        .about("Command line tool for rolling dice")
                        .arg(Arg::with_name("dices")
                            .multiple(true)
                            .help(help_msg())
                            .required(true)
                            .takes_value(true))
                        .get_matches();

    let dices = matches.values_of("dices").unwrap_or_default();

    // TODO: Add a way to include modifires on dice calls (e.g. 1d20+2)
    
    let mut total = 0;
    for i in dices {
        let dice = Dice::parse_dice(i);
        total += dice.roll();
    }

    let total_str = format!("Overall Total {}\n", total);
    println!("{}", total_str.yellow().bold());
}

fn help_msg<'s>() -> &'s str {
    return "Number of dice followed by the number of sides for each (e.g. 2d20)"
}