use async_trait::async_trait;
use domain::{entity::todo::Todo, error::DomainError, repository::todo_repository::TodoRepository};
use sqlx::{Pool, Sqlite, SqliteConnection};

#[derive(Debug, Clone)]
pub struct SqliteTodoRepository {
    pool: Pool<Sqlite>,
}

impl SqliteTodoRepository {
    pub fn new(pool: Pool<Sqlite>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TodoRepository for SqliteTodoRepository {
    async fn create(&self, todo: &Todo) -> Result<(), DomainError> {
        let tx = self.pool.begin().await;
        let mut tx = match tx {
            Ok(tx) => tx,
            Err(e) => return Err(DomainError::Infrastructure(e.into())),
        };
        InternalSqliteTodoRepository::create(todo, &mut tx).await?;
        Ok(())
    }
}

pub struct InternalSqliteTodoRepository {}

impl InternalSqliteTodoRepository {
    pub async fn create(todo: &Todo, conn: &mut SqliteConnection) -> Result<(), DomainError> {
        let todo = sqlx::query(
            r#"
            INSERT INTO todos (title)
            VALUES ($1)
            RETURNING id, title
            "#,
        )
        .bind(todo.title.as_str().to_string())
        .execute(&mut *conn)
        .await;
        match todo {
            Ok(_) => Ok(()),
            Err(e) => return Err(DomainError::Infrastructure(e.into())),
        }
    }
}
