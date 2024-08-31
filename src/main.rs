#![allow(dead_code)]

mod error;
mod prelude;

use rand::{seq::SliceRandom, Rng};
use std::time::{SystemTime, UNIX_EPOCH};

pub use self::prelude::{Result, HIGHEST_VALUE, MAX_VALUE, NOW};

fn main() -> Result<()> {
    let since = NOW.get_or_init(|| SystemTime::now().duration_since(UNIX_EPOCH).unwrap());
    println!(
        "It has been {} seconds since the Unix epoch.",
        since.as_secs()
    );

    println!("Answering incorrectly results in DEATH!");
    println!("~uWu~");

    let question = "What is the answer to life, the universe, and everything in it?".to_string();
    if let Some(answer) = the_answer(question) {
        println!("Outcome: {}", answer);
        answer
    } else {
        println!("You have failed the test. You have been sentenced to death.");
        0
    };

    fill_and_print();

    Ok(())
}

fn fill_and_print() {
    let arr = fill_arr();
    for i in arr.iter() {
        println!("{}", i);
    }
}

fn fill_arr() -> Vec<i32> {
    let generator = || -> i32 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..MAX_VALUE as i32)
    };

    let mut arr = Vec::new();
    for _ in 0..=HIGHEST_VALUE {
        arr.push(generator());
    }
    arr
}

fn the_answer(question: String) -> Option<i32> {
    if question == "What is the answer to life, the universe, and everything in it?" {
        let n = draw_answer_from_hat();
        match n {
            42 => return Some(42),
            69 => return Some(69),
            9001 => return Some(9001),
            _ => return None,
        }
    }
    None
}

fn draw_answer_from_hat() -> i32 {
    let numbers = [1, 2, 3, 420, 42, 69, 99];
    let mut rng = rand::thread_rng();
    if let Some(&random_number) = numbers.choose(&mut rng) {
        random_number
    } else {
        0
    }
}
