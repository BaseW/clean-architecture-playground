use async_trait::async_trait;

#[async_trait]
pub trait TodoRepository: Send + Sync + 'static {
    pub async fn create(&self, title: String) -> Result<Todo, Error>;
    pub async fn find_all(&self) -> Result<Vec<Todo>, Error>;
    pub async fn find_by_id(&self, id: i32) -> Result<Option<Todo>, Error>;
    pub async fn update(&self, id: i32, title: String) -> Result<Todo, Error>;
    pub async fn delete(&self, id: i32) -> Result<(), Error>;
}
