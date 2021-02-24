mod sql;

use anyhow::{Error, Result};
use data::Data;
use rusqlite::{Connection, NO_PARAMS};
use std::fs;
use std::str::FromStr;

fn main() -> Result<()> {
    let conn = Connection::open("./data/database.sqlite3")?;

    let mut paths = vec![];

    for table in type_tables(&conn)? {
        let ty = table.trim_start_matches("type_");
        let objects = objects(&table, &conn)?;

        if !objects.is_empty() {
            fs::create_dir_all(format!("./assets/data/{}", ty))?;
        }

        for object in objects {
            let value = toml::ser::to_string_pretty(&object)?;
            let path = format!("./assets/data/{}/{}.toml", ty, object.name().to_lowercase());

            let update = fs::read_to_string(&path)
                .map(|curr| &curr != &value)
                .unwrap_or(true);

            if update {
                fs::write(&path, value)?;
            }

            paths.push(path);
        }
    }

    for entry in walkdir::WalkDir::new("assets/data")
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let path = entry.path();
        if !paths
            .iter()
            .any(|p| p.as_str().ends_with(&path.to_string_lossy().into_owned()))
        {
            fs::remove_file(entry.path())?;
        }
    }

    Ok(())
}

fn type_tables(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt =
        conn.prepare("SELECT name FROM sqlite_master WHERE type = 'table' AND name LIKE 'type_%'")?;
    let query = stmt.query_map(NO_PARAMS, |row| row.get("name"))?;

    query
        .map(|r| match r {
            Ok(ok) => Ok(ok),
            Err(err) => Err(Error::new(err)),
        })
        .collect::<Result<_>>()
}

fn objects(table: &str, conn: &Connection) -> Result<Vec<Data>> {
    let ty = data::Type::from_str(table.trim_start_matches("type_"))?;
    let mut stmt = conn.prepare(&format!(r#"SELECT * FROM {};"#, table))?;
    let query = stmt.query_and_then(NO_PARAMS, |row| sql::data_from_row(ty, row))?;

    query.collect::<Result<_>>()
}
