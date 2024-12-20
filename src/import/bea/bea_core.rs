use crate::prelude::*;
use aid::prelude::*;
use derive_more::{Deref, DerefMut};
use indicatif::{ProgressBar, ProgressStyle};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::is_digit;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap},
    fmt,
    path::Path,
    time::Duration,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tracing::{info, trace};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
/// The `BeaDatumRaw` struct holds a record from the BEA website API before processing.
pub struct BeaDatumRaw {
    // #[serde(deserialize_with = "crate::import::deserialize_code_keys")]
    // code: Cainc5nCodeKey,
    /// The `code` field represents the BEA table code.
    pub code: String,
    /// The `geo_fips` field represents the FIPS number of the datum.
    pub geo_fips: i32,
    /// The `geo_name` field represents the FIPS description of the datum.
    pub geo_name: String,
    /// The `time_period` field represents the year of the datum.
    pub time_period: i32,
    /// The `description` contains a description of the data value.
    pub description: String,
    /// The `cl_unit` contains the unit of measure for the data value.
    #[serde(rename = "CL_UNIT")]
    pub cl_unit: String,
    /// The `unit_mult` is the numeric factor representation of the unit of measure.
    #[serde(rename = "UNIT_MULT")]
    pub unit_mult: i32,
    /// The `data_value` field represents the value of the datum.
    pub data_value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd, Deref, DerefMut)]
/// The `BeaDataRaw` struct contains a `records` field that holds a vector of type [`BeaDatumRaw`].
pub struct BeaDataRaw(Vec<BeaDatumRaw>);

impl BeaDataRaw {
    /// This method loads a `BeaDataRaw` from a CSV located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(120));
        bar.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        bar.set_message("Loading...");
        let records = from_csv(path)?;
        bar.finish_with_message("Loaded!");
        Ok(BeaDataRaw(records))
    }
}

/// The `BeaColumns` enum delineates fields in [`BeaDatum`] intended for display in a table.
#[derive(EnumIter, Debug, PartialEq, Clone)]
pub enum BeaColumns {
    /// Corresponds to the code key from [`BeaDatum`].
    Code,
    /// Corresponds to the geofips field from [`BeaDatum`]
    GeoFips,
    /// Corresponds to the geo_name field from [`BeaDatum`]
    GeoName,
    /// Corresponds to the time_period field from [`BeaDatum`]
    TimePeriod,
    /// Corresponds to the description field from [`BeaDatum`]
    Description,
    /// Corresponds to the unit description from [`BeaDatum`]
    Unit,
    /// Corresponds to the value field from [`BeaDatum`]
    Value,
}

impl BeaColumns {
    /// Returns a string representation of the variant, functions to produce header names for table
    /// display.
    pub fn names() -> Vec<String> {
        Self::iter()
            .map(|v| format!("{}", v))
            .collect::<Vec<String>>()
    }
}

