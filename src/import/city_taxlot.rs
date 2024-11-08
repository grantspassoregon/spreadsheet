//! The `city_taxlot` submodule contains data structures associated with the city version of the
//! county tax parcel GIS layer.
use crate::utils;
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::warn;

/// The `CityTaxlot` struct holds fields from the city version of the county tax parcel files.
#[derive(
    Debug, Clone, Default, Serialize, Deserialize, derive_getters::Getters, derive_setters::Setters,
)]
#[setters(prefix = "with_")]
#[serde(rename_all = "UPPERCASE")]
pub struct CityTaxlot {
    #[setters(doc = "Sets the value of the `account` field.")]
    account: String,
    #[setters(doc = "Sets the value of the `acreage` field.")]
    acreage: Option<f64>,
    #[setters(doc = "Sets the value of the `addr1` field.")]
    addr1: String,
    #[setters(doc = "Sets the value of the `addr2` field.")]
    addr2: Option<String>,
    #[setters(doc = "Sets the value of the `addr3` field.")]
    addr3: Option<String>,
    #[setters(doc = "Sets the value of the `address` field.")]
    address: String,
    #[serde(rename = "APPR_VALUE")]
    #[setters(doc = "Sets the value of the `appraised_value` field.")]
    appraised_value: Option<f64>,
    #[serde(rename = "ASSD_VALUE")]
    #[setters(doc = "Sets the value of the `assessed_value` field.")]
    assessed_value: Option<f64>,
    #[serde(rename = "BEDRMS")]
    #[setters(doc = "Sets the value of the `bedrooms` field.")]
    bedrooms: Option<f64>,
    #[serde(rename = "BLDG_CLASS")]
    #[setters(doc = "Sets the value of the `building_class` field.")]
    building_class: Option<String>,
    #[setters(doc = "Sets the value of the `block` field.")]
    block: Option<String>,
    #[setters(doc = "Sets the value of the `code` field.")]
    code: String,
    #[setters(doc = "Sets the value of the `comp_mtl` field.")]
    comp_mtl: Option<String>,
    #[setters(doc = "Sets the value of the `csz` field.")]
    csz: String,
    #[setters(doc = "Sets the value of the `deed_type` field.")]
    deed_type: String,
    #[serde(rename = "Doc_Link")]
    #[setters(doc = "Sets the value of the `doc_link` field.")]
    doc_link: String,
    #[serde(rename = "GIS_Acres")]
    #[setters(doc = "Sets the value of the `gis_acres` field.")]
    gis_acres: f64,
    #[setters(doc = "Sets the value of the `imp_value` field.")]
    imp_value: Option<f64>,
    #[setters(doc = "Sets the value of the `instrument_number` field.")]
    instrument_number: Option<String>,
    #[serde(rename = "LAND_APPR")]
    #[setters(doc = "Sets the value of the `land_appraised` field.")]
    land_appraised: Option<f64>,
    #[serde(rename = "LAND_MKT")]
    #[setters(doc = "Sets the value of the `land_market` field.")]
    land_market: Option<f64>,
    #[serde(rename = "Latitude")]
    #[setters(doc = "Sets the value of the `latitude` field.")]
    latitude: f64,
    #[setters(doc = "Sets the value of the `legal_acre` field.")]
    legal_acre: Option<f64>,
    #[setters(doc = "Sets the value of the `living_area` field.")]
    living_area: Option<f64>,
    #[setters(doc = "Sets the value of the `location_description` field.")]
    location_description: Option<String>,
    #[serde(rename = "Longitude")]
    #[setters(doc = "Sets the value of the `longitude` field.")]
    longitude: f64,
    #[setters(doc = "Sets the value of the `lot` field.")]
    lot: Option<String>,
    #[serde(rename = "Lot_1")]
    #[setters(doc = "Sets the value of the `lot_1` field.")]
    lot_1: Option<String>,
    #[setters(doc = "Sets the value of the `maint` field.")]
    maint: String,
    #[serde(rename = "MAPNUM")]
    #[setters(doc = "Sets the value of the `map_number` field.")]
    map_number: String,
    #[setters(doc = "Sets the value of the `mnx` field.")]
    mnx: String,
    #[serde(rename = "NAME")]
    #[setters(doc = "Sets the value of the `owner_name` field.")]
    owner_name: String,
    #[serde(rename = "NBHD")]
    #[setters(doc = "Sets the value of the `neighborhood` field.")]
    neighborhood: String,
    #[serde(rename = "PROP_CLASS")]
    #[setters(doc = "Sets the value of the `property_class` field.")]
    property_class: i32,
    #[serde(rename = "RMV")]
    #[setters(doc = "Sets the value of the `retail_market_value` field.")]
    retail_market_value: Option<f64>,
    #[setters(doc = "Sets the value of the `sale_date` field.")]
    sale_date: Option<String>,
    #[setters(doc = "Sets the value of the `sale_price` field.")]
    sale_price: Option<f64>,
    #[setters(doc = "Sets the value of the `sale_type` field.")]
    sale_type: Option<String>,
    #[serde(rename = "SD")]
    #[setters(doc = "Sets the value of the `school_district` field.")]
    school_district: String,
    #[setters(doc = "Sets the value of the `situs` field.")]
    situs: String,
    #[setters(doc = "Sets the value of the `situs_city` field.")]
    situs_city: String,
    #[setters(doc = "Sets the value of the `situs_pref` field.")]
    situs_pref: Option<String>,
    #[serde(rename = "SITUS_ST")]
    #[setters(doc = "Sets the value of the `situs_state` field.")]
    situs_state: String,
    #[setters(doc = "Sets the value of the `situs_suf0` field.")]
    situs_suf0: Option<String>,
    #[setters(doc = "Sets the value of the `situs_zip` field.")]
    situs_zip: i32,
    #[setters(doc = "Sets the value of the `sptb_codes` field.")]
    sptb_codes: Option<String>,
    #[serde(rename = "SQ_FT")]
    #[setters(doc = "Sets the value of the `square_feet` field.")]
    square_feet: Option<f64>,
    #[serde(rename = "Taxes")]
    #[setters(doc = "Sets the value of the `taxes` field.")]
    taxes: Option<f64>,
    #[serde(rename = "TYPE")]
    #[setters(doc = "Sets the value of the `lot_type` field.")]
    lot_type: String,
    #[serde(rename = "YR_BLT")]
    #[setters(doc = "Sets the value of the `year_built` field.")]
    year_built: Option<f64>,
    #[serde(rename = "Zone")]
    #[setters(doc = "Sets the value of the `zone` field.")]
    zone: String,
}

