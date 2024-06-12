use crate::*;
#[derive(Clone, Debug)]
pub struct Store {
    pub connection: PgPool,
}

///Communication with the database
impl Store {
    pub async fn new(db_url: &str) -> Self {
        let db_pool = match PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url).await {
            Ok(pool) => pool,
            Err(e) => panic!("Couldn't establish DB connection:{}", e), 
        };
   
        Store {
            connection: db_pool,
        }
    }

    ///Get all questions from memory
    pub async fn get_questions(
        &self,
    ) -> Result<Vec<Question>, FaqError> {
        match sqlx::query("SELECT * from questions")
            .map(|row: PgRow| Question {
                id: QuestionId(row.get("id")),
                title: row.get("title"),
                content: row.get("content"),
                tags: row.get("tags"),
            })
            .fetch_all(&self.connection)
            .await {
                Ok(questions) => Ok(questions),
                Err(e) => {
                    Err(FaqError::DatabaseError(e))
                }
            }
    }

pub async fn add_question(
   &self, 
   new_question: NewQuestion
) -> Result<Question, sqlx::Error> {
   match sqlx::query(
       "INSERT INTO questions (title, content, tags) 
       VALUES ($1, $2, $3)"
   )
    .bind(new_question.title)
    .bind(new_question.content)
    .bind(new_question.tags)
    .map(|row: PgRow| Question {
         id: QuestionId(row.get("id")),
         title: row.get("title"),
         content: row.get("content"),
         tags: row.get("tags"),
     })
    .fetch_one(&self.connection)
    .await {
         Ok(question) => Ok(question),
         Err(e) => Err(e),
     }
}

}
