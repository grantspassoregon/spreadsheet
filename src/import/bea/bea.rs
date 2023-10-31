use crate::{error, utils};
use indicatif::{ProgressBar, ProgressStyle};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::is_digit;
use nom::IResult;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, time::Duration};
use tracing::{info, trace};

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
        let records = utils::from_csv(path)?;
        bar.finish_with_message("Loaded!");
        Ok(BeaDataRaw { records })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

/// The `BeaData` struct holds BEA data processed into library form.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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
        let records = utils::from_csv(path)?;
        Ok(BeaData { records })
    }

    /// This method writes the vector of type [`BeaDatum`] in the `records` field of `BeaData` to a
    /// CSV file at location `title`.  Each element in the vector will become a row in the
    /// spreadsheet.
    pub fn to_csv<P: AsRef<std::path::Path>>(&mut self, title: P) -> Result<(), std::io::Error> {
        utils::to_csv(self.records_mut(), title)?;
        Ok(())
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
            if !hash.contains_key(&record.code()) {
                hash.insert(record.code(), record.description());
            }
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
}

impl TryFrom<BeaDataRaw> for BeaData {
    type Error = error::Error;

    fn try_from(raw: BeaDataRaw) -> Result<Self, Self::Error> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Converting BEA data.'}",
        )
        .unwrap();
        let bar = ProgressBar::new(raw.records_ref().len() as u64);
        bar.set_style(style);
        let mut res = Vec::new();
        let mut i = 0;
        let mut k = 0;
        for record in raw.records() {
            trace!("Processing row {}", i);
            i += 1;
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

/// This functions removes commas and note tags (trailing 'E's) from BEA values.  Called by
/// ['str_to_int'].
fn remove_comma<'a, 'b>(
    value: &'a str,
    num: Option<String>,
) -> IResult<&'a str, Option<String>> {
    let mut res = "".to_string();
    let mut out = None;
    if let Some(val) = num {
        res.push_str(&val);
    }
    let val = value.chars().nth(0);
    if let Some(chr) = val {
        if is_digit(chr as u8) {
            let (mut rem, val) = digit1(value)?;
            res.push_str(val);
            if rem.len() > 0 {
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
fn str_to_int(value: &str) -> Result<Option<i64>, error::Error> {
    let stub = "".to_string();
    if let Ok((_, res)) = remove_comma(value, Some(stub)) {
        if let Some(val) = res {
            Ok(Some(val.parse::<i64>()?))
        } else {
            Ok(None)
        }
    } else {
        Err(error::Error::ParseError)
    }
}
