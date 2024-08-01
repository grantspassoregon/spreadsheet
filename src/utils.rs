//! The `utils` module contains utility functions accessed by multiple data types, where declaring
//! a stand-alone function eliminates code duplication in different methods.
use crate::convert;
use jiff::civil;
use nom::character::complete;
use nom::{bytes, character, combinator};
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

/// The `mdy` function is a helper that converts "\[month\]/\[day\]/\[year\]" format into its
/// constituent parts and parses them to integers, for feeding into a datetime library (jiff).
pub fn mdy(input: &str) -> aid::prelude::Clean<(&str, civil::Date)> {
    // Strip preceding whitespace
    let (rem, _) = character::complete::space0(input)?;
    let (rem, month) = combinator::map_res(character::complete::digit1, str::parse)(rem)?;
    let (rem, _) = bytes::complete::tag("/")(rem)?;
    let (rem, day) = combinator::map_res(character::complete::digit1, str::parse)(rem)?;
    let (rem, _) = bytes::complete::tag("/")(rem)?;
    let (rem, year) = combinator::map_res(character::complete::digit1, str::parse)(rem)?;
    let date = civil::Date::new(year, month, day)?;
    Ok((rem, date))
}

/// The `hm12` function is a helper that converts "\[hour\]:\[minute\]\[am/pm\]" time format into its
/// constituent parts and parses them to integers, for feeding into a datetime library (jiff).
/// The `12` in the name refers to 12-hour time format.  This function will convert to 24-hour time
/// format consistent with the arguments to `jiff`.
pub fn hm12(input: &str) -> aid::prelude::Clean<(&str, civil::Time)> {
    // Strip preceding whitespace
    let (rem, _) = character::complete::space0(input)?;
    // The first set of digits is the hour.
    let (rem, mut hour) = combinator::map_res(character::complete::digit1, str::parse)(rem)?;
    // Hour and minutes are separated by a colon.
    let (rem, _) = bytes::complete::tag(":")(rem)?;
    // The next set of digits are the minutes.
    let (rem, minutes) = combinator::map_res(character::complete::digit1, str::parse)(rem)?;
    // The next set of characters are either "AM" or "PM".
    let (rem, meridian) = complete::alpha1(rem)?;
    match meridian {
        "AM" => {}
        "PM" => {
            if hour == 12 {
                // Corresponds to 12 in 24-hour time.
            } else {
                // Add 12 to the hour for afternoon times.
                hour += 12;
            }
        }
        _ => {
            tracing::warn!("Missing meridian designation.");
        }
    }
    let time = civil::Time::new(hour, minutes, 0, 0)?;
    Ok((rem, time))
}

/// The `hm24` function is a helper that converts "\[hour\]:\[minutes\]" time format into its
/// constituent parts and parses them to integers, for feeding into a datetime library (`jiff`).
/// The `24` in the name refers to 24-hour time format.  The function assumes hours are in 24-hour
/// time and does not correct for change in meridian.
pub fn hm24(input: &str) -> nom::IResult<&str, (i8, i8)> {
    // Strip preceding whitespace
    let (rem, _) = character::complete::space0(input)?;
    // The first set of digits is the hour.
    let (rem, hour) = combinator::map_res(character::complete::digit1, str::parse)(rem)?;
    // Hour and minutes are separated by a colon.
    let (rem, _) = bytes::complete::tag(":")(rem)?;
    // The next set of digits are the minutes.
    let (rem, minutes) = combinator::map_res(character::complete::digit1, str::parse)(rem)?;
    Ok((rem, (hour, minutes)))
}

/// The `datetime` function is a helper that converts "\[month\]/\[day\]/\[year\] \[24-hour\]:\[minutes\]"
/// datetime format to the [`civil::DateTime`] format.
pub fn datetime(input: &str) -> aid::prelude::Clean<(&str, civil::DateTime)> {
    // Start by converting the date.
    let (rem, date) = mdy(input)?;
    // Associated time is not always present.  Use default time when date is valid.
    // Convert the associated time.
    match hm24(rem) {
        Ok((rem, (hour, minutes))) => Ok((rem, date.at(hour, minutes, 0, 0))),
        Err(e) => {
            tracing::trace!("Associated time missing: {}", e.to_string());
            Ok((rem, date.at(0, 0, 0, 0)))
        }
    }
}

/// The `read_geo_line` method reads line geometry from a shapefile into a [`geo::geometry::Geometry`]
/// type.
pub fn read_geo_line(
    line: &shapefile::record::polyline::GenericPolyline<shapefile::record::point::PointZ>,
) -> geo::geometry::Geometry {
    // Wrap in convert type.
    let conv = convert::Convert::new(line.clone());
    // Convert from `shapefile` to `geo`.
    conv.into_geometry()
}

/// The `read_geo_point` method reads the geometry of a shapefile into a [`geo::geometry::Geometry`]
/// type.
pub fn read_geo_point(point: &shapefile::PointZ) -> geo::geometry::Geometry {
    // Wrap in convert type.
    let conv = convert::Convert::new(*point);
    // Convert from `shapefile` point to `geo` point.
    let geo_pt = conv.geo_point();
    // Convert from point type to Geometry.
    let geo: geo::geometry::Geometry = geo_pt.into();
    geo
}

/// The `read_char` associated method is a helper function to extract field names from a
/// [`shapefile::dbase::Record`].
pub fn read_char(record: &shapefile::dbase::Record, field: &str) -> Option<String> {
    let mut result = None;
    match record.get(field) {
        Some(shapefile::dbase::FieldValue::Character(Some(value))) => {
            result = Some(value.to_owned());
        }
        Some(shapefile::dbase::FieldValue::Character(None)) => {
            tracing::trace!("Field is already set to None.");
        }
        Some(v) => {
            tracing::warn!("Field value is the wrong type: {}", v);
        }
        None => {
            tracing::warn!("Unexpected None in field value.");
        }
    }
    result
}

/// The `read_field` associated method is a helper function to extract field names from a
/// [`shapefile::dbase::Record`].
pub fn read_num(record: &shapefile::dbase::Record, field: &str) -> Option<f64> {
    let mut result = None;
    match record.get(field) {
        Some(shapefile::dbase::FieldValue::Numeric(Some(value))) => {
            result = Some(value.to_owned());
        }
        Some(_) => {
            tracing::warn!("Field value is the wrong type.");
        }
        None => {
            tracing::warn!("Unexpected None in field value.");
        }
    }
    result
}
