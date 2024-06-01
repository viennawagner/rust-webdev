use crate::*;
#[derive(Clone, Debug)]
pub struct Store {
    pub questions: Arc<RwLock<HashMap<QuestionId, Question>>>,
}

///Storage for questions and, later, answers until we get a database up.
/// Internally, the data is stored in a HashMap where the ids are the keys,
/// this allows for quicker lookup.
impl Store {
    pub fn new() -> Self {
        Store {
            questions: Arc::new(RwLock::new(Self::init())),
        }
    }

    ///Add the given question to memory
    pub async fn add_question(self, question: Question) -> Self {
        self.questions.write().await.insert(question.id.clone(), question);
        self
    }

    /// Initializes with questions.json for now
    pub fn init() -> HashMap<QuestionId, Question> {
        let file = include_str!("../questions.json");
        serde_json::from_str(file).expect("can't read questions.json")
    }
}
