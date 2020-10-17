use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

pub struct UserManager {
    pool: SqlitePool,
}

#[derive(sqlx::FromRow)]
pub struct User {
    /// Student id
    pub student_id: String,
    /// Student name
    pub name: String,
    /// Card id
    pub card: String,
    /// Create time
    pub created_at: Option<i64>,
}

impl UserManager {
    pub fn new(pool: SqlitePool) {}

    pub async fn query_by_card(&self, card_id: String) -> anyhow::Result<Option<User>> {
        let stu: Option<User> =
            sqlx::query_as("SELECT student_id, name, card, created_at FROM user WHERE card = $1")
                .bind(card_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(stu)
    }

    pub async fn query_by_student_id(&self, student_id: String) -> anyhow::Result<Option<User>> {
        let stu: Option<User> =
            sqlx::query_as("SELECT student_id, name, card, created_at FROM user WHERE student_id = $1")
                .bind(student_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(stu)
    }
}
