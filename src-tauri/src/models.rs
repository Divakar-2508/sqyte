use std::collections::HashMap;
use std::str;

use rusqlite::{types::FromSql, Connection};
use serde::Serialize;

pub struct AppState {
    pub db: Connection,
}

#[derive(Debug, Serialize)]
pub struct Table {
    pub name: String,
    pub fields: Vec<Field>,
}

#[derive(Debug, Serialize)]
pub struct Field {
    pub name: String,
    pub field_type: FieldType,
}

#[derive(Debug, PartialEq, Serialize)]
pub enum FieldType {
    Text,
    Integer,
    Real,
    Boolean,
    Misc,
}

#[derive(Debug, Default, Serialize)]
pub struct RowData {
    pub id: i64,
    data: HashMap<String, FieldData>,
}

#[derive(Debug, Serialize)]
pub enum FieldData {
    Text(String),
    Integer(i64),
    Real(f64),
    Boolean(bool),
    Misc(Vec<u8>),
    Null,
}

impl<T: AsRef<str>> From<T> for FieldType {
    fn from(value: T) -> Self {
        match value.as_ref() {
            "INTEGER" => Self::Integer,
            "BOOL" => Self::Boolean,
            "TEXT" => Self::Text,
            "REAL" => Self::Real,
            _ => Self::Misc,
        }
    }
}

impl Table {
    pub fn get_tables(db: &Connection) -> rusqlite::Result<Vec<Table>> {
        let mut tables = Vec::new();

        let mut table_stat = db.prepare(
            "SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE 'sqlite_%'",
        )?;

        let table_names = table_stat.query_map([], |row| {
            let name: String = row.get("name")?;
            Ok(name)
        })?;

        for tb_name in table_names {
            let tb_name = tb_name?;
            let fields = Self::get_fields(db, &tb_name)?;
            tables.push(Table {
                name: tb_name,
                fields,
            });
        }

        Ok(tables)
    }

    fn get_fields(db: &Connection, table_name: &str) -> rusqlite::Result<Vec<Field>> {
        let sql = format!("PRAGMA table_info({})", table_name);
        let mut field_stat = db.prepare(&sql)?;

        let fields = field_stat.query_map([], |row| {
            let field_type = FieldType::from(row.get::<&str, String>("type")?);
            let name = row.get("name")?;

            Ok(Field { field_type, name })
        })?;

        fields.into_iter().collect()
    }

    pub fn fetch_data(
        &self,
        db: &Connection,
        limit: u32,
        offset: u32,
    ) -> rusqlite::Result<Vec<RowData>> {
        let query = format!(
            "SELECT rowid,{} from {} LIMIT {} OFFSET {}",
            self.fields
                .iter()
                .map(|field| field.name.as_str())
                .collect::<Vec<_>>()
                .join(" , "),
            &self.name,
            limit,
            offset
        );

        let mut sql = db.prepare(&query)?;
        let rows = sql.query_map([], |row| {
            let mut row_data = RowData::default();

            for field in self.fields.iter() {
                let mut data: FieldData = row.get(field.name.as_str())?;
                if let FieldData::Integer(val) = data {
                    if field.field_type == FieldType::Boolean {
                        data = FieldData::Boolean(val == 1);
                    }
                }
                row_data.insert(&field.name, data);
            }

            row_data.id = row.get("rowid").unwrap();

            Ok(row_data)
        })?;

        let mut data = Vec::new();

        for row in rows {
            data.push(row?);
        }

        Ok(data)
    }
}

impl RowData {
    fn insert(&mut self, name: &str, data: FieldData) {
        self.data.insert(name.to_string(), data);
    }

    pub fn get(&self, name: &str) -> Option<&FieldData> {
        self.data.get(name)
    }
}

impl FromSql for FieldData {
    fn column_result(value: rusqlite::types::ValueRef<'_>) -> rusqlite::types::FromSqlResult<Self> {
        match value {
            rusqlite::types::ValueRef::Text(val) => Ok(Self::Text(
                str::from_utf8(val).unwrap_or_default().to_string(),
            )),
            rusqlite::types::ValueRef::Real(val) => Ok(Self::Real(val)),
            rusqlite::types::ValueRef::Integer(val) => Ok(Self::Integer(val)),
            rusqlite::types::ValueRef::Blob(val) => Ok(Self::Misc(val.to_vec())),
            rusqlite::types::ValueRef::Null => Ok(Self::Null),
        }
    }
}
