use std::str::FromStr;

mod questions;
fn main() {
    let question = questions::Question::new(
        questions::QuestionId::from_str("1").expect("No id provided"),
        "First Question".to_string(),
        "Content of question".to_string(),
        Some(vec!("faq".to_string()))
    );
    println!("{}", question);
}
