use axum::{
    routing::{get, post},
    Extension, Router,
};
use presentation::{
    graphql::handler::{graphql_handler, graphql_playground_handler},
    rest::handler::get_todos,
};
use server::dependency_injection::{dependency_injection, MI, QI};
use sqlx::{Pool, Sqlite};
use std::{env, net::SocketAddr};
use tower::ServiceBuilder;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    let database_url = env::var("DATABASE_URL")?;
    let server_port = env::var("SERVER_PORT")?;
    let server_port = server_port.parse::<u16>()?;

    let pool: Pool<Sqlite> = Pool::connect(&database_url).await?;

    // sqlx::migrate!()
    //     .run(&pool)
    //     .await
    //     .expect("Migration failed.");

    let (query_use_case, schema) = dependency_injection(pool);

    let app = Router::new()
        .route("/graphiql", get(graphql_playground_handler))
        .route("/graphql", post(graphql_handler::<QI, MI>))
        .route("/todos", get(get_todos::<QI>))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(query_use_case.clone()))
                .layer(Extension(schema)),
        )
        .with_state(query_use_case);

    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
