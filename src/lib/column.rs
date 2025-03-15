#[derive(Debug)]
pub struct Column {
    _column_type: ColumnType,
    _is_optional: bool,
    _is_primary_key: bool,
    _is_foreign_key: bool,
}

#[derive(Debug)]
enum ColumnType {
    _UINT8,
    _SINT8,
    _UINT32,
    _SINT32,
    _FLOAT32,
    _FLOAT64,
    _STRING,
}
