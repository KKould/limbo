use std::{fmt::Display, ops::Deref};

pub(crate) struct Name(pub(crate) String);

impl Deref for Name {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone)]
pub(crate) struct Table {
    pub(crate) rows: Vec<Vec<Value>>,
    pub(crate) name: String,
    pub(crate) columns: Vec<Column>,
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub(crate) struct Column {
    pub(crate) name: String,
    pub(crate) column_type: ColumnType,
    pub(crate) primary: bool,
    pub(crate) unique: bool,
}

#[derive(Debug, Clone)]
pub(crate) enum ColumnType {
    Integer,
    Float,
    Text,
    Blob,
}

impl Display for ColumnType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Integer => write!(f, "INTEGER"),
            Self::Float => write!(f, "REAL"),
            Self::Text => write!(f, "TEXT"),
            Self::Blob => write!(f, "BLOB"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Null,
    Integer(i64),
    Float(f64),
    Text(String),
    Blob(Vec<u8>),
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Self::Null, Self::Null) => Some(std::cmp::Ordering::Equal),
            (Self::Null, _) => Some(std::cmp::Ordering::Less),
            (_, Self::Null) => Some(std::cmp::Ordering::Greater),
            (Self::Integer(i1), Self::Integer(i2)) => i1.partial_cmp(i2),
            (Self::Float(f1), Self::Float(f2)) => f1.partial_cmp(f2),
            (Self::Text(t1), Self::Text(t2)) => t1.partial_cmp(t2),
            (Self::Blob(b1), Self::Blob(b2)) => b1.partial_cmp(b2),
            // todo: add type coercions here
            _ => None,
        }
    }
}

fn to_sqlite_blob(bytes: &[u8]) -> String {
    format!(
        "X'{}'",
        bytes
            .iter()
            .fold(String::new(), |acc, b| acc + &format!("{:02X}", b))
    )
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Null => write!(f, "NULL"),
            Self::Integer(i) => write!(f, "{}", i),
            Self::Float(fl) => write!(f, "{}", fl),
            Self::Text(t) => write!(f, "'{}'", t),
            Self::Blob(b) => write!(f, "{}", to_sqlite_blob(b)),
        }
    }
}
