use address::prelude::*;
use aid::prelude::*;
use indicatif::ProgressBar;
use serde::{Deserialize, Serialize};

/// The `Vote` enum represents voting options on the 2023 public service revenue survey.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Vote {
    /// Option 1: $27/$88 Public Safety Utility Fee only
    UtilityFee,
    /// Option 2: 7% Food & Beverage Tax only
    FoodBeverageTax,
    /// Option 3: 2% General Sales Tax only
    GeneralSalesTax,
    /// Option 4: Utility Fee/Sales Tax
    UtilityFeeSalesTax,
    /// Option 5: Utility Fee/Food & Beverage Tax
    UtilityFeeFoodBeverageTax,
    /// Option 6: Reduce Police/Fire Staffing
    ReduceService,
}

impl TryFrom<&String> for Vote {
    type Error = Bandage;
    fn try_from(input: &String) -> Clean<Self> {
        let input = input.as_str();
        match input {
            "Option 1: $27/$88 Public Safety Utility Fee only" => Ok(Self::UtilityFee),
            "Option 2: 7% Food & Beverage Tax only" => Ok(Self::FoodBeverageTax),
            "Option 3: 2% General Sales Tax only" => Ok(Self::GeneralSalesTax),
            "Option 4: Utility Fee/Sales Tax" => Ok(Self::UtilityFeeSalesTax),
            "Option 5: Utility Fee/Food & Beverage Tax" => Ok(Self::UtilityFeeFoodBeverageTax),
            "Option 6: Reduce Police/Fire Staffing" => Ok(Self::ReduceService),
            _ => Err(Bandage::Parse),
        }
    }
}

/// The `JcSurveyRawItem` deserializes input survey data into string values, so that the library
/// can attempt to parse the provided address information into [`PartialAddress`] types.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct JcSurveyRawItem {
    /// The `city` field represents the city of residence for the respondent.
    pub city: Option<String>,
    /// The `address` field is a text field representing the physical address of residence in the
    /// city.
    pub address: Option<String>,
    /// The `option` field represents the selected [`Vote`] option for the respondent, captured as
    /// the text description associated with the selected survey option.
    pub option: Option<String>,
}

/// The `JcSurveyRaw` struct holds a vector of [`JcSurveyRawItem`] objects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JcSurveyRaw {
    /// The `records` field holds a vector of [`JcSurveyRawItem`] objects.
    pub records: Vec<JcSurveyRawItem>,
}

impl JcSurveyRaw {
    /// Creates a new `JcSurveyRaw` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = crate::prelude::from_csv(path)?;
        Ok(JcSurveyRaw { records })
    }
}

/// The `JcSurveyItem` represents a survey response where the submitted address text has been
/// parsed into a [`PartialAddress`] object.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JcSurveyItem {
    /// The `city` field represents the city of residence for the respondent.
    pub city: Option<String>,
    /// The `address` field is the [`PartialAddress`] object parsed from the submitted address text
    /// field in the survey.
    pub address: PartialAddress,
    /// The `option` field represents the selected [`Vote`] option for the respondent, parsed from
    /// the option description in the survey.
    pub option: Vote,
}

impl TryFrom<&JcSurveyRawItem> for JcSurveyItem {
    type Error = Bandage;
    fn try_from(raw: &JcSurveyRawItem) -> Clean<Self> {
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
                        Err(Bandage::Unknown)
                    }
                }
                Err(_) => Err(Bandage::Parse),
            }
        } else {
            Err(Bandage::Unknown)
        }
    }
}

/// The `JcSurvey` struct holds a vector of [`JcSurveyItem`] objects.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct JcSurvey {
    /// The `records` field holds a vector of [`JcSurveyItem`] objects.
    pub records: Vec<JcSurveyItem>,
}

impl JcSurvey {
    /// Creates a new `JcSurvey` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Clean<Self> {
        let records = crate::prelude::from_csv(path)?;
        let records = JcSurveyRaw { records };
        JcSurvey::try_from(&records)
    }

    /// Matches the [`PartialAddress`] in the `address` field of each `JcSurveyItem` in the
    /// `records` field of the `JcSurvey` object against the addresses in `other`.  The method
    /// gathers complete and partial address matches into a [`JcSurveyExport`] struct.
    pub fn validate(&self, other: &Addresses) -> JcSurveyExport {
        let mut records = Vec::new();
        for item in self.records.clone() {
            let res = MatchPartialRecord::compare(&item.address, other);
            let res = res.records();
            if !res.is_empty() && res[0].match_status() != MatchStatus::Missing {
                // if res[0].match_status() != MatchStatus::Missing {
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
                // }
            }
        }
        JcSurveyExport { records }
    }
}

impl TryFrom<&JcSurveyRaw> for JcSurvey {
    type Error = Bandage;
    fn try_from(raw: &JcSurveyRaw) -> Clean<Self> {
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

/// The `JcSurveyExportItem` represents a survey response that has been partially or fully matched to a physical
/// address. The struct includes coordinate data from the matched address to facilitate mapping of
/// survey responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JcSurveyExportItem {
    /// The `address` field contains the address label text string produced by the
    /// [`Address::label()`] method.
    pub address: String,
    /// The `option` field represents the selected [`Vote`] option for the respondent, parsed from
    /// the option description in the survey.
    pub option: Vote,
    /// The `x_coordinate` field represents the x coordinate for the spatial point assigned to the
    /// address.
    pub x_coordinate: f64,
    /// The `y_coordinate` field represents the y coordinate for the spatial point assigned to the
    /// address.
    pub y_coordinate: f64,
}

/// The `JcSurveyExport` struct holds a vector of [`JcSurveyExportItem`] objects.  This struct
/// provides a format for serializing the data into a CSV file.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JcSurveyExport {
    /// The `records` field holds a vector of [`JcSurveyExportItem`] objects.
    pub records: Vec<JcSurveyExportItem>,
}

impl JcSurveyExport {
    /// Write the contents of `JcSurveyExport` to a CSV file at location `title`.  Each element in
    /// the vector of type [`JcSurveyExportItem`] maps to a row of data on the CSV.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        crate::prelude::to_csv(&mut self.records, title)?;
        Ok(())
    }
}
