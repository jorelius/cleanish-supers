use crate::drivers::db::DBDriver;
use std::collections::HashMap;

// This is a simple in-memory database driver.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InMemoryDB {
    data: HashMap<String, String>,
}

impl InMemoryDB {
    pub fn new() -> Self {
        Self {
            data: HashMap::new(),
        }
    }
}

impl DBDriver for InMemoryDB {
    fn retrieve_all(&self) -> Result<Vec<String>, String> {
        Ok(self.data.values().cloned().collect())
    }

    fn find_by_id(&self, id: &str) -> Result<&str, String> {
        match self.data.get(id).map(|s| s as &str) {
            Some(s) => Ok(s),
            None => Err(format!("No record found for id: {}", id)),
        }
    }

    fn create(&mut self, id: &str, item: &str) -> Result<(), String> {
        match self.data.insert(id.to_string(), item.to_string()) {
            Some(_) => Err("Already exists".to_string()),
            None => Ok(()),
        }
    }

    fn update(&mut self, id: &str, item: &str) -> Result<(), String> { 
        self.data.insert(id.to_string(), item.to_string());
        Ok(())
     }

    fn delete(&mut self, id: &str) -> Result<(), String> {
        match self.data.remove(id) {
            Some(_) => Ok(()),
            None => Err("Delete Failed: Not found".to_string()),
        }
    }
}


#[cfg(test)]
mod tests {
    

}