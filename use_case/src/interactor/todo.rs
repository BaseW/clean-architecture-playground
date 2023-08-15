use async_trait::async_trait;
use domain::{entity::todo::Todo, repository::todo_repository::TodoRepository};

use crate::{
    dto::todo::{CreateTodoDto, TodoDto},
    error::UseCaseError,
    traits::todo::{MutationUseCase, QueryUseCase},
};

pub struct MutationInteractor<TR> {
    todo_repository: TR,
}

impl<TR> MutationInteractor<TR> {
    pub fn new(todo_repository: TR) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl<TR> MutationUseCase for MutationInteractor<TR>
where
    TR: TodoRepository,
{
    async fn create(&self, todo_data: CreateTodoDto) -> Result<TodoDto, UseCaseError> {
        let todo = Todo::try_from(todo_data)?;
        self.todo_repository.create(&todo).await?;
        Ok(todo.into())
    }

    async fn update(&self, todo_data: TodoDto) -> Result<TodoDto, UseCaseError> {
        let todo = Todo::try_from(todo_data)?;
        self.todo_repository.update(&todo).await?;
        Ok(todo.into())
    }

    async fn delete(&self, todo_id: i64) -> Result<i64, UseCaseError> {
        self.todo_repository.delete(todo_id).await?;
        Ok(todo_id)
    }
}

#[derive(Debug, Clone)]
pub struct QueryInteractor<TR> {
    todo_repository: TR,
}

impl<TR> QueryInteractor<TR> {
    pub fn new(todo_repository: TR) -> Self {
        Self { todo_repository }
    }
}

#[async_trait]
impl<TR> QueryUseCase for QueryInteractor<TR>
where
    TR: TodoRepository,
{
    async fn find_all(&self) -> Result<Vec<TodoDto>, UseCaseError> {
        let result = self.todo_repository.find_all().await;
        match result {
            Ok(todos) => {
                let todo_dtos = todos.into_iter().map(|todo| todo.into()).collect();
                Ok(todo_dtos)
            }
            Err(e) => Err(UseCaseError::from(e)),
        }
    }

    async fn find_by_id(&self, todo_id: i64) -> Result<Option<TodoDto>, UseCaseError> {
        let result = self.todo_repository.find_by_id(todo_id).await;
        match result {
            Ok(todo) => match todo {
                Some(todo) => Ok(Some(todo.into())),
                None => Ok(None),
            },
            Err(e) => Err(UseCaseError::from(e)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::{Arc, Mutex};

    #[derive(Debug, Clone)]
    pub struct MockTodoRepository {
        todos: Arc<Mutex<Vec<Todo>>>,
    }

    impl MockTodoRepository {
        pub fn new() -> Self {
            let todos = vec![Todo {
                id: 1,
                title: Some("task1".to_string()),
            }];
            let todos = Arc::new(Mutex::new(todos));
            Self { todos }
        }
    }

    #[async_trait]
    impl TodoRepository for MockTodoRepository {
        async fn create(&self, new_todo: &Todo) -> Result<(), domain::error::DomainError> {
            let original_todos = self.todos.clone();
            let mut todos = original_todos.lock().unwrap();
            let length = todos.len();
            let new_id = length as i64 + 1;
            let mut new_todos = Vec::new();
            for todo in todos.iter() {
                new_todos.push(todo.clone());
            }
            new_todos.push(Todo {
                id: new_id,
                title: new_todo.title.clone(),
            });
            *todos = new_todos;
            Ok(())
        }

        async fn find_all(&self) -> Result<Vec<Todo>, domain::error::DomainError> {
            let todos = self.todos.clone();
            let todos = todos.lock().unwrap();
            Ok(todos.clone())
        }

        async fn find_by_id(
            &self,
            todo_id: i64,
        ) -> Result<Option<Todo>, domain::error::DomainError> {
            let todos = self.todos.clone();
            let todos = todos.lock().unwrap();
            for todo in todos.iter() {
                if todo.id == todo_id {
                    return Ok(Some(todo.clone()));
                }
            }
            Ok(None)
        }

        async fn update(&self, new_todo: &Todo) -> Result<(), domain::error::DomainError> {
            let original_todos = self.todos.clone();
            let mut todos = original_todos.lock().unwrap();
            let mut new_todos = Vec::new();
            for todo in todos.iter() {
                if todo.id == new_todo.id {
                    new_todos.push(Todo {
                        id: todo.id,
                        title: new_todo.title.clone(),
                    });
                }
            }
            *todos = new_todos;
            Ok(())
        }

        async fn delete(&self, todo_id: i64) -> Result<(), domain::error::DomainError> {
            let original_todos = self.todos.clone();
            let mut todos = original_todos.lock().unwrap();
            let mut new_todos = Vec::new();
            for todo in todos.iter() {
                if todo.id != todo_id {
                    new_todos.push(todo.clone());
                }
            }
            *todos = new_todos;
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_create() {
        let todo_repository = MockTodoRepository::new();
        let mutation_interactor = MutationInteractor::new(todo_repository.clone());
        let todo_data = CreateTodoDto {
            title: "task2".to_string(),
        };
        let result = mutation_interactor.create(todo_data).await;
        assert!(result.is_ok());

        let query_interactor = QueryInteractor::new(todo_repository);
        let result = query_interactor.find_all().await;
        match result {
            Ok(todos) => {
                assert_eq!(todos.len(), 2);
                assert_eq!(todos[0].id, 1);
                assert_eq!(todos[0].title, Some("task1".to_string()));
                assert_eq!(todos[1].id, 2);
                assert_eq!(todos[1].title, Some("task2".to_string()));
            }
            Err(_) => {
                panic!()
            }
        }
    }

    #[tokio::test]
    async fn test_delete() {
        let todo_repository = MockTodoRepository::new();
        let mutation_interactor = MutationInteractor::new(todo_repository.clone());
        let result = mutation_interactor.delete(1).await;
        assert!(result.is_ok());

        let query_interactor = QueryInteractor::new(todo_repository);
        let result = query_interactor.find_all().await;
        match result {
            Ok(todos) => {
                assert_eq!(todos.len(), 0);
            }
            Err(_) => {
                panic!()
            }
        }
    }

    #[tokio::test]
    async fn test_find_all() {
        let todo_repository = MockTodoRepository::new();
        let query_interactor = QueryInteractor::new(todo_repository);
        let result = query_interactor.find_all().await;
        match result {
            Ok(todos) => {
                assert_eq!(todos.len(), 1);
                assert_eq!(todos[0].id, 1);
                assert_eq!(todos[0].title, Some("task1".to_string()));
            }
            Err(_) => panic!(),
        }
    }
}
