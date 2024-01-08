//! The `data` module holds generic data structures for processing imported data.
use crate::prelude::*;
use aid::prelude::*;
use indicatif::ProgressBar;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;
use tracing::{error, info};

/// The `IndustryCode` struct represents the NAICS Industry Code associated with a business.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct IndustryCode {
    #[serde(rename = "CODENUMBER")]
    code: i32,
    name: String,
    description: String,
}

impl IndustryCode {
    /// The NAICS sector code is the first two digits of the Industry Code.  This function
    /// returns the sector code for an [`IndustryCode`].
    pub fn sector_code(&self) -> i32 {
        let mut code_vec = Vec::new();
        self.code
            .to_string()
            .chars()
            .take(2)
            .map(|d| code_vec.push(d.to_digit(10).unwrap()))
            .for_each(drop);
        let code = code_vec[0] * 10 + code_vec[1];
        code as i32
    }

    /// The `sector` function matches the NAICS sector code with a string description of the sector.
    pub fn sector(&self) -> Option<String> {
        let code = self.sector_code();
        let sector = match code {
            11 => Some("Agriculture, Forestry, Fishing and Hunting"),
            21 => Some("Mining"),
            22 => Some("Utilities"),
            23 => Some("Construction"),
            31..=33 => Some("Manufacturing"),
            42 => Some("Wholesale Trade"),
            44..=45 => Some("Retail Trade"),
            48..=49 => Some("Transportation and Warehousing"),
            51 => Some("Information"),
            52 => Some("Finance and Insurance"),
            53 => Some("Real Estate Rental and Leasing"),
            54 => Some("Professional, Scientific and Technical Services"),
            55 => Some("Management of Companies and Enterprises"),
            56 => Some("Administrative, Support and Waste Services"),
            61 => Some("Educational Services"),
            62 => Some("Health Care and Social Assistance"),
            71 => Some("Arts, Entertainment and Recreation"),
            72 => Some("Accommodation and Food Services"),
            81 => Some("Other Services (except Public Administration)"),
            92 => Some("Public Administration"),
            _ => None,
        };
        // match sector {
        //     Some(value) => Some(value.to_string()),
        //     None => None,
        // }
        sector.map(|value| value.to_string())
    }

    /// The NAICS subsector code is the first four digits of the Industry Code.  This function
    /// returns the subsector code for an [`IndustryCode`].
    pub fn subsector_code(&self) -> i32 {
        let mut code_vec = Vec::new();
        self.code
            .to_string()
            .chars()
            .take(4)
            .map(|d| code_vec.push(d.to_digit(10).unwrap()))
            .for_each(drop);
        let code = code_vec[0] * 1000 + code_vec[1] * 100 + code_vec[2] * 10 + code_vec[3];
        code as i32
    }

