use crate::queries;

pub async fn status() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let status_response = queries::status(&client).await?;
    println!("{:#?}", status_response);

    Ok(())
}