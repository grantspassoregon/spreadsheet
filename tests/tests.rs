use address::business::BusinessLicenses;
use address::prelude::to_csv;
use aid::prelude::*;
use spreadsheet::data::*;
use spreadsheet::import::beehive;
use spreadsheet::import::utilities::{self, wastewater};
use spreadsheet::prelude::*;
use spreadsheet::utils;
use tracing::info;

#[test]
fn load_industry_codes() -> Result<(), std::io::Error> {
    utils::trace_init();

    let file_path = "./tests/test_data/business_categories.csv";
    let records = IndustryCodes::from_csv(file_path)?;
    info!("Records: {:?}", records.len());
    assert_eq!(records.len(), 370);

    Ok(())
}

#[test]
fn write_industry_info() -> Result<(), std::io::Error> {
    utils::trace_init();

    let file_path = "./tests/test_data/business_categories.csv";
    let records = IndustryCodes::from_csv(file_path)?;
    info!("Records: {:?}", records.len());
    let mut industry_info = IndustryInfos::from(&records);
    industry_info.to_csv("./tests/test_data/industry_info.csv".into())?;

    Ok(())
}

#[test]
fn load_licenses() -> Result<(), std::io::Error> {
    utils::trace_init();

    let file_path = "./tests/test_data/active_business.csv";
    let records = ActiveLicenses::from_csv(file_path)?;
    assert_eq!(records.len(), 2521);
    info!("Records: {:?}", records.len());

    Ok(())
}

#[test]
fn license_code() -> Result<(), std::io::Error> {
    utils::trace_init();

    let file_path = "./tests/test_data/active_business.csv";
    let records = ActiveLicenses::from_csv(file_path)?;
    info!("Records: {:?}", records.len());
    let license = records[0].license();
    let code = records.code(&license);
    assert_eq!(code, 812320);
    info!("Code is: {:?}", code);

    Ok(())
}

// #[test]
// fn business_info_from_matches() -> Result<(), std::io::Error> {
//     if let Ok(()) = tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .try_init()
//     {};
//     trace!("Subscriber initialized.");
//
//     let file = "c:/users/erose/Documents/business_matches.csv";
//     let businesses = BusinessMatchRecords::from_csv(file)?;
//     info!("Match records read.");
//     let file = "tests/test_data/business_categories.csv";
//     let codes = IndustryCodes::from_csv(file)?;
//     info!("Industry codes read.");
//     let mut businesses_info = BusinessesInfo::from_matches(&businesses, &codes);
//     info!("Records: {:?}", businesses_info.len());
//     businesses_info.to_csv("tests/test_data/businesses_export.csv".into())?;
//     info!("Businesses info written to csv.");
//
//     Ok(())
// }

// #[test]
// fn businesses_info() -> Result<(), std::io::Error> {
//     if let Ok(()) = tracing_subscriber::fmt()
//         .with_max_level(tracing::Level::TRACE)
//         .try_init()
//     {};
//     trace!("Subscriber initialized.");
//
//     let file_path = "./tests/test_data/active_business.csv";
//     let licenses = ActiveLicenses::from_csv(file_path)?;
//     let file_path = "./test_data/business_categories.csv";
//     let codes = IndustryCodes::from_csv(file_path)?;
//     let file_path = "c:/users/erose/documents/businesses_export.csv";
//     let businesses = Businesses::from_csv(file_path)?;
//     let mut businesses_info = BusinessesInfo::from_license(&businesses, &licenses, &codes);
//     info!("Records: {:?}", businesses_info.len());
//     businesses_info.to_csv("./test_data/businesses_import.csv".into())?;
//     info!("Businesses info written to csv.");
//
//     Ok(())
// }

#[test]
fn load_county_taxlots() -> Result<(), std::io::Error> {
    utils::trace_init();
    let file_path = "./tests/test_data/county_parcels.csv";
    let lots = CountyTaxlots::from_csv(file_path)?;
    assert_eq!(lots.len(), 5073);
    info!("Records: {:?}", lots.len());

    Ok(())
}

#[test]
fn convert_raw_bea() -> Clean<()> {
    // After running "download" from main in the `bea` crate, the download is in BeaDataRaw format.
    // Convert from raw to BeaData, then export to CSV and BIN.
    utils::trace_init();
    dotenvy::dotenv().ok();
    let raw = std::env::var("BEA_CAINC5N_RAW")?;
    let csv = std::env::var("BEA_CAINC5N_CSV")?;
    let dat = std::env::var("BEA_CAINC5N_DAT")?;

    let records = BeaDataRaw::from_csv(raw)?;
    let mut records = BeaData::try_from(records)?;
    info!("Records: {:?}", records.len());
    records.to_csv(csv)?;
    records.save(dat)?;
    Ok(())
}