    /// The `subsector` function matches the NAICS subsector code with a string description of the subsector.
    pub fn subsector(&self) -> Option<String> {
        let code = self.subsector_code();
        let subsector = match code {
            4411 => Some("Automobile Dealers"),
            4412 => Some("Other Motor Vehicle Dealers"),
            4413 => Some("Automotive Parts, Accessories and Tire Stores"),
            4421 => Some("Furniture Stores"),
            4422 => Some("Home Furnishings Stores"),
            4431 => Some("Electronics and Appliance Stores"),
            4441 => Some("Building Material and Supplies Dealers"),
            4442 => Some("Lawn and Garden Equipment and Supplies Stores"),
            4451 => Some("Grocery Stores"),
            4452 => Some("Specialty Food Stores"),
            4453 => Some("Beer, Wine and Liquor Stores"),
            4461 => Some("Health and Personal Care Stores"),
            4471 => Some("Gasoline Stations"),
            4481 => Some("Clothing Stores"),
            4482 => Some("Shoe Stores"),
            4483 => Some("Jewelry, Luggage and Leather Goods Stores"),
            4511 => Some("Sporting Goods, Hobby and Musical Instrument Stores"),
            4512 => Some("Book Stores and News Dealers"),
            4521 => Some("Department Stores"),
            4529 => Some("Other General Merchandise Stores"),
            4531 => Some("Florists"),
            4532 => Some("Office Supplies, Stationary and Gift Stores"),
            4533 => Some("Used Merchandise Stores"),
            4539 => Some("Other Miscellaneous Store Retailers"),
            4541 => Some("Electronic Shopping and Mail-Order Houses"),
            4542 => Some("Vending Machine Operators"),
            4543 => Some("Direct Selling Establishments"),
            7111 => Some("Performing Arts Companies"),
            7112 => Some("Spectator Sports"),
            7113 => Some("Promoters of Performing Arts, Sports and Similar Events"),
            7114 => Some(
                "Agents and Managers for Artists, Athletes, Entertainers and Other Public Figures",
            ),
            7115 => Some("Independent Artists, Writers and Performers"),
            7121 => Some("Museums, Historical Sites and Similar Institutions"),
            7131 => Some("Amusement Parks and Arcades"),
            7132 => Some("Gambling Industries"),
            7139 => Some("Other Amusement and Recreation Industries"),
            7211 => Some("Traveler Accommodation"),
            7212 => Some("RV (Recreational Vehicle) Parks and Recreational Camps"),
            7213 => Some("Rooming and Boarding Houses"),
            7223 => Some("Special Food Services"),
            7224 => Some("Drinking Places (Alcoholic Beverages)"),
            7225 => Some("Restaurants and Other Eating Places"),
            _ => None,
        };
        // match subsector {
        //     Some(value) => Some(value.to_string()),
        //     None => None,
        // }
        subsector.map(|value| value.to_string())
    }

    /// The `tourism` function matches a subsector code to a string description of the tourism
    /// category associated with the subsector.  The categories generally describe the areas of
    /// interest for tourism, used for symbolizing business locations on the web viewer, and
    /// enhancing search.
    pub fn tourism(&self) -> Option<String> {
        let code = self.subsector_code();
        let tourism = match code {
            4411 => Some("Shopping"),
            4412 => Some("Shopping"),
            4413 => Some("Shopping"),
            4421 => Some("Shopping"),
            4422 => Some("Shopping"),
            4431 => Some("Shopping"),
            4442 => Some("Shopping"),
            4451 => Some("Grocery Stores"),
            4452 => Some("Grocery Stores"),
            4453 => Some("Beer, Wine & Liquor Stores"),
            4461 => Some("Shopping"),
            4471 => Some("Gas"),
            4481 => Some("Clothing & Accessories"),
            4482 => Some("Clothing & Accessories"),
            4483 => Some("Clothing & Accessories"),
            4511 => Some("Shopping"),
            4512 => Some("Shopping"),
            4521 => Some("Shopping"),
            4529 => Some("Shopping"),
            4531 => Some("Shopping"),
            4532 => Some("Shopping"),
            4533 => Some("Shopping"),
            4539 => Some("Shopping"),
            4541 => Some("Shopping"),
            4542 => Some("Shopping"),
            4543 => Some("Shopping"),
            7111 => Some("Entertainment"),
            7112 => Some("Entertainment"),
            7113 => Some("Entertainment"),
            7115 => Some("Entertainment"),
            7121 => Some("Entertainment"),
            7131 => Some("Entertainment"),
            7132 => Some("Entertainment"),
            7139 => Some("Entertainment"),
            7211 => Some("Accommodation"),
            7212 => Some("Accommodation"),
            7213 => Some("Accommodation"),
            7223 => Some("Food & Drink"),
            7224 => Some("Cocktails, Wine & Beer"),
            7225 => Some("Food & Drink"),
            _ => None,
        };
        let tourism = match self.code {
            312120 => Some("Cocktails, Wine & Beer"),
            312130 => Some("Cocktails, Wine & Beer"),
            451110 => Some("Firearms"),
            522110 => Some("Bank"),
            522130 => Some("Bank"),
            721191 => Some("Bed & Breakfast Inn"),
            722515 => Some("Coffee & Snacks"),
            812111 => Some("Salon"),
            812112 => Some("Salon"),
            812113 => Some("Salon"),
            _ => tourism,
        };
        // match tourism {
        //     Some(value) => Some(value.to_string()),
        //     None => None,
        // }
        tourism.map(|value| value.to_string())
    }

