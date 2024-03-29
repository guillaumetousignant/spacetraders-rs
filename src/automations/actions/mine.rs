use crate::automations::utilities::wait_until;
use crate::queries;
use crate::queries::Query;
use crate::queries::StatusError;
use crate::spacetraders_api::errors;
use crate::spacetraders_api::responses::Cargo;
use log::info;
use reqwest::Client;
use tokio::sync::mpsc::Sender;

pub async fn mine(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
    ship_symbol: &str,
) -> Result<Cargo, Box<dyn std::error::Error + Send + Sync>> {
    let _ = queries::orbit(client, sender, token, ship_symbol).await?;
    let extract_response = queries::extract(client, sender, token, ship_symbol).await;

    let mut extract_response = if let Err(e) = extract_response {
        if e.is::<StatusError>() {
            let e = e.downcast::<StatusError>()?;
            let error = serde_json::from_str::<errors::CooldownResponse>(&e.message)?.error;

            if error.code == 4000 {
                let cooldown = error.data.cooldown.expiration;
                info!("Ship {ship_symbol} extraction is on cooldown until {cooldown}");
                wait_until(cooldown).await?;
                queries::extract(client, sender, token, ship_symbol).await?
            } else {
                return Err(e);
            }
        } else {
            return Err(e);
        }
    } else {
        extract_response?
    };

    info!(
        "Ship {ship_symbol} extracted {} units of {}",
        extract_response.extraction.yield_data.units, extract_response.extraction.yield_data.symbol
    );

    while extract_response.cargo.units < extract_response.cargo.capacity {
        wait_until(extract_response.cooldown.expiration).await?;
        extract_response = queries::extract(client, sender, token, ship_symbol).await?;

        info!(
            "Ship {ship_symbol} extracted {} units of {}",
            extract_response.extraction.yield_data.units,
            extract_response.extraction.yield_data.symbol
        );
    }

    Ok(extract_response.cargo)
}
