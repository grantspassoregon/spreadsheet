//! The `city_taxlot` submodule contains data structures associated with the city version of the
//! county tax parcel GIS layer.
use crate::utils;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::warn;

/// The `CityTaxlot` struct holds fields from the city version of the county tax parcel files.
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
    /// The `address` field represents the mailing address of a property owner.  This method
    /// returns the cloned value of the field.
    pub fn address(&self) -> String {
        self.address.clone()
    }

    /// This method returns a reference to the value of the `address` field.
    pub fn address_ref(&self) -> &String {
        &self.address
    }

    /// The `csz` field represents the city, state and zip of the property situs address.  This
    /// method returns the cloned value of the field.
    pub fn csz(&self) -> String {
        self.csz.clone()
    }

    /// The `owner_name` field represents the name of the property owner.  This method returns the
    /// cloned value of the field.
    pub fn owner_name(&self) -> String {
        self.owner_name.clone()
    }

    /// The `map_number` field represents the map taxlot number of the parcel.  The `parcel()`
    /// method returns the cloned value of the `map_number` field.
    pub fn parcel(&self) -> String {
        self.map_number.clone()
    }

    /// The `situs` field represents the situs address of the property.  This method returns the
    /// cloned value of the field.
    pub fn situs(&self) -> String {
        self.situs.clone()
    }

    /// This method returns a reference to the `situs` field.
    pub fn situs_ref(&self) -> &String {
        &self.situs
    }
}

/// The `CityTaxlots` struct contains a `records` field that holds a vector of type [`CityTaxlot`].
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct CityTaxlots {
    records: Vec<CityTaxlot>,
}

impl CityTaxlots {
    /// Creates a new `CityTaxlots` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(CityTaxlots { records })
    }

    /// The `records` field contains a vector of type [`CityTaxlot`].  This method returns the
    /// cloned value of the field.
    pub fn records(&self) -> Vec<CityTaxlot> {
        self.records.clone()
    }

    /// This method returns a reference to the `records` field.
    pub fn records_ref(&self) -> &Vec<CityTaxlot> {
        &self.records
    }

    /// This method returns a mutable reference to the `records` field.
    pub fn records_mut(&mut self) -> &mut Vec<CityTaxlot> {
        &mut self.records
    }

    /// The `addresses()` method returns the `address` field from each element of [`CityTaxlot`]
    /// collected into a vector of type `String`.
    pub fn addresses(&self) -> Vec<String> {
        self.records_ref()
            .par_iter()
            .map(|v| v.address())
            .collect::<Vec<String>>()
    }

    /// The `owner_names()` method returns the `owner_name` field from each element of [`CityTaxlot`]
    /// collected into a vector of type `String`.
    pub fn owner_names(&self) -> Vec<String> {
        self.records_ref()
            .par_iter()
            .map(|v| v.owner_name())
            .collect::<Vec<String>>()
    }

    /// Returns a vector of unique addresses associated with a given property owner `name`.
    pub fn associated_addresses(&self, name: &str) -> Vec<String> {
        let mut res = self
            .records_ref()
            .par_iter()
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

    /// Returns a vector of unique owner names associated with a given address `address`.
    pub fn associated_names(&self, address: &str) -> Vec<String> {
        let mut res = self
            .records_ref()
            .par_iter()
            .filter(|v| v.address() == address)
            .map(|v| v.owner_name())
            .collect::<Vec<String>>();
        res.sort();
        res.dedup();
        res
    }
}
