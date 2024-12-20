use address::{GrantsPassSpatialAddresses, Portable, SpatialAddresses};
use aid::prelude::*;
use clap::Parser;
use spreadsheet::prelude::*;
use spreadsheet::utils;
use tracing::info;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, help = CMD_HELP)]
    command: String,
    #[arg(
        short = 's',
        long,
        help = "Source of file.",
        default_missing_value = "None"
    )]
    source: Option<std::path::PathBuf>,
    #[arg(
        short = 't',
        long,
        help = "Target path to create file.",
        default_missing_value = "None"
    )]
    target: Option<std::path::PathBuf>,
    #[arg(
        short = 'd',
        long,
        help = "Data source.",
        default_missing_value = "None"
    )]
    data: Option<std::path::PathBuf>,
    #[arg(
        short = 'o',
        long,
        help = "Output file path.",
        default_missing_value = "None"
    )]
    out: Option<std::path::PathBuf>,
}

const CMD_HELP: &str = "
Command to execute, including:
* load_bea <PATH> -> Load BEA data stored locally on disk.
* load_parcels <PATH> -> Load taxlots from a CSV.
";

fn main() -> Clean<()> {
    utils::trace_init();
    let cli = Cli::parse();

    match cli.command.as_str() {
        "read_bea" => {
            dotenvy::dotenv().ok();
            let path = std::env::var("BEA_CAINC5N_CSV")?;
            let records = BeaData::from_csv(path)?;
            info!("Records: {}", records.len());
            let hash = records.linecode_hash();
            info!("Hash is {:#?}", hash);
        }
        "load_bea" => {
            dotenvy::dotenv().ok();
            let raw = std::env::var("BEA_CAINC5N_RAW")?;
            info!("Reading raw csv file.");
            let records = BeaDataRaw::from_csv(raw)?;
            info!("Converting raw csv data.");
            let mut records = BeaData::try_from(records)?;
            if let Some(path) = cli.source {
                records.to_csv(path)?;
            }
            info!("Records: {}", records.len());
        }
        "load_parcels" => {
            if let Some(path) = cli.source {
                info!("Importing county taxlots.");
                let records = CountyTaxlots::from_csv(path)?;
                info!("Records: {}", records.len());
                let mail = MailingList::try_from(&records)?;
                info!("Records processed: {}", mail.len());
                let mut mail = MailingListExport::from(&mail);
                mail.sort_by_key("properties");
                let mail: Vec<MailingListExportItem> = mail.iter().rev().cloned().collect();
                let mut mail = MailingListExport::new(mail);
                if let Some(file) = cli.target {
                    mail.to_csv(&file)?;
                    info!("Mailing list output to {}", &file.display());
                }
            }
        }
        "compare" => {
            if let Some(path) = cli.source {
                info!("Importing county taxlots.");
                let records = CountyTaxlots::from_csv(path)?;
                info!("Records: {}", records.len());
                if let Some(target) = cli.target {
                    let addresses = address::GrantsPassSpatialAddresses::from_csv(target)?;
                    let mut matches = records.compare(&addresses)?;
                    info!("Records: {:?}", matches.len());
                    if let Some(out) = cli.out {
                        info!("Writing results to {out:?}.");
                        matches.to_csv(out)?;
                    }
                }
            }
        }
        "jc_survey" => {
            if let Some(path) = cli.source {
                let records = JcSurvey::from_csv(path)?;
                info!("Survey records: {}", records.records.len());
                if let Some(file) = cli.data {
                    let city = GrantsPassSpatialAddresses::from_csv(file)?;
                    let city = SpatialAddresses::from(&city[..]);
                    info!("City addresses: {}", city.len());
                    let mut matches = records.validate(&city);
                    if let Some(target) = cli.target {
                        matches.to_csv(target)?;
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}
