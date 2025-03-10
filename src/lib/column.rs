#[derive(Debug)]
pub struct Column {
    column_type:    ColumnType,
    is_optional:    bool,
    is_primary_key: bool,
    is_foreign_key: bool,
}

#[derive(Debug)]
enum ColumnType {
    UINT8,
    SINT8,
    UINT32,
    SINT32,
    FLOAT32,
    FLOAT64,
    STRING,
}
