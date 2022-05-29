pub mod supers;

type RepositoryError = String;

pub trait Repository<T> {
    fn find_all(&mut self) -> Result<Vec<T>, RepositoryError>;
    fn find_by_id(&self, id: &str) -> Result<T, RepositoryError>;
    fn create(&mut self, item: &T) -> Result<(), RepositoryError>;
    fn update(&mut self, item: &T) -> Result<(), RepositoryError>;
    fn delete(&mut self, id: &str) -> Result<(), RepositoryError>;
}