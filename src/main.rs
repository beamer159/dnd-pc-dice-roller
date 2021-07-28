use crate::character::Character;
use std::io::Write;
use std::{env, io};

mod character;

enum Run {
    Continue,
    Quit,
}

fn main() {
    let mut args = env::args().skip(1);
    let dice = args.next();
    let soh = args.next();
    match dice {
        Some(d) => run_once(&d, soh),
        None => run_ongoing(),
    }
}

fn run_once(dice: &str, soh: Option<String>) {
    let dice = dice.parse().unwrap();
    let soh: i8 = soh.map_or(0, |x| x.parse().unwrap());
    let character = Character::new(dice, soh);
    println!("{}", character.roll());
}

fn run_ongoing() {
    let mut character = Character::new(6, 0);
    let mut previous_input: Option<String> = None;
    println!();
    loop {
        println!("{}", character);
        println!(
            "\n\
        R) Roll\n\
        M) View Map\n\
        D) Change Dice\n\
        S) Change Sleight of Hand modifier\n\
        Q) Quit\n"
        );
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        if let Run::Quit = process_input(&mut character, &input, &mut previous_input) {
            break;
        }
    }
}

fn process_input(
    character: &mut Character,
    input: &str,
    previous_input: &mut Option<String>,
) -> Run {
    match input.trim().to_uppercase().as_str() {
        "R" => {
            println!("\n{}\n", character.roll());
            *previous_input = Some("R".to_string());
            Run::Continue
        }
        "M" => {
            println!("\n{}", character.d20_map_string());
            *previous_input = Some("M".to_string());
            Run::Continue
        }
        "D" => {
            update_dice(character);
            *previous_input = Some("D".to_string());
            Run::Continue
        }
        "S" => {
            update_soh(character);
            *previous_input = Some("S".to_string());
            Run::Continue
        }
        "Q" => Run::Quit,
        "" => match previous_input {
            Some(i) => process_input(character, i, &mut None),
            None => {
                println!("Invalid option");
                Run::Continue
            }
        },
        _ => {
            println!("Invalid option\n");
            Run::Continue
        }
    }
}

fn update_dice(character: &mut Character) {
    print!("\nEnter dice number: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse() {
        Ok(dice) => {
            character.set_dice(dice);
            println!();
        }
        Err(_) => println!("Invalid dice number\n"),
    }
}

fn update_soh(character: &mut Character) {
    print!("\nEnter Sleight of Hand modifier: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    match input.trim().parse() {
        Ok(soh) => {
            character.set_soh(soh);
            println!("\n");
        }
        Err(_) => println!("Invalid Sleight of Hand modifier\n"),
    }
}
