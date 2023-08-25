use async_graphql::{EmptySubscription, Schema};
use infrastructure::todo_repository::SqliteTodoRepository;
use presentation::graphql::schema::{build_schema, Mutation, Query};
use sqlx::{Pool, Sqlite};
use use_case::interactor::todo::{MutationInteractor, QueryInteractor, TodoInteractor};

pub type QI = QueryInteractor<SqliteTodoRepository>;
pub type MI = MutationInteractor<SqliteTodoRepository>;
pub type UI = TodoInteractor<SqliteTodoRepository>;

pub fn dependency_injection(
    pool: Pool<Sqlite>,
) -> (QI, Schema<Query<QI>, Mutation<MI>, EmptySubscription>, UI) {
    let sqlite_todo_repository = SqliteTodoRepository::new(pool);

    let query_use_case = QueryInteractor::new(sqlite_todo_repository.clone());
    let mutation_use_case = MutationInteractor::new(sqlite_todo_repository.clone());

    let query = Query::new(query_use_case.clone());
    let mutation = Mutation::new(mutation_use_case);

    let use_case = TodoInteractor::new(sqlite_todo_repository);

    let schema = build_schema(query, mutation);

    (query_use_case, schema, use_case)
}
