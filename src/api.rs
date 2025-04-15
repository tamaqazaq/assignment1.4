use serde::Deserialize;
use reqwest::Client;

#[derive(Debug, Deserialize, serde::Serialize)]
pub struct CoinInfo {
    pub id: String,
    pub name: String,
    pub description: String,
    pub homepage: String,
    pub image: String,
    pub last_updated: String,
}

#[derive(Debug, Deserialize)]
struct CoinSearchResponse {
    coins: Vec<CoinEntry>,
}

#[derive(Debug, Deserialize)]
struct CoinEntry {
    id: String,
    name: String,
    symbol: String,
}

#[derive(Debug, Deserialize)]
struct CoinDetails {
    id: String,
    name: String,
    description: Description,
    image: Image,
    links: Links,
    last_updated: String,
}

#[derive(Debug, Deserialize)]
struct Description {
    en: String,
}

#[derive(Debug, Deserialize)]
struct Image {
    thumb: String,
}

#[derive(Debug, Deserialize)]
struct Links {
    homepage: Vec<String>,
}

pub async fn fetch_coin_info(symbol: &str) -> Result<Option<CoinInfo>, reqwest::Error> {
    let client = Client::new();
    
    let search_url = format!("https://api.coingecko.com/api/v3/search?query={}", symbol);
    let res = client.get(&search_url).send().await?;
    let search_data: CoinSearchResponse = res.json().await?;

    if let Some(coin) = search_data.coins.iter().find(|c| c.symbol.eq_ignore_ascii_case(symbol)) {
        let detail_url = format!("https://api.coingecko.com/api/v3/coins/{}", coin.id);
        let res = client.get(&detail_url).send().await?;
        let detail_data: CoinDetails = res.json().await?;

        Ok(Some(CoinInfo {
            id: detail_data.id,
            name: detail_data.name,
            description: detail_data.description.en,
            homepage: detail_data.links.homepage[0].clone(),
            image: detail_data.image.thumb,
            last_updated: detail_data.last_updated,
        }))
    } else {
        Ok(None)
    }
}
