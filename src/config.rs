use super::migrator::account_map::AccountMap;
use crate::fire_fly::FireFly;
use crate::up_bank::UpBank;
use color_eyre::eyre::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::vec;
use tracing::{debug, info};

fn default_import_hour_period() -> i64 {
    12
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename = "up_token")]
    pub up_token: String,
    #[serde(rename = "firefly_token")]
    pub firefly_token: String,
    #[serde(rename = "firefly_url")]
    pub firefly_url: String,
    #[serde(default = "default_import_hour_period")]
    pub import_hour_period: i64,
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let config: Config = serde_yaml::from_str(&contents)?;
        Ok(config)
    }

    pub async fn get_accounts(
        &self,
        up_bank: &UpBank,
        fire_fly: &FireFly,
    ) -> Result<Vec<AccountMap>> {
        let mut account_vector: Vec<AccountMap> = vec![];
        for up_account in &up_bank.accounts {
            let up_account_id = up_account.id.as_str();

            // Attempt to find unique up bank id in firefly
            match fire_fly
                .get_account_by_account_number(up_account_id)
                .await?
            {
                Some(fire_fly_account) => {
                    let account_id = fire_fly_account.id;
                    debug!(
                        "Found Up ID {} linked to Firefly ID {}",
                        up_account_id, account_id
                    );
                    let new_account = AccountMap::create(up_account_id.to_string(), account_id);
                    account_vector.push(new_account);
                }
                None => info!(
                    "Up Bank account ({}) was not found in firefly, ensure that this is expected",
                    up_account_id
                ),
            }
        }
        Ok(account_vector)
    }
}
