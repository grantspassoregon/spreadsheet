use spreadsheet::data::*;
use tracing::info;

#[test]
fn load_industry_codes() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file_path = "c:/users/erose/documents/business_categories.csv";
    let records = IndustryCodes::from_csv(file_path)?;
    info!("Records: {:?}", records.records.len());

    Ok(())
}

#[test]
fn write_industry_info() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file_path = "c:/users/erose/documents/business_categories.csv";
    let records = IndustryCodes::from_csv(file_path)?;
    info!("Records: {:?}", records.records.len());
    let mut industry_info = IndustryInfos::from(&records);
    industry_info.to_csv("c:/users/erose/documents/industry_info.csv".into())?;

    Ok(())
}

#[test]
fn load_businesses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file_path = "c:/users/erose/documents/businesses_export.csv";
    let records = Businesses::from_csv(file_path)?;
    info!("Records: {:?}", records.records.len());

    Ok(())
}

#[test]
fn load_licenses() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file_path = "c:/users/erose/documents/active_business.csv";
    let records = ActiveLicenses::from_csv(file_path)?;
    info!("Records: {:?}", records.records.len());

    Ok(())
}

#[test]
fn license_code() -> Result<(), std::io::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::TRACE)
        .try_init()
    {};
    info!("Subscriber initialized.");

    let file_path = "c:/users/erose/documents/active_business.csv";
    let records = ActiveLicenses::from_csv(file_path)?;
    info!("Records: {:?}", records.records.len());
    let license = records.records[0].license.clone();
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
    info!("Subscriber initialized.");

    let file_path = "c:/users/erose/documents/active_business.csv";
    let licenses = ActiveLicenses::from_csv(file_path)?;
    let file_path = "c:/users/erose/documents/business_categories.csv";
    let codes = IndustryCodes::from_csv(file_path)?;
    let file_path = "c:/users/erose/documents/businesses_export.csv";
    let businesses = Businesses::from_csv(file_path)?;
    let mut businesses_info = BusinessesInfo::from_license(&businesses, &licenses, &codes);
    info!("Records: {:?}", businesses_info.records.len());
    businesses_info.to_csv("c:/users/erose/documents/businesses_import.csv".into());
    info!("Businesses info written to csv.");

    Ok(())
}
