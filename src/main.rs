pub mod config;
pub mod fire_fly;
pub mod migrator;
pub mod operation;
pub mod up_bank;
use clap::{Parser, ValueEnum};
use color_eyre::eyre::Result;
use tracing::info;

use config::Config;

#[derive(Parser, Debug, Clone, ValueEnum)]
enum Action {
    Import,
    GetAccountInfo,
    PeriodicImport,
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    /// Name of the person to greet
    #[clap(env, short, long, value_parser)]
    start_date: Option<String>,
    #[clap(env, short, long, value_parser)]
    end_date: Option<String>,
    #[clap(env, short, long, value_parser)]
    date_range: Option<i64>,
    #[clap(env, value_enum, default_value_t = Action::Import)]
    action: Action,
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    tracing_subscriber::fmt::init();

    info!("Starting services");
    let config = Config::load("./config/settings.yaml")?;
    info!("Loaded config file");
    let args = Args::parse();
    info!("Parsed arguments and updated config");

    let mut up_bank = up_bank::UpBank::create(config.up_token.clone())?;
    let fire_fly = fire_fly::FireFly::create(
        config.firefly_token.clone(),
        config.firefly_url.clone(),
    )?;

    info!("Firefly and Up API initialised");
    up_bank.ping().await?;
    up_bank.populate_data().await?;
    info!("Up connected and information gathered");

    match args.action {
        Action::Import => operation::import_data(&args, &up_bank, &fire_fly, &config).await?,
        Action::GetAccountInfo => operation::print_out_up_bank_account_info(up_bank)?,
        Action::PeriodicImport => {
            operation::periodic_import(args, up_bank, fire_fly, config).await?
        }
    }

    Ok(())
}