impl fmt::Display for BeaColumns {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Code => write!(f, "Code"),
            Self::GeoFips => write!(f, "Fips"),
            Self::GeoName => write!(f, "Name"),
            Self::TimePeriod => write!(f, "Year"),
            Self::Description => write!(f, "Description"),
            Self::Unit => write!(f, "Unit"),
            Self::Value => write!(f, "Value"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
/// The `BeaDatum` struct holds data processed from a [`BeaDatumRaw`] struct.
pub struct BeaDatum {
    // #[serde(deserialize_with = "crate::import::deserialize_code_keys")]
    // code: Cainc5nCodeKey,
    /// The `code` field represents the BEA table code.
    pub code: String,
    /// The `geo_fips` field represents the FIPS number of the datum.
    pub geo_fips: i32,
    /// The `geo_name` field represents the FIPS description of the datum.
    pub geo_name: String,
    /// The `time_period` field represents the year of the datum.
    pub time_period: i32,
    /// The `description` contains a description of the data value.
    pub description: String,
    /// The `cl_unit` contains the unit of measure for the data value.
    pub cl_unit: String,
    /// The `unit_mult` is the numeric factor representation of the unit of measure.
    pub unit_mult: i32,
    /// The `data_value` field represents the value of the datum.
    pub data_value: i64,
}

impl BeaDatum {
    /// The `code` field represents the BEA table code.  This function returns the cloned value of
    /// the field.
    pub fn code(&self) -> String {
        self.code.clone()
    }

    /// The `geo_fips` field represents the FIPS number of the datum.  This function returns the
    /// value of the field.
    pub fn geo_fips(&self) -> i32 {
        self.geo_fips
    }

    /// The `geo_name` field represents the FIPS description of the datum.  This function returns the
    /// value of the field.
    pub fn geo_name(&self) -> String {
        self.geo_name.clone()
    }

    /// The `geo_name` field represents the FIPS description of the datum.  This function returns a reference to the
    /// value of the field.
    pub fn geo_name_ref(&self) -> &String {
        &self.geo_name
    }

    /// The `time_period` field represents the year of the datum.  This function returns the value
    /// of the field.
    pub fn time_period(&self) -> i32 {
        self.time_period
    }

    /// The `description` returns a description of the data value.  This function returns the
    /// cloned value of the field.
    pub fn description(&self) -> String {
        self.description.clone()
    }

    /// The `data_value` field represents the value of the datum.  This function returns the value
    /// of the field.
    pub fn data_value(&self) -> i64 {
        self.data_value
    }

    /// The `names` method returns the column names identified in [`BeaColumns`] for use as headers
    /// in a table.
    pub fn names() -> Vec<String> {
        BeaColumns::names()
    }

    /// The `columns` method returns the column values identified in [`BeaColumns`] for use in a
    /// table.
    pub fn columns(&self) -> Vec<String> {
        let mut values = Vec::new();
        for column in BeaColumns::iter() {
            match column {
                BeaColumns::Code => values.push(self.code.to_string()),
                BeaColumns::GeoFips => values.push(self.geo_fips.to_string()),
                BeaColumns::GeoName => values.push(self.geo_name.to_string()),
                BeaColumns::TimePeriod => values.push(self.time_period.to_string()),
                BeaColumns::Description => values.push(self.description.to_string()),
                BeaColumns::Unit => values.push(self.cl_unit.to_string()),
                BeaColumns::Value => values.push(format!("{}", self.data_value)),
            }
        }
        values
    }
}

/// The `BeaData` struct holds BEA data processed into library form.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd, Deref, DerefMut)]
pub struct BeaData(Vec<BeaDatum>);

impl BeaData {
    /// This method loads a `BeaData` from a CSV located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(120));
        bar.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        bar.set_message("Loading...");
        let records = from_csv(path)?;
        bar.finish_with_message("Loaded!");
        Ok(BeaData(records))
    }

    /// This method writes the vector of type [`BeaDatum`] in the `records` field of `BeaData` to a
    /// CSV file at location `title`.  Each element in the vector will become a row in the
    /// spreadsheet.
    pub fn to_csv<P: AsRef<std::path::Path>>(&mut self, title: P) -> Result<(), std::io::Error> {
        to_csv(self, title)?;
        Ok(())
    }

    /// The `save` method serializes the contents of self into binary and writes to a file at
    /// location `path`.  Errors bubble up from serialization in [`bincode`] or file system access during write.
    pub fn save<P: AsRef<Path>>(&self, path: P) -> Clean<()> {
        info!("Serializing to binary.");
        let encode = bincode::serialize(self)?;
        info!("Writing to file.");
        std::fs::write(path, encode)?;
        Ok(())
    }

    /// The `load` method deserializes the contents of a file at location `path` into [`BeaData`].
    /// May error reading the file, for example if the location is invalid, or when deserializing
    /// the binary if the format is invalid.
    pub fn load<P: AsRef<Path>>(path: P) -> Clean<Self> {
        info!("Deserializing from binary.");
        let bar = ProgressBar::new_spinner();
        bar.enable_steady_tick(Duration::from_millis(120));
        bar.set_style(
            ProgressStyle::with_template("{spinner:.blue} {msg}")
                .unwrap()
                .tick_strings(&[
                    "▹▹▹▹▹",
                    "▸▹▹▹▹",
                    "▹▸▹▹▹",
                    "▹▹▸▹▹",
                    "▹▹▹▸▹",
                    "▹▹▹▹▸",
                    "▪▪▪▪▪",
                ]),
        );
        bar.set_message("Loading...");
        let vec: Vec<u8> = std::fs::read(path)?;
        let decode: Self = bincode::deserialize(&vec[..])?;
        bar.finish_with_message("Loaded!");
        Ok(decode)
    }

    /// This functions returns unique line code values from the `records` vector.
    pub fn linecode_keys(&self) -> Vec<String> {
        let mut keys = self.iter().map(|r| r.code()).collect::<Vec<String>>();
        keys.sort();
        keys.dedup();
        keys
    }

    /// This function returns a HashMap of line code keys and description values.
    pub fn linecode_hash(&self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        for record in self.iter() {
            hash.entry(record.code())
                .or_insert_with(|| record.description());
            // if !hash.contains_key(&record.code()) {
            //     hash.insert(record.code(), record.description());
            // }
        }
        hash
    }

    /// This function returns a BTreeMap of line code keys and description values.
    pub fn linecode_btree(&self) -> BTreeMap<String, String> {
        let mut tree = BTreeMap::new();
        for record in self.iter() {
            tree.entry(record.code())
                .or_insert_with(|| record.description());
        }
        tree
    }

    /// This functions returns unique FIPS numbers from the `records` vector.
    pub fn geofips_keys(&self) -> Vec<i32> {
        let mut keys = self.iter().map(|r| r.geo_fips()).collect::<Vec<i32>>();
        keys.sort();
        keys.dedup();
        keys
    }

    /// This function returns a HashMap of geofips keys and description values.
    pub fn geofips_hash(&self) -> HashMap<i32, String> {
        let mut hash = HashMap::new();
        for record in self.iter() {
            hash.entry(record.geo_fips())
                .or_insert_with(|| record.geo_name());
        }
        hash
    }

    /// This function returns a BTreeMap of geofips keys and description values.
    pub fn geofips_btree(&self) -> BTreeMap<i32, String> {
        let mut tree = BTreeMap::new();
        for record in self.iter() {
            tree.entry(record.geo_fips())
                .or_insert_with(|| record.geo_name());
        }
        tree
    }

    /// This function returns unique year values from the `records` vector.
    pub fn time_period_keys(&self) -> Vec<i32> {
        let mut keys = self.iter().map(|r| r.time_period()).collect::<Vec<i32>>();
        keys.sort();
        keys.dedup();
        keys
    }

    /// Filters records in the struct based by comparing the string representation of values in the field specified in `filter` against the `test` value.  The `filter` field can take the values "year", "code", and "fips".
    pub fn filter(&self, filter: &str, test: &str) -> Self {
        trace!("Calling filter on {} records.", self.len());
        let mut records = Vec::new();
        match filter {
            "year" => {
                tracing::trace!("Filtering by year {}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| format!("{}", d.time_period()).as_str() == test)
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                )
            }
            "code" => {
                tracing::trace!("Filtering by code {}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| d.code() == test)
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                )
            }
            "fips" => {
                tracing::trace!("Filtering by fips {}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| format!("{}", d.geo_fips()).as_str() == test)
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                )
            }
            "location" => {
                tracing::trace!("Filtering by location {}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| d.geo_name().to_string().as_str() == test)
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                )
            }
            "description" => {
                tracing::trace!("Filtering by description {}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| d.description().to_string().as_str() == test)
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                )
            }
            _ => tracing::warn!("Invalid filter provided."),
        }
        Self(records)
    }

    /// Filters records in the struct based by comparing the string representation of values in the field specified in `filter` against the values in the `test` parameter.  The `filter` field can take the values "year", "code", and "fips".
    pub fn filter_many(&self, filter: &str, test: &[String]) -> Self {
        trace!("Calling filter on {} records.", self.len());
        let mut records = Vec::new();
        match filter {
            "year" => {
                tracing::trace!("Filtering by years {:?}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| test.contains(&format!("{}", d.time_period())))
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                );
            }
            "code" => {
                tracing::trace!("Filtering by codes {:?}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| test.contains(&d.code().to_string()))
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                );
            }
            "fips" => {
                tracing::trace!("Filtering by fips {:?}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| test.contains(&format!("{}", d.geo_fips())))
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                );
            }
            "location" => {
                tracing::trace!("Filtering by locations {:?}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| test.contains(&d.geo_name().to_string()))
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                );
            }
            "description" => {
                tracing::trace!("Filtering by descriptions {:?}", test);
                records.append(
                    &mut self
                        .iter()
                        .filter(|d| test.contains(&d.description().to_string()))
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                );
            }
            _ => tracing::warn!("Invalid filter provided."),
        }

        Self(records)
    }

    /// Filters records in the struct based upon string representations of the values for the
    /// "year", "code" and "fips" fields.
    pub fn search(&self, year: &str, code: &str, fips: &str) -> Self {
        trace!("Calling search.");
        self.filter("year", year)
            .filter("code", code)
            .filter("fips", fips)
    }

    /// The `names` method returns a vector of the header names identified in [`BeaColumns`] for
    /// use in a table view.
    pub fn names() -> Vec<String> {
        BeaColumns::names()
    }
}

impl TryFrom<BeaDataRaw> for BeaData {
    type Error = Bandage;