    /// The `from_code()` method creates an `IndustryCode` stuct by matching an industry code `code` against a known list of codes
    /// `industry_codes`.
    pub fn from_code(code: i32, industry_codes: &IndustryCodes) -> Self {
        let industry = industry_codes
            .records_ref()
            .iter()
            .filter(|r| r.code == code)
            .take(1)
            .cloned()
            .collect::<Vec<IndustryCode>>();
        industry[0].clone()
    }
}

/// The `IndustryCodes` struct holds a `records` field that contains a vector of type
/// [`IndustryCode`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndustryCodes {
    records: Vec<IndustryCode>,
}

impl IndustryCodes {
    /// The `records` field contains a vector of type [`IndustryCode`].  This function returns a
    /// reference to the vector.
    pub fn records_ref(&self) -> &Vec<IndustryCode> {
        &self.records
    }

    /// Read the contents of a CSV file at location `path` into an `IndustryCodes` struct.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(IndustryCodes { records })
    }
}

/// The `IndustryInfo` struct stores codes, names and descriptions for the NAICS Industry, Sector
/// and Subsector designations for a business.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndustryInfo {
    industry_code: i32,
    industry_name: String,
    industry_description: String,
    sector_code: i32,
    sector_name: String,
    subsector_code: i32,
    subsector_name: String,
    tourism: String,
}

impl From<&IndustryCode> for IndustryInfo {
    fn from(industry: &IndustryCode) -> Self {
        let industry_code = industry.code;
        let industry_name = industry.name.clone();
        let industry_description = industry.description.clone();
        let sector_code = industry.sector_code();
        let sector_name = match industry.sector() {
            Some(name) => name,
            None => "Unknown".to_string(),
        };
        let subsector_code = industry.subsector_code();
        let subsector_name = match industry.subsector() {
            Some(name) => name,
            None => "Unknown".to_string(),
        };
        let tourism = match industry.tourism() {
            Some(name) => name,
            None => "Unknown".to_string(),
        };
        IndustryInfo {
            industry_code,
            industry_name,
            industry_description,
            sector_code,
            sector_name,
            subsector_code,
            subsector_name,
            tourism,
        }
    }
}

/// The `IndustryInfos` struct holds a `records` field that contains a vector of type
/// [`IndustryInfo`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndustryInfos {
    records: Vec<IndustryInfo>,
}

impl From<&IndustryCodes> for IndustryInfos {
    fn from(industries: &IndustryCodes) -> Self {
        let mut records = Vec::new();
        for record in &industries.records {
            records.push(IndustryInfo::from(record));
        }
        IndustryInfos { records }
    }
}

impl IndustryInfos {
    /// Write the contents of `IndustryInfos` to a CSV file at location `title`.  Each element in
    /// the vector of type [`IndustryInfo`] maps to a row of data on the CSV.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(self.records_mut(), title)?;
        Ok(())
    }

    /// The `records` field contains a vector of type [`IndustryInfo`].  This function returns a
    /// mutable reference to the field.
    pub fn records_mut(&mut self) -> &mut Vec<IndustryInfo> {
        &mut self.records
    }
}

/// The `Business` struct represents a business license for the City of Grants Pass.
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Business {
    company_name: String,
    contact_name: Option<String>,
    business_type: String,
    dba: Option<String>,
    license: String,
    #[serde(rename = "CODENUMBER")]
    code: i32,
    notes: Option<String>,
    created_user: Option<String>,
    created_date: Option<String>,
    last_edited_user: Option<String>,
    last_edited_date: Option<String>,
    #[serde(rename = "FULLADDRESS")]
    street_address_label: String,
    x_coordinate: f64,
    y_coordinate: f64,
}

