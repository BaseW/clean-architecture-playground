use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct Todo {
    id: i64,
    title: Option<String>,
}

impl Todo {
    pub fn new(id: i64, title: Option<String>) -> Self {
        Self { id, title }
    }
}
