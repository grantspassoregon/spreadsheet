use crate::utils;
use address::{Address, Addresses, MatchPartialRecords};
use aid::prelude::Clean;
use derive_more::{Deref, DerefMut};
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use tracing::warn;

/// The `CountyTaxlot` struct holds data associated with Josephine County tax parcels.
#[derive(
    Debug, Clone, Default, Serialize, Deserialize, derive_getters::Getters, derive_setters::Setters,
)]
#[setters(prefix = "with_")]
#[serde(rename_all = "UPPERCASE")]
pub struct CountyTaxlot {
    #[setters(doc = "Sets the value of the `account` field.")]
    account: String,
    #[serde(rename = "ACCTSTATUS")]
    #[setters(doc = "Sets the value of the `account_status` field.")]
    account_status: String,
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
    #[serde(rename = "City")]
    #[setters(doc = "Sets the value of the `city` field.")]
    city: String,
    #[setters(doc = "Sets the value of the `code` field.")]
    code: String,
    #[setters(doc = "Sets the value of the `comp_mtl` field.")]
    comp_mtl: Option<String>,
    #[setters(doc = "Sets the value of the `csz` field.")]
    csz: String,
    #[setters(doc = "Sets the value of the `deed_type` field.")]
    deed_type: String,
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
    #[setters(doc = "Sets the value of the `legal_acre` field.")]
    legal_acre: Option<f64>,
    #[setters(doc = "Sets the value of the `living_area` field.")]
    living_area: Option<f64>,
    #[setters(doc = "Sets the value of the `location_description` field.")]
    location_description: Option<String>,
    #[setters(doc = "Sets the value of the `lot` field.")]
    lot: Option<String>,
    #[serde(rename = "Lot_1")]
    #[setters(doc = "Sets the value of the `lot_1` field.")]
    lot_1: Option<String>,
    #[setters(doc = "Sets the value of the `maint` field.")]
    maint: String,
    #[serde(rename = "MapNum")]
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
    #[setters(doc = "Sets the value of the `qq` field.")]
    qq: String,
    #[serde(rename = "RMV")]
    #[setters(doc = "Sets the value of the `retail_market_value` field.")]
    retail_market_value: Option<f64>,
    #[serde(rename = "RNG")]
    #[setters(doc = "Sets the value of the `range` field.")]
    range: i32,
    #[setters(doc = "Sets the value of the `sale_date` field.")]
    sale_date: Option<String>,
    #[setters(doc = "Sets the value of the `sale_price` field.")]
    sale_price: Option<f64>,
    #[setters(doc = "Sets the value of the `sale_type` field.")]
    sale_type: Option<String>,
    #[serde(rename = "SD")]
    #[setters(doc = "Sets the value of the `school_district` field.")]
    school_district: String,
    #[serde(rename = "SEC")]
    #[setters(doc = "Sets the value of the `section` field.")]
    section: i32,
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
    #[serde(rename = "SITUS_SUFF")]
    #[setters(doc = "Sets the value of the `situs_street_name_suffix` field.")]
    situs_street_name_suffix: String,
    #[setters(doc = "Sets the value of the `situs_zip` field.")]
    situs_zip: i32,
    #[setters(doc = "Sets the value of the `sptb_codes` field.")]
    sptb_codes: Option<String>,
    #[serde(rename = "SQ_FT")]
    #[setters(doc = "Sets the value of the `square_feet` field.")]
    square_feet: Option<f64>,
    #[serde(rename = "ST_NAME")]
    #[setters(doc = "Sets the value of the `situs_street_name` field.")]
    situs_street_name: String,
    #[serde(rename = "ST_NO")]
    #[setters(doc = "Sets the value of the `situs_address_number` field.")]
    situs_address_number: String,
    #[serde(rename = "State")]
    #[setters(doc = "Sets the value of the `mailing_state` field.")]
    mailing_state: String,
    #[serde(rename = "Taxes")]
    #[setters(doc = "Sets the value of the `taxes` field.")]
    taxes: Option<f64>,
    #[serde(rename = "TWN")]
    #[setters(doc = "Sets the value of the `town` field.")]
    town: i32,
    #[serde(rename = "TYPE")]
    #[setters(doc = "Sets the value of the `lot_type` field.")]
    lot_type: String,
    #[serde(rename = "YR_BLT")]
    #[setters(doc = "Sets the value of the `year_built` field.")]
    year_built: Option<f64>,
    #[serde(rename = "ZIP")]
    #[setters(doc = "Sets the value of the `mailing_zip` field.")]
    mailing_zip: String,
    #[serde(rename = "Zone")]
    #[setters(doc = "Sets the value of the `zone` field.")]
    zone: String,
}

impl CountyTaxlot {
    /// Compare a taxlot situs address against an address record.
    pub fn compare<
        T: Address + Clone + Send + Sync + galileo::galileo_types::geo::GeoPoint<Num = f64>,
        U: Addresses<T>,
    >(
        &self,
        addresses: &U,
    ) -> Clean<MatchPartialRecords> {
        let (_, address) = address::Parser::address(self.situs())?;
        let matches = address::MatchPartialRecord::compare(&address, addresses);
        Ok(matches)
    }
}

/// The `CountyTaxlots` struct holds a `records` field that contains a vector of type
/// [`CountyTaxlot`].
#[derive(Debug, Clone, Default, Serialize, Deserialize, Deref, DerefMut)]
pub struct CountyTaxlots(Vec<CountyTaxlot>);

impl CountyTaxlots {
    /// Writes the contents of [`CountyTaxlots`] to a CSV file at the location specified in `path`.
    /// Each element in the vector of type [`CountyTaxlot`] maps to a row on the spreadsheet.
    pub fn from_csv<P: AsRef<std::path::Path>>(path: P) -> Result<Self, std::io::Error> {
        let records = utils::from_csv(path)?;
        Ok(CountyTaxlots(records))
    }

    /// The `addresses()` method returns the `address` field from each element of [`CountyTaxlot`]
    /// collected into a vector of type `String`.
    pub fn addresses(&self) -> Vec<String> {
        self.par_iter()
            .map(|v| v.address())
            .cloned()
            .collect::<Vec<String>>()
    }

    /// The `owner_names()` method returns the `owner_name` field from each element of [`CountyTaxlot`]
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

    /// Compare taxlot situs addresses against an address record.
    pub fn compare<
        T: Address + Clone + Send + Sync + galileo::galileo_types::geo::GeoPoint<Num = f64>,
        U: Addresses<T>,
    >(
        &self,
        addresses: &U,
    ) -> Clean<MatchPartialRecords> {
        let mut results = Vec::new();
        self.iter()
            .map(|v| match v.compare(addresses) {
                Ok(records) => results.append(&mut records.to_vec()),
                Err(e) => warn!("No partial match obtained: {}", e.to_string()),
            })
            .for_each(drop);
        // for lot in self.iter() {
        //     let check = lot.compare(addresses)?;
        //     for record in check.iter() {
        //         results.push(record.clone());
        //     }
        // }
        Ok(MatchPartialRecords::new(results))
    }
}
