use aid::prelude::*;
use spreadsheet::data::*;
use spreadsheet::prelude::*;
use tracing::{info, trace};

#[test]
fn load_industry_codes() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let file_path = "./tests/test_data/business_categories.csv";
    let records = IndustryCodes::from_csv(file_path)?;
    info!("Records: {:?}", records.records_ref().len());

    Ok(())
}

#[test]
fn write_industry_info() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let file_path = "./tests/test_data/business_categories.csv";
    let records = IndustryCodes::from_csv(file_path)?;
    info!("Records: {:?}", records.records_ref().len());
    let mut industry_info = IndustryInfos::from(&records);
    industry_info.to_csv("./tests/test_data/industry_info.csv".into())?;

    Ok(())
}

#[test]
fn load_businesses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let file_path = "c:/users/erose/documents/businesses_export.csv";
    let records = Businesses::from_csv(file_path)?;
    info!("Records: {:?}", records.records_ref().len());

    Ok(())
}

#[test]
fn load_licenses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let file_path = "./tests/test_data/active_business.csv";
    let records = ActiveLicenses::from_csv(file_path)?;
    info!("Records: {:?}", records.records_ref().len());

    Ok(())
}

#[test]
fn license_code() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let file_path = "./tests/test_data/active_business.csv";
    let records = ActiveLicenses::from_csv(file_path)?;
    info!("Records: {:?}", records.records_ref().len());
    let license = records.records_ref()[0].license();
    let code = records.code(&license);
    info!("Code is: {:?}", code);

    Ok(())
}

#[test]
fn businesses_info() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let file_path = "./tests/test_data/active_business.csv";
    let licenses = ActiveLicenses::from_csv(file_path)?;
    let file_path = "./test_data/business_categories.csv";
    let codes = IndustryCodes::from_csv(file_path)?;
    let file_path = "c:/users/erose/documents/businesses_export.csv";
    let businesses = Businesses::from_csv(file_path)?;
    let mut businesses_info = BusinessesInfo::from_license(&businesses, &licenses, &codes);
    info!("Records: {:?}", businesses_info.records_ref().len());
    businesses_info.to_csv("./test_data/businesses_import.csv".into())?;
    info!("Businesses info written to csv.");

    Ok(())
}

#[test]
fn load_county_taxlots() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    let file_path = "./tests/test_data/county_parcels.csv";
    let lots = CountyTaxlots::from_csv(file_path)?;
    info!("Records: {:?}", lots.records_ref().len());

    Ok(())
}

#[test]
fn convert_raw_bea() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    dotenv::dotenv().ok();
    let raw = std::env::var("BEA_CAINC5N_RAW")?;
    let csv = std::env::var("BEA_CAINC5N_CSV")?;

    let records = BeaDataRaw::from_csv(raw)?;
    let mut records = BeaData::try_from(records)?;
    info!("Records: {:?}", records.records_ref().len());
    records.to_csv(csv)?;
    Ok(())
}

#[test]
fn print_code_keys() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    dotenv::dotenv().ok();
    let csv = std::env::var("BEA_CAINC5N_CSV")?;

    let mut records = BeaData::from_csv(csv)?;
    let mut keys = records
        .records_mut()
        .iter()
        .map(|r| r.code())
        .collect::<Vec<String>>();
    keys.sort();
    keys.dedup();
    let mut wtr = csv::Writer::from_path("c:/users/erose/documents/bea/bea_cainc5n_code_keys.csv")?;
    for key in keys {
        wtr.serialize(key)?;
    }
    wtr.flush()?;
    Ok(())
}
