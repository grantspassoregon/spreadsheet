use clap::Parser;
use spreadsheet::{data, error, import};
use tracing::{info, trace};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: String,
    #[arg(
        short = 's',
        long,
        help = "Source of file.",
        default_missing_value = "None"
    )]
    source: Option<String>,
}

fn main() -> Result<(), error::Error> {
    if let Ok(()) = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .try_init()
    {};
    trace!("Subscriber initialized.");
    let cli = Cli::parse();

    match cli.command.as_str() {
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
            info!("Records: {}", records.records().len());
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
                mail.to_csv("p:/cbd_multistory_mailing_list.csv")?;
            }
        }
        _ => {}
    }

    Ok(())
}
