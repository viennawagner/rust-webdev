mod questions;
fn main() {
    let question = questions::Question::new(
        questions::QuestionId("1".to_string()),
        "First Question".to_string(),
        "Content of question".to_string(),
        ["faq".to_string()]
    );
    println!("{}", question);
}
