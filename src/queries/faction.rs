use super::get_rate_limit;
use super::StatusError;
use super::TooManyRetriesError;
use super::N_RETRIES;
use super::URL;
use crate::spacetraders_api::responses::Faction;
use reqwest::Client;
use reqwest::StatusCode;

pub async fn faction(
    client: &Client,
    faction: &str,
) -> Result<Faction, Box<dyn std::error::Error>> {
    for _ in 0..N_RETRIES {
        let response = client
            .get(format!("{URL}/factions/{faction}"))
            .send()
            .await?;
        match response.status() {
            StatusCode::OK => return Ok(response.json().await?),
            StatusCode::TOO_MANY_REQUESTS => {
                let duration = get_rate_limit(&response)?;
                tokio::time::sleep(duration).await;
            }
            _ => {
                return Err(StatusError {
                    status: response.status(),
                    url: response.url().clone(),
                }
                .into())
            }
        }
    }

    Err(TooManyRetriesError.into())
}