impl Business {
    /// The `code` field represents the NAICS industry code for the business.  This method returns
    /// the value of the field.
    pub fn code(&self) -> i32 {
        self.code
    }
}

/// The `Businesses` struct contains a `records` field holding a vector of type [`Business`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Businesses {
    records: Vec<Business>,
}

impl Businesses {
    /// Read the contents of a CSV file at location `path` into an `Businesses` struct.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(Businesses { records })
    }

    /// The `records` field contains a vector of type [`Business`].  This function returns a
    /// reference to the vector.
    pub fn records_ref(&self) -> &Vec<Business> {
        &self.records
    }
}

/// The `BusinessInfo` struct aggregates spatial business information with license info.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessInfo {
    company_name: String,
    contact_name: Option<String>,
    dba: Option<String>,
    street_address_label: String,
    license: String,
    industry_code: i32,
    industry_name: String,
    sector_code: i32,
    sector_name: Option<String>,
    subsector_code: i32,
    subsector_name: Option<String>,
    tourism: Option<String>,
    notes: Option<String>,
    created_user: Option<String>,
    created_date: Option<String>,
    last_edited_user: Option<String>,
    last_edited_date: Option<String>,
    x_coordinate: f64,
    y_coordinate: f64,
}

impl BusinessInfo {
    /// Creates a new `BusinessInfo` from a [`Business`] struct, an [`ActiveLicenses`] struct, and
    /// an [`IndustryCodes`] struct.
    pub fn from_license(
        business: &Business,
        licenses: &ActiveLicenses,
        codes: &IndustryCodes,
    ) -> Self {
        let company_name = business.company_name.clone();
        let contact_name = business.contact_name.clone();
        let dba = business.dba.clone();
        let street_address_label = business.street_address_label.clone();
        let license = business.license.clone();
        let industry_code = licenses.code(&business.license);
        let industry = IndustryCode::from_code(industry_code, codes);
        let industry_name = industry.name.clone();
        let sector_code = industry.sector_code();
        let sector_name = industry.sector().clone();
        let subsector_code = industry.subsector_code();
        let subsector_name = industry.subsector();
        let tourism = industry.tourism();
        let notes = business.notes.clone();
        let created_user = business.created_user.clone();
        let created_date = business.created_date.clone();
        let last_edited_user = business.last_edited_user.clone();
        let last_edited_date = business.last_edited_date.clone();
        let x_coordinate = business.x_coordinate;
        let y_coordinate = business.y_coordinate;
        BusinessInfo {
            company_name,
            contact_name,
            dba,
            street_address_label,
            license,
            industry_code,
            industry_name,
            sector_code,
            sector_name,
            subsector_code,
            subsector_name,
            tourism,
            notes,
            created_user,
            created_date,
            last_edited_user,
            last_edited_date,
            x_coordinate,
            y_coordinate,
        }
    }

    /// Creates a new `BusinessInfo` from a [`Business`] struct and
    /// an [`IndustryCodes`] struct.
    pub fn from_codes(business: &Business, codes: &IndustryCodes) -> Self {
        let company_name = business.company_name.clone();
        let contact_name = business.contact_name.clone();
        let dba = business.dba.clone();
        let street_address_label = business.street_address_label.clone();
        let license = business.license.clone();
        let industry_code = business.code();
        let industry = IndustryCode::from_code(industry_code, codes);
        let industry_name = industry.name.clone();
        let sector_code = industry.sector_code();
        let sector_name = industry.sector().clone();
        let subsector_code = industry.subsector_code();
        let subsector_name = industry.subsector();
        let tourism = industry.tourism();
        let notes = business.notes.clone();
        let created_user = business.created_user.clone();
        let created_date = business.created_date.clone();
        let last_edited_user = business.last_edited_user.clone();
        let last_edited_date = business.last_edited_date.clone();
        let x_coordinate = business.x_coordinate;
        let y_coordinate = business.y_coordinate;
        BusinessInfo {
            company_name,
            contact_name,
            dba,
            street_address_label,
            license,
            industry_code,
            industry_name,
            sector_code,
            sector_name,
            subsector_code,
            subsector_name,
            tourism,
            notes,
            created_user,
            created_date,
            last_edited_user,
            last_edited_date,
            x_coordinate,
            y_coordinate,
        }
    }
}

