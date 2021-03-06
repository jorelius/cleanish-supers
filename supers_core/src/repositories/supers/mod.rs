use async_trait::async_trait;

use crate::drivers::db::DBDriver;
use crate::entities::supers::Super;
use crate::repositories::Repository;
use crate::repositories::RepositoryError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SupersRepository<D: Sized>
where
    D: DBDriver,
{
    db: D,
}

impl<D: Sized> SupersRepository<D>
where
    D: DBDriver,
{
    pub fn new(db: D) -> Self {
        Self { db }
    }
}

#[async_trait]
impl<D: Sized> Repository<Super> for SupersRepository<D>
where
    D: DBDriver + Send + Sync,
{
    async fn find_all(&mut self) -> Result<Vec<Super>, RepositoryError> {
        let supers = self.db.retrieve_all().unwrap();
        supers.iter().map(|s| convert_str_to_super(&s)).collect()
    }

    async fn find_by_id(&self, id: &str) -> Result<Super, RepositoryError> {
        let super_str = self.db.find_by_id(id)?;

        convert_str_to_super(&super_str)
    }

    async fn create(&mut self, spr: &Super) -> Result<(), RepositoryError> {
        let super_str = convert_super_to_str(&spr)?;
        self.db.create(&spr.id, super_str.as_str())
    }

    async fn update(&mut self, spr: &Super) -> Result<(), RepositoryError> {
        let super_str = convert_super_to_str(&spr)?;
        self.db.update(&spr.id, super_str.as_str())
    }

    async fn delete(&mut self, id: &str) -> Result<(), RepositoryError> {
        self.db.delete(id)
    }
}

fn convert_super_to_str(s: &Super) -> Result<String, RepositoryError> {
    match serde_json::to_string(&s) {
        Ok(supr) => Ok(supr),
        Err(e) => Err(format!("{}", e)),
    }
}

fn convert_str_to_super(super_str: &str) -> Result<Super, RepositoryError> {
    match serde_json::from_str(super_str) {
        Ok(supr) => Ok(supr),
        Err(e) => Err(format!("{}", e)),
    }
}
