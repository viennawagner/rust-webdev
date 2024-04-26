use crate::*;
#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Store {
    pub questions: HashMap<questions::QuestionId, questions::Question>,
}

///Storage for questions and, later, answers until we get a database up.
/// Internally, the data is stored in a HashMap where the ids are the keys,
/// this allows for quicker lookup.
impl Store {
    pub fn new() -> Self {
        Store {
            questions: HashMap::new(),
        }
    }

    ///Add the given question to memory
    pub fn add_question(mut self, question: questions::Question) -> Self {
        self.questions.insert(question.id.clone(), question);
        self
    }

    /// Initializes with questions.json for now
    pub fn init() -> HashMap<questions::QuestionId, questions::Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}