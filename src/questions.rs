#[derive(Debug)]
pub struct Question {
    id: QuestionId,
    title: String,
    content: String,
    tags: Option<Vec<String>>
}
#[derive(Debug)]
pub struct QuestionId(pub String);

impl Question {
    pub fn new(
        id: QuestionId,
        title: String,
        content: String,
        tags: Option<Vec<String>>
    ) -> Self {
        Question {
            id,
            title,
            content,
            tags,
        }
    }
}

impl std::fmt::Display for QuestionId {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(f, "id: {}", self.0)
    }
}

impl std::fmt::Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error>
    {
        write!(
            f,
            "{}, title: {}, content: {}, tags: {:?}",
            self.id, self.title, self.content, self.tags
        )
    }
}