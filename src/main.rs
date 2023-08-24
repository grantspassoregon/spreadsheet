use spreadsheet::{error, import};
use clap::Parser;
use tracing::{info, trace};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short = 'c', long, help = "Command to execute.")]
    command: String,
    #[arg(short = 's', long, help = "Source of file.", default_missing_value = "None")]
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
            // let csv = std::env::var("BEA_CAINC5N_CSV")?;

            info!("Reading raw csv file.");
            let records = import::BeaDataRaw::from_csv(raw)?;
            info!("Converting raw csv data.");
            let mut records = import::BeaData::try_from(records)?;
            if let Some(path) = cli.source {
                records.to_csv(path)?;
            }
            info!("Records: {}", records.records().len());
        },
        _ => {},
    }

    Ok(())
}