#[test]
fn print_code_keys() -> Clean<()> {
    utils::trace_init();
    dotenvy::dotenv().ok();
    let csv = std::env::var("BEA_CAINC5N_CSV")?;

    tracing::info!("Opening csv: {csv}");
    let records = BeaData::from_csv(csv)?;
    let mut keys = records.iter().map(|r| r.code()).collect::<Vec<String>>();
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
    utils::trace_init();
    dotenvy::dotenv().ok();
    let dat = std::env::var("BEA_CAINC5N_DAT")?;
    let records = BeaData::load(dat)?;
    let mut keys = records.iter().map(|r| r.geo_fips).collect::<Vec<i32>>();
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
    utils::trace_init();
    let records = BeaData::try_from(BeaDataRaw::from_csv("tests/test_data/bea.csv")?)?;
    let mut keys = records.iter().map(|r| r.geo_fips).collect::<Vec<i32>>();
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
    utils::trace_init();

    let situs = "c:/users/erose/documents/redwood_business_mailing_list.csv";
    let situs = BusinessesInfo::from_csv(situs)?;
    info!("Businesses read: {}.", situs.len());
    let mailing = "c:/users/erose/documents/business_licenses_mailing_20240530.csv";
    let mailing = BusinessLicenses::from_csv(mailing)?;
    info!("Business licenses loaded: {} entries.", mailing.len());
    let mut mailing = mailing.deduplicate();
    mailing.detype_subaddresses()?;
    info!("Business licenses deduplicated: {} entries.", mailing.len());

    let mut mail = Vec::new();
    let mut missing = Vec::new();
    for site in situs.iter() {
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

#[test]
fn load_wastewater_events_raw() -> Result<(), std::io::Error> {
    utils::trace_init();

    let file_path = "./tests/test_data/wastewater_events_20240808.csv";
    let records = beehive::EventsRaw::from_csv(file_path)?;
    info!("Records: {:?}", records.len());
    let records = beehive::Events::from(records);
    info!("Records: {:?}", records.len());
    // assert_eq!(records.len(), 370);

    Ok(())
}

// Reads wastewater devices from a shapefile.
// From ArcPro, export the wastewater devices point layer as a shapefile.
// Set the projection to EPSG-3587
// This test confirms the shapefile on record loads without issue.
#[test]
fn read_wastewater_device() -> aid::prelude::Clean<()> {
    utils::trace_init();
    let device =
        wastewater::device::Devices::from_shp_z("c:/users/erose/shapefiles/wastewater_device.shp")?;
    tracing::info!("Devices found: {}", device.len());
    tracing::info!("Devices found: {:#?}", device[0]);
    Ok(())
}

#[test]
fn read_wastewater_line() -> aid::prelude::Clean<()> {
    utils::trace_init();
    let lines =
        wastewater::line::Lines::from_shp_z("c:/users/erose/shapefiles/wastewater_line.shp")?;
    tracing::info!("Lines found: {}", lines.len());
    tracing::info!("Line inspection: {:#?}", lines[0]);
    Ok(())
}

#[test]
fn read_wastewater_junction() -> aid::prelude::Clean<()> {
    utils::trace_init();
    let junctions = wastewater::junction::Junctions::from_shp_z(
        "c:/users/erose/shapefiles/wastewater_junction.shp",
    )?;
    tracing::info!("Junctions found: {}", junctions.len());
    tracing::info!("Junction inspection: {:#?}", junctions[0]);
    Ok(())
}

#[test]
fn read_wastewater_events() -> aid::prelude::Clean<()> {
    utils::trace_init();

    let file_path = "./tests/test_data/wastewater_events_20240808.csv";
    let records = beehive::EventsRaw::from_csv(file_path)?;
    let records = beehive::Events::from(records);
    info!("Records: {:?}", records.len());
    let device =
        wastewater::device::Devices::from_shp_z("c:/users/erose/shapefiles/wastewater_device.shp")?;
    tracing::info!("Devices found: {}", device.len());
    if let Some(device_events) = records.from_devices(&device) {
        tracing::info!("Device Events found: {}", device_events.len());
    }
    let lines =
        wastewater::line::Lines::from_shp_z("c:/users/erose/shapefiles/wastewater_line.shp")?;
    tracing::info!("Lines found: {}", lines.len());
    if let Some(line_events) = records.from_lines(&lines) {
        tracing::info!("Line Events found: {}", line_events.len());
    }
    let junctions = wastewater::junction::Junctions::from_shp_z(
        "c:/users/erose/shapefiles/wastewater_junction.shp",
    )?;
    tracing::info!("Junctions found: {}", junctions.len());
    if let Some(junction_events) = records.from_junctions(&junctions) {
        tracing::info!("Junction Events found: {}", junction_events.len());
    }
    Ok(())
}

#[test]
// Load beehive events from a csv file.
// Export wastewater devices to .shp from ArcPro.
// Set projection to EPSG-3587.
// Convert to Events layers and export to geojson.
// Open geojson in QGIS and export to shapefile.
// Open in ArcPro and force the correct projection.
fn write_wastewater_events() -> aid::prelude::Clean<()> {
    utils::trace_init();

    let file_path = "./tests/test_data/wastewater_events_20240808.csv";
    let records = beehive::EventsRaw::from_csv(file_path)?;
    let records = beehive::Events::from(records);
    info!("Records: {:?}", records.len());
    let device =
        wastewater::device::Devices::from_shp_z("c:/users/erose/shapefiles/wastewater_device.shp")?;
    tracing::info!("Devices found: {}", device.len());
    if let Some(device_events) = records.from_devices(&device) {
        tracing::info!("Device Events found: {}", device_events.len());
        let path = "C:/users/erose/geojson/device_events.geojson";
        device_events.geojson(path)?;
    }
    let lines =
        wastewater::line::Lines::from_shp_z("c:/users/erose/shapefiles/wastewater_line.shp")?;
    tracing::info!("Lines found: {}", lines.len());
    if let Some(line_events) = records.from_lines(&lines) {
        tracing::info!("Line Events found: {}", line_events.len());
        let path = "C:/users/erose/geojson/line_events.geojson";
        line_events.geojson(path)?;
    }
    let junctions = wastewater::junction::Junctions::from_shp_z(
        "c:/users/erose/shapefiles/wastewater_junction.shp",
    )?;
    tracing::info!("Junctions found: {}", junctions.len());
    if let Some(junction_events) = records.from_junctions(&junctions) {
        tracing::info!("Junction Events found: {}", junction_events.len());
        let path = "C:/users/erose/geojson/junction_events.geojson";
        junction_events.geojson(path)?;
    }

    Ok(())
}

#[test]
fn read_manhole_cards() -> aid::prelude::Clean<()> {
    utils::trace_init();
    let card = wastewater::manhole_card::ManholeCard::read_dir("o:/beehive/collection/ssmh")?;
    tracing::info!("Manhole cards: {:?}", card.len());
    let device =
        wastewater::device::Devices::from_shp_z("c:/users/erose/shapefiles/wastewater_device.shp")?;
    tracing::info!("Devices found: {}", device.len());
    let cards = wastewater::manhole_card::ManholeCards::from_devices(
        &device,
        &card[..],
        "o:/beehive/collection/ssmh".into(),
    );
    tracing::info!("Cards found: {}", cards.len());
    let target = "c:/users/erose/geojson/manhole_cards.geojson";
    cards.geojson(target)?;
    let orphans = cards.orphans(&card[..]);
    tracing::info!("Orphans: {}", orphans.len());
    Ok(())
}

#[test]
fn read_cctv_inspections() -> aid::prelude::Clean<()> {
    utils::trace_init();
    let names = utilities::cctv::InspectionFiles::from_path("O:\\Beehive\\COLLECTION\\MJ'S WORK IN PROGRESS\\00 TV REPORTS\\PDF TRANSITIONS\\ADDED TO BEEHIVE")?;
    // let names = utilities::cctv::InspectionFiles::from_path("o:/beehive/collection/mj's work in progress/00 tv reports/pdf transitions/added to beehive")?;
    let lines =
        wastewater::line::Lines::from_shp_z("c:/users/erose/shapefiles/wastewater_line.shp")?;
    tracing::info!("Lines found: {}", lines.len());
    let reports = utilities::cctv::Inspections::from_files(&names, &lines)?;
    tracing::info!("CCTV Reports: {:?}", reports.len());
    let out = "c:/users/erose/geojson/cctv_reports.geojson";
    reports.geojson(out)?;

    Ok(())
}

#[test]
fn load_wastewater_event_cctv_reports() -> aid::prelude::Clean<()> {
    utils::trace_init();

    let file_path = "./tests/test_data/wastewater_events_20240808.csv";
    let records = beehive::EventsRaw::from_csv(file_path)?;
    info!("Records: {:?}", records.len());
    let records = beehive::Events::from(records);
    info!("Records: {:?}", records.len());
    let names = utilities::cctv::InspectionFiles::from_path("O:\\Beehive\\COLLECTION\\MJ'S WORK IN PROGRESS\\00 TV REPORTS\\PDF TRANSITIONS\\ADDED TO BEEHIVE")?;
    let lines =
        wastewater::line::Lines::from_shp_z("c:/users/erose/shapefiles/wastewater_line.shp")?;
    tracing::info!("Lines found: {}", lines.len());
    let reports = utilities::cctv::Inspections::from_files(&names, &lines)?;
    tracing::info!("CCTV Reports: {:?}", reports.len());
    let records = records.build_cctv_reports(&reports);
    tracing::info!("Records after adding reports: {}", records.len());
    let mut cctv = records.clone();
    cctv.retain(|v| v.cctv().is_some());
    tracing::info!("Records with cctv attached {}", cctv.len());

    Ok(())
}

#[test]
// Load beehive events from a csv file.
// Export wastewater devices to .shp from ArcPro.
// Set projection to EPSG-3587.
// Convert to Events layers and export to geojson.
// Open geojson in QGIS and export to shapefile.
// Open in ArcPro and force the correct projection.
fn write_wastewater_events_with_reports() -> aid::prelude::Clean<()> {
    utils::trace_init();

    // Load devices, lines and junctions
    let file_path = "./tests/test_data/wastewater_events_20240808.csv";
    let events = beehive::EventsRaw::from_csv(file_path)?;
    let mut events = beehive::Events::from(events);
    info!("Records: {:?}", events.len());
    let device =
        wastewater::device::Devices::from_shp_z("c:/users/erose/shapefiles/wastewater_device.shp")?;
    tracing::info!("Devices found: {}", device.len());
    let lines =
        wastewater::line::Lines::from_shp_z("c:/users/erose/shapefiles/wastewater_line.shp")?;
    tracing::info!("Lines found: {}", lines.len());
    let junctions = wastewater::junction::Junctions::from_shp_z(
        "c:/users/erose/shapefiles/wastewater_junction.shp",
    )?;
    tracing::info!("Junctions found: {}", junctions.len());

    // Load manhole cards.
    let card = wastewater::manhole_card::ManholeCard::read_dir("o:/beehive/collection/ssmh")?;
    tracing::info!("Manhole cards: {:?}", card.len());
    let cards = wastewater::manhole_card::ManholeCards::from_devices(
        &device,
        &card[..],
        "o:/beehive/collection/ssmh".into(),
    );
    tracing::info!("Cards found: {}", cards.len());
    // Connect manhole cards to events.
    events.add_manhole_cards(&cards);

    // Load cctv reports.
    let names = utilities::cctv::InspectionFiles::from_path("O:\\Beehive\\COLLECTION\\MJ'S WORK IN PROGRESS\\00 TV REPORTS\\PDF TRANSITIONS\\ADDED TO BEEHIVE")?;
    let reports = utilities::cctv::Inspections::from_files(&names, &lines)?;
    tracing::info!("CCTV Reports: {:?}", reports.len());

    // Connect cctv reports to events.
    let events = events.build_cctv_reports(&reports);
    tracing::info!("Records after adding reports: {}", events.len());

    // Write events to shapefiles.
    if let Some(device_events) = events.from_devices(&device) {
        tracing::info!("Device Events found: {}", device_events.len());
        let path = "C:/users/erose/geojson/device_events.geojson";
        device_events.geojson(path)?;
    }
    if let Some(line_events) = events.from_lines(&lines) {
        tracing::info!("Line Events found: {}", line_events.len());
        let path = "C:/users/erose/geojson/line_events.geojson";
        line_events.geojson(path)?;
    }
    if let Some(junction_events) = events.from_junctions(&junctions) {
        tracing::info!("Junction Events found: {}", junction_events.len());
        let path = "C:/users/erose/geojson/junction_events.geojson";
        junction_events.geojson(path)?;
    }

    Ok(())
}
