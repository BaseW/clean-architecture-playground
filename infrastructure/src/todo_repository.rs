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
        let result = tx.commit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
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
        let result = tx.commit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
    }

    async fn delete(&self, todo_id: i64) -> Result<(), DomainError> {
        let tx = self.pool.begin().await;
        let mut tx = match tx {
            Ok(tx) => tx,
            Err(e) => return Err(DomainError::Infrastructure(e.into())),
        };
        InternalSqliteTodoRepository::delete(todo_id, &mut tx).await?;
        let result = tx.commit().await;
        match result {
            Ok(_) => Ok(()),
            Err(e) => Err(DomainError::Infrastructure(e.into())),
        }
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
        println!("todo: {:?}", todo);
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
        println!("todo: {:?}", todo);
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

#[cfg(test)]
mod tests {
    use super::*;
    use domain::entity::todo::Todo;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn prepare_table(conn: &mut SqliteConnection) {
        sqlx::query(
            r#"
            CREATE TABLE todos (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                title TEXT NOT NULL
            )
            "#,
        )
        .execute(&mut *conn)
        .await
        .unwrap();
    }

    #[tokio::test]
    async fn test_internal() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();

        prepare_table(&mut conn).await;

        let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
        match todos {
            Ok(todos) => {
                assert_eq!(todos.len(), 0);
            }
            Err(_) => panic!("failed to fetch todos"),
        };

        let todo = Todo {
            id: 0,
            title: Some("task1".to_string()),
        };
        let result = InternalSqliteTodoRepository::create(&todo, &mut conn).await;
        let todo = match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 1);
                        let todo = todos[0].clone();
                        assert_eq!(todo.title, Some("task1".to_string()));
                        todo
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to create todo"),
        };

        let result = InternalSqliteTodoRepository::find_by_id(todo.id, &mut conn).await;
        match result {
            Ok(todo) => match todo {
                Some(todo) => {
                    assert_eq!(todo.title, Some("task1".to_string()));
                }
                None => panic!("failed to fetch todo"),
            },
            Err(_) => panic!("failed to fetch todo"),
        };

        let result = InternalSqliteTodoRepository::update(&todo, &mut conn).await;
        match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 1);
                        let todo = todos[0].clone();
                        assert_eq!(todo.title, Some("task1".to_string()));
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to update todo"),
        };

        let result = InternalSqliteTodoRepository::delete(todo.id, &mut conn).await;
        match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 0);
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to delete todo"),
        };
    }

    #[tokio::test]
    async fn test_create() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();

        prepare_table(&mut conn).await;

        let repository = SqliteTodoRepository::new(pool);
        let todo = Todo {
            id: 0,
            title: Some("task1".to_string()),
        };
        let result = repository.create(&todo).await;
        match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 1);
                        let todo = todos[0].clone();
                        assert_eq!(todo.title, Some("task1".to_string()));
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to create todo"),
        };
    }

    #[tokio::test]
    async fn test_update() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();

        prepare_table(&mut conn).await;

        let repository = SqliteTodoRepository::new(pool);
        let todo = Todo {
            id: 0,
            title: Some("task1".to_string()),
        };
        let result = repository.create(&todo).await;
        let todo = match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 1);
                        let todo = todos[0].clone();
                        assert_eq!(todo.title, Some("task1".to_string()));
                        todo
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to create todo"),
        };

        let todo = Todo {
            id: todo.id,
            title: Some("task2".to_string()),
        };
        let result = repository.update(&todo).await;
        match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 1);
                        let todo = todos[0].clone();
                        assert_eq!(todo.title, Some("task2".to_string()));
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to update todo"),
        };
    }

    #[tokio::test]
    async fn test_delete() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();

        prepare_table(&mut conn).await;

        let repository = SqliteTodoRepository::new(pool);
        let todo = Todo {
            id: 0,
            title: Some("task1".to_string()),
        };
        let result = repository.create(&todo).await;
        let todo = match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 1);
                        let todo = todos[0].clone();
                        assert_eq!(todo.title, Some("task1".to_string()));
                        todo
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to create todo"),
        };

        let result = repository.delete(todo.id).await;
        match result {
            Ok(_) => {
                let todos = InternalSqliteTodoRepository::find_all(&mut conn).await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 0);
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to delete todo"),
        };
    }

    #[tokio::test]
    async fn test_find_all() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();

        prepare_table(&mut conn).await;

        let repository = SqliteTodoRepository::new(pool);

        let todos = repository.find_all().await;
        match todos {
            Ok(todos) => {
                assert_eq!(todos.len(), 0);
            }
            Err(_) => panic!("failed to fetch todos"),
        };

        let todo = Todo {
            id: 0,
            title: Some("task1".to_string()),
        };
        let result = InternalSqliteTodoRepository::create(&todo, &mut conn).await;
        match result {
            Ok(_) => {
                let todos = repository.find_all().await;
                match todos {
                    Ok(todos) => {
                        assert_eq!(todos.len(), 1);
                        let todo = todos[0].clone();
                        assert_eq!(todo.title, Some("task1".to_string()));
                        todo
                    }
                    Err(_) => panic!("failed to fetch todos"),
                }
            }
            Err(_) => panic!("failed to create todo"),
        };
    }

    #[tokio::test]
    async fn test_find_by_id() {
        let pool = SqlitePoolOptions::new()
            .connect("sqlite::memory:")
            .await
            .unwrap();
        let mut conn = pool.acquire().await.unwrap();

        prepare_table(&mut conn).await;

        let repository = SqliteTodoRepository::new(pool);

        let todo = repository.find_by_id(1).await;
        match todo {
            Ok(todo) => {
                if todo.is_some() {
                    panic!("todo should not be found");
                }
            }
            Err(_) => panic!("failed to fetch todo"),
        };

        let todo = Todo {
            id: 0,
            title: Some("task1".to_string()),
        };
        let result = InternalSqliteTodoRepository::create(&todo, &mut conn).await;
        match result {
            Ok(_) => {
                let todo = repository.find_by_id(1).await;
                match todo {
                    Ok(todo) => match todo {
                        Some(todo) => {
                            assert_eq!(todo.title, Some("task1".to_string()));
                            todo
                        }
                        None => panic!("todo should be found"),
                    },
                    Err(_) => panic!("failed to fetch todo"),
                }
            }
            Err(_) => panic!("failed to create todo"),
        };
    }
}