/// The `BusinessesInfo` struct contains a `records` field that holds a vector of type
/// [`BusinessInfo`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessesInfo {
    records: Vec<BusinessInfo>,
}

impl BusinessesInfo {
    /// Creates a new `BusinessesInfo` from a [`Businesses`] struct, an [`ActiveLicenses`] struct, and
    /// an [`IndustryCodes`] struct.
    pub fn from_license(
        businesses: &Businesses,
        licenses: &ActiveLicenses,
        codes: &IndustryCodes,
    ) -> Self {
        let records = businesses
            .records
            .iter()
            .map(|r| BusinessInfo::from_license(r, licenses, codes))
            .collect::<Vec<BusinessInfo>>();
        BusinessesInfo { records }
    }

    /// Read the contents of a CSV file at location `path` into an `BusinessesInfo` struct.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(BusinessesInfo { records })
    }

    /// Write the contents of `BusinessesInfo` to a CSV file at location `title`.  Each element in
    /// the vector of type [`BusinessInfo`] maps to a row of data on the CSV.
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        to_csv(self.records_mut(), title)?;
        Ok(())
    }

    /// The `records` field contains a vector of type [`BusinessInfo`].  This function returns a
    /// reference to the vector.
    pub fn records_ref(&self) -> &Vec<BusinessInfo> {
        &self.records
    }

    /// This function returns a mutable reference to the vector of type [`BusinessInfo`] in the
    /// `records` field.
    pub fn records_mut(&mut self) -> &mut Vec<BusinessInfo> {
        &mut self.records
    }
}

/// Deprecated.  Industry code has been added to the [`Business`] struct.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveLicense {
    #[serde(rename = "CodeNumber")]
    industry_code: i32,
    #[serde(rename = "LICENSENUMBER")]
    license: String,
}

impl ActiveLicense {
    /// Returns the cloned value of the `license` field.
    pub fn license(&self) -> String {
        self.license.clone()
    }

    /// Returns a reference to the `license` field.
    pub fn license_ref(&self) -> &String {
        &self.license
    }
}

/// Deprecated.  Industry code has been added to the [`Business`] struct.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveLicenses {
    records: Vec<ActiveLicense>,
}

impl ActiveLicenses {
    /// The `records` field contains a vector of type [`ActiveLicense`].  This function returns a
    /// reference to the vector.
    pub fn records_ref(&self) -> &Vec<ActiveLicense> {
        &self.records
    }

    /// Read the contents of a CSV file at location `path` into an `ActiveLicenses` struct.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = from_csv(path)?;
        Ok(ActiveLicenses { records })
    }

    /// Returns the industry code for a business given a license number.
    pub fn code(&self, license: &str) -> i32 {
        let result = self
            .records_ref()
            .iter()
            .filter(|r| r.license == license)
            .cloned()
            .collect::<Vec<ActiveLicense>>();
        if result.is_empty() {
            error!("Could not process license {:?}", license);
        }
        result[0].industry_code
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct FlatList(Vec<String>);

impl FlatList {
    pub fn new(list: Vec<String>) -> Self {
        FlatList(list)
    }
}

impl fmt::Display for FlatList {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut flat = "".to_string();
        for (i, item) in self.0.iter().enumerate() {
            if i > 0 {
                flat.push_str(", ");
            }
            flat.push_str(item);
        }
        write!(f, "{}", flat)
    }
}

/// The `MailingListItem` struct holds the imported mailing data from a tax parcel.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailingListItem {
    name: String,
    properties: usize,
    situs_addresses: FlatList,
    mailing_address: FlatList,
    mailing_csz: FlatList,
    associated_names: FlatList,
    parcels: FlatList,
}

