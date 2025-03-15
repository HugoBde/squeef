use crate::table::Table;

#[derive(Debug)]
pub struct Database {
    pub name:   String,
    pub tables: Vec<Table>,
}

impl Database {
    pub fn new(name: String) -> Database {
        Database {
            name,
            tables: vec![],
        }
    }
}