    fn try_from(raw: BeaDataRaw) -> Clean<Self> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Converting BEA data.'}",
        )
        .unwrap();
        let bar = ProgressBar::new(raw.len() as u64);
        bar.set_style(style);
        let mut res = Vec::new();
        let mut k = 0;
        for (i, record) in raw.iter().cloned().enumerate() {
            trace!("Processing row {}", i);
            let value = str_to_int(&record.data_value)?;
            if let Some(num) = value {
                res.push(BeaDatum {
                    code: record.code,
                    geo_fips: record.geo_fips,
                    geo_name: record.geo_name,
                    time_period: record.time_period,
                    description: record.description,
                    cl_unit: record.cl_unit,
                    unit_mult: record.unit_mult,
                    data_value: num,
                });
            } else {
                trace!(
                    "Dropped record {:?}, fips {}, year {}: NaN",
                    record.code,
                    record.geo_fips,
                    record.time_period
                );
                k += 1;
            }
            bar.inc(1);
        }
        info!("Dropped {} records with NA values.", k);
        Ok(BeaData(res))
    }
}

impl From<Vec<BeaDatum>> for BeaData {
    fn from(records: Vec<BeaDatum>) -> Self {
        tracing::trace!("Calling From for BeaData.");
        Self(records)
    }
}

