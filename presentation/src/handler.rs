use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptySubscription, Schema,
};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    response::{Html, IntoResponse},
    Extension,
};

use crate::graphql::{Mutation, Query};
use use_case::traits::todo::{MutationUseCase, QueryUseCase};

pub async fn graphql_handler<QUC, MUC>(
    schema: Extension<Schema<Query<QUC>, Mutation<MUC>, EmptySubscription>>,
    req: GraphQLRequest,
) -> GraphQLResponse
where
    QUC: QueryUseCase + Clone,
    MUC: MutationUseCase,
{
    schema.execute(req.into_inner()).await.into()
}

pub async fn graphql_playground_handler() -> impl IntoResponse {
    Html(playground_source(GraphQLPlaygroundConfig::new("/graphql")))
}
