use clap::Parser;
use spreadsheet::{data, error, import};
use tracing::{info, trace};

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
    source: Option<String>,
}

const CMD_HELP: &str = "
Command to execute, including:
* load_bea <PATH> -> Load BEA data stored locally on disk.
* load_parcels <PATH> -> Load taxlots from a CSV.
";

fn main() -> Result<(), error::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    let cli = Cli::parse();

    match cli.command.as_str() {
        "read_bea" => {
            dotenv::dotenv().ok();
            let path = std::env::var("BEA_CAINC5N_CSV")?;
            let records = import::BeaData::from_csv(path)?;
            info!("Records: {}", records.records_ref().len());
            let hash = records.linecode_hash();
            info!("Hash is {:#?}", hash);
        }
        "load_bea" => {
            dotenv::dotenv().ok();
            let raw = std::env::var("BEA_CAINC5N_RAW")?;
            info!("Reading raw csv file.");
            let records = import::BeaDataRaw::from_csv(raw)?;
            info!("Converting raw csv data.");
            let mut records = import::BeaData::try_from(records)?;
            if let Some(path) = cli.source {
                records.to_csv(path)?;
            }
            info!("Records: {}", records.records_ref().len());
        }
        "load_parcels" => {
            if let Some(path) = cli.source {
                let records = import::CityTaxlots::from_csv(path)?;
                info!("Records: {}", records.records_ref().len());
                let mail = data::MailingList::try_from(&records)?;
                info!("Records processed: {}", mail.records_ref().len());
                let mut mail = data::MailingListExport::from(&mail);
                mail.sort_by_key("properties");
                let mail: Vec<data::MailingListExportItem> =
                    mail.records_ref().iter().rev().cloned().collect();
                let mut mail = data::MailingListExport::new(mail);
                mail.to_csv("c:/users/erose/documents/mailing_list.csv")?;
            }
        }
        _ => {}
    }

    Ok(())
}
