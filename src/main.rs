fn main() {
    println!("Hello, world!");
    println!("Hello, world!");

    let question = "What is the answer to life, the universe, and everything?".to_string();

    if the_answer(question) == 42 {
        println!("Correct!");
    } else {
        println!("Incorrect!");
    }
}

fn the_answer(question: String) -> i32 {
    let base = "42".to_string();
    if question == "What is the answer to life, the universe, and everything?" {
        return base.parse().unwrap();
    }
    0
}
