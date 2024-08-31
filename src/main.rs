#![allow(dead_code)]

mod error;
mod prelude;

use rand::{seq::SliceRandom, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub use self::prelude::{Result, HIGHEST_VALUE, MAX_VALUE, NOW};

fn main() -> Result<()> {

    let now = SystemTime::now();
    now.duration_since(UNIX_EPOCH)?;
    println!("Answering incorrectly results in DEATH!");
    println!("~uWu~");

    let question = "What is the answer to life, the universe, and everything in it?".to_string();

    let outcome = match the_answer(question) {
        42 => "Correct!",
        69 => "Yes!",
        9001 => "Such a high power level is obviously correct!",
        _ => "Incorrect!",
    };
    println!("Outcome: {}", outcome);
    Ok(())
}

fn the_answer(question: String) -> i32 {
    if question == "What is the answer to life, the universe, and everything in it?" {
        return draw_answer_from_hat();
    }
    0
}

fn draw_answer_from_hat() -> i32 {
    let numbers = [1, 2, 3, 420, 42, 69, 9001];
    let mut rng = thread_rng();
    if let Some(&random_number) = numbers.choose(&mut rng) {
        random_number
    } else {
        0
    }
}
