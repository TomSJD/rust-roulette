use std::io::stdin;
use rand::{Rng, thread_rng};

const MENU: [&str; 3] = ["1. Play", "2. View Stats", "3. Exit"];
const BETS: [&str; 3] = ["1. Colour", "2. Even/Odd", "3. High/Low"];

const REDS: [u32; 18] = [1, 3, 5, 7, 9, 12, 14, 16, 18, 19, 21, 23, 25, 27, 30, 32, 34, 36];
const BLACKS: [u32; 18] = [2, 4, 6, 8, 10, 11, 13, 15, 17, 20, 22, 24, 26, 28, 29, 31, 33, 35];

struct Player {
    name: String,
    money: f32,
}

fn main() {
    let mut running: bool = true;
    let mut player: Player = create_player();

    while running {
        println!();
        println!("What would you like to do?");
        output_array(&MENU);
        match read_user_input().as_str() {
            "1" => play_roulette(&mut player),
            "2" => display_stats(&player),
            "3" => running = false,
            _ => println!("Not an option!"),
        };
    }
}

fn play_roulette(player: &mut Player) {
    let bet_type: u8 = loop {
        println!();
        println!("Choose a bet type.");
        output_array(&BETS);
        let input: u8 = match read_user_input().parse() {
            Ok(value) => value,
            Err(_) => continue,
        };
        if input > BETS.len() as u8 {
            println!("Please enter a valid option!");
            continue;
        }
        break input;
    };

    let rolled_number: u32 = thread_rng().gen_range(0..37);
    let bet_amount: f32 = choose_bet_amount(&player);
    player.money -= bet_amount;

    let win: bool = match bet_type {
        1 => play_black_red(&rolled_number),
        2 => play_odd_even(&rolled_number),
        3 => play_low_high(&rolled_number),
        _ => false,
    };

    println!("The ball landed on {rolled_number}");
    if win {
        println!("You won and doubled your money!");
        player.money += bet_amount * 2.0;
        println!("You now have £{}", player.money);
    } else {
        println!("You lost! (cringe bro. sucks for you I guess:/)");
    }
}

fn choose_bet_amount(player: &Player) -> f32 {
    let amount: f32 = loop {
        println!("Enter an amount to bet.");
        match read_user_input().parse() {
            Ok(value) => {
                if value < 1.0 || value > player.money {
                    println!("Cannot bet more than you have or less than 1!");
                    continue;
                }
                break value;
            },
            Err(_) => {
                println!("Not a valid number!");
                continue;
            },
        };
    };
    amount
}

fn play_black_red(rolled_number: &u32) -> bool {
    let black_or_red: String = loop {
        println!("Would you like to play black or red?");
        let input: String = read_user_input();
        if input == "black" || input == "red" {
            break input;
        }
    };
    match black_or_red.as_str() {
        "red" => REDS.contains(rolled_number),
        "black" => BLACKS.contains(rolled_number),
        _ => false,
    }
}

fn play_odd_even(rolled_number: &u32) -> bool {
    let odd_or_even: String = loop {
        println!("Would you like to play odd or even?");
        let input: String = read_user_input();
        if input == "odd" || input == "even" {
            break input;
        }
    };
    match odd_or_even.as_str() {
        "odd" => is_odd(&rolled_number),
        "even" => !is_odd(&rolled_number),
        _ => false,
    }
}

fn play_low_high(rolled_number: &u32) -> bool {
    let high_or_low: String = loop {
        println!("Would you like to play high or low?");
        let input: String = read_user_input();
        if input == "low" || input == "high" {
            break input;
        }
    };
    match high_or_low.as_str() {
        "low" => rolled_number <= &18 && rolled_number > &0,
        "high" => rolled_number > &18,
        _ => false,
    }
}

fn display_stats(player: &Player) {
    println!();
    println!("Player stats.");
    println!("Name: {}", player.name);
    println!("Money: £{}", player.money);
    println!();
}

fn create_player() -> Player {
    println!("Enter your name.");
    let name: String = read_user_input();
    Player {
        name,
        money: 1000.0,
    }
}

fn read_user_input() -> String {
    let mut input: String = String::new();
    stdin().read_line(&mut input).expect("Failed to read input");
    String::from(input.trim())
}

fn output_array(array: &[&str]) {
    for item in array {
        println!("{item}");
    }
}

fn is_odd(number: &u32) -> bool {
    0==number%2
}
