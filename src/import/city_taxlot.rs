use serde::{Deserialize, Serialize};
use tracing::warn;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub struct CityTaxlot {
    account: String,
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
    code: String,
    comp_mtl: Option<String>,
    csz: String,
    deed_type: String,
    #[serde(rename = "Doc_Link")]
    doc_link: String,
    #[serde(rename = "GIS_Acres")]
    gis_acres: f64,
    imp_value: Option<f64>,
    instrument_number: Option<String>,
    #[serde(rename = "LAND_APPR")]
    land_appraised: Option<f64>,
    #[serde(rename = "LAND_MKT")]
    land_market: Option<f64>,
    #[serde(rename = "Latitude")]
    latitude: f64,
    legal_acre: Option<f64>,
    living_area: Option<f64>,
    location_description: Option<String>,
    #[serde(rename = "Longitude")]
    longitude: f64,
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
    #[serde(rename = "RMV")]
    retail_market_value: Option<f64>,
    sale_date: Option<String>,
    sale_price: Option<f64>,
    sale_type: Option<String>,
    #[serde(rename = "SD")]
    school_district: String,
    #[serde(rename = "Shape_Area")]
    shape_area: f64,
    #[serde(rename = "Shape_Length")]
    shape_length: f64,
    situs: String,
    situs_city: String,
    situs_pref: Option<String>,
    #[serde(rename = "SITUS_ST")]
    situs_state: String,
    situs_suf0: Option<String>,
    situs_zip: i32,
    sptb_codes: Option<String>,
    #[serde(rename = "SQ_FT")]
    square_feet: Option<f64>,
    #[serde(rename = "Taxes")]
    taxes: Option<f64>,
    #[serde(rename = "TYPE")]
    lot_type: String,
    #[serde(rename = "YR_BLT")]
    year_built: Option<f64>,
    #[serde(rename = "Zone")]
    zone: String,
}

impl CityTaxlot {
    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn owner_name(&self) -> String {
        self.owner_name.clone()
    }

    pub fn situs(&self) -> String {
        self.situs.clone()
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CityTaxlots {
    records: Vec<CityTaxlot>,
}

impl CityTaxlots {
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let mut data = Vec::new();
        let file = std::fs::File::open(path)?;
        let mut rdr = csv::Reader::from_reader(file);

        for result in rdr.deserialize() {
            let record: CityTaxlot = result?;
            data.push(record);
        }

        Ok(CityTaxlots { records: data })
    }

    pub fn records(&self) -> Vec<CityTaxlot> {
        self.records.clone()
    }

    pub fn records_ref(&self) -> &Vec<CityTaxlot> {
        &self.records
    }

    pub fn records_mut(&mut self) -> &mut Vec<CityTaxlot> {
        &mut self.records
    }

    pub fn addresses(&self) -> Vec<String> {
        self.records()
            .iter()
            .map(|v| v.address())
            .collect::<Vec<String>>()
    }

    pub fn owner_names(&self) -> Vec<String> {
        self.records()
            .iter()
            .map(|v| v.owner_name())
            .collect::<Vec<String>>()
    }

    pub fn associated_addresses(&self, name: &str) -> Vec<String> {
        let mut res = self
            .records()
            .iter()
            .filter(|v| v.owner_name() == name)
            .map(|v| v.address())
            .collect::<Vec<String>>();
        res.sort();
        res.dedup();
        if res.len() > 1 {
            warn!("{} has {} associated addresses.", name, res.len());
        }
        res
    }

    pub fn associated_names(&self, address: &str) -> Vec<String> {
        let mut res = self
            .records()
            .iter()
            .filter(|v| v.address() == address)
            .map(|v| v.owner_name())
            .collect::<Vec<String>>();
        res.sort();
        res.dedup();
        res
    }
}
