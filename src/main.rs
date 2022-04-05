use chrono::{Datelike, NaiveDate, Weekday};
use rand::Rng;
use std::io;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    let ce = NaiveDate::from_num_days_from_ce(0);

    // TODO: this should be configurable via command line options
    let begin = (NaiveDate::from_ymd(1980, 1, 1) - ce).num_days();
    let end = (NaiveDate::from_ymd(2039, 12, 31) - ce).num_days();

    let d = rng.gen_range(begin..=end);
    let date = NaiveDate::from_num_days_from_ce(d.try_into().unwrap());

    println!("{}", date);
    let ans = date.weekday();
    let now = Instant::now();

    loop {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        match guess.trim().parse::<Weekday>() {
            Ok(wd) => {
                if ans == wd {
                    println!("Correct!");
                    println!("{}ms", now.elapsed().as_millis());
                    break;
                }
                println!("Retry");
            }
            Err(e) => println!("failed to parse as a weekday. Retry, {:?}", e),
        };
    }
}