impl MailingListItem {
    /// Creates a new `MailingListItem` from an [`import::CityTaxlots`] struct.
    pub fn from_city_parcels(
        name: &str,
        parcels: &CityTaxlots,
        done: &mut std::collections::HashSet<String>,
    ) -> Clean<Self> {
        done.insert(name.to_string());
        let addr = parcels.associated_addresses(name);
        if !addr.is_empty() {
            let mut csz = parcels
                .records_ref()
                .iter()
                .filter(|v| addr.contains(v.address_ref()))
                .map(|v| v.csz())
                .collect::<Vec<String>>();
            csz.sort();
            csz.dedup();
            let mailing = &addr[0];
            let names = parcels.associated_names(mailing);
            for i in &names {
                if !done.contains(i) {
                    done.insert(i.to_string());
                }
            }
            let names = names
                .par_iter()
                .filter(|v| *v != name)
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
            let situs = parcels
                .records_ref()
                .par_iter()
                .filter(|v| v.address_ref() == mailing)
                .map(|v| v.situs())
                .collect::<Vec<String>>();
            let tax_parcels = parcels
                .records_ref()
                .par_iter()
                .filter(|v| situs.contains(v.situs_ref()))
                .map(|v| v.parcel())
                .collect::<Vec<String>>();
            Ok(MailingListItem {
                name: name.to_string(),
                properties: situs.len(),
                situs_addresses: FlatList::new(situs),
                mailing_address: FlatList::new(addr),
                mailing_csz: FlatList::new(csz),
                associated_names: FlatList::new(names),
                parcels: FlatList::new(tax_parcels),
            })
        } else {
            Err(Bandage::Unknown)
        }
    }

    /// Creates a new `MailingListItem` from an [`import::CountyTaxlots`] struct.
    pub fn from_county_parcels(
        name: &str,
        parcels: &CountyTaxlots,
        done: &mut std::collections::HashSet<String>,
    ) -> Clean<Self> {
        done.insert(name.to_string());
        let addr = parcels.associated_addresses(name);
        if !addr.is_empty() {
            let mut csz = parcels
                .records_ref()
                .iter()
                .filter(|v| addr.contains(v.address_ref()))
                .map(|v| v.csz())
                .collect::<Vec<String>>();
            csz.sort();
            csz.dedup();
            let mailing = &addr[0];
            let names = parcels.associated_names(mailing);
            for i in &names {
                if !done.contains(i) {
                    done.insert(i.to_string());
                }
            }
            let names = names
                .par_iter()
                .filter(|v| *v != name)
                .map(|v| v.to_string())
                .collect::<Vec<String>>();
            let situs = parcels
                .records_ref()
                .par_iter()
                .filter(|v| v.address_ref() == mailing)
                .map(|v| v.situs())
                .collect::<Vec<String>>();
            let tax_parcels = parcels
                .records_ref()
                .par_iter()
                .filter(|v| situs.contains(v.situs_ref()))
                .map(|v| v.parcel())
                .collect::<Vec<String>>();
            Ok(MailingListItem {
                name: name.to_string(),
                properties: situs.len(),
                situs_addresses: FlatList::new(situs),
                mailing_address: FlatList::new(addr),
                mailing_csz: FlatList::new(csz),
                associated_names: FlatList::new(names),
                parcels: FlatList::new(tax_parcels),
            })
        } else {
            Err(Bandage::Unknown)
        }
    }

}

/// The `MailingList` struct contains a `records` field that holds a vector of type
/// [`MailingListItem`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailingList {
    records: Vec<MailingListItem>,
}

impl MailingList {
    /// The `records` field contains a vector of type [`MailingListItem`].  This function returns a
    /// reference to the vector.
    pub fn records_ref(&self) -> &Vec<MailingListItem> {
        &self.records
    }
}

