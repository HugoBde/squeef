use crate::column::Column;

#[derive(Debug)]
pub struct Table {
    pub name: String,
    _columns: Vec<Column>,
}

impl Table {
    pub fn new(name: String) -> Table {
        Table {
            name,
            _columns: vec![],
        }
    }
}
