use crate::column::Column;

#[derive(Debug)]
pub enum Command {
    CreateDatabase { name: String },
    CreateTable { name: String, cols: Vec<Column> },
    OpenDatabase { name: String },
    ListDatabases,
    ListTables,
}
