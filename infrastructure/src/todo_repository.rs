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

    async fn find_all(&self) -> Result<Vec<Todo>, DomainError> {
        let conn = self.pool.acquire().await;
        let mut conn = match conn {
            Ok(conn) => conn,
            Err(e) => return Err(DomainError::Infrastructure(e.into())),
        };
        InternalSqliteTodoRepository::find_all(&mut conn).await
    }

    async fn find_by_id(&self, id: i64) -> Result<Option<Todo>, DomainError> {
        let conn = self.pool.acquire().await;
        let mut conn = match conn {
            Ok(conn) => conn,
            Err(e) => return Err(DomainError::Infrastructure(e.into())),
        };
        InternalSqliteTodoRepository::find_by_id(id, &mut conn).await
    }

    async fn update(&self, todo: &Todo) -> Result<(), DomainError> {
        let tx = self.pool.begin().await;
        let mut tx = match tx {
            Ok(tx) => tx,
            Err(e) => return Err(DomainError::Infrastructure(e.into())),
        };
        InternalSqliteTodoRepository::update(todo, &mut tx).await?;
        Ok(())
    }

    async fn delete(&self, todo_id: i64) -> Result<(), DomainError> {
        let tx = self.pool.begin().await;
        let mut tx = match tx {
            Ok(tx) => tx,
            Err(e) => return Err(DomainError::Infrastructure(e.into())),
        };
        InternalSqliteTodoRepository::delete(todo_id, &mut tx).await?;
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
        .bind(todo.title.as_ref().unwrap_or(&"".to_string()))
        .execute(&mut *conn)
        .await;
        match todo {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
    }

    pub async fn find_all(conn: &mut SqliteConnection) -> Result<Vec<Todo>, DomainError> {
        let todos = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, title
            FROM todos
            ORDER BY id
            "#,
        )
        .fetch_all(&mut *conn)
        .await;
        match todos {
            Ok(todos) => Ok(todos),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
    }

    pub async fn find_by_id(
        id: i64,
        conn: &mut SqliteConnection,
    ) -> Result<Option<Todo>, DomainError> {
        let todo = sqlx::query_as!(
            Todo,
            r#"
            SELECT id, title
            FROM todos
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&mut *conn)
        .await;
        match todo {
            Ok(todo) => Ok(todo),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
    }

    pub async fn update(todo: &Todo, conn: &mut SqliteConnection) -> Result<(), DomainError> {
        let todo = sqlx::query(
            r#"
            UPDATE todos
            SET title = $1
            WHERE id = $2
            "#,
        )
        .bind(todo.title.as_ref().unwrap_or(&"".to_string()))
        .bind(todo.id)
        .execute(&mut *conn)
        .await;
        match todo {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
    }

    pub async fn delete(todo_id: i64, conn: &mut SqliteConnection) -> Result<(), DomainError> {
        let todo = sqlx::query(
            r#"
            DELETE FROM todos
            WHERE id = $1
            "#,
        )
        .bind(todo_id)
        .execute(&mut *conn)
        .await;
        match todo {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
    }
}
