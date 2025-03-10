use crate::column::Column;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    columns:  Vec<Column>,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            name,
            columns: vec![],
        }
    }
}