impl TryFrom<&CityTaxlots> for MailingList {
    type Error = Bandage;
    fn try_from(parcels: &CityTaxlots) -> Clean<Self> {
        info!("Importing from city parcels.");
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Importing city parcels...'}",
        )
        .unwrap();
        let mut names = parcels.owner_names();
        names.sort();
        names.dedup();
        let bar = ProgressBar::new(names.len() as u64);
        bar.set_style(style);
        let mut records = Vec::new();
        let mut done = std::collections::HashSet::new();
        for name in names {
            if !done.contains(&name) {
                records.push(MailingListItem::from_city_parcels(
                    &name, parcels, &mut done,
                )?);
            }
            bar.inc(1);
        }
        Ok(MailingList { records })
    }
}

impl TryFrom<&CountyTaxlots> for MailingList {
    type Error = Bandage;
    fn try_from(parcels: &CountyTaxlots) -> Clean<Self> {
        info!("Importing from county parcels.");
        let style = indicatif::ProgressStyle::with_template(
            "[{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {'Importing county parcels...'}",
        )
        .unwrap();
        let mut names = parcels.owner_names();
        names.sort();
        names.dedup();
        let bar = ProgressBar::new(names.len() as u64);
        bar.set_style(style);
        let mut records = Vec::new();
        let mut done = std::collections::HashSet::new();
        for name in names {
            if !done.contains(&name) {
                records.push(MailingListItem::from_county_parcels(
                    &name, parcels, &mut done,
                )?);
            }
            bar.inc(1);
        }
        Ok(MailingList { records })
    }
}

/// The `MailingListExportItem` struct holds mailing list data in export format.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailingListExportItem {
    name: String,
    properties: usize,
    situs_addresses: String,
    mailing_address: String,
    mailing_csz: String,
    associated_names: String,
    parcels: String,
}

impl From<&MailingListItem> for MailingListExportItem {
    fn from(item: &MailingListItem) -> Self {
        MailingListExportItem {
            name: item.name.clone(),
            properties: item.properties,
            situs_addresses: format!("{}", item.situs_addresses),
            mailing_address: format!("{}", item.mailing_address),
            mailing_csz: format!("{}", item.mailing_csz),
            associated_names: format!("{}", item.associated_names),
            parcels: format!("{}", item.parcels),
        }
    }
}

/// The `MailingListExport` struct contains a `records` field that holds a vector of type
/// [`MailingListExportItem`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MailingListExport {
    records: Vec<MailingListExportItem>,
}

impl MailingListExport {
    /// Creates a new `MailingListExport` struct from a `records` vector of type
    /// [`MailingListExportItem`]
    pub fn new(records: Vec<MailingListExportItem>) -> Self {
        MailingListExport { records }
    }

    /// Write the contents of `MailingListExport` to a CSV file at location `title`.  Each element in
    /// the vector of type [`MailingListExportItem`] maps to a row of data on the CSV.
    pub fn to_csv<P: AsRef<std::path::Path>>(&mut self, path: P) -> Clean<()> {
        to_csv(self.records_mut(), path)?;
        Ok(())
    }

    /// The `records` field contains a vector of type [`MailingListExportItem`].  This function returns a
    /// reference to the vector.
    pub fn records_ref(&self) -> &Vec<MailingListExportItem> {
        &self.records
    }

    /// This function returns a mutable reference to the vector of type [`MailingListExportItem`] in the
    /// `records` field.
    pub fn records_mut(&mut self) -> &mut Vec<MailingListExportItem> {
        &mut self.records
    }

    /// Sorts elements of `records` by `key`.  The `key` parameter takes the values "properties"
    /// and "name".
    pub fn sort_by_key(&mut self, key: &str) {
        match key {
            "properties" => self.records.sort_by_key(|v| v.properties),
            "name" => self.records.sort_by_key(|v| v.name.clone()),
            _ => {}
        }
    }
}

impl From<&MailingList> for MailingListExport {
    fn from(items: &MailingList) -> Self {
        let records = items
            .records_ref()
            .iter()
            .map(MailingListExportItem::from)
            .collect::<Vec<MailingListExportItem>>();
        MailingListExport { records }
    }
}
