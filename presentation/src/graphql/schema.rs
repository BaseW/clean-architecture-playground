use crate::{error::PresentationalError, graphql::object::Todo};
use async_graphql::{Context, EmptySubscription, Object, Schema};
use use_case::{
    dto::todo::{CreateTodoDto, TodoDto},
    traits::todo::{MutationUseCase, QueryUseCase},
};

pub struct Query<QUC> {
    query_use_case: QUC,
}

impl<QUC> Query<QUC>
where
    QUC: QueryUseCase,
{
    pub fn new(query_use_case: QUC) -> Self {
        Self { query_use_case }
    }
}

#[Object]
impl<QUC> Query<QUC>
where
    QUC: QueryUseCase,
{
    async fn todos(&self, _context: &Context<'_>) -> Result<Vec<Todo>, PresentationalError> {
        let todos = self.query_use_case.find_all().await?;
        let todo_objects = todos
            .into_iter()
            .map(|todo| Todo::new(todo.id, todo.title))
            .collect();
        Ok(todo_objects)
    }

    async fn todo(
        &self,
        _context: &Context<'_>,
        id: i64,
    ) -> Result<Option<Todo>, PresentationalError> {
        let todo = self.query_use_case.find_by_id(id).await?;
        match todo {
            Some(todo) => Ok(Some(Todo::new(todo.id, todo.title))),
            None => Ok(None),
        }
    }
}

pub struct Mutation<MUC> {
    mutation_use_case: MUC,
}

impl<MUC> Mutation<MUC>
where
    MUC: MutationUseCase,
{
    pub fn new(mutation_use_case: MUC) -> Self {
        Self { mutation_use_case }
    }
}

#[Object]
impl<MUC> Mutation<MUC>
where
    MUC: MutationUseCase,
{
    async fn create_todo(
        &self,
        _context: &Context<'_>,
        title: String,
    ) -> Result<Todo, PresentationalError> {
        let todo = self
            .mutation_use_case
            .create(CreateTodoDto { title })
            .await?;
        Ok(Todo::new(todo.id, todo.title))
    }

    async fn update_todo(
        &self,
        _context: &Context<'_>,
        id: i64,
        title: String,
    ) -> Result<Todo, PresentationalError> {
        let todo = self
            .mutation_use_case
            .update(TodoDto {
                id,
                title: Some(title),
            })
            .await?;
        Ok(Todo::new(todo.id, todo.title))
    }

    async fn delete_todo(
        &self,
        _context: &Context<'_>,
        id: i64,
    ) -> Result<i64, PresentationalError> {
        self.mutation_use_case.delete(id).await?;
        Ok(id)
    }
}

pub fn build_schema<QUC, MUC>(
    query: Query<QUC>,
    mutation: Mutation<MUC>,
) -> Schema<Query<QUC>, Mutation<MUC>, EmptySubscription>
where
    QUC: QueryUseCase,
    MUC: MutationUseCase,
{
    Schema::build(query, mutation, EmptySubscription).finish()
}
