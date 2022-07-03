pub mod supers;

use async_trait::async_trait;

type RepositoryError = String;

#[async_trait]
pub trait Repository<T> {
    async fn find_all(&mut self) -> Result<Vec<T>, RepositoryError>;
    async fn find_by_id(&self, id: &str) -> Result<T, RepositoryError>;
    async fn create(&mut self, item: &T) -> Result<(), RepositoryError>;
    async fn update(&mut self, item: &T) -> Result<(), RepositoryError>;
    async fn delete(&mut self, id: &str) -> Result<(), RepositoryError>;
}