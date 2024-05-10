#![deny(clippy::all)]

#[macro_use]
extern crate napi_derive;

use napi::{bindgen_prelude::*};
use rusqlite::{Connection};
use serde::{Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct OpenedList {
    pub entries: Vec<Entry>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct Entry {
    pub folder_uri: Option<String>,
    pub label: Option<String>,
    pub workspace: Option<Workspace>,
    pub file_uri: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
#[napi(object)]
pub struct Workspace {
    pub id: String,
    pub config_path: String,
}


fn open(path: String) -> Result<Connection> {
    Connection::open(&path).map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to open SQLite database at {}: {}", path, err))
    })
}

fn query_value_for_key(conn: &Connection, key: &str) -> Result<String> {
    let sql = "select KEY AS _key, value as _value from ItemTable WHERE ItemTable.key = ?";

    let mut stmt = conn.prepare(sql).map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to prepare SQL statement. Error: {}", err))
    })?;

    let value = stmt.query_row(&[key], |row| row.get::<_, String>(1)).map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to prepare SQL statement. Error: {}", err))
    })?;

    drop(stmt);

    Ok(value)
}

#[warn(dead_code)]
#[napi]
pub fn get_opened_list(path: String) -> Result<OpenedList> {
    let conn = open(path)?;

    let value = query_value_for_key(&conn, "history.recentlyOpenedPathsList")?;

    conn.close().map_err(|(_conn, err)| {
        Error::new(Status::GenericFailure, format!("Failed to prepare SQL statement. Error: {}", err))
    })?;

    let list: OpenedList = serde_json::from_str(&*value).map_err(|err| {
        Error::new(Status::GenericFailure, format!("Failed to json deserialize. Error: {}", err))
    })?;

    Ok(list)
}