/// The `CityTaxlots` struct contains a `records` field that holds a vector of type [`CityTaxlot`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct CityTaxlots(Vec<CityTaxlot>);

impl CityTaxlots {
    /// Creates a new `CityTaxlots` struct from a CSV file located at `path`.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(CityTaxlots(records))
    }

    /// The `addresses()` method returns the `address` field from each element of [`CityTaxlot`]
    /// collected into a vector of type `String`.
    pub fn addresses(&self) -> Vec<String> {
        self.par_iter()
            .map(|v| v.address())
            .cloned()
            .collect::<Vec<String>>()
    }

    /// The `owner_names()` method returns the `owner_name` field from each element of [`CityTaxlot`]
    /// collected into a vector of type `String`.
    pub fn owner_names(&self) -> Vec<String> {
        self.par_iter()
            .map(|v| v.owner_name())
            .cloned()
            .collect::<Vec<String>>()
    }

    /// Returns a vector of unique addresses associated with a given property owner `name`.
    pub fn associated_addresses(&self, name: &str) -> Vec<String> {
        let mut res = self
            .par_iter()
            .filter(|v| v.owner_name() == name)
            .map(|v| v.address())
            .cloned()
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
            .par_iter()
            .filter(|v| v.address() == address)
            .map(|v| v.owner_name())
            .cloned()
            .collect::<Vec<String>>();
        res.sort();
        res.dedup();
        res
    }
}
