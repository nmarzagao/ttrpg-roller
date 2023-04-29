use rand::Rng;
use std::process::exit;
use colored::Colorize;

pub struct Dice {
    num_dice: u32,
    num_sides: u32,
}

impl Dice {
    pub fn roll(&self) -> u32{
        let mut rng = rand::thread_rng();
        let mut sum = 0;

        let t = format!("\nRolling {}d{}:\n", self.num_dice, self.num_sides);
        println!("{}", t.bold());
        
        for _i in 0..self.num_dice {
            let roll = rng.gen_range(1..=self.num_sides);

            let s = format!("Roll {}: {}", _i + 1, roll);
            if roll <= (self.num_sides / 2) {
                println!("{}", s.red());
            }
            else {
                println!("{}", s.green());
            }
            
            sum += roll;
        }

        let sum_str = format!("\nTotal: {}\n", sum);
        
        println!("{}", sum_str.blue());
        println!("----------------------------");
        return sum;
        
    }


    pub fn parse_dice(dice_string: &str) -> Dice {
        let parts: Vec<&str> = dice_string.split('d').collect();
        if parts.len() != 2 {
            println!("Invalid dice: {}", dice_string);
            exit(1);
        }

        let num_dice = match parts[0].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invaid dice {}", dice_string);
                exit(1);
            }
        };

        let num_sides = match parts[1].parse() {
            Ok(n) => n,
            Err(_) => {
                println!("Invaid dice {}", dice_string);
                exit(1);
            }
        };

        let dice = Dice { num_dice, num_sides };
        return dice
    }    
}