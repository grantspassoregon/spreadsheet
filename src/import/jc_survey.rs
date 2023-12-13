use crate::prelude::*;
use address::prelude::*;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Vote {
    UtilityFee,
    FoodBeverageTax,
    GeneralSalesTax,
    UtilityFeeSalesTax,
    UtilityFeeFoodBeverageTax,
    ReduceService,
}

impl TryFrom<&String> for Vote {
    type Error = Error;
    fn try_from(input: &String) -> SheetResult<Self> {
        let input = input.as_str();
        match input {
            "Option 1: $27/$88 Public Safety Utility Fee only" => Ok(Self::UtilityFee),
            "Option 2: 7% Food & Beverage Tax only" => Ok(Self::FoodBeverageTax),
            "Option 3: 2% General Sales Tax only" => Ok(Self::GeneralSalesTax),
            "Option 4: Utility Fee/Sales Tax" => Ok(Self::UtilityFeeSalesTax),
            "Option 5: Utility Fee/Food & Beverage Tax" => Ok(Self::UtilityFeeFoodBeverageTax),
            "Option 6: Reduce Police/Fire Staffing" => Ok(Self::ReduceService),
            _ => Err(Error::ParseError),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JcSurveyRawItem {
    pub city: Option<String>,
    pub address: Option<String>,
    pub option: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JcSurveyRaw {
    pub records: Vec<JcSurveyRawItem>,
}

impl JcSurveyRaw {
    /// Creates a new `JcSurveyRaw` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(JcSurveyRaw { records })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JcSurveyItem {
    pub city: Option<String>,
    pub address: PartialAddress,
    pub option: Vote,
}

impl TryFrom<&JcSurveyRawItem> for JcSurveyItem {
    type Error = Error;
    fn try_from(raw: &JcSurveyRawItem) -> SheetResult<Self> {
        let city = raw.city.clone();
        if let Some(raw_address) = raw.address.clone() {
            match parse_address(&raw_address.to_uppercase()) {
                Ok((_, addr)) => {
                    tracing::trace!("Parsed to {:#?}", &addr);
                    let mut address = addr.clone();
                    if let Some(ident) = addr.subaddress_identifier() {
                        address.set_subaddress_identifier(&ident.to_uppercase())
                    };
                    if let Some(option) = raw.option.clone() {
                        let option = Vote::try_from(&option)?;
                        Ok(Self {
                            city,
                            address,
                            option,
                        })
                    } else {
                        Err(Error::UnknownError)
                    }
                }
                Err(_) => Err(Error::ParseError),
            }
        } else {
            Err(Error::UnknownError)
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JcSurvey {
    pub records: Vec<JcSurveyItem>,
}

impl JcSurvey {
    /// Creates a new `JcSurvey` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> SheetResult<Self> {
        let records = from_csv(path)?;
        let records = JcSurveyRaw { records };
        Ok(JcSurvey::try_from(&records)?)
    }

    pub fn validate(&self, other: &Addresses) -> JcSurveyExport {
        let mut records = Vec::new();
        for item in self.records.clone() {
            let res = MatchPartialRecord::compare(&item.address, other);
            let res = res.records();
            if !res.is_empty() {
                if res[0].match_status() != MatchStatus::Missing {
                    let address = res[0].address_label();
                    let option = item.option;
                    let x_coordinate = res[0].longitude().unwrap();
                    let y_coordinate = res[0].latitude().unwrap();
                    records.push(JcSurveyExportItem {
                        address,
                        option,
                        x_coordinate,
                        y_coordinate,
                    })
                }
            }
        }
        JcSurveyExport { records }
    }
}

impl TryFrom<&JcSurveyRaw> for JcSurvey {
    type Error = Error;
    fn try_from(raw: &JcSurveyRaw) -> SheetResult<Self> {
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Parsing addresses.'}",
        )
        .unwrap();
        let bar = ProgressBar::new(raw.records.len() as u64);
        bar.set_style(style);
        let mut records = Vec::new();
        for record in raw.records.clone() {
            tracing::trace!("Trying {:#?}", &record.address);
            let item = JcSurveyItem::try_from(&record)?;
            records.push(item);
            bar.inc(1);
        }
        Ok(JcSurvey { records })
    }
}

impl From<&JcSurvey> for PartialAddresses {
    fn from(data: &JcSurvey) -> Self {
        let mut records = Vec::new();
        for record in data.records.clone() {
            records.push(record.address);
        }
        Self::new(records)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JcSurveyExportItem {
    pub address: String,
    pub option: Vote,
    pub x_coordinate: f64,
    pub y_coordinate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JcSurveyExport {
    pub records: Vec<JcSurveyExportItem>,
}

impl JcSurveyExport {
    /// Write the contents of `JcSurveyExport` to a CSV file at location `title`.  Each element in
    /// the vector of type [`JcSurveyExportItem`] maps to a row of data on the CSV.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(&mut self.records, title)?;
        Ok(())
    }
}
