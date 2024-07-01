use address::business::BusinessLicenses;
use address::business::BusinessMatchRecords;
use address::prelude::to_csv;
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
fn business_info_from_matches() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let file = "c:/users/erose/Documents/business_matches.csv";
    let businesses = BusinessMatchRecords::from_csv(file)?;
    info!("Match records read.");
    let file = "tests/test_data/business_categories.csv";
    let codes = IndustryCodes::from_csv(file)?;
    info!("Industry codes read.");
    let mut businesses_info = BusinessesInfo::from_matches(&businesses, &codes);
    info!("Records: {:?}", businesses_info.records_ref().len());
    businesses_info.to_csv("tests/test_data/businesses_export.csv".into())?;
    info!("Businesses info written to csv.");

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
    let dat = std::env::var("BEA_CAINC5N_DAT")?;

    let records = BeaDataRaw::from_csv(raw)?;
    let mut records = BeaData::try_from(records)?;
    info!("Records: {:?}", records.records_ref().len());
    records.to_csv(csv)?;
    records.save(dat)?;
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

#[test]
fn print_fips_tree() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    dotenv::dotenv().ok();
    let dat = std::env::var("BEA_CAINC5N_DAT")?;
    let mut records = BeaData::load(dat)?;
    let mut keys = records
        .records_mut()
        .iter()
        .map(|r| r.geo_fips)
        .collect::<Vec<i32>>();
    keys.sort();
    keys.dedup();
    let mut wtr = csv::Writer::from_path("tests/test_data/bea_fips.csv")?;
    for key in keys {
        wtr.serialize(key)?;
    }
    wtr.flush()?;
    Ok(())
}

#[test]
fn print_fips_tree2() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    let mut records = BeaData::try_from(BeaDataRaw::from_csv("tests/test_data/bea.csv")?)?;
    let mut keys = records
        .records_mut()
        .iter()
        .map(|r| r.geo_fips)
        .collect::<Vec<i32>>();
    keys.sort();
    keys.dedup();
    let mut wtr = csv::Writer::from_path("tests/test_data/bea_fips.csv")?;
    for key in keys {
        wtr.serialize(key)?;
    }
    wtr.flush()?;
    Ok(())
}

#[test]
fn business_mailing() -> Clean<()> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    trace!("Subscriber initialized.");

    let situs = "c:/users/erose/documents/redwood_business_mailing_list.csv";
    let situs = BusinessesInfo::from_csv(situs)?;
    info!("Businesses read: {}.", situs.records_ref().len());
    let mailing = "c:/users/erose/documents/business_licenses_mailing_20240530.csv";
    let mailing = BusinessLicenses::from_csv(mailing)?;
    info!("Business licenses loaded: {} entries.", mailing.len());
    let mut mailing = mailing.deduplicate();
    mailing.detype_subaddresses()?;
    info!("Business licenses deduplicated: {} entries.", mailing.len());

    let mut mail = Vec::new();
    let mut missing = Vec::new();
    for site in situs.records_ref() {
        let matching = mailing.clone().filter("license", site.license());
        if !matching.is_empty() {
            mail.push(matching[0].clone());
        } else {
            missing.push(site);
        }
    }
    tracing::info!("Mailing list: {} records", mail.len());
    to_csv(
        &mut mail,
        "c:/users/erose/documents/business_mailing_20240530.csv".into(),
    )?;
    tracing::info!("Missing: {} records", missing.len());
    to_csv(
        &mut missing,
        "c:/users/erose/documents/business_mailing_missing_20240530.csv".into(),
    )?;

    Ok(())
}
