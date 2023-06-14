use serde::{Deserialize, Serialize};
use tracing::{error, info};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct IndustryCode {
    #[serde(rename = "CODENUMBER")]
    code: i32,
    name: String,
    description: String,
}

impl IndustryCode {
    pub fn sector_code(&self) -> i32 {
        let mut code_vec = Vec::new();
        self.code.to_string().chars().take(2).map(|d| code_vec.push(d.to_digit(10).unwrap())).for_each(drop);
        let code = code_vec[0] * 10 + code_vec[1];
        code as i32
    }
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
        match sector {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }
    pub fn subsector_code(&self) -> i32 {
        let mut code_vec = Vec::new();
        self.code.to_string().chars().take(4).map(|d| code_vec.push(d.to_digit(10).unwrap())).for_each(drop);
        let code = code_vec[0] * 1000 + code_vec[1] * 100 + code_vec[2] * 10 + code_vec[3];
        code as i32
    }
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
            7114 => Some("Agents and Managers for Artists, Athletes, Entertainers and Other Public Figures"),
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
        match subsector {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }
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
        match tourism {
            Some(value) => Some(value.to_string()),
            None => None,
        }
    }

    pub fn from_code(code: i32, industry_codes: &IndustryCodes) -> Self {
        let industry = industry_codes.records.iter().cloned().filter(|r| r.code == code).collect::<Vec<IndustryCode>>();
        industry[0].clone()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct IndustryCodes {
    pub records: Vec<IndustryCode>,
}

impl IndustryCodes {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: IndustryCode = result?;
            data.push(record);
        }

        Ok(IndustryCodes { records: data })

    }
}

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
    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.records.clone() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Business {
    company_name: String,
    contact_name: Option<String>,
    business_type: String,
    dba: Option<String>,
    license: String,
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


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Businesses {
    pub records: Vec<Business>,
}

impl Businesses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: Business = result?;
            data.push(record);
        }

        Ok(Businesses { records: data })
    }
}

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
    pub fn from_license(business: &Business, licenses: &ActiveLicenses, codes: &IndustryCodes) -> Self {
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
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BusinessesInfo {
    pub records: Vec<BusinessInfo>,
}

impl BusinessesInfo {
    pub fn from_license(businesses: &Businesses, licenses: &ActiveLicenses, codes: &IndustryCodes) -> Self {
        let records = businesses.records.iter().map(|r| BusinessInfo::from_license(r, licenses, codes)).collect::<Vec<BusinessInfo>>();
        BusinessesInfo { records }
    }

    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: BusinessInfo = result?;
            data.push(record);
        }

        Ok(BusinessesInfo { records: data })
    }

    pub fn to_csv(&mut self, title: std::path::PathBuf) -> Result<(), std::io::Error> {
        let mut wtr = csv::Writer::from_path(title)?;
        for i in self.records.clone() {
            wtr.serialize(i)?;
        }
        wtr.flush()?;
        Ok(())
    }
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveLicense {
    #[serde(rename = "CodeNumber")]
    pub industry_code: i32,
    #[serde(rename = "LICENSENUMBER")]
    pub license: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ActiveLicenses {
    pub records: Vec<ActiveLicense>,
}


impl ActiveLicenses {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: ActiveLicense = result?;
            data.push(record);
        }

        Ok(ActiveLicenses { records: data })
    }

    pub fn code(&self, license: &str) -> i32 {
        let result = self.records.iter().cloned().filter(|r| r.license == license).collect::<Vec<ActiveLicense>>();
        if result.is_empty() {
            error!("Could not process license {:?}", license);
        }
        result[0].industry_code
    }
}

