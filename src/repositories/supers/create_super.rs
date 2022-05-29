use crate::repositories::supers::Repository;
use crate::drivers::db::sqlite::SqliteDriver;

impl Repository {
    pub fn create_super(&self, super_name: &str, powers: &str) -> Result<Super> {
        let mut stmt = self.db.prepare("INSERT INTO supers (name, powers) VALUES (?1, ?2)")?;
        let mut params = stmt.params();
        params.append(super_name);
        params.append(powers);
        let id = stmt.insert(params)?;
        Ok(Super {
            id,
            name: super_name.to_string(),
            powers: powers.to_string(),
        })
    }
}