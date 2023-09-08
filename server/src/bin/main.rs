use axum::{
    routing::{get, post},
    Extension, Router,
};
use presentation::{
    graphql::handler::{graphql_handler, graphql_playground_handler},
    grpc::proto_impl::{todo, TodoServiceImpl, TodoServiceServer},
    rest::handler::{create_todo, delete_todo, get_todo, get_todos, update_todo},
};
use server::dependency_injection::{dependency_injection, MI, QI, UI};
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

    let (query_use_case, schema, use_case) = dependency_injection(pool);

    let app = Router::new()
        .route("/graphiql", get(graphql_playground_handler))
        .route("/graphql", post(graphql_handler::<QI, MI>))
        .route(
            "/todos",
            get(get_todos::<UI>)
                .post(create_todo::<UI>)
                .put(update_todo::<UI>)
                .delete(delete_todo::<UI>),
        )
        .route("/todos/:id", get(get_todo::<UI>))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(query_use_case))
                .layer(Extension(schema))
                .layer(Extension(use_case.clone())),
        );

    let reflection_service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(todo::FILE_DESCRIPTOR_SET)
        .build()
        .unwrap();

    let addr = SocketAddr::from(([0, 0, 0, 0], server_port));
    let grpc_addr = SocketAddr::from(([0, 0, 0, 0], server_port + 1));
    let handle = tokio::spawn(async move {
        println!("Listening on http://{}", addr);
        axum::Server::bind(&addr)
            .serve(app.into_make_service())
            .await
            .expect("Server failed to start.");
    });
    let grpc_handle = tokio::spawn(async move {
        println!("Listening on http://{}", grpc_addr);
        tonic::transport::Server::builder()
            .add_service(reflection_service)
            .add_service(TodoServiceServer::<TodoServiceImpl<UI>>::new(
                TodoServiceImpl::<UI> {
                    tu: use_case.clone(),
                },
            ))
            .serve(grpc_addr)
            .await
            .expect("gRPC Server failed to start.");
    });

    handle.await?;
    grpc_handle.await?;

    Ok(())
}
