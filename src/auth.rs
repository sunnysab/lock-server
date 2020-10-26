use chrono::Utc;
use sqlx::sqlite::{SqlitePool, SqliteQueryAs};

pub struct UserManager {
    pool: SqlitePool,
}

pub(crate) type CardIdType = i64;

#[derive(sqlx::FromRow)]
pub struct User {
    /// Student id
    pub student_id: String,
    /// Student name
    pub name: String,
    /// Card id
    pub card: CardIdType,
    /// Create time
    pub created_at: Option<i64>,
}

impl User {
    /// Create a new user.
    pub fn new(student_id: String, name: String, card: CardIdType) -> User {
        User {
            student_id,
            name,
            card,
            created_at: Some(Utc::now().naive_local().timestamp()),
        }
    }
}

impl UserManager {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// Query user basic information by card id.
    pub async fn query_by_card(&self, card_id: CardIdType) -> anyhow::Result<Option<User>> {
        let stu: Option<User> =
            sqlx::query_as("SELECT student_id, name, card, created_at FROM user WHERE card = $1")
                .bind(card_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(stu)
    }

    /// Query user basic information by id.
    pub async fn query_by_student_id(&self, student_id: String) -> anyhow::Result<Option<User>> {
        let stu: Option<User> =
            sqlx::query_as("SELECT student_id, name, card, created_at FROM user WHERE student_id = $1")
                .bind(student_id)
                .fetch_optional(&self.pool)
                .await?;
        Ok(stu)
    }

    /// Append a new user
    pub async fn add(&self, new_user: User) -> anyhow::Result<()> {
        sqlx::query("INSERT INTO user (student_id, name, card) VALUES ($1, $2, $3)")
            .bind(new_user.student_id)
            .bind(new_user.name)
            .bind(new_user.card)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    /// Remove an existed user.
    pub async fn remove(&self, student_id: String) -> anyhow::Result<()> {
        sqlx::query("DELETE FROM user WHERE student_id = $1")
            .bind(student_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}
