use std::io;
use std::io::Write;

fn prompt_user(msg: &str) -> String {
    let mut res = String::new();

    print!("{}", msg);

    io::stdout().flush().expect("FLUSH_ERR");
    io::stdin().read_line(&mut res).expect("READ_ERR");
    res.trim().to_string()
}

fn main() {
    loop {
        let year = prompt_user("Year: ");

        let year: u32 = match year.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Exiting program, because the year could not be parsed.");
                break;
            }
        };

        if is_leap_year(&year) {
            println!("{} is a leap year.", year);
        } else {
            println!("{} is not a leap year.", year);
        }

        // let choice = prompt_user("Check another year? ");

        // if choice == "y" {
        //     continue;
        // } else if choice == "n" {
        //     break;
        // }
    }
}

fn is_leap_year(year: &u32) -> bool {
    if year % 4 == 0 {
        // println!("4 divides {}.", year);
        if year % 100 == 0 {
            // println!("100 divides {}.", year);
            if year % 400 == 0 {
                // println!("400 divides {}.", year);
                return true;
            } else {
                // println!("400 does not divide {}.", year);
            }
        } else {
            // println!("100 does not divide {}.", year);
            return true;
        }
    } else {
        // println!("4 does not divide {}.", year);
    }
    false
}