/// This functions removes commas and note tags (trailing 'E's) from BEA values.  Called by
/// ['str_to_int'].
// fn remove_comma<'a, 'b>(value: &'a str, num: Option<String>) -> IResult<&'a str, Option<String>> {
fn remove_comma(value: &str, num: Option<String>) -> IResult<&str, Option<String>> {
    let mut res = "".to_string();
    let mut out = None;
    if let Some(val) = num {
        res.push_str(&val);
    }
    let val = value.chars().next();
    if let Some(chr) = val {
        if is_digit(chr as u8) {
            let (mut rem, val) = digit1(value)?;
            res.push_str(val);
            if !rem.is_empty() {
                (rem, _) = alt((tag(","), tag(" E")))(rem)?;
                (rem, out) = remove_comma(rem, Some(res))?;
                Ok((rem, out))
            } else {
                Ok((rem, Some(res)))
            }
        } else {
            Ok(("", out))
        }
    } else {
        Ok(("", out))
    }
}

/// This function converts the string representation of a number where thousands are separated by
/// commas into an integer type.  Calls ['remove_comma'].  Called by [`BeaData::try_from()`].
fn str_to_int(value: &str) -> Clean<Option<i64>> {
    let stub = "".to_string();
    if let Ok((_, res)) = remove_comma(value, Some(stub)) {
        if let Some(val) = res {
            Ok(Some(val.parse::<i64>()?))
        } else {
            Ok(None)
        }
    } else {
        Err(Bandage::Parse)
    }
}
