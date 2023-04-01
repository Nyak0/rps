use rand::Rng;
use std::collections::HashMap;
use std::io::{stdin, stdout, Write};

fn main() {
    let opt: HashMap<i8, String> = HashMap::from([
        (1, "rock"),
        (2, "paper"),
        (3, "scissors"),
    ]);

    loop {
        print!("(1 = rock, 2 = paper, 3 = scissors): ");
        stdout().flush().unwrap();

        // User
        let mut user_option = String::new();
        stdin().read_line(&mut user_option).unwrap();
        let user_option: i8 = match user_option.trim().parse() {
            Ok(num) if (1..=3).contains(&num) => num,
            _ => {
                println!("Ivalid option. Try again.");
                continue;
            }
        };

        // PC
        let pc: i8 = rand::thread_rng().gen_range(1..4);

        // Game
        let mts: i8 = user_option - pc;
        let result: &str = if mts < 0 {
            "You lose!"
        } else if mts > 0 {
            "You win!"
        } else {
            "Tie..."
        };

        println!(
            "You chose {}, pc chose {}, {}",
            opt.get(&user_option).unwrap(),
            opt.get(&pc).unwrap(),
            result
        );
    }
}
