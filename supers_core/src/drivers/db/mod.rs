pub mod memory;

type DBDriverError = String;

pub trait DBDriver {
    fn retrieve_all(&self) -> Result<Vec<String>, DBDriverError>;
    fn find_by_id(&self, id: &str) -> Result<&str, DBDriverError>;
    fn create(&mut self, id: &str, item: &str) -> Result<(), DBDriverError>;
    fn update(&mut self, id: &str, item: &str) -> Result<(), DBDriverError>;
    fn delete(&mut self, id: &str) -> Result<(), DBDriverError>;
}