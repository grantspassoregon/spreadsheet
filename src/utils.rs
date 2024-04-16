//! The `utils` module contains utility functions accessed by multiple data types, where declaring
//! a stand-alone function eliminates code duplication in different methods.
use serde::de::{Deserialize, DeserializeOwned, Deserializer};
use serde::Serialize;

/// Function for deserailizing ArcGIS data that may contain either empty (Null) fields, or fields
/// with string value "\<Null\>", either of which should translate to `None`.
pub fn deserialize_arcgis_data<'de, D: Deserializer<'de>>(
    de: D,
) -> Result<Option<String>, D::Error> {
    let intermediate = Deserialize::deserialize(de)?;

    match intermediate {
        None => Ok(None),
        Some("<Null>") => Ok(None),
        Some(other_value) => Ok(Some(other_value.to_string())),
    }
}

/// Generic function to serialize data types into a CSV file.  Called by methods to avoid code
/// duplication.
pub fn to_csv<T: Serialize + Clone, P: AsRef<std::path::Path>>(
    item: &mut [T],
    title: P,
) -> Result<(), std::io::Error> {
    let mut wtr = csv::Writer::from_path(title)?;
    for i in item.iter().cloned() {
        wtr.serialize(i)?;
    }
    wtr.flush()?;
    Ok(())
}

/// Generic function to deserialize data types from a CSV file.  Called by methods to avoid code
/// duplication.
pub fn from_csv<T: DeserializeOwned + Clone, P: AsRef<std::path::Path>>(
    path: P,
) -> Result<Vec<T>, std::io::Error> {
    let mut records = Vec::new();
    let file = std::fs::File::open(path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut dropped = 0;
    for result in rdr.deserialize() {
        match result {
            Ok(record) => records.push(record),
            Err(e) => {
                tracing::trace!("Dropping: {}", e.to_string());
                dropped += 1;
            }
        }
    }
    tracing::info!("{} records dropped.", dropped);

    Ok(records)
}
