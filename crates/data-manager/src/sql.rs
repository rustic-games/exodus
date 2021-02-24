use anyhow::{anyhow, Error, Result};
use data::*;
use rusqlite::Row;
use std::num::NonZeroU16;
use std::str::FromStr;
use uuid::Uuid;

pub trait FromRow: Sized {
    fn from_row(row: &Row<'_>) -> Result<Self, Error>;
}

// types -----------------------------------------------------------------------

impl FromRow for Plant {
    fn from_row(row: &Row<'_>) -> Result<Self, Error> {
        let id = get_id(row)?;
        let name = FromRow::from_row(row)?;
        let habit = plant::Habit::from_str(&row.get::<_, String>("habit")?)?;
        let max_height = FromRow::from_row(row)?;

        Ok(Self {
            id,
            name,
            habit,
            max_height,
        })
    }
}

// properties ------------------------------------------------------------------

impl FromRow for property::Name {
    fn from_row(row: &Row<'_>) -> Result<Self, Error> {
        let name: String = row.get("name")?;
        Ok(name.into())
    }
}

impl FromRow for property::MaxHeight {
    fn from_row(row: &Row<'_>) -> Result<Self, Error> {
        let cm = row.get("max_height_cm")?;

        NonZeroU16::new(cm)
            .ok_or(anyhow!("zero max height"))
            .map(Into::into)
    }
}

// misc ------------------------------------------------------------------------

/// Convert a row to a `Data` enum variant.
pub(crate) fn data_from_row(ty: Type, row: &Row<'_>) -> Result<Data, Error> {
    match ty {
        Type::Plant => Plant::from_row(row).map(Data::Plant),
    }
}

/// Get a validated UUID (v4) from the `id` column.
fn get_id(row: &Row<'_>) -> Result<Uuid, Error> {
    let id: String = row.get("id")?;
    let id = Uuid::parse_str(&id)?;

    if !matches!(id.get_version(), Some(uuid::Version::Random)) {
        return Err(anyhow!("non-random (v4) uuid"));
    }

    Ok(id)
}
