use crate::prelude::*;
use aid::prelude::*;
use indicatif::{ProgressBar, ProgressStyle};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::is_digit;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path, time::Duration, fmt};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use tracing::{info, trace};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
#[serde(rename_all = "PascalCase")]
/// The `BeaDatumRaw` struct holds a record from the BEA website API before processing.
pub struct BeaDatumRaw {
    // #[serde(deserialize_with = "crate::import::deserialize_code_keys")]
    // code: Cainc5nCodeKey,
    code: String,
    geo_fips: i32,
    geo_name: String,
    time_period: i32,
    description: String,
    #[serde(rename = "CL_UNIT")]
    cl_unit: String,
    #[serde(rename = "UNIT_MULT")]
    unit_mult: i32,
    data_value: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
/// The `BeaDataRaw` struct contains a `records` field that holds a vector of type [`BeaDatumRaw`].
pub struct BeaDataRaw {
    records: Vec<BeaDatumRaw>,
}

impl BeaDataRaw {
    /// The `records` field holds a vector of type [`BeaDatumRaw`].  This function returns the
    /// cloned value of the field.
    pub fn records(&self) -> Vec<BeaDatumRaw> {
        self.records.clone()
    }

    /// This method returns a reference to the `records` field.
    pub fn records_ref(&self) -> &Vec<BeaDatumRaw> {
        &self.records
    }

    /// This method returns a mutable reference to the `records` field.
    pub fn records_mut(&mut self) -> &mut Vec<BeaDatumRaw> {
        &mut self.records
    }

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
        Ok(BeaDataRaw { records })
    }
}

#[derive(EnumIter, Debug, PartialEq, Clone)]
pub enum BeaColumns {
    Code,
    GeoFips,
    GeoName,
    TimePeriod,
    Description,
    Unit,
    Value,
}

impl BeaColumns {
    pub fn names() -> Vec<String> {
        Self::iter().map(|v| format!("{}", v)).collect::<Vec<String>>()
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
    code: String,
    geo_fips: i32,
    geo_name: String,
    time_period: i32,
    description: String,
    #[serde(rename = "CL_UNIT")]
    cl_unit: String,
    #[serde(rename = "UNIT_MULT")]
    unit_mult: i32,
    data_value: i64,
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
                BeaColumns::Code => values.push(format!("{}", self.code)),
                BeaColumns::GeoFips => values.push(format!("{}", self.geo_fips)),
                BeaColumns::GeoName => values.push(format!("{}", self.geo_name)),
                BeaColumns::TimePeriod => values.push(format!("{}", self.time_period)),
                BeaColumns::Description => values.push(format!("{}", self.description)),
                BeaColumns::Unit => values.push(format!("{}", self.cl_unit)),
                BeaColumns::Value => values.push(format!("{}", self.data_value)),
            }
        }
        values
    }
}

/// The `BeaData` struct holds BEA data processed into library form.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct BeaData {
    records: Vec<BeaDatum>,
}

impl BeaData {
    /// The `records` field holds a vector of type [`BeaDatum`].  This function returns the
    /// cloned value of the field.
    pub fn records(&self) -> Vec<BeaDatum> {
        self.records.clone()
    }

    /// This method returns a reference to the `records` field.
    pub fn records_ref(&self) -> &Vec<BeaDatum> {
        &self.records
    }

    /// This method returns a mutable reference to the `records` field.
    pub fn records_mut(&mut self) -> &mut Vec<BeaDatum> {
        &mut self.records
    }

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
        Ok(BeaData { records })
    }

    /// This method writes the vector of type [`BeaDatum`] in the `records` field of `BeaData` to a
    /// CSV file at location `title`.  Each element in the vector will become a row in the
    /// spreadsheet.
    pub fn to_csv<P: AsRef<std::path::Path>>(&mut self, title: P) -> Result<(), std::io::Error> {
        to_csv(self.records_mut(), title)?;
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
        let mut keys = self
            .records_ref()
            .iter()
            .map(|r| r.code())
            .collect::<Vec<String>>();
        keys.sort();
        keys.dedup();
        keys
    }

    /// This function returns a HashMap of line code keys and description values.
    pub fn linecode_hash(&self) -> HashMap<String, String> {
        let mut hash = HashMap::new();
        for record in self.records_ref() {
            hash.entry(record.code())
                .or_insert_with(|| record.description());
            // if !hash.contains_key(&record.code()) {
            //     hash.insert(record.code(), record.description());
            // }
        }
        hash
    }

    /// This functions returns unique FIPS numbers from the `records` vector.
    pub fn geofips_keys(&self) -> Vec<i32> {
        let mut keys = self
            .records_ref()
            .iter()
            .map(|r| r.geo_fips())
            .collect::<Vec<i32>>();
        keys.sort();
        keys.dedup();
        keys
    }

    /// This function returns a HashMap of geofips keys and description values.
    pub fn geofips_hash(&self) -> HashMap<i32, String> {
        let mut hash = HashMap::new();
        for record in self.records_ref() {
            hash.entry(record.geo_fips())
                .or_insert_with(|| record.geo_name());
            // if !hash.contains_key(&record.geo_fips()) {
            //     hash.insert(record.geo_fips(), record.geo_name());
            // }
        }
        hash
    }

    /// This function returns unique year values from the `records` vector.
    pub fn time_period_keys(&self) -> Vec<i32> {
        let mut keys = self
            .records_ref()
            .iter()
            .map(|r| r.time_period())
            .collect::<Vec<i32>>();
        keys.sort();
        keys.dedup();
        keys
    }

    /// Filters records in the struct based by comparing the string representation of values in the field specified in `filter` against the `test` value.  The `filter` field can take the values "year", "code", and "fips".
    pub fn filter(&self, filter: &str, test: &str) -> Self {
        trace!("Calling filter on {} records.", self.records_ref().len());
        let mut records = Vec::new();
        match filter {
            "year" => {
                tracing::trace!("Filtering by year {}", test);
                records.append(
                    &mut self
                        .records_ref()
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
                        .records_ref()
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
                        .records_ref()
                        .iter()
                        .filter(|d| format!("{}", d.geo_fips()).as_str() == test)
                        .cloned()
                        .collect::<Vec<BeaDatum>>(),
                )
            }
            _ => tracing::warn!("Invalid filter provided."),
        }
        Self { records }
    }

    /// Filters records in the struct based upon string representations of the values for the
    /// "year", "code" and "fips" fields.
    pub fn search(&self, year: &str, code: &str, fips: &str) -> Self {
        trace!("Calling search.");
        self.filter("year", year)
            .filter("code", code)
            .filter("fips", fips)
    }
}

impl TryFrom<BeaDataRaw> for BeaData {
    type Error = Bandage;

    fn try_from(raw: BeaDataRaw) -> Clean<Self> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Converting BEA data.'}",
        )
        .unwrap();
        let bar = ProgressBar::new(raw.records_ref().len() as u64);
        bar.set_style(style);
        let mut res = Vec::new();
        let mut k = 0;
        for (i, record) in raw.records().into_iter().enumerate() {
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
        Ok(BeaData { records: res })
    }
}

impl From<Vec<BeaDatum>> for BeaData {
    fn from(records: Vec<BeaDatum>) -> Self {
        tracing::trace!("Calling From for BeaData.");
        Self { records }
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
