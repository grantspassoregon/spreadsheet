use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct CountyTaxlot {
    account: String,
    #[serde(rename = "ACCTSTATUS")]
    account_status: String,
    acreage: Option<f64>,
    addr1: String,
    addr2: Option<String>,
    addr3: Option<String>,
    address: String,
    #[serde(rename = "APPR_VALUE")]
    appraised_value: Option<f64>,
    #[serde(rename = "ASSD_VALUE")]
    assessed_value: Option<f64>,
    #[serde(rename = "BEDRMS")]
    bedrooms: Option<f64>,
    #[serde(rename = "BLDG_CLASS")]
    building_class: Option<String>,
    block: Option<String>,
    #[serde(rename = "City")]
    city: String,
    code: String,
    comp_mtl: Option<String>,
    csz: String,
    deed_type: String,
    #[serde(rename = "GIS_Acres")]
    gis_acres: f64,
    imp_value: Option<f64>,
    instrument_number: Option<String>,
    #[serde(rename = "LAND_APPR")]
    land_appraised: Option<f64>,
    #[serde(rename = "LAND_MKT")]
    land_market: Option<f64>,
    legal_acre: Option<f64>,
    living_area: Option<f64>,
    location_description: Option<String>,
    lot: Option<String>,
    #[serde(rename = "Lot_1")]
    lot_1: Option<String>,
    maint: String,
    #[serde(rename = "MAPNUM")]
    map_number: String,
    mnx: String,
    #[serde(rename = "NAME")]
    owner_name: String,
    #[serde(rename = "NBHD")]
    neighborhood: String,
    #[serde(rename = "PROP_CLASS")]
    property_class: i32,
    qq: String,
    #[serde(rename = "RMV")]
    retail_market_value: Option<f64>,
    #[serde(rename = "RNG")]
    range: i32,
    sale_date: Option<String>,
    sale_price: Option<f64>,
    sale_type: Option<String>,
    #[serde(rename = "SD")]
    school_district: String,
    #[serde(rename = "SEC")]
    section: i32,
    situs: String,
    situs_city: String,
    situs_pref: Option<String>,
    #[serde(rename = "SITUS_ST")]
    situs_state: String,
    situs_suf0: Option<String>,
    #[serde(rename = "SITUS_SUFF")]
    situs_street_name_suffix: String,
    situs_zip: i32,
    sptb_codes: Option<String>,
    #[serde(rename = "SQ_FT")]
    square_feet: Option<f64>,
    #[serde(rename = "ST_NAME")]
    situs_street_name: String,
    #[serde(rename = "ST_NO")]
    situs_address_number: String,
    #[serde(rename = "State")]
    mailing_state: String,
    #[serde(rename = "Taxes")]
    taxes: Option<f64>,
    #[serde(rename = "TWN")]
    town: i32,
    #[serde(rename = "TYPE")]
    lot_type: String,
    #[serde(rename = "YR_BLT")]
    year_built: Option<f64>,
    #[serde(rename = "ZIP")]
    mailing_zip: String,
    #[serde(rename = "Zone")]
    zone: String,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CountyTaxlots {
    pub records: Vec<CountyTaxlot>,
}

impl CountyTaxlots {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: CountyTaxlot = result?;
            data.push(record);
        }

        Ok(CountyTaxlots { records: data })
    }
}
