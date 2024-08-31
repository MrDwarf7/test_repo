use rand::seq::SliceRandom;
use rand::thread_rng;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    let now = SystemTime::now();
    match now.duration_since(UNIX_EPOCH) {
        Ok(duration) => {
            let seconds_since = duration.as_secs();
            if seconds_since % 5 == 0 {
                println!("UWUUUUUUUUUUUUUUUUUUUUU");
            } else {
                println!("YOU HAVE COMMITTED HERESY!");
            }
        }
        Err(_) => {
            println!("Error occurred while calculating duration.");
        }
    }
    println!("Answering incorrectly results in DEATH!");
    println!("~uWu~");

    let question = "What is the answer to life, the universe, and everything?".to_string();

    let outcome = match the_answer(question) {
        42 => "Correct!",
        69 => "Yes!",
        9001 => "Such a high power level is obviously correct!",
        _ => "Incorrect!",
    };
    println!("Outcome: {}", outcome);
}

fn the_answer(question: String) -> i32 {
    if question == "What is the answer to life, the universe, and everything?" {
        return draw_answer_from_hat();
    }
    0
}

fn draw_answer_from_hat() -> i32 {
    let numbers = [1, 2, 3, 42, 69, 9001];
    let mut rng = thread_rng();
    if let Some(&random_number) = numbers.choose(&mut rng) {
        random_number
    } else {
        0
    }
}
