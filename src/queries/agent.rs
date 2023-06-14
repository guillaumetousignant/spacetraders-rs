use super::Query;
use super::URL;
use crate::spacetraders_api::responses::Agent;
use crate::spacetraders_api::responses::AgentResponse;
use reqwest::Client;
use tokio::sync::mpsc::Sender;
use tokio::sync::oneshot;

pub async fn agent(
    client: &Client,
    sender: &Sender<Query>,
    token: &str,
) -> Result<Agent, Box<dyn std::error::Error + Send + Sync>> {
    let request = client.get(format!("{URL}/my/agent")).bearer_auth(token);
    let (resp_tx, resp_rx) = oneshot::channel();
    sender
        .send(Query {
            request,
            response: resp_tx,
        })
        .await?;
    Ok(resp_rx.await??.json::<AgentResponse>().await?.data)
}
