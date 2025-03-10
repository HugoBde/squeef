use std::collections::HashMap;

use crate::table::Table;

#[derive(Debug)]
pub struct Database {
    pub name:   String,
    pub tables: HashMap<String, Table>,
}

impl Database {
    pub fn new(name: String) -> Database {
        Database {
            name,
            tables: HashMap::new(),
        }
    }
}
