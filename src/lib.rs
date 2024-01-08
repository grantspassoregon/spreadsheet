#![warn(missing_docs)]
#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://www.grantspassoregon.gov/DocumentCenter/View/31368/GPLogo_450W-PNG"
)]
#![doc(html_playground_url = "https://play.rust-lang.org/")]
pub mod data;
pub mod import;
pub mod utils;

/// The `prelude` module contains exports intended for user convenience.
pub mod prelude {
    pub use crate::data::{
        ActiveLicense, ActiveLicenses, Business, BusinessInfo, Businesses, BusinessesInfo,
        MailingList, MailingListExport, MailingListExportItem, MailingListItem,
    };
    pub use crate::import::{
        BeaData, BeaDataRaw, BeaDatum, BeaDatumRaw, CityTaxlot, CityTaxlots, CountyTaxlot,
        CountyTaxlots, JcSurvey,
    };
    pub use crate::utils::{from_csv, to_csv};
}
