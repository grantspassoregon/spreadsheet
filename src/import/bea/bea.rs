use crate::error;
use indicatif::ProgressBar;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::character::is_digit;
use nom::IResult;
use serde::{Deserialize, Serialize};
use tracing::{info, trace};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BeaDatumRaw {
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
pub struct BeaDataRaw {
    records: Vec<BeaDatumRaw>,
}

impl BeaDataRaw {
    pub fn records_ref(&self) -> &Vec<BeaDatumRaw> {
        &self.records
    }

    pub fn records_mut(&mut self) -> &mut Vec<BeaDatumRaw> {
        &mut self.records
    }

    pub fn records(&self) -> Vec<BeaDatumRaw> {
        self.records.clone()
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: BeaDatumRaw = result?;
            data.push(record);
        }

        Ok(BeaDataRaw { records: data })
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct BeaDatum {
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
    pub fn code(&self) -> String {
        self.code.clone()
    }

    pub fn description(&self) -> String {
        self.description.clone()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BeaData {
    records: Vec<BeaDatum>,
}

impl BeaData {
    pub fn records_ref(&self) -> &Vec<BeaDatum> {
        &self.records
    }

    pub fn records_mut(&mut self) -> &mut Vec<BeaDatum> {
        &mut self.records
    }

    pub fn records(&self) -> Vec<BeaDatum> {
        self.records.clone()
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: BeaDatum = result?;
            data.push(record);
        }
        Ok(BeaData { records: data })
    }

    pub fn to_csv<P: AsRef<std::path::Path>>(&mut self, title: P) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.records.clone() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
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
                info!(
                    "Dropped record {}, fips {}, year {}: NaN",
                    record.code, record.geo_fips, record.time_period
                );
            }
            bar.inc(1);
        }
        Ok(BeaData { records: res })
    }
}

pub fn remove_comma<'a, 'b>(
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

pub fn str_to_int(value: &str) -> Result<Option<i64>, error::Error> {
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
